// pub mod modes_display;
// see https://crates.io/crates/phf

use phf::phf_map;

// LISTENING MODE information

pub(crate)

static MODE_DISPLAY_MAP: phf::Map<&'static str, &'static str> = phf_map! {
"0101" => "[)(]PLIIx MOVIE",
"0102" => "[)(]PLII MOVIE",
"0103" => "[)(]PLIIx MUSIC",
"0104" => "[)(]PLII MUSIC",
"0105" => "[)(]PLIIx GAME",
"0106" => "[)(]PLII GAME",
"0107" => "[)(]PROLOGIC",
"0108" => "Neo:6 CINEMA",
"0109" => "Neo:6 MUSIC",
"010a" => "XM HD Surround",
"010b" => "NEURAL SURR  ",
"010c" => "2ch Straight Decode",
"010d" => "[)(]PLIIz HEIGHT",
"010e" => "WIDE SURR MOVIE",
"010f" => "WIDE SURR MUSIC",
"0110" => "STEREO",
"0111" => "Neo:X CINEMA",
"0112" => "Neo:X MUSIC",
"0113" => "Neo:X GAME",
"0114" => "NEURAL SURROUND+Neo:X CINEMA",
"0115" => "NEURAL SURROUND+Neo:X MUSIC",
"0116" => "NEURAL SURROUND+Neo:X GAMES",
"1101" => "[)(]PLIIx MOVIE",
"1102" => "[)(]PLIIx MUSIC",
"1103" => "[)(]DIGITAL EX",
"1104" => "DTS +Neo:6 / DTS-HD +Neo:6",
"1105" => "ES MATRIX",
"1106" => "ES DISCRETE",
"1107" => "DTS-ES 8ch ",
"1108" => "multi ch Straight Decode",
"1109" => "[)(]PLIIz HEIGHT",
"110a" => "WIDE SURR MOVIE",
"110b" => "WIDE SURR MUSIC",
"110c" => "Neo:X CINEMA ",
"110d" => "Neo:X MUSIC",
"110e" => "Neo:X GAME",
"0201" => "ACTION",
"0202" => "DRAMA",
"0203" => "SCI-FI",
"0204" => "MONOFILM",
"0205" => "ENT.SHOW",
"0206" => "EXPANDED",
"0207" => "TV SURROUND",
"0208" => "ADVANCEDGAME",
"0209" => "SPORTS",
"020a" => "CLASSICAL",
"020b" => "ROCK/POP",
"020c" => "UNPLUGGED",
"020d" => "EXT.STEREO",
"020e" => "PHONES SURR.",
"020f" => "FRONT STAGE SURROUND ADVANCE FOCUS",
"0210" => "FRONT STAGE SURROUND ADVANCE WIDE",
"0211" => "SOUND RETRIEVER AIR",
"0301" => "[)(]PLIIx MOVIE +THX",
"0302" => "[)(]PLII MOVIE +THX",
"0303" => "[)(]PL +THX CINEMA",
"0304" => "Neo:6 CINEMA +THX",
"0305" => "THX CINEMA",
"0306" => "[)(]PLIIx MUSIC +THX",
"0307" => "[)(]PLII MUSIC +THX",
"0308" => "[)(]PL +THX MUSIC",
"0309" => "Neo:6 MUSIC +THX",
"030a" => "THX MUSIC",
"030b" => "[)(]PLIIx GAME +THX",
"030c" => "[)(]PLII GAME +THX",
"030d" => "[)(]PL +THX GAMES",
"030e" => "THX ULTRA2 GAMES",
"030f" => "THX SELECT2 GAMES",
"0310" => "THX GAMES",
"0311" => "[)(]PLIIz +THX CINEMA",
"0312" => "[)(]PLIIz +THX MUSIC",
"0313" => "[)(]PLIIz +THX GAMES",
"0314" => "Neo:X CINEMA + THX CINEMA",
"0315" => "Neo:X MUSIC + THX MUSIC",
"0316" => "Neo:X GAMES + THX GAMES",
"1301" => "THX Surr EX",
"1302" => "Neo:6 +THX CINEMA",
"1303" => "ES MTRX +THX CINEMA",
"1304" => "ES DISC +THX CINEMA",
"1305" => "ES 8ch +THX CINEMA ",
"1306" => "[)(]PLIIx MOVIE +THX",
"1307" => "THX ULTRA2 CINEMA",
"1308" => "THX SELECT2 CINEMA",
"1309" => "THX CINEMA",
"130a" => "Neo:6 +THX MUSIC",
"130b" => "ES MTRX +THX MUSIC",
"130c" => "ES DISC +THX MUSIC",
"130d" => "ES 8ch +THX MUSIC",
"130e" => "[)(]PLIIx MUSIC +THX",
"130f" => "THX ULTRA2 MUSIC",
"1310" => "THX SELECT2 MUSIC",
"1311" => "THX MUSIC",
"1312" => "Neo:6 +THX GAMES",
"1313" => "ES MTRX +THX GAMES",
"1314" => "ES DISC +THX GAMES",
"1315" => "ES 8ch +THX GAMES",
"1316" => "[)(]EX +THX GAMES",
"1317" => "THX ULTRA2 GAMES",
"1318" => "THX SELECT2 GAMES",
"1319" => "THX GAMES",
"131a" => "[)(]PLIIz +THX CINEMA",
"131b" => "[)(]PLIIz +THX MUSIC",
"131c" => "[)(]PLIIz +THX GAMES",
"131d" => "Neo:X CINEMA + THX CINEMA",
"131e" => "Neo:X MUSIC + THX MUSIC",
"131f" => "Neo:X GAME + THX GAMES",
"0401" => "STEREO",
"0402" => "[)(]PLII MOVIE",
"0403" => "[)(]PLIIx MOVIE",
"0404" => "Neo:6 CINEMA",
"0405" => "AUTO SURROUND Straight Decode",
"0406" => "[)(]DIGITAL EX",
"0407" => "[)(]PLIIx MOVIE",
"0408" => "DTS +Neo:6",
"0409" => "ES MATRIX",
"040a" => "ES DISCRETE",
"040b" => "DTS-ES 8ch ",
"040c" => "XM HD Surround",
"040d" => "NEURAL SURR  ",
"040e" => "RETRIEVER AIR",
"040f" => "Neo:X CINEMA",
"0410" => "Neo:X CINEMA ",
"0501" => "STEREO",
"0502" => "[)(]PLII MOVIE",
"0503" => "[)(]PLIIx MOVIE",
"0504" => "Neo:6 CINEMA",
"0505" => "ALC Straight Decode",
"0506" => "[)(]DIGITAL EX",
"0507" => "[)(]PLIIx MOVIE",
"0508" => "DTS +Neo:6",
"0509" => "ES MATRIX",
"050a" => "ES DISCRETE",
"050b" => "DTS-ES 8ch ",
"050c" => "XM HD Surround",
"050d" => "NEURAL SURR  ",
"050e" => "RETRIEVER AIR",
"050f" => "Neo:X CINEMA",
"0510" => "Neo:X CINEMA ",
"0601" => "STEREO",
"0602" => "[)(]PLII MOVIE",
"0603" => "[)(]PLIIx MOVIE",
"0604" => "Neo:6 CINEMA",
"0605" => "STREAM DIRECT NORMAL Straight Decode",
"0606" => "[)(]DIGITAL EX",
"0607" => "[)(]PLIIx MOVIE",
"0608" => "(nothing)",
"0609" => "ES MATRIX",
"060a" => "ES DISCRETE",
"060b" => "DTS-ES 8ch ",
"060c" => "Neo:X CINEMA",
"060d" => "Neo:X CINEMA ",
"0701" => "STREAM DIRECT PURE 2ch",
"0702" => "[)(]PLII MOVIE",
"0703" => "[)(]PLIIx MOVIE",
"0704" => "Neo:6 CINEMA",
"0705" => "STREAM DIRECT PURE Straight Decode",
"0706" => "[)(]DIGITAL EX",
"0707" => "[)(]PLIIx MOVIE",
"0708" => "(nothing)",
"0709" => "ES MATRIX",
"070a" => "ES DISCRETE",
"070b" => "DTS-ES 8ch ",
"070c" => "Neo:X CINEMA",
"070d" => "Neo:X CINEMA ",
"0881" => "OPTIMUM",
"0e01" => "HDMI THROUGH",
"0f01" => "MULTI CH IN"
};
