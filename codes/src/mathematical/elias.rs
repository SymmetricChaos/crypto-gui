use super::{elias_integers::EliasVariant, Code, EliasCodeIntegers, IOMode, LetterAndWordCode};
use crate::errors::Error;

pub struct EliasCode {
    pub maps: LetterAndWordCode<String>,
    pub integer_code: EliasCodeIntegers,
    pub mode: IOMode,
    pub variant: EliasVariant,
}

impl EliasCode {}

impl Default for EliasCode {
    fn default() -> Self {
        let codes = EliasCodeIntegers::default();

        let mut maps = LetterAndWordCode::<String>::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| codes.encode_u32((n + 1) as u32));

        Self {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
            variant: EliasVariant::Delta,
        }
    }
}

impl Code for EliasCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if self.mode == IOMode::Integer {
            self.integer_code.encode(text)
        } else if self.mode == IOMode::Letter {
            let mut output = String::new();
            for c in text.chars() {
                let code = self.maps.get_by_letter(c)?;
                output.push_str(&code)
            }
            Ok(output)
        } else {
            let mut output = String::new();
            for w in text.split(" ") {
                let code = self.maps.get_by_word(w)?;
                output.push_str(code)
            }
            Ok(output)
        }
    }

    fn decode(&self, _text: &str) -> Result<String, Error> {
        // let mut output = String::new();
        // let mut buffer = String::with_capacity(self.max_code_len);
        // let mut ctr = 0;
        // for b in text.chars() {
        //     buffer.push(b);
        //     ctr += 1;
        //     if let Some(s) = self.map_inv.get(&buffer) {
        //         output.push(*s);
        //         buffer.clear();
        //         ctr = 0;
        //     }
        //     // If we have an impossible code ignore it and start again, it will eventually
        //     // resychronize
        //     if ctr == self.max_code_len {
        //         buffer.clear();
        //         ctr = 0;
        //     }
        // }
        // Ok(output)
        todo!()
    }
}

#[cfg(test)]
mod elias_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
