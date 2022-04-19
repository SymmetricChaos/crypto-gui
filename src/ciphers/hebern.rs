use std::{fmt, collections::HashMap};
use crate::errors::CipherError;

pub fn position_map(text: &str) -> HashMap<char,usize> {
    let mut map = HashMap::with_capacity(text.chars().count());
    for (pos,c) in text.chars().enumerate() {
        map.insert(c, pos);
    }
    map
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
	// TODO: this needs to 
    pub fn new(wiring_str:  &str, positions: HashMap<char,usize>) -> Result<HebernRotor,CipherError> {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = vec![0; size];
        let mut wiring_ltr = vec![0; size];
        for w in wiring_str.chars().map(|x| *positions.get(&x).unwrap() ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
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