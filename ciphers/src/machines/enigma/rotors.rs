use super::char_to_usize;
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    sync::LazyLock,
};

// Specifically the Enigma rotor
#[derive(Copy, Clone, Debug)]
pub struct Rotor {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    pub notch: (usize, usize),
    pub position: usize,
    pub ring: usize,
    pub wiring_str: &'static str,
    pub name: &'static str,
}

impl Rotor {
    // TODO: Clearer initialization?
    pub fn new(name: &'static str, wiring_str: &'static str, notch: (usize, usize)) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| char_to_usize(x)).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor {
            name,
            wiring_rtl,
            wiring_ltr,
            notch,
            position: 0,
            ring: 0,
            wiring_str,
        }
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26
    }

    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (26 + entry + self.position - self.ring) % 26;
        let inner = self.wiring_rtl[inner_position];
        (inner + 26 - self.position + self.ring) % 26
    }

    pub fn encrypt_ltr(&self, entry: usize) -> usize {
        let inner_position = (26 + entry + self.position - self.ring) % 26;
        let inner = self.wiring_ltr[inner_position];
        (inner + 26 - self.position + self.ring) % 26
    }
}

// Rotor equality is only based on the wiring not a specific position
impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(26);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Reflector {
    wiring: [usize; 26],
    pub wiring_str: &'static str,
    pub name: &'static str,
}

impl Reflector {
    // TODO: Clearer initialization?
    pub fn new(name: &'static str, wiring_str: &'static str) -> Reflector {
        let mut wiring_internal: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| char_to_usize(x)).enumerate() {
            wiring_internal[w.0] = w.1;
        }
        Reflector {
            name,
            wiring: wiring_internal,
            wiring_str,
        }
    }

    // We take and return usize to be consistent with Rotor
    // No decrypt is needed as reflectors are reciprocal
    pub fn encrypt(&self, entry: usize) -> usize {
        self.wiring[entry]
    }
}

// Reflector equality is only based on the wiring
impl PartialEq for Reflector {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl fmt::Display for Reflector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.wiring_str)
    }
}

pub static ROTOR_VEC: LazyLock<Vec<Rotor>> = LazyLock::new(|| {
    vec![
        Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", (16, 16)),
        Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", (4, 4)),
        Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", (21, 21)),
        Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", (9, 9)),
        Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", (25, 25)),
        Rotor::new("VI", "JPGVOUMFYQBENHZRDKASXLICTW", (12, 25)),
        Rotor::new("VII", "NZJHGRCXMYSWBOUFAIVLPEKQDT", (12, 25)),
        Rotor::new("VIII", "FKQHTLXOCBJSPDZRAMEWNIUYGV", (12, 25)),
    ]
});

pub static ROTOR_MAP: LazyLock<HashMap<&'static str, Rotor>> =
    LazyLock::new(|| HashMap::from_iter(ROTOR_VEC.iter().map(|rotor| (rotor.name, rotor.clone()))));

pub static REFLECTOR_VEC: LazyLock<Vec<Reflector>> = LazyLock::new(|| {
    vec![
        Reflector::new("Alpha", "LEYJVCNIXWPBQMDRTAKZGFUHOS"),
        Reflector::new("Beta", "FSOKANUERHMBTIYCWLQPZXVGJD"),
        Reflector::new("A", "EJMZALYXVBWFCRQUONTSPIKHGD"),
        Reflector::new("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        Reflector::new("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        Reflector::new("B-thin", "ENKQAUYWJICOPBLMDXZVFTHRGS"),
        Reflector::new("C-thin", "RDOBJNTKVEHMLFCWZAXGYIPSUQ"),
    ]
});

pub static REFLECTOR_MAP: LazyLock<HashMap<&'static str, Reflector>> = LazyLock::new(|| {
    HashMap::from_iter(
        REFLECTOR_VEC
            .iter()
            .map(|rotor| (rotor.name, rotor.clone())),
    )
});
