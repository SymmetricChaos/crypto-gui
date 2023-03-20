use std::collections::HashMap;

use crate::errors::Error;

use super::Code;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct GammaGen {
    n: usize,
    prefix: String,
}

impl GammaGen {
    pub fn new() -> Self {
        GammaGen {
            n: 1,
            prefix: String::new(),
        }
    }
}

impl Iterator for GammaGen {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.n == 1 {
            self.n += 1;
            return Some("1".to_string());
        } else {
            if self.n.is_power_of_two() {
                self.prefix.push('0');
            }
            let out = format!("{}{:b}", self.prefix, self.n);
            self.n += 1;
            Some(out)
        }
    }
}

pub struct DeltaGen {
    n: usize,
    prefix: String,
}

impl DeltaGen {
    pub fn new() -> Self {
        DeltaGen {
            n: 1,
            prefix: String::new(),
        }
    }
}

impl Iterator for DeltaGen {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.n == 1 {
            self.n += 1;
            return Some("1".to_string());
        } else {
            if self.n.is_power_of_two() {
                self.prefix.push('0');
            }
            let out = format!("{}{:b}", self.prefix, self.n);
            self.n += 1;
            Some(out)
        }
    }
}

pub enum EliasMode {
    Delta,
    Gamma,
    Omega,
}

impl EliasMode {
    pub fn codes(&self) -> impl Iterator<Item = String> {
        match self {
            EliasMode::Delta => todo!(),
            EliasMode::Gamma => GammaGen::new(),
            EliasMode::Omega => todo!(),
        }
    }
}

pub struct EliasCode {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: String,
    old_alphabet: String,
    max_code_len: usize,
    mode: EliasMode,
}

impl EliasCode {
    pub fn control_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    // This needs to be called before encoding or decoding to be
    // sure that the maps are up to date. In the egui interface
    // this is taken care of by embedding it in the chars_codes()
    // method.
    // It would make more sense to put it in the control_alphabet()
    // method but that causes a panic due to interaction with
    // the chars_codes() method.
    pub fn set_maps(&mut self) {
        if self.alphabet != self.old_alphabet {
            let codes = self.mode.codes();
            self.map.clear();
            self.map_inv.clear();
            for (l, c) in self.alphabet.chars().zip(codes) {
                self.map.insert(l, c.clone());
                self.map_inv.insert(c.clone(), l);
            }
            self.max_code_len = self.map[&self.alphabet.chars().last().unwrap()]
                .chars()
                .count();
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.set_maps();
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get(&x).unwrap()))
    }
}

impl Default for EliasCode {
    fn default() -> Self {
        let alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        let codes = GammaGen::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l, c) in alphabet.chars().zip(codes) {
            map.insert(l, c.clone());
            map_inv.insert(c, l);
        }
        let max_code_len = map[&alphabet.chars().last().unwrap()].chars().count();
        EliasCode {
            map,
            map_inv,
            alphabet: alphabet.clone(),
            old_alphabet: alphabet,
            max_code_len,
            mode: EliasMode::Gamma,
        }
    }
}

impl Code for EliasCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(&self.map[&s])
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.max_code_len);
        let mut ctr = 0;
        for b in text.chars() {
            buffer.push(b);
            ctr += 1;
            if let Some(s) = self.map_inv.get(&buffer) {
                output.push(*s);
                buffer.clear();
                ctr = 0;
            }
            // If we have an impossible code ignore it and start again, it will eventually
            // resychronize
            if ctr == self.max_code_len {
                buffer.clear();
                ctr = 0;
            }
        }
        Ok(output)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod elias_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "0100001000100001100100011010010100011000000101100000101000001001001000001111001100000100000010000001100000001011100011010001110000010011001110010000001010110001001010000100010001011011000011010000010010000101000100000010001";

    #[test]
    fn encrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
