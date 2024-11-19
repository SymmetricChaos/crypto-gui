use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use crate::{errors::CodeError, traits::Code};

fn utf8_text_to_rle(text: &str) -> Vec<(char,u32)> {
    let mut out = Vec::new();
    let mut ctr = 0;
    let mut cur_char = text.chars().next().unwrap();
    for c in text.chars() {

        if c != cur_char {
            out.push((cur_char,ctr));
            cur_char = c;
            ctr = 0;
        }
        ctr += 1;
    }
    out.push((cur_char,ctr));
    out
}

fn utf8_rle_to_text(arr: &Vec<(char,u32)>) -> String {
    let mut out = String::new();
    for (c, l) in arr {
        for _ in 0..*l {
            out.push(*c);
        }
    }
    out
}


lazy_static! {
    // Any single codepoint, followed by a space, followed by a number, followed by either a space of EOF
    pub static ref RLE_FORMAT: Regex = Regex::new(r"(.) ([0-9]+)(?: |$)").unwrap();
}

#[derive(Default)]
pub struct RunLengthEncoding {
    
}

impl Code for RunLengthEncoding {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(utf8_text_to_rle(text).into_iter().map(|(c,n)| format!("{c} {n}")).join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let v = RLE_FORMAT.captures_iter(text).map(|caps| 
            (caps.get(1).unwrap().as_str().chars().next().unwrap(),
            u32::from_str_radix(caps.get(2).unwrap().as_str(), 10).expect("could not convert capture group to u32"))
        ).collect_vec();
        Ok(utf8_rle_to_text(&v))
    }
}

#[cfg(test)]
mod rle_tests {
    use super::*;

    const PLAINTEXT: &'static str = "1aaaaaaaaPPPPPPPPPPPPPPP   e";
    const ENCODEDTEXT: &'static str = "1 1 a 8 P 15   3 e 1";

    #[test]
    fn encode_test(){
        let code = RunLengthEncoding::default();
        assert_eq!(ENCODEDTEXT, code.encode(PLAINTEXT).unwrap())
    }

    #[test]
    fn decode_test(){
        let code = RunLengthEncoding::default();
        assert_eq!(PLAINTEXT, code.decode(ENCODEDTEXT).unwrap())
    }
}