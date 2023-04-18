use super::Code;
use crate::errors::Error;
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap};

// https://en.wikipedia.org/wiki/Elias_delta_coding
// https://en.wikipedia.org/wiki/Elias_gamma_coding
// https://en.wikipedia.org/wiki/Elias_omega_coding

lazy_static! {
    pub static ref DLETA: OmegaGen = OmegaGen::new();
}

pub enum EliasMode {
    Delta,
    Gamma,
    Omega,
}

pub struct OmegaGen {
    n: usize,
}

impl OmegaGen {
    pub fn new() -> Self {
        OmegaGen { n: 1 }
    }
}

impl Iterator for OmegaGen {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
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
        Some((temp_n as u32, out))
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
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
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
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
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

pub struct EliasCodeIntegers {
    pub variant: EliasMode,
    cached_codes_delta: RefCell<HashMap<u32, String>>,
    cached_codes_gamma: RefCell<HashMap<u32, String>>,
    cached_codes_omega: RefCell<HashMap<u32, String>>,
}

impl EliasCodeIntegers {
    pub fn encode_u32(&self, n: u32) -> String {
        match self.variant {
            EliasMode::Delta => todo!(),
            EliasMode::Gamma => todo!(),
            EliasMode::Omega => todo!(),
        }
        // Quickly check if the number has been encoded before
        if let Some(code) = self.cached_codes.borrow().get(&n) {
            return code.clone();
        }
    }

    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, Error> {
        todo!()
    }

    fn extend_caches(&self) {
        let (n, c) = DLETA.next().unwrap();
        self.cached_codes_delta.borrow_mut().insert(n, c);
    }
}

impl Default for EliasCodeIntegers {
    fn default() -> Self {}
}

impl Code for EliasCodeIntegers {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| Error::invalid_input_group(s))?;
            out.push_str(&self.encode_u32(n))
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        todo!()
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod elias_int_tests {
    use super::*;

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

    const PLAINTEXT: &'static str = "";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encode_test() {
        let code = EliasCodeIntegers::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = EliasCodeIntegers::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
