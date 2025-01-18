use super::string_to_u32s;
use crate::{errors::CodeError, traits::Code};

pub fn u32_to_exp_golomb(n: u32) -> String {
    if n == u32::MAX {
        String::from("000000000000000000000000000000000100000000000000000000000000000000")
    } else {
        let s = format!("{:b}", n + 1);
        let mut z = "0".repeat(s.len());
        z.push_str(&s);
        z
    }
}

pub struct ExpGolomb {
    pub spaced: bool,
}

impl Default for ExpGolomb {
    fn default() -> Self {
        Self { spaced: false }
    }
}

impl ExpGolomb {}

impl Code for ExpGolomb {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for n in string_to_u32s(text, ",")? {
            out.push(u32_to_exp_golomb(n));
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}
