use super::char_to_usize;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
 
#[derive(Clone,Debug,Copy)]
pub struct Rotor {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    pub notch: (usize,usize),
    pub position: usize,
    pub ring: usize,
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
impl Rotor {
	// TODO: Clearer initialization?
    pub fn new(name: &'static str, wiring_str:  &'static str, notch: (usize,usize)) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| char_to_usize(x) ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ name, wiring_rtl, wiring_ltr, notch, position: 0, ring: 0, wiring_str }
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.wiring_rtl[inner_position];
        (inner+26-self.position+self.ring) % 26
    }
 
    pub fn encrypt_ltr(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.wiring_ltr[inner_position];
        (inner+26-self.position+self.ring) % 26
    }
}

// Rotor equality is only based on the wiring not a specific position
impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(26);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}
 
#[derive(Clone,Debug,Copy)]
pub struct Reflector {
    wiring: [usize; 26],
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
impl Reflector {
	// TODO: Clearer initialization?
    pub fn new(name: &'static str, wiring_str: &'static str) -> Reflector {
        let mut wiring_internal: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| char_to_usize(x) ).enumerate() {
            wiring_internal[w.0] = w.1;
        }
        Reflector{ name, wiring: wiring_internal, wiring_str }
    }
 
    // We take and return usize to be consistent with Rotor
    // No decrypt is needed as reflectors are reciprocal
    pub fn encrypt(&self, entry: usize) -> usize {
        self.wiring[entry]
    }
}

// Reflector equality is only based on the wiring not a specific position
impl PartialEq for Reflector {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl fmt::Display for Reflector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.wiring_str)
    }
}
 
lazy_static! {
    pub static ref ROTORS: HashMap<&'static str, Rotor> = {
        let mut m = HashMap::new();
        m.insert("I",    Rotor::new("I",    "EKMFLGDQVZNTOWYHXUSPAIBRCJ", (16,16) ));
        m.insert("II",   Rotor::new("II",   "AJDKSIRUXBLHWTMCQGZNPYFVOE", (4,4) ));
        m.insert("III",  Rotor::new("III",  "BDFHJLCPRTXVZNYEIWGAKMUSQO", (21,21) ));
        m.insert("IV",   Rotor::new("IV",   "ESOVPZJAYQUIRHXLNFTGKDCMWB", (9,9) ));
        m.insert("V",    Rotor::new("V",    "VZBRGITYUPSDNHLXAWMJQOFECK", (25,25) ));
        m.insert("VI",   Rotor::new("VI",   "JPGVOUMFYQBENHZRDKASXLICTW", (12,25) ));
        m.insert("VII",  Rotor::new("VII",  "NZJHGRCXMYSWBOUFAIVLPEKQDT", (12,25) ));
        m.insert("VIII", Rotor::new("VIII", "FKQHTLXOCBJSPDZRAMEWNIUYGV", (12,25) ));
        m
    };
 
    pub static ref REFLECTORS: HashMap<&'static str, Reflector> = {
        let mut m = HashMap::new();
        m.insert("Alpha",  Reflector::new("Alpha",  "LEYJVCNIXWPBQMDRTAKZGFUHOS"));
        m.insert("Beta",   Reflector::new("Beta",   "FSOKANUERHMBTIYCWLQPZXVGJD"));
        m.insert("A",      Reflector::new("A",      "EJMZALYXVBWFCRQUONTSPIKHGD"));
        m.insert("B",      Reflector::new("B",      "YRUHQSLDPXNGOKMIEBFZCWVJAT"));
        m.insert("C",      Reflector::new("C",      "FVPJIAOYEDRZXWGCTKUQSBNMHL"));
        m.insert("B-thin", Reflector::new("B-thin", "ENKQAUYWJICOPBLMDXZVFTHRGS"));
        m.insert("C-thin", Reflector::new("C-thin", "RDOBJNTKVEHMLFCWZAXGYIPSUQ"));
        m
    };
}

 