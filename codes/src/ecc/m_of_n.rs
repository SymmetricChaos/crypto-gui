use itertools::Itertools;
use num::{integer::binomial, Integer};
use utils::bits::{bits_from_str, Bit};

use crate::{errors::CodeError, traits::Code};

pub fn u32_to_m_of_n(n: u32, w: usize, l: usize) -> Option<String> {
    let mut n = n;
    let mut s = Vec::with_capacity(l);
    let mut counted_weight = 0;
    while n != 0 {
        let (q, r) = n.div_rem(&2);
        if r == 0 {
            s.push('0');
        } else if r == 1 {
            counted_weight += 1;
            s.push('1');
        } else {
            unreachable!("remainder after division by 2 is always 0 or 1")
        }
        n = q;
    }
    if counted_weight > w || s.len() >= l {
        return None;
    }
    while s.len() < l - w {
        s.push('0');
    }
    s.reverse();
    for _ in 0..(w - counted_weight) {
        s.push('1');
    }
    while s.len() < l {
        s.push('0')
    }
    Some(s.into_iter().join(""))
}

pub struct MofNCode {
    pub weight: usize,
    pub length: usize,
    pub spaced: bool,
}

impl MofNCode {
    pub fn n_data_bits(&self) -> usize {
        self.length - self.weight
    }

    pub fn validate(&self) -> Result<(), CodeError> {
        if self.length > 10 {
            return Err(CodeError::state(
                "lengths greater than 10 not currently supported",
            ));
        }
        if self.weight >= self.length {
            return Err(CodeError::state("weight must be less than length"));
        }
        Ok(())
    }

    pub fn total_codes(&self) -> usize {
        binomial(self.length, self.weight)
    }
}

impl Default for MofNCode {
    fn default() -> Self {
        Self {
            weight: 2,
            length: 5,
            spaced: false,
        }
    }
}

impl Code for MofNCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;

        let n_data_bits = self.n_data_bits();
        if bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .count()
            % n_data_bits
            != 0
        {
            return Err(CodeError::Input(format!(
                "when encoding an {}-of-{} code the input must have a length that is a multiple of {}",
                self.weight, self.length, n_data_bits
            )));
        };

        let bits = bits_from_str(text).map_err(|e| CodeError::Input(e.to_string()))?;

        let mut out = Vec::new();
        let mut counted_weight = 0;
        let mut buffer = String::new();
        for chunk in &bits.chunks(n_data_bits) {
            for bit in chunk {
                counted_weight += bit;
                buffer.push(char::from(bit));
            }

            if counted_weight > self.weight {
                return Err(CodeError::Input(format!(
                    "encoutered more than {} set bits",
                    self.weight
                )));
            }

            buffer.push_str(&"1".repeat(self.weight - counted_weight));
            while buffer.len() < self.length {
                buffer.push('0')
            }
            out.push(buffer.clone());

            counted_weight = 0;
            buffer.clear();
        }

        Ok(out.join(if self.spaced { ", " } else { "" }))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;

        let n_data_bits = self.n_data_bits();

        let bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % self.length != 0 {
            return Err(CodeError::Input(format!(
                "when decoding an {}-of-{} code the input must have a length that is a multiple of {}",
                self.weight, self.length, self.length
            )));
        };

        let mut out = String::new();
        let mut ctr = 0;
        let mut counted_weight = 0;
        let mut buffer = String::new();
        for bit in bits {
            ctr += 1;
            counted_weight += bit;
            if ctr <= n_data_bits {
                buffer.push(char::from(bit));
            } else if ctr == self.length {
                if counted_weight == self.weight {
                    out.push_str(&buffer)
                } else {
                    out.push_str(&"?".repeat(n_data_bits))
                }
                ctr = 0;
                counted_weight = 0;
                buffer.clear();
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod m_of_n_tests {
    use super::*;
    const PLAINTEXT: &'static str = "011110000";

    // #[test]
    // fn encode_int_test() {
    //     for i in 0..20 {
    //         println!("{:?}", u32_to_m_of_n(i, 2, 5))
    //     }
    // }

    #[test]
    fn encode_test() {
        let code = MofNCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), "011001100000011");
    }

    #[test]
    fn decode_test() {
        let code = MofNCode::default();
        assert_eq!(code.decode("011001100000011").unwrap(), "011110000");
    }

    #[test]
    fn decode_with_errors_test() {
        let code = MofNCode::default();
        assert_eq!(code.decode("011001100001011").unwrap(), "011110???");
    }
}
