use itertools::Itertools;

use super::string_to_u32s;
use crate::{errors::CodeError, traits::Code};

pub fn u32_to_exp_golomb(n: u32) -> String {
    if n == u32::MAX {
        String::from("00000000000000000000000000000000100000000000000000000000000000000")
    } else {
        let s = format!("{:b}", n + 1);
        let mut z = "0".repeat(s.len() - 1);
        z.push_str(&s);
        z
    }
}

// TODO: This will fail to decode u32::MAX
pub fn recognize_exp_golomb(s: &str) -> Vec<Option<u32>> {
    let mut out = Vec::new();
    let mut ctr = 0;
    let mut counting_up = true;
    let mut buffer = String::new();
    for b in s.chars().filter(|c| *c == '0' || *c == '1') {
        if counting_up {
            if b == '0' {
                ctr += 1;
                if ctr > 32 {
                    out.push(None);
                }
            }
            if b == '1' {
                if ctr == 0 {
                    out.push(Some(0));
                    continue;
                }
                buffer.push(b);
                counting_up = false;
            }
        } else {
            buffer.push(b);
            ctr -= 1;
            if ctr <= 0 {
                let n = u32::from_str_radix(&buffer, 2).unwrap();
                out.push(Some(n - 1));
                counting_up = true;
                buffer.clear();
                ctr = 0;
                continue;
            }
        }
    }
    out
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
        Ok(recognize_exp_golomb(text)
            .into_iter()
            .map(|s| match s {
                Some(n) => n.to_string(),
                None => String::from("�"),
            })
            .join(", "))
    }
}

#[cfg(test)]
mod golomb_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 8, 1234";
    const ENCODEDTEXT: &'static str = "1010011001000001001000000000010011010011";
    const ENCODEDTEXT_SP: &'static str = "1, 010, 011, 00100, 0001001, 000000000010011010011";

    #[test]
    fn encode_test() {
        let mut code = ExpGolomb::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn decode_test() {
        let code = ExpGolomb::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT);
    }
}
