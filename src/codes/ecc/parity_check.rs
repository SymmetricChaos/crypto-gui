use super::{bits_from_bitstring, Bit};
use crate::{codes::Code, errors::Error};

pub struct ParityBit {
    pub block_size: usize,
    pub inverted: bool,
}

impl ParityBit {}

impl Default for ParityBit {
    fn default() -> Self {
        Self {
            block_size: 4,
            inverted: false,
        }
    }
}

impl Code for ParityBit {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut parity = Bit::Zero;
        let mut ctr = 0;
        let mut out = String::new();
        for bit in bits_from_bitstring(text) {
            ctr += 1;
            out.push(bit.as_char());
            parity ^= bit;

            if ctr == self.block_size {
                if self.inverted {
                    out.push(parity.flipped().as_char());
                } else {
                    out.push(parity.as_char());
                }

                ctr = 0;
                parity = Bit::Zero;
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut parity = Bit::Zero;
        let mut ctr = 0;
        let mut out = String::new();
        let mut buffer = String::new();
        for bit in bits_from_bitstring(text) {
            ctr += 1;

            if ctr == self.block_size + 1 {
                if self.inverted {
                    if parity != bit {
                        out.push_str(&buffer);
                    } else {
                        out.push_str(&"�".repeat(self.block_size))
                    }
                } else {
                    if parity == bit {
                        out.push_str(&buffer);
                    } else {
                        out.push_str(&"�".repeat(self.block_size))
                    }
                }
                ctr = 0;
                parity = Bit::Zero;
                buffer.clear();
            } else {
                parity ^= bit;
                buffer.push(bit.as_char());
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod parity_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = ParityBit::default();
        assert_eq!(code.encode("111010010000").unwrap(), "111011001000000");
    }

    #[test]
    fn test_encode_inv() {
        let mut code = ParityBit::default();
        code.inverted = true;
        assert_eq!(code.encode("111010010000").unwrap(), "111001001100001");
    }

    #[test]
    fn test_decode() {
        let code = ParityBit::default();
        assert_eq!(code.decode("111011001000000").unwrap(), "111010010000");
    }

    #[test]
    fn test_decode_inv() {
        let mut code = ParityBit::default();
        code.inverted = true;
        assert_eq!(code.decode("111001001100001").unwrap(), "111010010000");
    }

    #[test]
    fn test_decode_with_err() {
        let mut code = ParityBit::default();
        code.block_size = 4;
        assert_eq!(code.decode("111001001000000").unwrap(), "����10010000");
    }
}
