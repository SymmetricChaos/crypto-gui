use crate::{errors::CodeError, traits::Code};

use super::{bits_from_bitstring, check_bitstring};

pub struct MofNCode {
    pub weight: usize,
    pub length: usize,
}

impl MofNCode {
    pub fn n_data_bits(&self) -> usize {
        self.length - self.weight
    }
}

impl Default for MofNCode {
    fn default() -> Self {
        Self {
            weight: 2,
            length: 5,
        }
    }
}

impl Code for MofNCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        let n_data_bits = self.n_data_bits();
        if bits_from_bitstring(text).count() % n_data_bits != 0 {
            return Err(CodeError::Input(format!(
                "when encoding an {}-of-{} code must have a length that is a multiple of {}",
                self.weight, self.length, n_data_bits
            )));
        };

        let bits = bits_from_bitstring(text);

        let mut out = String::new();
        let mut ctr = 0;
        let mut counted_weight = 0;
        let mut buffer = String::new();
        for bit in bits {
            let b = bit;
            ctr += 1;
            counted_weight += b;
            buffer.push(b.as_char());

            if ctr == n_data_bits {
                buffer.push_str(&"1".repeat(self.weight - counted_weight));
                while buffer.len() < self.length {
                    buffer.push('0')
                }
                out.push_str(&buffer);
                ctr = 0;
                counted_weight = 0;
                buffer.clear();
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        let n_data_bits = self.n_data_bits();
        if bits_from_bitstring(text).count() % self.length != 0 {
            return Err(CodeError::Input(format!(
                "when decoding an {}-of-{} code must have a length that is a multiple of {}",
                self.weight, self.length, self.length
            )));
        };

        let bits = bits_from_bitstring(text);

        let mut out = String::new();
        let mut ctr = 0;
        let mut counted_weight = 0;
        let mut buffer = String::new();
        for bit in bits {
            ctr += 1;
            counted_weight += bit;
            if ctr <= n_data_bits {
                buffer.push(bit.as_char());
            } else if ctr == self.length {
                if counted_weight == self.weight {
                    out.push_str(&buffer)
                } else {
                    out.push_str(&"�".repeat(n_data_bits))
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
        assert_eq!(code.decode("011001100001011").unwrap(), "011110���");
    }
}