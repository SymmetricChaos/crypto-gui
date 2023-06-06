use crate::{ecc::check_bitstring, errors::CodeError, traits::Code};

pub struct HammingCode {
    pub parity_bits: u32,
}

impl Default for HammingCode {
    fn default() -> Self {
        Self { parity_bits: 3 }
    }
}

impl HammingCode {
    pub fn total_bits(&self) -> u32 {
        2_u32.pow(self.parity_bits) - 1
    }

    pub fn data_bits(&self) -> u32 {
        self.total_bits() - self.parity_bits
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if self.parity_bits > 6 || self.parity_bits < 2 {
            return Err(CodeError::state(
                "only parity bits from 2 to 6 are supported",
            ));
        }
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if self.parity_bits > 6 || self.parity_bits < 2 {
            return Err(CodeError::state(
                "only parity bits from 2 to 6 are supported",
            ));
        }
        check_bitstring(text)?;

        todo!("match on self.parity bits")
    }
}

#[cfg(test)]
mod hamming_tests {
    use super::*;
}
