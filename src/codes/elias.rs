use bimap::BiMap;

use super::Code;
use crate::errors::Error;
// https://en.wikipedia.org/wiki/Elias_omega_coding
// https://en.wikipedia.org/wiki/Elias_gamma_coding
// https://en.wikipedia.org/wiki/Elias_delta_coding

pub struct OmegaGen {
    n: usize,
}

impl OmegaGen {
    pub fn new() -> Self {
        OmegaGen { n: 1 }
    }
}

impl Iterator for OmegaGen {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut temp_n = self.n as u32;
        let mut out = String::from("0");
        while temp_n > 1 {
            println!("{temp_n}");
            out.insert_str(0, &format!("{:b}", temp_n));
            temp_n = temp_n.ilog2();
            println!("{temp_n}");
        }
        println!("");
        self.n += 1;
        Some(out)
    }
}

pub struct GammaGen {
    n: usize,
    prefix: String,
}

impl GammaGen {
    pub fn new() -> Self {
        GammaGen {
            n: 0,
            prefix: String::new(),
        }
    }
}

impl Iterator for GammaGen {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.n += 1;
        if self.n == 1 {
            return Some("1".to_string());
        } else {
            if self.n.is_power_of_two() {
                self.prefix.push('0');
            }
            let out = format!("{}{:b}", self.prefix, self.n);
            Some(out)
        }
    }
}

pub struct DeltaGen {
    n: usize,
}

impl DeltaGen {
    pub fn new() -> Self {
        DeltaGen { n: 0 }
    }
}

impl Iterator for DeltaGen {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.n += 1;
        if self.n == 1 {
            Some("1".into())
        } else {
            let p = self.n.ilog2() as usize;
            let l = (p + 1).ilog2() as usize;
            let mut out = "0".repeat(l);
            out.push_str(&format!("{:b}", p + 1));
            out.push_str(&format!("{:b}", self.n)[1..]);
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
    pub fn codes(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            EliasMode::Delta => Box::new(DeltaGen::new()),
            EliasMode::Gamma => Box::new(GammaGen::new()),
            EliasMode::Omega => Box::new(OmegaGen::new()),
        }
    }
}

pub struct EliasCode {
    map: BiMap<char, String>,
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
            for (l, c) in self.alphabet.chars().zip(codes) {
                self.max_code_len = c.chars().count();
                self.map.insert(l, c.clone());
            }
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.set_maps();
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get_by_left(&x).unwrap()))
    }
}

impl Default for EliasCode {
    fn default() -> Self {
        let alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        let codes = GammaGen::new();
        let mut map = BiMap::new();
        for (l, c) in alphabet.chars().zip(codes) {
            map.insert(l, c);
        }
        let max_code_len = map.right_values().map(|c| c.chars().count()).max().unwrap();
        EliasCode {
            map,
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
            let code = self
                .map
                .get_by_left(&s)
                .ok_or_else(|| Error::invalid_input_char(s))?;
            output.push_str(&code)
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        // let mut output = String::new();
        // let mut buffer = String::with_capacity(self.max_code_len);
        // let mut ctr = 0;
        // for b in text.chars() {
        //     buffer.push(b);
        //     ctr += 1;
        //     if let Some(s) = self.map_inv.get(&buffer) {
        //         output.push(*s);
        //         buffer.clear();
        //         ctr = 0;
        //     }
        //     // If we have an impossible code ignore it and start again, it will eventually
        //     // resychronize
        //     if ctr == self.max_code_len {
        //         buffer.clear();
        //         ctr = 0;
        //     }
        // }
        // Ok(output)
        todo!()
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
    fn delta_codes() {
        let codes = DeltaGen::new();
        for (code, check) in codes.zip([
            "1", "0100", "0101", "01100", "01101", "01110", "01111", "00100000", "00100001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn gamma_codes() {
        let codes = GammaGen::new();
        for (code, check) in codes.zip([
            "1", "010", "011", "00100", "00101", "00110", "00111", "0001000", "0001001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn omega_codes() {
        let codes = OmegaGen::new();
        for (code, check) in codes.zip([
            "0", "100", "110", "101000", "101010", "101100", "101110", "1110000", "1110010",
        ]) {
            assert_eq!(code, check)
        }
    }

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
