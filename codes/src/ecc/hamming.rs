use crate::{ecc::check_bitstring, errors::CodeError, traits::Code};

pub struct HammingCode {
    pub parity_bits: usize,
}

impl HammingCode {}

impl Default for HammingCode {
    fn default() -> Self {
        Self { parity_bits: 3 }
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }
}

#[cfg(test)]
mod hamming_tests {
    use super::*;
}
