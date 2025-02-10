use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;

crate::lazy_regex!(ITF_PATTERN, r"^1010([01]{14})*1101$");

crate::lazy_bimap!(
    ITF_LEFT: BiMap<char, &'static str> =
        "0123456789".chars().zip([
            "1011001", "1101011", "1001011", "1100101", "1011011", "1101101", "1001101", "1010011",
            "1101001", "1001001"]);
    ITF_RIGHT: BiMap<char, &'static str> =
        "0123456789".chars().zip([
            "0100110", "0010100", "0110100", "0011010", "0100100", "0010010", "0110010", "0101100",
            "0010110", "0110110"]);
);

const START: &'static str = "1010";
const END: &'static str = "1101";

pub struct Itf {
    pub insert_zero: bool,
}

impl Default for Itf {
    fn default() -> Self {
        Self { insert_zero: true }
    }
}

fn encode_itf_pair(pair: (char, char)) -> Result<(&'static str, &'static str), CodeError> {
    Ok((
        ITF_LEFT
            .get_by_left(&pair.0)
            .ok_or_else(|| CodeError::invalid_input_char(pair.0))?,
        ITF_RIGHT
            .get_by_left(&pair.1)
            .ok_or_else(|| CodeError::invalid_input_char(pair.1))?,
    ))
}

impl Itf {}

impl Code for Itf {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if !text.is_ascii() {
            return Err(CodeError::input("found non-ASCII characters"));
        }

        if text.is_empty() {
            return Err(CodeError::input("empty input"));
        }

        let mut out = String::new();
        out.push_str(START);

        if text.len() % 2 != 0 {
            if self.insert_zero {
                let x = text.chars().chunks(2);
                for a in x.into_iter() {
                    let p: (char, char) = match a.collect_tuple() {
                        Some(p) => p,
                        None => break,
                    };
                    let codes = encode_itf_pair(p)?;
                    out.push_str(codes.0);
                    out.push_str(codes.1);
                }
                let codes = encode_itf_pair((text.chars().last().unwrap(), '0'))?;
                out.push_str(codes.0);
                out.push_str(codes.1);
            } else {
                return Err(CodeError::input(
                    "ITF codes encode an even number of digits",
                ));
            }
        } else {
            let x = text.chars().chunks(2);
            for a in x.into_iter() {
                let p: (char, char) = a.collect_tuple().unwrap();
                let codes = encode_itf_pair(p)?;
                out.push_str(codes.0);
                out.push_str(codes.1);
            }
        }

        out.push_str(END);
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let trimmed = text.trim_matches('0');
        if !ITF_PATTERN.is_match(trimmed) {
            return Err(CodeError::input("not structured as a ITF code"));
        }

        let n_pairs = (trimmed.len() - 8) / 14;

        let mut out = String::new();
        for i in 0..n_pairs {
            let left_start = 4 + (i * 14);
            let left_group = dbg!(&trimmed[left_start..left_start + 7]);
            let left_digit = ITF_LEFT
                .get_by_right(left_group)
                .ok_or_else(|| CodeError::invalid_input_group(left_group))?;
            out.push(*left_digit);

            let right_start = left_start + 7;
            let right_group = dbg!(&trimmed[right_start..right_start + 7]);
            let right_digit = ITF_RIGHT
                .get_by_right(right_group)
                .ok_or_else(|| CodeError::invalid_input_group(right_group))?;
            out.push(*right_digit);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod itf_tests {
    use super::*;

    #[test]
    fn encode() {
        let code = Itf::default();
        assert_eq!(
            code.encode("123").unwrap(),
            "101011010110110100110010101001101101"
        );
    }

    #[test]
    fn decode() {
        let code = Itf::default();
        assert_eq!(
            code.decode("101011010110110100110010101001101101").unwrap(),
            "1230"
        );
    }
}
