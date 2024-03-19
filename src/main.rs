use telnet::{Telnet, Event};
use telnet::{Action, TelnetOption};
use std::thread;

fn main() {
    println!("Hello, world!");
    // let host = "ptt.cc";
    let host = "192.168.86.32";
    // let host = "localhost";
    let mut connection = Telnet::connect((host, 23), 256)
            .expect("Couldn't connect to the server...");

    let res = connection.negotiate(&Action::Will, TelnetOption::Echo);

    // println!("Connection: {}", connection);

    // println!("{}", res); // no formatter for res

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

    let handle = thread::spawn(|| { write_loop(connection); });

    loop {
        let event = connection.read().expect("Read error");

        if let Event::Data(buffer) = event {
            line += 1;
            // Debug: print the data buffer
            println!("Got event {:?}", buffer);
            let r = String::from_utf8_lossy(&buffer);
            println!("Line {}: {}", line, r);
            decode(r.to_string());

            println!(
                "Receive: {}",
                std::str::from_utf8(&buffer[..]).unwrap_or("Bad utf-8 bytes")
            );

            // process the data buffer
        }
    }
}


fn write_loop(connection: Telnet) -> bool {

    loop {
        let mut line = String::new();
        let r = std::io::stdin().read_line(&mut line); // including '\n'
        println!("Got user line {}", line);
    
    } 
}

fn decode(s: String) -> bool {
    print!("Original string is {}", s);
    if ! s.starts_with("FL") {
        println!("String does not start with FL");
        return false;
    }
    let s = match s.strip_prefix("FL") {
        Some(v) => v,
        None => return false,
    };

    let mut url = "";
    let mut i = 0;

    return true;
}
