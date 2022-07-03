use std::fmt;

use itertools::Itertools;

use crate::{
    errors::CipherError,
    global_rng::get_global_rng,
    text_aux::{PresetAlphabet, VecString}, ciphers::Cipher,
};

#[derive(Clone, Debug)]
pub struct HebernRotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr: Vec<usize>,
    pub position: usize,
    pub wiring_str: String,
    size: usize,
    pub editable: bool,
    pub error: String,
}

impl HebernRotor {
    pub fn new(wiring_str: &str, alphabet: &VecString) -> Result<HebernRotor, CipherError> {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = vec![0; size];
        let mut wiring_ltr = vec![0; size];

        for (pos, c) in wiring_str.chars().enumerate() {
            let n = alphabet
                .get_pos_of(c)
                .ok_or(CipherError::invalid_input_char(c))?;
            wiring_rtl[pos] = n;
            wiring_ltr[n] = pos;
        }
        Ok(HebernRotor {
            wiring_rtl,
            wiring_ltr,
            position: 0,
            wiring_str: wiring_str.to_string(),
            size,
            editable: false,
            error: String::new(),
        })
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }

    // We will use and return usize instead of char to avoid constantly converting types
    pub fn rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_rtl[inner_position];
        (inner + self.size - self.position) % self.size
    }

    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_ltr[inner_position];
        (inner + self.size - self.position) % self.size
    }

    pub fn set(&mut self, alphabet: &VecString) -> Result<(),CipherError> {
        if !self.editable {
            return Ok(())
        }

        let total_size = self.wiring_str.chars().count();
        if total_size != self.size {
            return Err(CipherError::General(format!("must provide exactly {} characters", self.size)))
        }
        let unique_size = self.wiring_str.chars().unique().count();
        if unique_size != total_size {
            return Err(CipherError::General(String::from("duplicate characters are not allowed")))
        }

        let mut new_wiring_rtl = vec![0; self.size];
        let mut new_wiring_ltr = vec![0; self.size];
        for (pos, c) in self.wiring_str.chars().enumerate() {
            let n = alphabet
                .get_pos_of(c)
                .ok_or(CipherError::invalid_input_char(c))?;

            new_wiring_rtl[pos] = n;
            new_wiring_ltr[n] = pos;
        }
        self.wiring_rtl = new_wiring_rtl;
        self.wiring_ltr = new_wiring_ltr;
        Ok(())
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

#[derive(Clone, Debug)]
pub struct HebernRotorCage {
    pub rotors: Vec<HebernRotor>,
    pub locks: Vec<bool>,
    pub alphabet_string: String,
    pub alphabet: VecString,
    counters: Vec<u8>,
    rotor_size: u8,
}

impl HebernRotorCage {
    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
        &mut self.alphabet_string
    }

    pub fn add_rotor(&mut self) {
        self.rotors
            .push(HebernRotor::new(&self.alphabet.to_string(), &self.alphabet).unwrap());
        self.counters.push(0);
        self.locks.push(false);
    }

    pub fn del_rotor(&mut self) {
        self.rotors.pop();
        self.counters.pop();
        self.locks.pop();
    }

    pub fn step(&mut self) {
        // the first rotor always steps
        // the stepping only continues if a rotor completes a full turn by returning to zero
        for (n, ctr) in self.counters.iter_mut().enumerate() {
            self.rotors[n].step();
            *ctr = (*ctr + 1) % self.rotor_size;
            if *ctr != 0 {
                break;
            }
        }
    }

    pub fn encrypt_char(&self, c: char) -> char {
        let mut n = self.alphabet.get_pos_of(c).unwrap();
        for rtr in self.rotors.iter() {
            n = rtr.ltr(n)
        }
        self.alphabet.get_char_at(n).unwrap()
    }

    pub fn decrypt_char(&self, c: char) -> char {
        let mut n = self.alphabet.get_pos_of(c).unwrap();
        for rtr in self.rotors.iter().rev() {
            n = rtr.rtl(n)
        }
        self.alphabet.get_char_at(n).unwrap()
    }
}

impl Default for HebernRotorCage {
    fn default() -> Self {
        let alphabet_string = String::from(PresetAlphabet::BasicLatin);
        let alphabet = VecString::from(&alphabet_string);

        let counters = vec![0; 5];

        let locks = vec![false; 5];

        let mut rotors = Vec::with_capacity(5);
        rotors.push(HebernRotor::new("WQHUFATCNKXZLEJIMRGOBPYVSD", &alphabet).unwrap());
        rotors.push(HebernRotor::new("PTYAUOJWCIRKDXVBGMSZENLHQF", &alphabet).unwrap());
        rotors.push(HebernRotor::new("DZFNREAUCYVSKJPXOHLBITWGQM", &alphabet).unwrap());
        rotors.push(HebernRotor::new("CXIZEGVAYWORLQKJPDFNSTBUHM", &alphabet).unwrap());
        rotors.push(HebernRotor::new("BWQZTNLAFPVJGSYIOMEXHUCDRK", &alphabet).unwrap());

        Self {
            rotors,
            locks,
            alphabet_string,
            alphabet,
            counters,
            rotor_size: 26,
        }
    }
}

pub struct Hebern {
    pub rotors: HebernRotorCage,
}

impl Hebern {
    fn validate_text(&self, text: &str) -> Option<char> {
        for c in text.chars() {
            if !self.rotors.alphabet.contains(c) {
                return Some(c);
            }
        }
        None
    }
}

impl Default for Hebern {
    fn default() -> Self {
        Self {
            rotors: HebernRotorCage::default(),
        }
    }
}

impl Cipher for Hebern {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if let Some(c) = self.validate_text(text) {
            return Err(CipherError::invalid_input_char(c));
        }
        let mut rotors = self.rotors.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(rotors.encrypt_char(c));
            rotors.step();
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        if let Some(c) = self.validate_text(text) {
            return Err(CipherError::invalid_input_char(c));
        }
        let mut rotors = self.rotors.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(rotors.decrypt_char(c));
            rotors.step();
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        todo!("{:?}", &mut get_global_rng())
    }

    fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod hebern_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "PHJXRXAVPGSDMLKZFFFGGKFYYMVMLXAYHEP";

    #[test]
    fn encrypt_test() {
        let cipher = Hebern::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Hebern::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
