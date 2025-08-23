use crate::traits::Code;
use bimap::BiMap;
use utils::{errors::GeneralError, preset_alphabet::Alphabet};

crate::lazy_bimap!(
    // Yes, ALFA and JULIETT are meant to be spelled that way
    // Yes, the spelling of the numerals is correct even though the pronunciation is different
    NATO: BiMap<char,&str> =
        Alphabet::BasicLatin.chars().zip([
            "ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
            "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
            "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
            "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
            "XRAY", "YANKEE", "ZULU"].into_iter());

    CCB: BiMap<char,&str> =
        Alphabet::BasicLatin.chars().zip([
            "ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
            "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
            "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
            "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
            "XRAY", "YOKE", "ZEBRA"].into_iter());

    WESTERN_UNION_1912: BiMap<char,&str> =
        Alphabet::BasicLatin.chars().zip([
            "ADAMS", "BOSTON", "CHICAGO", "DENVER", "EASY", "FRANK",
            "GEORGE", "HENRY", "IDA", "JERSEY", "KING", "LINCOLN",
            "MARY", "NEWARK", "OCEAN", "PETER", "QUEEN", "ROGER",
            "SUGAR", "TEXAS", "UNION", "VIOLET", "WILLIAM",
            "XRAY", "YALE", "ZERO"].into_iter());

    WESTERN_UNION_1942: BiMap<char,&str> =
        Alphabet::BasicLatin.chars().zip([
            "ADAMS", "BOSTON", "CHICAGO", "DENVER", "EASY", "FRANK",
            "GEORGE", "HENRY", "IDA", "JOHN", "KING", "LINCOLN",
            "MARY", "NEWYORK", "OCEAN", "PETER", "QUEEN", "ROGER",
            "SUGAR", "THOMAS", "UNION", "VICTORY", "WILLIAM",
            "XRAY", "YOUNG", "ZERO"].into_iter());

    US_NAVY_1908: BiMap<char, &str> =
        Alphabet::BasicLatin.chars().zip([
            "ACTOR", "BAKER", "CANTEEN", "DIVER", "EAGLE", "FISHER", "GANGWAY", "HALLIARD",
            "INSECT", "JOKCEY", "KNAPSACK", "LUGGER", "MUSKET", "NEPTUNE", "OYSTER", "PISTOL",
            "QUADRANT", "REEFER", "SHIPMATE", "TOPSAIL", "UNLOAD", "VESSEL", "WINDAGE", "XRAY",
            "YEOMAN", "ZEBRA"].into_iter());

    US_NAVY_1908_SHORT: BiMap<char, &str> =
        Alphabet::BasicLatin.chars().zip([
            "ASH", "BACK", "CHAIN", "DOG", "EGG", "FOX", "GIG", "HORSE", "ICE", "JAKE", "KING",
            "LASH", "MULE", "NET", "OAK", "PAGE", "QUAIL", "RAFT", "SCOUT", "TIDE", "USE",
            "VAST", "WINCH", "XRAY", "YACHT", "ZOO"].into_iter());

    US_MILITARY_1941: BiMap<char, &str> =
        Alphabet::BasicLatin.chars().zip([
            "ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX", "GEORGE", "HOW", "ITEM", "JIG",
            "KING", "LOVE", "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER", "SAIL", "TARE",
            "UNCLE", "VICTOR", "WILLIAM", "XRAY", "YOKE", "ZEBRA"].into_iter());

    UK_ARMY_1904: BiMap<char, &'static str> =
        Alphabet::BasicLatin.chars().zip([
            "ACK", "BEER", "CORK", "DON", "EDDY", "FREDDY", "GEORGE", "HARRY", "INK", "JUB",
            "KING", "LONDON", "EMMA", "NUTS", "ORANGE", "PIP", "QUAD", "ROBERT", "ESSES",
            "TOC", "UNCLE", "VIC", "WILLIAM", "XERXES", "YELLOW", "ZEBRA"].into_iter());
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellingAlphabetMode {
    Nato,
    Ccb,
    Wu1912,
    Wu1942,
    Usn1908,
    Usn1908Alt,
    Us1941,
    Uka1904,
    FirstLetter,
}

impl SpellingAlphabetMode {
    pub fn alphabet(&self) -> &str {
        match self {
            Self::FirstLetter => "",
            _ => Alphabet::BasicLatin.slice(),
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
            Self::Usn1908Alt => US_NAVY_1908_SHORT.get_by_left(&c),
            Self::Uka1904 => UK_ARMY_1904.get_by_left(&c),
            Self::FirstLetter => None,
        }
    }

    pub fn decode(&self, s: &str) -> Option<&char> {
        match self {
            Self::Nato => NATO.get_by_right(s),
            Self::Ccb => CCB.get_by_right(s),
            Self::Wu1912 => WESTERN_UNION_1912.get_by_right(s),
            Self::Wu1942 => WESTERN_UNION_1942.get_by_right(s),
            Self::Us1941 => US_MILITARY_1941.get_by_right(s),
            Self::Usn1908 => US_NAVY_1908.get_by_right(s),
            Self::Usn1908Alt => US_NAVY_1908_SHORT.get_by_right(s),
            Self::Uka1904 => UK_ARMY_1904.get_by_right(s),
            Self::FirstLetter => None,
        }
    }
}

pub struct SpellingAlphabet {
    pub variant: SpellingAlphabetMode,
}

impl SpellingAlphabet {
    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &&str)> + '_ {
        self.variant
            .alphabet()
            .chars()
            .map(|c| (c, self.variant.encode(c).unwrap()))
    }
}

impl Default for SpellingAlphabet {
    fn default() -> Self {
        Self {
            variant: SpellingAlphabetMode::Nato,
        }
    }
}

impl Code for SpellingAlphabet {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        if self.variant == SpellingAlphabetMode::FirstLetter {
            Err(GeneralError::state(
                "Cannot encode while in First Letter mode",
            ))
        } else {
            let mut out = String::new();
            for c in text.chars() {
                out.push_str(
                    self.variant
                        .encode(c)
                        .ok_or(GeneralError::invalid_input_char(c))?,
                )
            }
            Ok(out)
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        if self.variant == SpellingAlphabetMode::FirstLetter {
            Ok(text
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap()) // Unwrap is guaranteed valid because string is not empty
                .collect())
        } else {
            let mut out = String::new();
            for s in text.split_whitespace().filter(|s| !s.is_empty()) {
                out.push(
                    *self
                        .variant
                        .decode(s)
                        .ok_or(GeneralError::invalid_input_group(s))?,
                )
            }
            Ok(out)
        }
    }
}
