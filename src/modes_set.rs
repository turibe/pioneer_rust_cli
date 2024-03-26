use phf::phf_map;

// This is the ****SR listening mode set:
pub(crate) 
static MODE_SET_MAP: phf::Map<&'static str, &'static str> = phf_map! {
"0001" => "STEREO (cyclic)",
"0010" => "STANDARD",
"0009" => "STEREO (direct set)",
"0011" => "(2ch source)",
"0013" => "PRO LOGIC2 MOVIE",
"0018" => "PRO LOGIC2x MOVIE",
"0014" => "PRO LOGIC2 MUSIC",
"0019" => "PRO LOGIC2x MUSIC",
"0015" => "PRO LOGIC2 GAME",
"0020" => "PRO LOGIC2x GAME",
"0031" => "PRO LOGIC2z HEIGHT",
"0032" => "WIDE SURROUND MOVIE",
"0033" => "WIDE SURROUND MUSIC",
"0012" => "PRO LOGIC",
"0016" => "Neo:6 CINEMA",
"0017" => "Neo:6 MUSIC",
"0028" => "XM HD SURROUND",
"0029" => "NEURAL SURROUND",
"0037" => "Neo:X CINEMA",
"0038" => "Neo:X MUSIC",
"0039" => "Neo:X GAME",
"0040" => "NEURAL SURROUND+Neo:X CINEMA",
"0041" => "NEURAL SURROUND+Neo:X MUSIC",
"0042" => "NEURAL SURROUND+Neo:X GAME",
"0021" => "(Multi ch source)",
"0022" => "(Multi ch source)+DOLBY EX",
"0023" => "(Multi ch source)+PRO LOGIC2x MOVIE",
"0024" => "(Multi ch source)+PRO LOGIC2x MUSIC",
"0034" => "(Multi-ch Source)+PRO LOGIC2z HEIGHT",
"0035" => "(Multi-ch Source)+WIDE SURROUND MOVIE",
"0036" => "(Multi-ch Source)+WIDE SURROUND MUSIC",
"0025" => "(Multi ch source)DTS-ES Neo:6",
"0026" => "(Multi ch source)DTS-ES matrix",
"0027" => "(Multi ch source)DTS-ES discrete",
"0030" => "(Multi ch source)DTS-ES 8ch discrete",
"0043" => "(Multi ch source)+Neo:X CINEMA ",
"0044" => "(Multi ch source)+Neo:X MUSIC",
"0045" => "(Multi ch source)+Neo:X GAME",
"0100" => "ADVANCED SURROUND (cyclic)",
"0101" => "ACTION",
"0103" => "DRAMA",
"0102" => "SCI-FI",
"0105" => "MONO FILM",
"0104" => "ENTERTAINMENT SHOW",
"0106" => "EXPANDED THEATER",
"0116" => "TV SURROUND",
"0118" => "ADVANCED GAME",
"0117" => "SPORTS",
"0107" => "CLASSICAL",
"0110" => "ROCK/POP",
"0109" => "UNPLUGGED",
"0112" => "EXTENDED STEREO",
"0003" => "Front Stage Surround Advance Focus",
"0004" => "Front Stage Surround Advance Wide",
"0153" => "RETRIEVER AIR",
"0113" => "PHONES SURROUND",
"0050" => "THX (cyclic)",
"0051" => "PROLOGIC + THX CINEMA",
"0052" => "PL2 MOVIE + THX CINEMA",
"0053" => "Neo:6 CINEMA + THX CINEMA",
"0054" => "PL2x MOVIE + THX CINEMA",
"0092" => "PL2z HEIGHT + THX CINEMA",
"0055" => "THX SELECT2 GAMES",
"0068" => "THX CINEMA (for 2ch)",
"0069" => "THX MUSIC (for 2ch)",
"0070" => "THX GAMES (for 2ch)",
"0071" => "PL2 MUSIC + THX MUSIC",
"0072" => "PL2x MUSIC + THX MUSIC",
"0093" => "PL2z HEIGHT + THX MUSIC",
"0073" => "Neo:6 MUSIC + THX MUSIC",
"0074" => "PL2 GAME + THX GAMES",
"0075" => "PL2x GAME + THX GAMES",
"0094" => "PL2z HEIGHT + THX GAMES",
"0076" => "THX ULTRA2 GAMES",
"0077" => "PROLOGIC + THX MUSIC",
"0078" => "PROLOGIC + THX GAMES",
"0201" => "Neo:X CINEMA + THX CINEMA",
"0202" => "Neo:X MUSIC + THX MUSIC",
"0203" => "Neo:X GAME + THX GAMES",
"0056" => "THX CINEMA (for multi ch)",
"0057" => "THX SURROUND EX (for multi ch)",
"0058" => "PL2x MOVIE + THX CINEMA (for multi ch)",
"0095" => "PL2z HEIGHT + THX CINEMA (for multi ch)",
"0059" => "ES Neo:6 + THX CINEMA (for multi ch)",
"0060" => "ES MATRIX + THX CINEMA (for multi ch)",
"0061" => "ES DISCRETE + THX CINEMA (for multi ch)",
"0067" => "ES 8ch DISCRETE + THX CINEMA (for multi ch)",
"0062" => "THX SELECT2 CINEMA (for multi ch)",
"0063" => "THX SELECT2 MUSIC (for multi ch)",
"0064" => "THX SELECT2 GAMES (for multi ch)",
"0065" => "THX ULTRA2 CINEMA (for multi ch)",
"0066" => "THX ULTRA2 MUSIC (for multi ch)",
"0079" => "THX ULTRA2 GAMES (for multi ch)",
"0080" => "THX MUSIC (for multi ch)",
"0081" => "THX GAMES (for multi ch)",
"0082" => "PL2x MUSIC + THX MUSIC (for multi ch)",
"0096" => "PL2z HEIGHT + THX MUSIC (for multi ch)",
"0083" => "EX + THX GAMES (for multi ch)",
"0097" => "PL2z HEIGHT + THX GAMES (for multi ch)",
"0084" => "Neo:6 + THX MUSIC (for multi ch)",
"0085" => "Neo:6 + THX GAMES (for multi ch)",
"0086" => "ES MATRIX + THX MUSIC (for multi ch)",
"0087" => "ES MATRIX + THX GAMES (for multi ch)",
"0088" => "ES DISCRETE + THX MUSIC (for multi ch)",
"0089" => "ES DISCRETE + THX GAMES (for multi ch)",
"0090" => "ES 8CH DISCRETE + THX MUSIC (for multi ch)",
"0091" => "ES 8CH DISCRETE + THX GAMES (for multi ch)",
"0204" => "Neo:X CINEMA + THX CINEMA (for multi ch)",
"0205" => "Neo:X MUSIC + THX MUSIC (for multi ch)",
"0206" => "Neo:X GAME + THX GAMES (for multi ch)",
"0005" => "AUTO SURR/STREAM DIRECT (cyclic)",
"0006" => "AUTO SURROUND",
"0151" => "Auto Level Control (A.L.C.)",
"0007" => "DIRECT",
"0008" => "PURE DIRECT",
"0152" => "OPTIMUM SURROUND",
};

/***
inverseModeSetMap = {}

for (k,v) in modeSetMap.items():
	vkey = v.lower().strip()
	inverseModeSetMap[vkey] = k
***/

// extern crate lazy_static;
use lazy_static::lazy_static;

use std::{borrow::Borrow, collections::HashMap};

lazy_static! {
	pub(crate)
    static ref INVERSE_MODE_SET_MAP: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
		for (k,v) in MODE_SET_MAP.entries() {
			let v1 = v.to_lowercase();
			m.insert(v1, k.borrow());
		};
		m
	};
}

use std::collections::HashSet;

pub
fn get_modes_with_prefix(prefix: &str) -> HashSet<&String> {
    let mut s = HashSet::new();
    for k in INVERSE_MODE_SET_MAP.keys() {
		if k.starts_with(prefix) {
			s.insert(k);
		}
	}
	return s;
}
