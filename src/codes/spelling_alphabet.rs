use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;
use crate::errors::Error;

fn make_maps(
    alphabet: &'static str,
    codes: &[&'static str],
) -> (HashMap<char, &'static str>, HashMap<&'static str, char>) {
    let mut map = HashMap::new();
    let mut map_inv = HashMap::new();
    for (l, w) in alphabet.chars().zip(codes.iter()) {
        map.insert(l, *w);
        map_inv.insert(*w, l);
    }
    (map, map_inv)
}

lazy_static! {
    // Yes, ALFA and JULIETT are meant to be spelled that way
    // Yes, the spelling of the numerals is correct even though the pronunciation is different
    pub static ref NATO: (HashMap<char,&'static str>, HashMap<&'static str, char>) = make_maps(
                                                "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
                                                &["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                                                "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                                                "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                                                "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                                                "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                                                "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"]);

    pub static ref CCB: (HashMap<char,&'static str>, HashMap<&'static str, char>) = make_maps(
                                              "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
                                              &["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                                              "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                                              "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                                              "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                                              "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                                              "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"]);
}

pub enum SpellingAlphabetMode {
    Nato,
    Ccb,
}

impl SpellingAlphabetMode {
    pub fn alphabet(&self) -> &str {
        match self {
            SpellingAlphabetMode::Nato => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            SpellingAlphabetMode::Ccb => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        }
    }

    pub fn encode(&self, c: char) -> Option<&&str> {
        match self {
            SpellingAlphabetMode::Nato => NATO.0.get(&c),
            SpellingAlphabetMode::Ccb => CCB.0.get(&c),
        }
    }

    pub fn decode(&self, s: &str) -> Option<&char> {
        match self {
            SpellingAlphabetMode::Nato => NATO.1.get(s),
            SpellingAlphabetMode::Ccb => CCB.1.get(s),
        }
    }
}

pub struct SpellingAlphabet {
    mode: SpellingAlphabetMode,
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
        Ok(text.chars().map(|c| self.mode.encode(c).unwrap()).join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        Ok(text
            .split(" ")
            .map(|s| self.mode.decode(s).unwrap())
            .collect())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}
