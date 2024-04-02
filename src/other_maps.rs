use phf::phf_map;
use serde_json::Value;
use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::sync::Mutex;


pub static ERROR_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "E02" => "NOT AVAILABLE NOW",
    "E03" => "INVALID COMMAND",
    "E04" => "COMMAND ERROR",
    "E06" => "PARAMETER ERROR",
    "B00" => "BUSY"
};

pub static VTC_RESOLUTION_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "AUTO Resolution",
    "01" => "PURE Resolution",
    "02" => "Reserved Resolution",
    "03" => "R480/576 Resolution",
    "04" => "720p Resolution",
    "05" => "1080i Resolution",
    "06" => "1080p Resolution",
    "07" => "1080/24p Resolution"
};

pub struct InputMap;

impl InputMap {
    pub fn read_from_file(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj = parsed.as_object();
        let m = match obj {
            Some(m) => {
                m
            }
            None => {
                return Err("error".into());
            }
        };
        println!("Read map from {}", path);
        for x in m.iter() {
            println!("{} - {}", x.0, x.1)
        }
        let mut result_map: HashMap<String, String> = HashMap::new();
        for x in m.iter() {
            match x.1.as_str() {
                Some(s) => {
                    result_map.insert(x.0.to_string(), s.to_string());
                }
                None => {
                    println!("Bad value in JSON map: {}", x.1);
                }
            }            
        }
        return Ok(result_map);
    }
}
// default set:

pub static DEFAULT_INPUT_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "PHONO",
    "01" => "CD",
    "02" => "TUNER",
    "04" => "DVD",
    "05" => "TV",
    "06" => "SAT/CBL",
    "10" => "VIDEO",
    "12" => "MULTI CH IN",
    "13" => "USB-DAC",
    "15" => "DVR/BDR",
    "17" => "iPod/USB",
    "19" => "HDMI1",
    "20" => "HDMI2",
    "21" => "HDMI3",
    "22" => "HDMI4",
    "23" => "HDMI5",
    "24" => "HDMI6",
    "26" => "NETWORK",
    "33" => "ADAPTER PORT",
    "31" => "HDMI", //  cyclic
    "38" => "INTERNET RADIO",
    "41" => "PANDORA",
    "44" => "MEDIA SERVER",
    "45" => "FAVORITES",
    "47" => "DMR",
    "48" => "MHL",
};

pub static COMMAND_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "on" => "PO",
    "off" => "PF",
    "up" => "VU",
    "+" => "VU",
    "down" => "VD",
    "-" => "VD",
    "mute" => "MO",
    "unmute" => "MF",

    "volume" => "?V",

    "tone" => "9TO", // cyclic
    "tone off" => "0TO",
    "tone on" => "1TO",
    "treble up" => "TI",
    "treble down" => "TD",
    "treble reset" => "06TR",
    "bass up" => "BI",
    "bass down" => "BD",
    "bass reset" => "06BA",

    "mcacc" => "MC0", // cyclic

    // phase control is recommended to be on:
    "phase" => "IS9", // cyclic

    // cycle through stereo modes:
    "stereo" => "0001SR",
    "unplugged" => "0109SR",
    "extended" => "0112SR",

    "mode" => "?S",

    "loud" => "9ATW", // cyclic

    // TODO: could have a pandora mode, radio mode, etc.
    // Pandora ones:
    "start" => "30NW",
    "next" => "13NW",
    "pause" => "11NW",
    "play" => "10NW",
    "previous" => "12NW",
    "stop" => "20NW",
    "clear" => "33NW",
    "repeat" => "34NW",
    "random" => "35NW",
    "menu" => "36NW",

    "info" => "?GAH",
    "list" => "?GAI",
    "top menu" => "19IP",

    // Tuner ones:
    "nextpreset" => "TPI",
    "prevpreset" => "TPD",
    "mpx" => "05TN",

    // Cyclic mode shortcuts:
    // cycles through thx modes, but input must be THX:
    "thx" => "0050SR",
    // cycles through surround modes (shortcut for "mode" command):
    "surr" => "0100SR"

};

pub static SOURCE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Intenet Radio",
    "01" => "Media Server",
    "06" => "SiriusXM",
    "07" => "Pandora",
    "10" => "AirPlay",
    "11" => "Digital Media Renderer (DMR)"
};

pub static TYPE_MAP: phf::Map<&'static str, &'static str> = phf_map! {    
    "20" => "Track",
    "21" => "Artist",
    "22" => "Album",
    "23" => "Time",
    "24" => "Genre",
    "25" => "Chapter Number",
    "26" => "Format",
    "27" => "Bitrate",
    "28" => "Category",
    "29" => "Composer1",
    "30" => "Composer2",
    "31" => "Buffer",
    "32" => "Channel"
};

pub static SCREEN_TYPE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Message",
    "01" => "List",
    "02" => "Playing (Play)",
    "03" => "Playing (Pause)",
    "04" => "Playing (Fwd)",
    "05" => "Playing (Rev)",
    "06" => "Playing (Stop)",
    "99" => "Invalid"
};

pub static AIF_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "32kHz",
    "01" => "44.1kHz",
    "02" => "48kHz",
    "03" => "88.2kHz",
    "04" => "96kHz",
    "05" => "176.4kHz",
    "06" => "192kHz",
    "07" => "---" 
};

use lazy_static::lazy_static;

lazy_static! {
    pub(crate)
    static ref INPUT_MAP:Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

lazy_static! {
    pub(crate)
    static ref REVERSE_INPUT_MAP:Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}