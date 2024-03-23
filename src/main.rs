
use bytebuffer::ByteBuffer;

mod other_maps;
use crate::other_maps::{AIF_MAP, COMMAND_MAP, ERROR_MAP, SCREEN_TYPE_MAP, SOURCE_MAP, TYPE_MAP, VTC_RESOLUTION_MAP};

use telnet::{Event, Telnet};
use telnet::{Action, TelnetOption};
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self};

mod modes_display;

use crate::modes_display::MODE_DISPLAY_MAP;

// use text_io::read;

mod modes_set;
use modes_set::MODE_SET_MAP;
use modes_set::INVERSE_MODE_SET_MAP;

// use crate::modes_display;

fn main() -> ! {
    println!("Hello, world!");
    // let host = "ptt.cc";
    let host = "192.168.86.32";
    // let host = "localhost";
    let mut connection = Telnet::connect((host, 23), 256)
            .expect("Couldn't connect to the server...");

    let _res = connection.negotiate(&Action::Will, TelnetOption::Echo);

    /*** Send ON at start:
    let buffer: [u8; 4] = [80, 79, 13, 10]; // ON
    // let buffer: [u8; 4] = [80, 70, 13, 10]; // OFF    
    let s = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("S is {}", s);
    connection.write(&buffer).expect("Write Error");
    ***/

    let mut line_number: i32 = 0;

    let (transmitter, receiver) = mpsc::channel::<String>();
    let _handle: thread::JoinHandle<()> = thread::spawn(move || { user_input_loop(transmitter); });

    loop {
        // println!("Running main loop");
        match receiver.try_recv() {
            Err(_e) => {
                // println!("Receive error {}", _e);
            },
            Ok(msg) => {                
                // println!("Got {}", msg);
                let mut bytebuffer = ByteBuffer::new();
                bytebuffer.write_bytes(msg.as_bytes());
                let tail: [u8; 2] = [13, 10]; // b"\r\n"
                bytebuffer.write_bytes(&tail);
                let _write = connection.write(bytebuffer.as_bytes()).expect("Write Error");
                // println!("Wrote bytebuffer");
            }
        }
        // println!("try_recv done");
        // busy wait:
        // let event = connection.read_nonblocking().expect("Read error");
        // too slow:
        // let event = connection.read().expect("Read error");
        let timeout:u32 = (1_000_000 * 1_000) / 20;
        let event = connection.read_timeout(Duration::new(0,timeout)).expect("Read error");
        // println!("Read done");
        if let Event::Data(buffer) = event {
            line_number += 1;
            // Debug: print the data buffer
            // println!("Got event {:?}", buffer);
            let r = String::from_utf8_lossy(&buffer);
            // println!("Line {}: {}", line_number, r);
            let srec: &str = &r.to_string();
            for l in srec.split_ascii_whitespace() {
                // srec = remove_suffix(srec, "\r\n");
                process_status_line(l.to_string());
            }
             // println!("Received: {}", std::str::from_utf8(&buffer[..]).unwrap_or("Bad utf-8 bytes"));
        }
    }
}

fn process_status_line(srec:String) {
    if srec.starts_with("E0") {
        match ERROR_MAP.get(&srec) {            
            Some(s) => { println!("{}", s); },
            None => { println!("Unknown error code {}", srec); }
        };
        return;
    }
    if srec.starts_with("FL") {
        if !decode_fl(&srec[2..]) {
            println!("Couldn't decode FL {}", srec);
        };
        return;
    }
    match decode_tone(&srec) {
        Some(tonestr) => {
            println!("{}", tonestr);
            return;
        }
        None => {}
    };
    match decode_geh(&srec) {
        Some(gehstr) => {
            println!("{}", gehstr);
            return;
        }
        None => {}
    }
    if srec.starts_with("AST") {
        decode_ast(&srec[3..]);
        return;
    }
    if srec.starts_with("VTC") {
        let s = decode_vtc(&srec[3..]);
        println!("{}", s);
        return;
    }
    if srec.starts_with("SR") {
        let code = &srec[2..];
        match MODE_SET_MAP.get(code) {
            Some(v) => {
                println!("mode is {} ({})", v, srec);
            },
            None => {
                println!("unknown SR mode {}", srec);
            }
        }
        return;
    }
    if srec.starts_with("LM") {
        let key = &srec[2..];
        let ms = match MODE_DISPLAY_MAP.get(key) {
                Some(v) => v,
                None => "unknown"
            };
        println!("Listening mode is {} ({})", ms, srec);
        return;
    }
    println!("Unknown status line {}", srec);
}

fn decode_ast(s: &str) -> bool {
    println!("Audio input signal: {}", decode_ais(&s[0..2]));
    println!("Audio input frequency: {}", decode_aif(&s[2..4]));
    return false;
}

fn decode_ais(s:&str) -> &str {
    if "00" <= s && s <= "02" {
        return "ANALOG";
    }
    if s=="03" || s=="04" {
        return "PCM";
    }
    if s=="05" {
        return "DOLBY DIGITAL";
    }
    if s=="06" {
        return "DTS";
    }
    if s=="07" {
        return "DTS-ES Matrix";
    }
    if s=="08" {
        return "DTS-ES Discrete";
    }
    if s=="09" {
        return "DTS 96/24";
    }
    if s=="10" {
        return "DTS 96/24 ES Matrix";
    }
    if s=="11" {
        return "DTS 96/24 ES Discrete";
    }
    if s=="12" {
        return "MPEG-2 AAC";
    }
    if s=="13" {
        return "WMA9 Pro";
    }
    if s=="14" {
        return "DSD->PCM";
    }
    if s=="15" {
        return "HDMI THROUGH";
    }
    if s=="16" {
        return "DOLBY DIGITAL PLUS";
    }
    if s=="17" {
        return "DOLBY TrueHD";
    }
    if s=="18" {
        return "DTS EXPRESS";
    }
    if s=="19" {
        return "DTS-HD Master Audio";
    }
    if "20" <= s && s <= "26" {
        return "DTS-HD High Resolution";
    }
    if s=="27" {
        return "DTS-HD Master Audio";
    }
    return "unknown ais";
}

fn decode_aif(s:&str) -> &str {
    match AIF_MAP.get(s) {
        Some(v) => return v,
        None => return "unknown"
    }
}

fn decode_vtc(s: &str) -> &str {
    match VTC_RESOLUTION_MAP.get(s) {
        Some(v) => v,
        None => "unknown VTC resolution"
    }
}


fn decode_tone(s: &str) -> Option<String> {
    if s.starts_with("TR") {
        let dbs = db_level(s);
        let fs = format!("treble at {}", dbs);
        return Some(fs);
    }
    if s.starts_with("BA") {
        let dbs = db_level(s);
        let fs = format!("bass at {}", dbs);
        return Some(fs);
    }
    if s == "TO0" {
        return Some("tone off".to_string());
    }
    if s == "TO1" {
        return Some("tone on".to_string());
    }
    return None;
}

fn decode_geh(s: &str) -> Option<String> {
    if s.starts_with("GDH") {
        let sbytes = &s.to_string()[3..];
        let toslice = sbytes.to_string();
        let fs = format!("items {} to {} of total {} ",
             &toslice[0..5], &toslice[5..10], &toslice[10..]);
        return Some(fs);
    }
    if s.starts_with("GBH") {
        let toslice = s.to_string();
        let fs = format!("max list number: {}", &toslice[2..]);
        return Some(fs);
    }
    if s.starts_with("GCH") {
        let key = &s.to_string()[3..5];
        let val = match SCREEN_TYPE_MAP.get(key) {
            Some(sv) => sv,
            None => "unknown"
        };
        let fs = format!("{} - {}", val, s);
        return Some(fs);
    }
    if s.starts_with("GHH") {
        let key = &s.to_string()[2..];
        let val = match SOURCE_MAP.get(key) {
            Some(sv) => sv,
            None => "unknown"
        };
        let fs = format!("source: {}", val);
        return Some(fs);
    }
    if ! s.starts_with("GEH") { return None; }
    let suf = &s[3..];
    let key = &suf[3..5];
    let binding = format!("unknown ({})", key).to_string();
    let typeval: &str = match TYPE_MAP.get(key) {
        Some(sv) => sv,
        None => &binding
    };
    let info = &suf[5..];
    return Some(format!("{} : {}", typeval, info));
}


fn db_level(s: &str) -> String {
    let stripped= &s.to_string()[2..]; // just need to cut first two
    let my_int_option = stripped.parse::<i32>();
    match my_int_option {
        Ok(my_int) => {
            let db = 6 - my_int;
            return format!("{mydb}dB", mydb=db);
        }
        Err(_) => {
            return format!("Error parsing DB level, string was {}", s);
        }
    }
}

fn remove_suffix<'x>(s: &'x str, suffix: &str) -> &'x str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s
    }
}

fn user_input_loop(transmitter: std::sync::mpsc::Sender<String>) -> bool {

    loop {
        
        print!("Command: ");
        let _flush = std::io::stdout().lock().flush();
        let mut line = String::new();
        let _r = std::io::stdin().read_line(&mut line); // including '\n'
        // let mut line: String = read!("{}\n");
        line = line.trim().to_string();
        // println!("Got user line {}", line);
        if line == "" { continue; }
        if line == "quit" || line == "exit" {
            std::process::exit(0);
        }
        if line == "status" {
            let _ = transmitter.send("?BA".to_owned());
            let _ = transmitter.send("?TR".to_owned());
            let _ = transmitter.send("?TO".to_owned());
            let _ = transmitter.send("?L".to_owned());
            let _ = transmitter.send("?AST".to_owned());
            // # send(tn, "?VTC") # not very interesting if always AUTO
            continue;
        }
        let v: Vec<&str> = line.split(" ").collect();
        let base = v[0];
        // let arg1: &str = if v.len() > 1 { v[1] } else {""};
        
        if base == "help" || base == "?" {
            if v.len() > 1 {
                if v[1] == "mode" {
                    print_mode_help();
                    continue
                }
            }
            else {
                print_help();
            }
            continue;
        }
        if base == "select" {
            println!("TODO");
            continue;
        }
        if base == "display" {
            println!("TODO");
            continue;
        }
        if base == "mode" {
            println!("TODO");
            continue;
        }
        match COMMAND_MAP.get(&line) {
            Some(s) => {
                let _res = transmitter.send(s.to_string());
                continue;
            },
            None => {}
        }
        let numoption = line.parse::<i32>();
        match numoption {
            Ok(i) => {
                println!("Got integer {}", i);
                continue;
            }
            Err(_) => {}
        }
        println!("Sending raw command {}", line);
        let _send = transmitter.send(line);
        println!("Sent user line");
    } 
}

fn print_mode_help() -> bool {
    println!("mode [mode]\tfor one of:");
    for k in INVERSE_MODE_SET_MAP.keys() {
        println!("{}", k);
    };
    return true;
}

// returns true if successful decoding:
// TODO: return string, add unit tests.
fn decode_fl(s: &str) -> bool {

    let v: Vec<u8> = s.as_bytes().to_vec();
    
    let mut urlbytes = ByteBuffer::new();
    let ampersand: [u8; 1] = [37]; // b"%"
    let mut i = 2;
    while i < v.len() {
          urlbytes.write_bytes(&ampersand);
          urlbytes.write_bytes(&[v[i], v[i+1]]);
          i += 2;
    }
    // now need to do equivalent of urllib.parse.unquote
    // TODO: why the extra "%" ?
    let binary = urlencoding::decode_binary(urlbytes.as_bytes());
    let decoded = String::from_utf8_lossy(&binary);
    println!("{}", decoded);
    return true;
}

fn print_help() {
    println!("Help: ------------ ");
    for k in COMMAND_MAP.keys() {
        println!("{}", k);
    }
    println!("quit or exit to finish");
}
