use crate::{
    codes::{ecc::check_bitstring, Code},
    errors::Error,
};

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
    fn encode(&self, text: &str) -> Result<String, Error> {
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod hamming_tests {
    use super::*;
}
