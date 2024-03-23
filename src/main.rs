
use bytebuffer::ByteBuffer;
pub(crate)

use telnet::{Event, Telnet};
use telnet::{Action, TelnetOption};
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self};

use crate::other_maps::COMMAND_MAP;

// use text_io::read;

mod modes_display;
mod modes_set;
pub mod other_maps;

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
        let v = other_maps::ERROR_MAP.get(&srec);
            match v {
                Some(s) => { println!("{}", s); },
                None => { println!("Unknown error code {}", srec); }
            }
        return;
    }
    if srec.starts_with("FL") {
        if !decode(srec.to_owned()) { // TODO: decode buffer directly?
            println!("Couldn't decode {}", srec);
           return;
        };
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
    // TODO: more cases here
    return None;
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
                    // print_mode_help();
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

// returns true if successful decoding:
// TODO: return string, add unit tests.
fn decode(s: String) -> bool {
    // print!("Original string is {}", s);
    if ! s.starts_with("FL") {
        // println!("String does not start with FL");
        return false;
    }
    let s = match s.strip_prefix("FL") {
        Some(v) => v,
        None => return false,
    };
    // remove first two? TODO

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
