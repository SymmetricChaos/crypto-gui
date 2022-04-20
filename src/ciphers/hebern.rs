use std::{fmt, collections::HashMap};

use crate::errors::CipherError;

pub struct Alphabet {
    characters: Vec<char>,
    positions: HashMap<char,usize>
}

impl Alphabet {
    pub fn new(alphabet: &str) -> Self {
        let characters = alphabet.chars().collect();
        let mut positions = HashMap::with_capacity(alphabet.chars().count());
        for (pos,c) in alphabet.chars().enumerate() {
            positions.insert(c, pos);
        }
        Self{ characters, positions }
    }

    pub fn get_char(&self, n: usize) -> Option<char> {
        self.characters.get(n).map(|n| *n)
    }
    
    pub fn get_pos(&self, c: char) -> Option<usize> {
        self.positions.get(&c).map(|n| *n)
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.characters.iter().collect::<String>())
    }
}

 
// Specifically the Enigma rotor
#[derive(Clone,Debug)]
pub struct HebernRotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr:  Vec<usize>,
    pub position: usize,
    pub wiring_str: String,
    size: usize,
}

impl HebernRotor {

    pub fn new(wiring_str: &str, alphabet: &Alphabet) -> Result<HebernRotor,CipherError> {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = vec![0; size];
        let mut wiring_ltr = vec![0; size];

        for (pos, c) in wiring_str
                .chars().enumerate() {
            let n = alphabet.get_pos(c).ok_or(CipherError::invalid_input_char(c))?;
            wiring_rtl[pos] = n;
            wiring_ltr[n]   = pos;
        }
        Ok(HebernRotor{ wiring_rtl, wiring_ltr, position: 0, wiring_str: wiring_str.to_string(), size })
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner =  self.wiring_rtl[inner_position];
        (inner + self.size - self.position) % self.size
    }

    pub fn encrypt_ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_ltr[inner_position];
        (inner + self.size - self.position) % self.size
    }
}


impl fmt::Display for HebernRotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(self.size);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}

pub struct Hebern {
    pub rotors: Vec<HebernRotor>,
    pub alphabet_string: String,
    alphabet: Alphabet,
}