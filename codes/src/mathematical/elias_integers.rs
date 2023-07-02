use num::Zero;
use utils::bits::{bits_from_bitstring, bits_to_int_little_endian, Bit};

use crate::{errors::CodeError, traits::Code};
use std::{cell::RefCell, collections::BTreeMap};

// https://en.wikipedia.org/wiki/Elias_delta_coding
// https://en.wikipedia.org/wiki/Elias_gamma_coding
// https://en.wikipedia.org/wiki/Elias_omega_coding

pub struct OmegaGen {
    pub n: u32,
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
        }
        self.n += 1;
        Some((temp_n, out))
    }
}

pub struct GammaGen {
    pub n: u32,
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
            return Some((self.n, "1".to_string()));
        } else {
            if self.n.is_power_of_two() {
                self.prefix.push('0');
            }
            let out = format!("{}{:b}", self.prefix, self.n);
            Some((self.n, out))
        }
    }
}

pub struct DeltaGen {
    pub n: u32,
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
            Some((self.n, "1".into()))
        } else {
            let p = self.n.ilog2() as usize;
            let l = (p + 1).ilog2() as usize;
            let mut out = "0".repeat(l);
            out.push_str(&format!("{:b}", p + 1));
            out.push_str(&format!("{:b}", self.n)[1..]);
            Some((self.n, out))
        }
    }
}

pub enum EliasVariant {
    Delta,
    Gamma,
    Omega,
}

pub struct EliasCodeIntegers {
    pub variant: EliasVariant,
    delta: RefCell<DeltaGen>,
    delta_cache: RefCell<BTreeMap<u32, String>>,
    gamma: RefCell<GammaGen>,
    gamma_cache: RefCell<BTreeMap<u32, String>>,
    omega: RefCell<OmegaGen>,
    omega_cache: RefCell<BTreeMap<u32, String>>,
}

impl EliasCodeIntegers {
    pub fn encode_u32_delta(&self, n: u32) -> String {
        if let Some(code) = self.delta_cache.borrow().get(&n) {
            code.clone()
        } else {
            while self.delta.borrow().n < n {
                let (n, c) = self.delta.borrow_mut().next().unwrap();
                self.delta_cache.borrow_mut().insert(n, c);
            }
            self.delta_cache.borrow().get(&n).unwrap().clone()
        }
    }

    pub fn encode_u32_gamma(&self, n: u32) -> String {
        if let Some(code) = self.gamma_cache.borrow().get(&n) {
            return code.clone();
        } else {
            while self.gamma.borrow().n < n {
                let (n, c) = self.gamma.borrow_mut().next().unwrap();
                self.gamma_cache.borrow_mut().insert(n, c);
            }
            self.gamma_cache.borrow().get(&n).unwrap().clone()
        }
    }

    pub fn encode_u32_omega(&self, n: u32) -> String {
        if let Some(code) = self.omega_cache.borrow().get(&n) {
            return code.clone();
        } else {
            while self.omega.borrow().n < n {
                let (n, c) = self.omega.borrow_mut().next().unwrap();
                self.omega_cache.borrow_mut().insert(n, c);
            }
            self.omega_cache.borrow().get(&n).unwrap().clone()
        }
    }

    pub fn encode_u32(&self, n: u32) -> String {
        match self.variant {
            EliasVariant::Delta => self.encode_u32_delta(n),
            EliasVariant::Gamma => self.encode_u32_gamma(n),
            EliasVariant::Omega => self.encode_u32_omega(n),
        }
    }

    pub fn decode_to_u32_delta(
        &self,
        bits: &mut dyn Iterator<Item = Bit>,
    ) -> Result<Vec<u32>, CodeError> {
        let mut out = Vec::new();
        let mut buffer = Vec::new();
        let mut zero_ctr = 0;
        loop {
            if let Some(b) = bits.next() {
                buffer.push(b);
                // Count up zeroes until a one is reached
                if b.is_zero() {
                    zero_ctr += 1;
                    continue;
                } else {
                    // Once we reach a one get extra bits equal to the zeroes seen
                    for _ in 0..zero_ctr {
                        if let Some(b) = bits.next() {
                            buffer.push(b)
                        } else {
                            return Err(CodeError::input("partial or malformed input"));
                        }
                    }
                    // Convert the bits into an integer
                    let t = bits_to_int_little_endian(&buffer) - 1;

                    // Take that many more bits
                    buffer.clear();
                    for _ in 0..t {
                        if let Some(b) = bits.next() {
                            buffer.push(b)
                        } else {
                            return Err(CodeError::input("partial or malformed input"));
                        }
                    }

                    let f = bits_to_int_little_endian(&buffer);

                    out.push(2_u32.pow(t) + f);
                    // Clear buffer and counter
                    buffer.clear();
                    zero_ctr = 0;
                }
            } else {
                break;
            }
        }
        Ok(out)
    }

    pub fn decode_to_u32_gamma(
        &self,
        bits: &mut dyn Iterator<Item = Bit>,
    ) -> Result<Vec<u32>, CodeError> {
        todo!()
    }

    pub fn decode_to_u32_omega(
        &self,
        bits: &mut dyn Iterator<Item = Bit>,
    ) -> Result<Vec<u32>, CodeError> {
        todo!()
    }

    // Operates on a single codegroup
    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
        let mut filtered = bits_from_bitstring(text).ok_or(CodeError::input(
            "input should be only 0s, 1s, and whitespace",
        ))?;
        match self.variant {
            EliasVariant::Delta => self.decode_to_u32_delta(&mut filtered),
            EliasVariant::Gamma => self.decode_to_u32_gamma(&mut filtered),
            EliasVariant::Omega => self.decode_to_u32_omega(&mut filtered),
        }
    }
}

impl Default for EliasCodeIntegers {
    fn default() -> Self {
        Self {
            variant: EliasVariant::Delta,
            delta: RefCell::new(DeltaGen::new()),
            delta_cache: Default::default(),
            gamma: RefCell::new(GammaGen::new()),
            gamma_cache: Default::default(),
            omega: RefCell::new(OmegaGen::new()),
            omega_cache: Default::default(),
        }
    }
}

impl Code for EliasCodeIntegers {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| CodeError::invalid_input_group(s))?;
            out.push_str(&self.encode_u32(n))
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod elias_int_tests {
    use super::*;

    #[test]
    fn delta_codes() {
        let codes = DeltaGen::new();
        for ((_, code), check) in codes.zip([
            "1", "0100", "0101", "01100", "01101", "01110", "01111", "00100000", "00100001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn gamma_codes() {
        let codes = GammaGen::new();
        for ((_, code), check) in codes.zip([
            "1", "010", "011", "00100", "00101", "00110", "00111", "0001000", "0001001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn omega_codes() {
        let codes = OmegaGen::new();
        for ((_, code), check) in codes.zip([
            "0", "100", "110", "101000", "101010", "101100", "101110", "1110000", "1110010",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn delta_decode_u32() {
        let code = EliasCodeIntegers::default();
        let codes = "101000101011000110101110011110010000000100001";
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            code.decode_to_u32(codes).unwrap()
        );
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
