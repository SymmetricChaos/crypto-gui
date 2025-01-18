use itertools::Itertools;
use num::Integer;

use crate::{errors::CodeError, traits::Code};

use super::string_to_u32s;

pub fn to_binary(mut n: u32, k: u32) -> String {
    let mut s = Vec::with_capacity(k as usize);
    while n != 0 {
        let (q, r) = n.div_rem(&2);
        match r {
            0 => s.push('0'),
            1 => s.push('1'),
            _ => unreachable!("remainder is always less than 2"),
        }
        n = q;
    }
    while (s.len() as u32) < k {
        s.push('0');
    }
    s.iter().rev().collect()
}

pub struct TruncatedBinary {
    pub spaced: bool,
    n: u32,
    k: u32,
    u: u32,
}

impl Default for TruncatedBinary {
    fn default() -> Self {
        Self::new(10)
    }
}

impl TruncatedBinary {
    pub fn new(n: u32) -> Self {
        let k = n.ilog2();
        let u = (1 << (k + 1)) - n;
        Self {
            spaced: false,
            n,
            k,
            u,
        }
    }

    pub fn k(&self) -> u32 {
        self.k
    }

    pub fn u(&self) -> u32 {
        self.u
    }

    pub fn set_consts(&mut self, n: u32) {
        self.n = n;
        self.k = self.n.ilog2();
        self.u = (1 << (self.k + 1)) - self.n;
    }

    pub fn u32_to_bits(&self, x: u32) -> String {
        assert!(x < self.n);
        if x < self.u {
            to_binary(x, self.k)
        } else {
            to_binary(x + self.u, self.k + 1)
        }
    }

    pub fn recognize_code(&self, text: &str) -> Option<u32> {
        if text.len() == self.k as usize {
            let n = u32::from_str_radix(text, 2).unwrap();
            if n < self.u {
                return Some(n);
            } else {
                return None;
            }
        } else if text.len() == (self.k + 1) as usize {
            let n = u32::from_str_radix(text, 2).unwrap() - self.u;
            return Some(n);
        } else {
            None
        }
    }
}

impl Code for TruncatedBinary {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for x in string_to_u32s(text, ",")? {
            if x > self.n {
                out.push(String::from("�"));
            } else {
                out.push(self.u32_to_bits(x))
            }
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let mut buffer = String::with_capacity((self.k + 1) as usize);
        for c in text.chars().filter(|c| *c == '0' || *c == '1') {
            buffer.push(c);
            if let Some(n) = self.recognize_code(&buffer) {
                out.push(n.to_string());
                buffer.clear();
            }
        }
        let mut t = out.into_iter().map(|n| n.to_string()).join(", ");
        if !buffer.is_empty() {
            t.push_str(", �");
        }
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN10: &str = "0, 1, 2, 3, 4, 5, 6, 7, 8, 9";
    const ENC10: &str = "0000010100111001011100110111101111";
    const ENC10SP: &str = "000, 001, 010, 011, 100, 101, 1100, 1101, 1110, 1111";

    #[test]
    fn encode_10_test() {
        let mut code = TruncatedBinary::new(10);
        assert_eq!(ENC10, code.encode(PLAIN10).unwrap());
        code.spaced = true;
        assert_eq!(ENC10SP, code.encode(PLAIN10).unwrap());
    }

    #[test]
    fn decode_10_test() {
        let mut code = TruncatedBinary::new(10);
        assert_eq!(PLAIN10, code.decode(ENC10).unwrap());
        code.spaced = true;
        assert_eq!(PLAIN10, code.decode(ENC10SP).unwrap());
    }
}
