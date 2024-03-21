
use bytebuffer::ByteBuffer;
pub(crate)

use telnet::{Event, Telnet};
use telnet::{Action, TelnetOption};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self};

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

    // println!("Connection: {}", connection);

    // let my_string: &str = "PO\r\n";
    // let buffer : [u8] = my_string.as_bytes();

    // let buffer: [u8; 4] = [83, 76, 77, 84];
    let buffer: [u8; 4] = [80, 79, 13, 10]; // ON
    // let buffer: [u8; 4] = [80, 70, 13, 10]; // OFF
    
    let s = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("S is {}", s);

    connection.write(&buffer).expect("Write Error");

    let mut line = 0;

    let (transmitter, receiver) = mpsc::channel::<String>();

    let _handle: thread::JoinHandle<()> = thread::spawn(move || { user_input_loop(transmitter); });

    loop {
        println!("Running main loop");
        match receiver.try_recv() {
            Err(e) => {
                println!("Receive error {}", e);
            },
            Ok(msg) => {                
                println!("Got {}", msg);
                let mut bytebuffer = ByteBuffer::new();
                bytebuffer.write_bytes(msg.as_bytes());
                let tail: [u8; 2] = [13, 10]; // b"\r\n"
                bytebuffer.write_bytes(&tail);
                let _write = connection.write(bytebuffer.as_bytes()).expect("Write Error");
                println!("Wrote bytebuffer");
            }
        }
        // println!("try_recv done");
        let event = connection.read_timeout(Duration::new(1,0)).expect("Read error");
        // println!("Read done");
        if let Event::Data(buffer) = event {
            line += 1;
            // Debug: print the data buffer
            // println!("Got event {:?}", buffer);
            let r = String::from_utf8_lossy(&buffer);
            println!("Line {}: {}", line, r);
            let mut srec: &str = &r.to_string();
            srec = remove_suffix(srec, "\r\n");
            if srec.starts_with("E0") {
                let v = other_maps::ERROR_MAP.get(srec);
                match v {
                    Some(s) => { println!("{}", s); },
                    None => { println!("Unknown error code {}", srec); }
                }
            continue;
            };
            if srec.starts_with("FL") {
                if ! decode(srec.to_owned()) { // TODO: decode buffer directly?
                    println!("Received: {}", std::str::from_utf8(&buffer[..]).unwrap_or("Bad utf-8 bytes"));
                    continue;
                };
            };
            let decode_tone: Option<String> = decode_tone(&srec);
            match decode_tone {
                Some(tonestr) => {
                    println!("{}", tonestr);
                    continue;
                }
                None => {}
            };

        }
    }
}

// TODO: return option
fn decode_tone(s: &str) -> Option<String> {
    if s.starts_with("TR") {
        let dbs = db_level(s);
        let fs = format!("treble at {}", s);
        return Some(format!("treble at {}", s));
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

fn db_level(s: &str) -> String {
    let stripped= &s.to_string()[2..]; // just need to cut first two
    let my_int = stripped.parse::<i32>().unwrap(); // TODO: get rid of unwarps?
    let db = 6 - my_int;
    return format!("{mydb}dB", mydb=db);
}

fn remove_suffix<'x>(s: &'x str, suffix: &str) -> &'x str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s
    }
}

fn user_input_loop(transmitter: std::sync::mpsc::Sender<String>) -> bool {

    loop {
        let mut line = String::new();
        let _r = std::io::stdin().read_line(&mut line); // including '\n'
        if line == "\n" { continue; }
        println!("Got user line {}", line);
        transmitter.send(line).unwrap();
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
