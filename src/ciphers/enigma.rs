use super::Cipher;
use crate::{errors::CipherError, text_types::PresetAlphabet};

use lazy_static::lazy_static;
use rand::prelude::ThreadRng;
use std::collections::HashMap;
 
#[derive(Clone,Debug,Copy)]
pub struct Rotor {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    pub notch: (usize,usize),
    pub position: usize,
    pub ring: usize,
    pub wiring_str: &'static str,
}
 
impl Rotor {
	// TODO: Clearer initialization?
    pub fn new(wiring_str:  &'static str, notch: (usize,usize)) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, notch, position: 0, ring: 0, wiring_str }
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
 
#[derive(Clone,Debug,Copy)]
pub struct Reflector {
    wiring: [usize; 26],
    pub wiring_str: &'static str,
}
 
impl Reflector {
	// TODO: Clearer initialization?
    pub fn new(wiring_str: &'static str) -> Reflector {
        let mut wiring_internal: [usize; 26] = [0; 26];
        for w in wiring_str.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_internal[w.0] = w.1;
        }
        Reflector{ wiring: wiring_internal, wiring_str }
    }
 
    // We take and return usize to be consistent with Rotor
    // No decrypt is needed as reflectors are reciprocal
    pub fn encrypt(&self, entry: usize) -> usize {
        self.wiring[entry]
    }
}
 
lazy_static! {
    pub static ref ROTORS: HashMap<&'static str, Rotor> = {
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
 
    pub static ref REFLECTORS: HashMap<&'static str, Reflector> = {
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

 
// References
// This is the M3 Enigma
// https://github.com/aurbano/EnigmaM3_py
// https://cryptii.com/pipes/EnigmaM3-machine
 
 
// These two functions are justified as only ASCII uppercase letters are used in Enigma
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
}
 
impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let wiring = match pairs.len() == 0 {
            true =>  HashMap::<char,char>::new(),
            false => parse_plugboard(pairs),
        };
        Plugboard{ wiring }
    }
 
    pub fn swap(&self, character: char) -> char {
        if self.wiring.contains_key(&character) {
            self.wiring[&character]
        } else {
            character
        }
    }
}

impl Default for Plugboard {
    fn default() -> Self {
        Self { wiring: HashMap::new() }
    }
}
 
// This will be the mutating inner state of the Enigma machine. Each time we 
// encrypt with Enigma this state is cloned and run.
// Cloning Rotors and Reflectors is cheap as they are Copy. Plugboard is 
// small and so should be cheap to Clone.
#[derive(Clone,Debug)]
pub struct EnigmaState {
    pub plugboard: Plugboard,
    pub rotors: [Rotor; 3],
    pub reflector: Reflector,
}
 
impl EnigmaState {
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors[2].position == self.rotors[2].notch.0 || self.rotors[2].position == self.rotors[2].notch.1;
        self.rotors[2].step();
        if on_notch {
            on_notch = self.rotors[1].position == self.rotors[1].notch.0 || self.rotors[1].position == self.rotors[1].notch.1;
            self.rotors[1].step();
            if on_notch {
                self.rotors[0].step();
            }
        }
    }
 
    // The message key
    pub fn set_rotors(&mut self, rotor_positions: (usize,usize,usize)) {
        self.rotors[0].position = rotor_positions.0;
        self.rotors[1].position = rotor_positions.1;
        self.rotors[2].position = rotor_positions.2;
    }

    pub fn set_rings(&mut self, rotor_ring_positions: (usize,usize,usize)) {
        self.rotors[0].ring = rotor_ring_positions.0;
        self.rotors[1].ring = rotor_ring_positions.1;
        self.rotors[2].ring = rotor_ring_positions.2;
    }
 
    // Notice that the signal goes through the rotors starting on the right with the 3rd rotor, 
    // then through the reflector, and back through from left to right starting with the 1st rotor
    fn encrypt_char(&mut self, c: char) -> char {
        self.advance_rotors();
        //self.get_rotor_positions();
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors[2].encrypt_rtl(x);
        x = self.rotors[1].encrypt_rtl(x);
        x = self.rotors[0].encrypt_rtl(x);
        x = self.reflector.encrypt(x);
        x = self.rotors[0].encrypt_ltr(x);
        x = self.rotors[1].encrypt_ltr(x);
        x = self.rotors[2].encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }
}

impl Default for EnigmaState {
    fn default() -> Self {
        Self { plugboard: Default::default(), rotors: [ROTORS["I"], ROTORS["II"], ROTORS["III"]], reflector: REFLECTORS["A"] }
    }
}
 
 
 
#[derive(Clone,Debug)]
pub struct EnigmaM3 {
    alphabet: String,
    state: EnigmaState,
}
 
 
impl Cipher for EnigmaM3 {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut inner_state = self.state.clone();
        Ok(text.chars().map(|c| inner_state.encrypt_char(c)).collect())
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt(text)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("Enigma uses a historically accurate alphabet that should not be changed")
    }

    fn validate_settings(&self) -> Result<(),CipherError> {
        todo!()
    }
}
 
 
impl Default for EnigmaM3 {
    fn default() -> Self {
        Self { alphabet: String::from(PresetAlphabet::BasicLatin), state: Default::default() }
    }
}