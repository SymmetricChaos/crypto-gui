use super::{bits_from_bitstring, Bit};
use crate::{codes::Code, errors::Error};

pub struct PairtyBit {
    pub block_size: usize,
    pub inverted: bool,
}

impl PairtyBit {}

impl Default for PairtyBit {
    fn default() -> Self {
        Self {
            block_size: 4,
            inverted: false,
        }
    }
}

impl Code for PairtyBit {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut parity = Bit::Zero;
        let mut ctr = 0;
        let mut out = String::new();
        for bit in bits_from_bitstring(text) {
            ctr += 1;
            out.push(bit.as_char());
            parity ^= bit;

            if ctr == self.block_size {
                out.push(parity.as_char());
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
            parity ^= bit;

            if ctr == self.block_size + 1 {
                if parity == bit {
                    out.push_str(&buffer);
                } else {
                    out.push_str(&"�".repeat(self.block_size))
                }
                ctr = 0;
                parity = Bit::Zero;
                buffer.clear();
            } else {
                buffer.push(bit.as_char());
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod repetition_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let code = PairtyBit::default();
        assert_eq!(code.encode("111010010000").unwrap(), "111011001000000");
    }

    #[test]
    fn test_decode() {
        let code = PairtyBit::default();
        assert_eq!(code.decode("111011001000000").unwrap(), "111010010000");
    }

    #[test]
    fn test_decode_with_err() {
        let mut code = PairtyBit::default();
        code.block_size = 4;
        assert_eq!(code.decode("").unwrap(), "");
    }
}
