
use bytebuffer::ByteBuffer;

pub mod other_maps;
use crate::other_maps::{InputMap, AIF_MAP, CHANNEL_DECODE_MAP, COMMAND_MAP, ERROR_MAP, SCREEN_TYPE_MAP, SOURCE_MAP, TYPE_MAP, VTC_RESOLUTION_MAP};
use crate::other_maps::{DEFAULT_INPUT_MAP, INPUT_MAP, REVERSE_INPUT_MAP};

use telnet::{Event, Telnet};
use telnet::{Action, TelnetOption};

use std::borrow::Cow;
use lazy_static::lazy_static;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self};

mod modes_display;

use crate::modes_display::MODE_DISPLAY_MAP;

mod modes_set;
use modes_set::{get_modes_with_prefix, MODE_SET_MAP};
use modes_set::INVERSE_MODE_SET_MAP;

fn main() -> ! {
    let host = "192.168.86.32";
    println!("Connecting to AVR at address {}", host);
    let mut connection = Telnet::connect((host, 23), 256)
            .expect("Couldn't connect to the host...");

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
    let mut leftover = "".to_string();

    let (transmitter, receiver) = mpsc::channel::<String>();
    let _handle: thread::JoinHandle<()> = thread::spawn(move || { user_input_loop(transmitter); });

    let mut debug = false;

    loop { // receiving from the user channel

        match receiver.try_recv() {
            Err(_e) => {
                // println!("Receive error {}", _e);
            },
            Ok(msg) => {
                if debug {
                    println!("Got from user side: {}", msg);
                }
                if msg == "debug" {
                    debug = ! debug;
                    println!("Debug is now {}", debug);
                    continue;
                }

                let mut bytebuffer = ByteBuffer::new();
                bytebuffer.write_bytes(msg.as_bytes());
                let tail: [u8; 2] = [13, 10]; // b"\r\n"
                bytebuffer.write_bytes(&tail);
                let _write = connection.write(bytebuffer.as_bytes()).expect("Write Error");
                // println!("Wrote bytebuffer");
            }
        }
        // FIXME: some lines can be split between one event and the next.
        let timeout:u32 = (1_000_000 * 1_000) / 80;
        let event = connection.read_timeout(Duration::new(0,timeout)).expect("Read error");

        if let Event::Data(buffer) = event {
            line_number += 1;

            if debug {
                println!("Got event from AVR: {:?}", buffer);
            }
            let r = String::from_utf8_lossy(&buffer);
            if debug {
                println!("Line {}: {}", line_number, r);
            }
            let srec = r.to_string();
            let mut v: Vec<&str> = srec.split("\r\n").collect();

            if v.len() == 0 {
                continue;
            }
            
            let f = leftover.to_owned() + v.first().expect("should never happen");
            if leftover.len() > 0 {
                v.remove(0);
                for seg in f.split("\r\n") {
                    v.insert(0, seg);
                }
                println!("Adding leftover {}, got {}\n", leftover, f);
                if leftover.ends_with('\r') {
                    println!("!Weird leftover!");
                }
            }
        
            if !srec.ends_with("\r\n") && v.len() > 0 {
                leftover = v.pop().expect("should never happen").to_string();
            }
            else {
                leftover = "".to_string();
            }
            for l in v {
                let s = process_status_line(l.to_string());
                if s.len() > 0 {
                    println!("{}", s);
                }
            }
            // println!("Received: {}", std::str::from_utf8(&buffer[..]).unwrap_or("Bad utf-8 bytes"));
        }
    }
}

fn learn_input_from(s: &str) {
    let id = &s[0..2];
    let name = &s[3..];
    match INPUT_MAP.lock().unwrap().get(id) {
        Some(s) => if s != name {
            println!("Updating source {} to {}", id, name);
        },
        None => { }
    }
    INPUT_MAP.lock().unwrap().insert(id.to_owned(), name.to_owned());
}

// Processes a status line (string) received from the AVR, returning a human-readable string
// with the information it contains:
fn process_status_line(srec:String) -> String {

    if srec.len() == 0 {
        return srec;
    }

    match ERROR_MAP.get(&srec) {
            Some(s) => { return s.to_string(); }
            None => {}
    }

    if srec.starts_with("RGB") {
        learn_input_from(&srec[3..]);
        return "".to_string();
    }

    if srec.starts_with("FL") {
        return decode_fl(&srec[2..]).to_owned();
    }

    match decode_tone(&srec) {
        Some(tonestr) => {
            return tonestr;
        }
        None => {}
    }

    match decode_geh(&srec) {
        Some(gehstr) => {
            return gehstr;
        }
        None => {}
    }

    if srec == "PWR0" {
        return "Power is ON".to_owned();
    }
    if srec == "PWR1" {
        return "Power is OFF".to_owned();
    }

    if srec.starts_with("FN") {
        let is = match INPUT_MAP.lock().unwrap().get(&srec[2..]) {
            Some(s) => s.to_string(),
            None => format!("unknown ({})", srec)
        };
        return format!("Input is {}", is);
    }
    if srec.starts_with("ATW") {
        let fl:&str = if srec == "ATW1" { "on" } else  { "off" };
        return format!("loudness is {}", fl);
    }

    if srec.starts_with("ATC") {
        let fl:&str = if srec == "ATC1" { "on" } else  { "off" };
        return format!("eq is {}", fl);
    }

    if srec.starts_with("ATD") {
        let fl:&str = if srec == "ATD1" { "on" } else { "off" };
        return format!("standing wave is {}", fl);
    }

    if srec.starts_with("ATE") {
        let num = &srec[3..];
        if "00" <= num && num <= "16" {
            println!("Phase control: {} ms", num);
        }
        else if num == "97" {
                println!("Phase control: AUTO");
        }
        else if num == "98" {
            println!("Phase control: UP");
        }
        else if num == "99" {
            println!("Phase control: DOWN");
        }
        else {
            println!("Phase control: unknown ({})", srec);
        };
    }

    // translate_mode is below
    if srec.starts_with("AST") {
        return decode_ast(&srec[3..]);
    }
    if srec.starts_with("VTC") {
        return decode_vtc(&srec[3..]);
    }
    if srec.starts_with("SR") {
        let code = &srec[2..];
        return match MODE_SET_MAP.get(code) {
            Some(v) => {
                format!("mode is {} ({})", v, srec)
            },
            None => {
                format!("unknown SR mode {}", srec)
            }
        };
    }
    // translate_mode
    if srec.starts_with("LM") {
        let key = &srec[2..];
        let ms = match MODE_DISPLAY_MAP.get(key) {
                Some(v) => v,
                None => "unknown"
            };
        return format!("Listening mode is {} ({})", ms, srec);
    }
    if srec.starts_with("VOL") {
        return "".to_string();
    }
    return format!("Unknown status line '{}'", srec);
}


fn decode_ast(s: &str) -> String {
    let s1 = format!("Audio input signal: {}", decode_ais(&s[0..2]));
    let s2 = format!("Audio input frequency: {}", decode_aif(&s[2..4]));
    let binding = "-".to_string() + s;
    let sc:Vec<char> = binding.chars().collect();
    let mut s3 = s1 + "\n" + &s2;
    println!("raw: {}", s);
    s3 += "\nInput Channels: ";
    for i in &CHANNEL_DECODE_MAP {
        let idx:usize = (*(i.0)).try_into().unwrap();
        if idx < sc.len() && sc[idx] == '1' {
            s3 += &((i.1).to_string() + ", ");
        }
    }
    s3 += "\nOutput Channels: ";
    for i in &CHANNEL_DECODE_MAP {
        let idx:usize = (21i8 + i.0).try_into().unwrap();
        if idx < sc.len() &&  sc[idx] == '1' {
            s3 += &((i.1).to_string() + ", ");
        }
    }

    return s3;
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

fn decode_vtc(s: &str) -> String {
    match VTC_RESOLUTION_MAP.get(s) {
        Some(v) => v.to_string(),
        None => "unknown VTC resolution".to_owned()
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

lazy_static! {
    pub(crate)
    static ref CONFIG_FILE_PATH: Cow<'static, str> = shellexpand::tilde("~/pioneer_avr_sources.json");
}


fn user_input_loop(transmitter: std::sync::mpsc::Sender<String>) -> bool {
    let mut debug = false;

    // let mut input_map:HashMap<String, String>;

    let filename:&str = &CONFIG_FILE_PATH;
    let path = shellexpand::tilde(filename);

    let mopt  = InputMap::read_from_file(&path.clone().into_owned());
    
    match mopt {
        Ok(m) =>  {
            for x in &m {
                INPUT_MAP.lock().unwrap().insert(x.0.to_string(), x.1.to_string());
            }
        }
        Err(e) => {
            println!("Got error reading {}: {}", path, e);
            for x in &DEFAULT_INPUT_MAP {
                INPUT_MAP.lock().unwrap().insert(x.0.to_string(), x.1.to_string());
            }
        }
    }
    
    // let mut reverse_input_map = HashMap::new();
    for x in INPUT_MAP.lock().unwrap().iter() {
        REVERSE_INPUT_MAP.lock().unwrap().insert(x.1.to_ascii_lowercase(), x.0.to_owned()+"FN");
    }

    loop {
        print!("Command: ");
        let _flush = std::io::stdout().lock().flush();
        let mut line = String::new();
        let _r = std::io::stdin().read_line(&mut line); // including '\n'
        // let mut line: String = read!("{}\n");
        line = line.trim().to_string().to_lowercase();
        if debug {
            println!("Got user line '{}'", line);
        }
        if line == "" { continue; }
        if line == "quit" || line == "exit" {
            std::process::exit(0);
        }
        if line == "status" {
            for c in ["?P", "?F", "?BA", "?TR", "?TO", "?L", "?AST"] {
                let _ = transmitter.send(c.to_owned());
            }
            // # send(tn, "?VTC") # not very interesting if always AUTO
            continue;
        }
        if line == "learn" {
            for i in 0..60 {
                let mystr = format!("?RGB{:02}", i);
                // println!("{}", mystr);
                let _ = transmitter.send(mystr);
            }
            continue;
        }
        if line == "save" {
            let _ = InputMap::save_input_map(&CONFIG_FILE_PATH);
            continue;
        }
        let v: Vec<&str> = line.split(" ").collect();
        let base = v[0];
        // let arg1: &str = if v.len() > 1 { v[1] } else {""};

        if base == "help" || base == "?" {
            if v.len() > 1 {
                if v[1] == "mode" || v[1] == "modes" {
                    print_mode_help();
                    continue
                }
            }
            else {
                print_help();
            }
            continue;
        }
        if line == "modes" {
            print_mode_help();
            continue;
        }
        if base == "debug" {
            debug = ! debug;
            let _ = transmitter.send("debug".to_string());
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
        if base == "mode" && v.len() > 1 {
            // println!("Attempting to change mode");
            match change_mode(v) {
                Some(code) => {
                    let _res = transmitter.send(code);
                },
                None => {}
            };
            continue;
        }
        match COMMAND_MAP.get(&line) {
            Some(s) => {
                let cmd = s.to_string();
                println!("Sending command {}", cmd);
                let _res = transmitter.send(cmd);
                continue;
            },
            None => {}
        }
        match REVERSE_INPUT_MAP.lock().unwrap().get(&line) {
            Some(s) => {
                let cmd = s.to_string();
                println!("Sending source change command {}", cmd);
                let _ = transmitter.send(cmd);
                continue;
            },
            None => {}
        }
        let numoption = line.parse::<i32>();
        match numoption {
            Ok(mut i) => {
                println!("Got integer {}", i);
                if i > 0 {
                    i = i32::min(i, 10);
                    println!("Volume up {}", i);
                    for _ in 0..i {
                        let _ = transmitter.send("VU".to_string());
                    }
                }
                else {
                    i = i32::abs(i32::max(i, -30));
                    println!("Volume down {}", i);
                    for _ in 0..i {
                        let _ = transmitter.send("VD".to_string());
                    }
                }
                continue;
            }
            Err(_) => {}
        }
        if line == "sources" || line == "inputs" {
            for x in REVERSE_INPUT_MAP.lock().unwrap().iter() {
                println!("{} ({})", x.0, x.1);
            }
            continue;
        }
        println!("Sending raw command {}", line);
        let _send = transmitter.send(line);
        if debug {
            println!("Sent user line");
        }
    }
}


fn print_mode_help() {
    println!("mode [mode]\tfor one of:");
    for k in INVERSE_MODE_SET_MAP.keys() {
        println!("{}", k);
    };
}


// TODO: add unit tests.
fn decode_fl(s: &str) -> String {

    let v: Vec<u8> = s.as_bytes().to_vec();

    let mut urlbytes = ByteBuffer::new();
    let ampersand: [u8; 1] = [37]; // b"%"
    let mut i = 2;
    while i < v.len() {
          urlbytes.write_bytes(&ampersand);
          urlbytes.write_bytes(&[v[i], v[i+1]]);
          i += 2;
    }
    // now need to do equivalent of urllib.parse.unquote:
    let binary = urlencoding::decode_binary(urlbytes.as_bytes());
    let decoded = String::from_utf8_lossy(&binary);
    return decoded.to_string();
}


fn print_help() {
    println!("Help: ------------ ");
    for k in COMMAND_MAP.keys() {
        println!("{}", k);
    }
    println!("quit or exit to finish");
}


fn change_mode(vec: Vec<&str>) -> Option<String> {
    if vec.len() < 2 {
        return None;
    }
    let modestring: String = (vec[1..]).join(" ").to_lowercase();
    if modestring == "help" {
        print_mode_help();
        return None;
    }
    let mset = get_modes_with_prefix(&modestring);
    // println!("get_modes_with_prefix got {}", mset.len());
    if mset.len() == 0 {
        println!("Unknown mode {}", modestring);
        return None;
    }
    if mset.len() > 1 {
        println!("Which one do you mean? Options are:");
        for s in mset {
            println!("{}", s);
        }
        return None;
    }
    let mode = mset.iter().next().unwrap().to_owned().to_owned();
    let m = INVERSE_MODE_SET_MAP.get(&mode).unwrap();
    println!("Trying to change to modestring {} ({})", mode, m.to_owned());
    return Some(m.to_string());
}

#[cfg(test)]
mod tests {

    #[test]
    fn my_test() {
        println!("testing!");
    }

}
