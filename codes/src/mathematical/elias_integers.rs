use crate::errors::CodeError;
use num::Zero;
use std::collections::BTreeMap;
use utils::bits::{bits_from_str, bits_to_int_little_endian, Bit};

// https://en.wikipedia.org/wiki/Elias_delta_coding
// https://en.wikipedia.org/wiki/Elias_gamma_coding
// https://en.wikipedia.org/wiki/Elias_omega_coding

pub struct OmegaGen {
    pub n: u32,
}

impl OmegaGen {
    pub fn new() -> Self {
        OmegaGen { n: 0 }
    }
}

impl Iterator for OmegaGen {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
        self.n += 1;
        let mut temp_n = self.n;
        let mut out = String::from("0");
        while temp_n > 1 {
            out.insert_str(0, &format!("{:b}", temp_n));
            temp_n = temp_n.ilog2();
        }
        Some((self.n, out))
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EliasVariant {
    Delta,
    Gamma,
    Omega,
}

pub struct EliasCodeIntegers {
    pub variant: EliasVariant,
    current_max: u32,
    delta: DeltaGen,
    pub delta_cache: BTreeMap<u32, String>,
    gamma: GammaGen,
    pub gamma_cache: BTreeMap<u32, String>,
    omega: OmegaGen,
    pub omega_cache: BTreeMap<u32, String>,
}

impl EliasCodeIntegers {
    pub fn extend_all(&mut self, value: u32) {
        while value > self.current_max {
            let (n, c) = self.delta.next().unwrap();
            self.delta_cache.insert(n, c);

            let (n, c) = self.gamma.next().unwrap();
            self.gamma_cache.insert(n, c);

            let (n, c) = self.omega.next().unwrap();
            self.omega_cache.insert(n, c);

            self.current_max += 1;
        }
    }

    pub fn encode_u32(&self, n: u32) -> Option<&String> {
        match self.variant {
            EliasVariant::Delta => self.delta_cache.get(&n),
            EliasVariant::Gamma => self.gamma_cache.get(&n),
            EliasVariant::Omega => self.omega_cache.get(&n),
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
                    // Once we reach a one clear everything but the one from the buffer and get bits for the zeroes counted
                    buffer.clear();
                    buffer.push(Bit::One);
                    for _ in 0..zero_ctr {
                        if let Some(b) = bits.next() {
                            buffer.push(b)
                        } else {
                            return Err(CodeError::input("partial or malformed input"));
                        }
                    }
                    // Convert the bits into an integer

                    out.push(bits_to_int_little_endian(&buffer));
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

    pub fn decode_to_u32_omega(
        &self,
        bits: &mut dyn Iterator<Item = Bit>,
    ) -> Result<Vec<u32>, CodeError> {
        let mut out = Vec::new();
        let mut buffer = Vec::new();
        let mut n = 1;
        loop {
            if let Some(b) = bits.next() {
                buffer.push(b);
                // If we reach a zero stop and return n
                if b.is_zero() {
                    out.push(n);
                    // Reset n
                    n = 1;
                } else {
                    // If we reached a 1 take the next n bits as a number and make them the new value of n
                    for _ in 0..n {
                        if let Some(b) = bits.next() {
                            buffer.push(b)
                        } else {
                            return Err(CodeError::input("partial or malformed input"));
                        }
                    }
                    n = bits_to_int_little_endian(&buffer);
                    buffer.clear();
                }
            } else {
                break;
            }
        }
        Ok(out)
    }

    // Operates on a single codegroup
    pub fn decode_to_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
        let mut filtered = bits_from_str(text).map_err(|e| CodeError::input(&e.to_string()))?;
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
            current_max: 0,
            delta: DeltaGen::new(),
            delta_cache: Default::default(),
            gamma: GammaGen::new(),
            gamma_cache: Default::default(),
            omega: OmegaGen::new(),
            omega_cache: Default::default(),
        }
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

    #[test]
    fn gamma_decode_u32() {
        let mut code = EliasCodeIntegers::default();
        code.variant = EliasVariant::Gamma;
        let codes = "10100110010000101001100011100010000001001";
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            code.decode_to_u32(codes).unwrap()
        );
    }

    #[test]
    fn omega_decode_u32() {
        let mut code = EliasCodeIntegers::default();
        code.variant = EliasVariant::Omega;
        let codes = "010011010100010101010110010111011100001110010";
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            code.decode_to_u32(codes).unwrap()
        );
    }
}
