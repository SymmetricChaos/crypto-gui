use crate::traits::Code;
use bimap::BiMap;
use utils::{errors::GeneralError, text_functions::bimap_from_iter};

const FIVE_NEEDLE_CODES: [&'static str; 20] = [
    r"/|||\", r"/||\|", r"|/||\", r"/|\||", r"|/|\|", r"||/|\", r"/\|||", r"|/\||", r"||/\|",
    r"|||/\", r"\/|||", r"|\/||", r"||\/|", r"|||\/", r"\|/||", r"|\|/|", r"||\|/", r"\||/|",
    r"|\||/", r"\|||/",
];

pub struct Needle {
    pub alphabet: String,
    map: BiMap<char, &'static str>,
}

impl Needle {
    pub fn set_map(&mut self) {
        self.map = bimap_from_iter(self.alphabet.chars().zip(FIVE_NEEDLE_CODES.into_iter()));
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &str)> + '_> {
        Box::new(self.alphabet.chars().zip(FIVE_NEEDLE_CODES.into_iter()))
    }
}

impl Default for Needle {
    fn default() -> Self {
        let alphabet = String::from("ABDEFGHIKLMNOPRSTUWY");
        let map = bimap_from_iter(alphabet.chars().zip(FIVE_NEEDLE_CODES.into_iter()));
        Needle { alphabet, map }
    }
}

impl Code for Needle {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut vec = Vec::with_capacity(text.len());
        for c in text.chars() {
            let code = self
                .map
                .get_by_left(&c)
                .ok_or_else(|| GeneralError::invalid_input_char(c))?;
            vec.push(*code)
        }
        Ok(vec.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let codes = text.split(" ");
        let mut output = String::with_capacity(codes.clone().count());
        for code in codes {
            let c = self
                .map
                .get_by_right(code)
                .ok_or_else(|| GeneralError::invalid_input_group(code))?;
            output.push(*c)
        }
        Ok(output)
    }
}

#[cfg(test)]
mod needle_code_tests {
    use super::*;

    const PTEXT: &'static str = "ABDE";
    const CODETEXT: &'static str = r"/|||\ /||\| |/||\ /|\||";

    #[test]
    fn encrypt_test() {
        let code = Needle::default();
        assert_eq!(code.encode(PTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Needle::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PTEXT);
    }
}
