use crate::{
    errors::Error,
    text_aux::{
        bytes_as_text::{num_to_string_width, u32_from_string, NumRep},
        PresetAlphabet, VecString,
    },
};

use super::Code;

pub struct BlockCode {
    pub width: usize,
    pub rep: NumRep,
    pub alphabet: VecString,
    //pub symbols: VecString,
}

impl Default for BlockCode {
    fn default() -> Self {
        BlockCode {
            width: 5,
            rep: NumRep::Binary,
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            //symbols: VecString::from("01"),
        }
    }
}

impl BlockCode {
    fn num_to_string(&self, n: &usize) -> String {
        num_to_string_width(&n, self.rep, self.width)
    }

    pub fn assign_width(&mut self, width: usize) {
        if width >= 3 && width <= 8 {
            self.width = width
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, String)> + '_> {
        Box::new(
            self.alphabet
                .chars()
                .enumerate()
                .map(|(n, c)| (c, self.num_to_string(&n))),
        )
    }

    pub fn valid_code_width(&self) -> bool {
        let n_symbols = self.alphabet.chars().count();
        let min_width = (n_symbols as f32).log(self.rep.radix() as f32).ceil() as usize;
        min_width < self.width
    }
}

impl Code for BlockCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::with_capacity(text.len());
        for c in text.chars() {
            let n = self.alphabet.get_pos(c).ok_or_else(|| {
                Error::Input(format!(
                    "The character `{c}` is not in the selected alphabet"
                ))
            })?;
            out.push(self.num_to_string(&n));
        }
        Ok(out.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();

        for group in text.split(" ") {
            let n = u32_from_string(group, self.rep)
                .map_err(|_| Error::Input(format!("The code group `{group}` is not valid")))?
                as usize;
            out.push(
                self.alphabet
                    .get_char_at(n)
                    .expect("tried to access character outside alphabet range"),
            )
        }

        Ok(out)
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod block_code_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
