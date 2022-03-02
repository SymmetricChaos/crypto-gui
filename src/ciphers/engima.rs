use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
 
#[derive(Clone,Debug,Copy)]
pub struct Rotor {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    notch: (usize,usize),
    position: usize,
    ring: usize,
    pub wiring_display: &'static str,
}
 
impl Rotor {
	// TODO: Clearer initialization?
    pub fn new(wiring: &str, notch: (usize,usize)) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, notch, position: 0, ring: 0, wiring_display: wiring }
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26
    }
 
    pub fn set_ring(&mut self, n: usize) {
        self.ring = n;
    }
 
    pub fn set_position(&mut self, n: usize) {
        self.position = n;
    }
 
    pub fn get_ring(&self) -> usize {
        self.ring
    }
 
    pub fn get_position(&self) -> usize {
        self.position
    }
 
    pub fn get_notch(&self) -> (usize,usize) {
        self.notch
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
 
#[derive(Clone,Debug,Copy)]
pub struct Reflector {
    wiring: [usize; 26],
    pub wiring_display: &'static str,
}
 
impl Reflector {
	// TODO: Clearer initialization?
    pub fn new(wiring: &str) -> Reflector {
        let mut wiring_internal: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_internal[w.0] = w.1;
        }
        Reflector{ wiring: wiring_internal, wiring_display: wiring }
    }
 
    // We take and return usize to be consistent with Rotor
    // No decrypt is needed as reflectors are reciprocal
    pub fn encrypt(&self, entry: usize) -> usize {
        self.wiring[entry]
    }
}
 
lazy_static! {
    pub static ref ROTORS: HashMap<&'static str, Rotor<'static>> = {
        let mut m = HashMap::new();
        m.insert("I",    Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", (16,16) ));
        m.insert("II",   Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", (4,4) ));
        m.insert("III",  Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", (21,21) ));
        m.insert("IV",   Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", (9,9) ));
        m.insert("V",    Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", (25,25) ));
        m.insert("VI",   Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", (12,25) ));
        m.insert("VII",  Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", (12,25) ));
        m.insert("VIII", Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", (12,25) ));
        m
    };
 
    pub static ref REFLECTORS: HashMap<&'static str, Reflector<'static>> = {
        let mut m = HashMap::new();
        m.insert("Alpha",  Reflector::new("LEYJVCNIXWPBQMDRTAKZGFUHOS"));
        m.insert("Beta",   Reflector::new("FSOKANUERHMBTIYCWLQPZXVGJD"));
        m.insert("A",      Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"));
        m.insert("B",      Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"));
        m.insert("C",      Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"));
        m.insert("B-thin", Reflector::new("ENKQAUYWJICOPBLMDXZVFTHRGS"));
        m.insert("C-thin", Reflector::new("RDOBJNTKVEHMLFCWZAXGYIPSUQ"));
        m
    };
}


use std::collections::HashMap;
use super::{Rotor, rotors::Reflector};
 
// References
// This is the M3 Enigma
// https://github.com/aurbano/EnigmaM3_py
// https://cryptii.com/pipes/EnigmaM3-machine
 
 
//These two functions are justified as only ASCII uppercase letters are used in Enigma
fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}
 
fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}
 
 
 
fn parse_plugboard(pairs: &str) -> HashMap<char,char> {
    let mut wiring = HashMap::new();
    let digraphs = pairs.split(" ");
    for d in digraphs {
        if d.len() != 2 {
            panic!("plugboard settings must be pairs of letters")
        }
        let mut cs = d.chars();
        let a = cs.next().unwrap();
        let b = cs.next().unwrap();
        wiring.insert(a,b);
        wiring.insert(b,a);
    }
    wiring
}
 
#[derive(Clone,Debug)]
pub struct Plugboard {
    wiring: HashMap<char,char>,
    pairs: String,
}
 
impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let wiring = match pairs.len() == 0 {
            true =>  HashMap::<char,char>::new(),
            false => parse_plugboard(pairs),
        };
        Plugboard{ wiring, pairs }
    }
 
    pub fn swap(&self, character: char) -> char {
        if self.wiring.contains_key(&character) {
            self.wiring[&character]
        } else {
            character
        }
    }
 
}
 
// This will be the mutating inner state of the Enigma machine. Each time we 
// encrypt with Enigma this state is cloned and run.
pub struct EnigmaState {
    plugboard: Plugboard,
    rotors: (Rotor, Rotor, Rotor),
    reflector: Reflector,
}
 
impl EnigmaState {
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors.2.get_position() == self.rotors.2.get_notch().0 || self.rotors.2.get_position() == self.rotors.2.get_notch().1;
        self.rotors.2.step();
        if on_notch {
            on_notch = self.rotors.1.get_position() == self.rotors.1.get_notch().0 || self.rotors.1.get_position() == self.rotors.1.get_notch().1;
            self.rotors.1.step();
            if on_notch {
                self.rotors.0.step();
            }
        }
    }
 
        // The message key
    pub fn set_rotors(&mut self, rotor_positions: (usize,usize,usize)) {
        self.rotors.0.set_position(rotor_positions.0);
        self.rotors.1.set_position(rotor_positions.1);
        self.rotors.2.set_position(rotor_positions.2);
    }
 
    // Notice that the signal goes through the rotors starting on the right with the 3rd rotor, 
    // then through the reflector, and back through from left to right starting with the 1st rotor
    fn encrypt_char(&mut self, c: char) -> char {
        self.advance_rotors();
        //self.get_rotor_positions();
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors.2.encrypt_rtl(x);
        x = self.rotors.1.encrypt_rtl(x);
        x = self.rotors.0.encrypt_rtl(x);
        x = self.reflector.encrypt(x);
        x = self.rotors.0.encrypt_ltr(x);
        x = self.rotors.1.encrypt_ltr(x);
        x = self.rotors.2.encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }
 
}
 
 
 
#[derive(Clone,Debug)]
pub struct EnigmaM3 {
    state: EngimaState,
    ring_positions: (usize, usize, usize),
}
 
 
impl Cipher for Enigma {
    pub fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut inner_state = self.state.clone();
        text.chars().map(|c| inner_state.encrypt_char(c)).collect()
    }
 
    pub fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt(text)
    }
}
 
 
impl Default for Enigma {
 
}