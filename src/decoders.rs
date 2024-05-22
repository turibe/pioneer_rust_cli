pub(crate) 

use bytebuffer::ByteBuffer;

// pub(crate)

use crate::other_maps::{AIF_MAP, CHANNEL_DECODE_MAP, SCREEN_TYPE_MAP, SOURCE_MAP, TYPE_MAP, VTC_RESOLUTION_MAP};

// TODO: add unit tests.
pub fn decode_fl(s: &str) -> String {

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

pub fn decode_ast(s: &str) -> String {
    let s1 = format!("Audio input signal: {}", decode_ais(&s[0..2]));
    let s2 = format!("Audio input frequency: {}", decode_aif(&s[2..4]));
    let binding = "-".to_string() + s;
    let sc:Vec<char> = binding.chars().collect();
    let mut s3 = s1 + "\n" + &s2;
    // println!("raw: {}", s);
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

pub fn decode_ais(s:&str) -> &str {
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

pub fn decode_vtc(s: &str) -> String {
    match VTC_RESOLUTION_MAP.get(s) {
        Some(v) => v.to_string(),
        None => "unknown VTC resolution".to_owned()
    }
}

pub fn decode_tone(s: &str) -> Option<String> {
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

pub fn decode_geh(s: &str) -> Option<String> {
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
