use crate::{errors::CodeError, mathematical::truncated_binary::TruncatedBinary, traits::Code};
use num::Integer;

pub struct Golomb {
    m: u32,
}

impl Golomb {
    pub fn u32_to_bits(&self, x: u32) -> String {
        let (q, r) = x.div_rem(&self.m);
        // Encode the q portion in unary
        let mut out = "1".repeat(q as usize);
        out.push('0');

        // Encode the remainder with truncated binary
        let b = self.m.ilog2();
        out.push_str(&TruncatedBinary::new(b).u32_to_bits(r));
        out
    }
}

impl Code for Golomb {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}
