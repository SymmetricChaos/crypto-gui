use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;

use super::Code;
use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};

lazy_static! {
    // Yes, ALFA and JULIETT are meant to be spelled that way
    // Yes, the spelling of the numerals is correct even though the pronunciation is different
    pub static ref NATO: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().zip([
            "ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
            "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
            "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
            "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
            "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
            "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].into_iter())
    );


    pub static ref CCB: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().zip([
            "ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
            "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
            "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
            "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
            "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
            "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].into_iter())

    );

    pub static ref WESTERN_UNION_1912: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip([
            "ADAMS", "BOSTON", "CHICAGO", "DENVER", "EASY", "FRANK",
            "GEORGE", "HENRY", "IDA", "JERSEY", "KING", "LINCOLN",
            "MARY", "NEWARK", "OCEAN", "PETER", "QUEEN", "ROGER",
            "SUGAR", "TEXAS", "UNION", "VIOLET", "WILLIAM",
            "XRAY", "YALE", "ZERO"].into_iter())
    );

    pub static ref WESTERN_UNION_1942: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip([
            "ADAMS", "BOSTON", "CHICAGO", "DENVER", "EASY", "FRANK",
            "GEORGE", "HENRY", "IDA", "JOHN", "KING", "LINCOLN",
            "MARY", "NEWYORK", "OCEAN", "PETER", "QUEEN", "ROGER",
            "SUGAR", "THOMAS", "UNION", "VICTORY", "WILLIAM",
            "XRAY", "YOUNG", "ZERO"].into_iter())
    );

    pub static ref US_NAVY_1908: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip([
            "ACTOR", "BAKER", "CANTEEN", "DIVER", "EAGLE", "FISHER",
            "GANGWAY", "HALLIARD", "INSECT", "JOKCEY", "KNAPSACK", "LUGGER",
            "MUSKET", "NEPTUNE", "OYSTER", "PISTOL", "QUADRANT", "REEFER",
            "SHIPMATE", "TOPSAIL", "UNLOAD", "VESSEL", "WINDAGE",
            "XRAY", "YEOMAN", "ZEBRA"].into_iter())
    );


    pub static ref US_NAVY_1908_ALT: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip([
            "ASH", "BACK", "CHAIN", "DOG", "EGG", "FOX",
            "GIG", "HORSE", "ICE", "JAKE", "KING", "LASH",
            "MULE", "NET", "OAK", "PAGE", "QUAIL", "RAFT",
            "SCOUT", "TIDE", "USE", "VAST", "WINCH",
            "XRAY", "YACHT", "ZOO"].into_iter())
    );

    pub static ref US_MILITARY_1941: BiMap<char,&'static str> = bimap_from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip([
            "ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
            "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
            "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
            "SAIL", "TARE", "UNCLE", "VICTOR", "WILLIAM",
            "XRAY", "YOKE", "ZEBRA"].into_iter())
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellingAlphabetMode {
    Nato,
    Ccb,
    Wu1912,
    Wu1942,
    Usn1908,
    Usn1908Alt,
    Us1941,
}

impl SpellingAlphabetMode {
    pub fn alphabet(&self) -> &str {
        match self {
            Self::Nato => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            Self::Ccb => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            Self::Wu1912 | Self::Wu1942 | Self::Us1941 | Self::Usn1908 | Self::Usn1908Alt => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            }
        }
    }

    pub fn encode(&self, c: char) -> Option<&&str> {
        match self {
            Self::Nato => NATO.get_by_left(&c),
            Self::Ccb => CCB.get_by_left(&c),
            Self::Wu1912 => WESTERN_UNION_1912.get_by_left(&c),
            Self::Wu1942 => WESTERN_UNION_1942.get_by_left(&c),
            Self::Us1941 => US_MILITARY_1941.get_by_left(&c),
            Self::Usn1908 => US_NAVY_1908.get_by_left(&c),
            Self::Usn1908Alt => US_NAVY_1908_ALT.get_by_left(&c),
        }
    }

    pub fn decode(&self, s: &str) -> Option<&char> {
        match self {
            SpellingAlphabetMode::Nato => NATO.get_by_right(s),
            SpellingAlphabetMode::Ccb => CCB.get_by_right(s),
            SpellingAlphabetMode::Wu1912 => WESTERN_UNION_1912.get_by_right(s),
            SpellingAlphabetMode::Wu1942 => WESTERN_UNION_1942.get_by_right(s),
            SpellingAlphabetMode::Us1941 => US_MILITARY_1941.get_by_right(s),
            SpellingAlphabetMode::Usn1908 => US_NAVY_1908.get_by_right(s),
            SpellingAlphabetMode::Usn1908Alt => US_NAVY_1908_ALT.get_by_right(s),
        }
    }
}

pub struct SpellingAlphabet {
    pub mode: SpellingAlphabetMode,
}

impl SpellingAlphabet {
    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &&str)> + '_ {
        self.mode
            .alphabet()
            .chars()
            .map(|c| (c, self.mode.encode(c).unwrap()))
    }
}

impl Default for SpellingAlphabet {
    fn default() -> Self {
        Self {
            mode: SpellingAlphabetMode::Nato,
        }
    }
}

// These will panic change them to return CodeError on failure
impl Code for SpellingAlphabet {
    fn encode(&self, text: &str) -> Result<String, Error> {
        Ok(text
            .chars()
            .map(|c| self.mode.encode(c).unwrap_or(&"�"))
            .join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        Ok(text
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| self.mode.decode(s).unwrap_or(&'�'))
            .collect())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}
