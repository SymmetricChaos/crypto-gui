use crate::{errors::CodeError, traits::Code};

pub struct RunLengthEncoding {
    
}

impl RunLengthEncoding {
    fn utf8_text_to_rle(text: &str) -> Vec<(char,u32)> {
        let mut out = Vec::new();
        let mut ctr = 0;
        let mut cur_char = '\0';
        for c in text.chars() {
            if c == cur_char {
                ctr += 1; //
            } else {
                out.push((cur_char,ctr));
                cur_char = c;
                ctr = 0;
            }
        }
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
}

impl Code for RunLengthEncoding {
    fn encode(&self, text: &str) -> Result<String, CodeError> {

        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}