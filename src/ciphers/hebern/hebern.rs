use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{PresetAlphabet, VecString},
};

use super::rotor::HebernRotor;

#[derive(Clone, Debug)]
pub struct HebernRotorCage {
    pub rotors: Vec<HebernRotor>,
    counters: Vec<u8>,
    rotor_size: u8,
}

impl HebernRotorCage {
    pub fn add_rotor(&mut self, alphabet: &VecString) {
        self.rotors
            .push(HebernRotor::new(&alphabet.to_string(), alphabet).unwrap());
        self.counters.push(0);
    }

    pub fn del_rotor(&mut self) {
        self.rotors.pop();
        self.counters.pop();
    }

    pub fn step(&mut self) {
        // the first rotor always steps
        // the stepping only continues if a rotor completes a full turn by returning to zero
        for (rotor, ctr) in self.rotors.iter_mut().zip(self.counters.iter_mut()) {
            rotor.step();
            *ctr = (*ctr + 1) % self.rotor_size;
            if *ctr != 0 {
                break;
            }
        }
    }

    pub fn encrypt_char(&self, c: char, alphabet: &VecString) -> char {
        let mut n = alphabet.get_pos_of(c).unwrap();
        for rtr in self.rotors.iter() {
            n = rtr.ltr(n)
        }
        alphabet.get_char_at(n).unwrap()
    }

    pub fn decrypt_char(&self, c: char, alphabet: &VecString) -> char {
        let mut n = alphabet.get_pos_of(c).unwrap();
        for rtr in self.rotors.iter().rev() {
            n = rtr.rtl(n)
        }
        alphabet.get_char_at(n).unwrap()
    }
}

impl Default for HebernRotorCage {
    fn default() -> Self {
        let counters = vec![0; 5];
        let alphabet = VecString::from(PresetAlphabet::BasicLatin);

        let mut rotors = Vec::with_capacity(5);
        rotors.push(HebernRotor::new("WQHUFATCNKXZLEJIMRGOBPYVSD", &alphabet).unwrap());
        rotors.push(HebernRotor::new("PTYAUOJWCIRKDXVBGMSZENLHQF", &alphabet).unwrap());
        rotors.push(HebernRotor::new("DZFNREAUCYVSKJPXOHLBITWGQM", &alphabet).unwrap());
        rotors.push(HebernRotor::new("CXIZEGVAYWORLQKJPDFNSTBUHM", &alphabet).unwrap());
        rotors.push(HebernRotor::new("BWQZTNLAFPVJGSYIOMEXHUCDRK", &alphabet).unwrap());

        Self {
            rotors,
            counters,
            rotor_size: 26,
        }
    }
}

pub struct Hebern {
    pub rotors: HebernRotorCage,
    pub alphabet_string: String,
    pub alphabet: VecString,
}

impl Hebern {
    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    fn validate_text(&self, text: &str) -> Option<char> {
        for c in text.chars() {
            if !self.alphabet.contains(c) {
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
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
        }
    }
}

impl Cipher for Hebern {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        if let Some(c) = self.validate_text(text) {
            return Err(Error::invalid_input_char(c));
        }
        let mut rotors = self.rotors.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(rotors.encrypt_char(c, &self.alphabet));
            rotors.step();
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        if let Some(c) = self.validate_text(text) {
            return Err(Error::invalid_input_char(c));
        }
        let mut rotors = self.rotors.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            out.push(rotors.decrypt_char(c, &self.alphabet));
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
