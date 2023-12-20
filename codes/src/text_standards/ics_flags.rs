use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use regex::Regex;
use utils::text_functions::bimap_from_iter;

const ICS_MEANING: [&'static str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

const ICS_FLAG_BLAZON: [&'static str; 26] = [
    "Swallowtailed, per pale argent and azure",
    "Swallowtailed, gules",
    "Azure, a fess gules fimbriated argent",
    "Or, a Spanish fess azure",
    "Per fess azure and gules",
    "Argent, a lozenge throughout gules",
    "Paly of six or and azure",
    "Per pale argent and gules",
    "Or, a pellet",
    "Azure, a fess argent",
    "Per pale or and azure",
    "Quarterly or and sable",
    "Azure, a saltire argent",
    "Chequy of sixteen azure and argent",
    "Per bend gules and or",
    "Azure, an inescutcheon argent",
    "Or",
    "Gules, a cross or",
    "Argent, an inescutcheon azure",
    "Tierced in pale gules, argent and azure",
    "Quarterly gules and argent",
    "Argent, a saltire gules",
    "Azure, an inescutcheon gules fimbriated argent",
    "Argent, a cross azure",
    "Bendy sinister of ten or and gules",
    "Per saltire or, sable, gules and azure",
];

lazy_static! {
    pub static ref ICS_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ICS_MEANING.into_iter().zip(ICS_FLAG_BLAZON.into_iter()));
    pub static ref ICS_REGEX: Regex = Regex::new(r"[A-Z]| |.").unwrap();
}

pub struct IcsFlags {}

impl IcsFlags {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        Box::new(ICS_MEANING.into_iter().zip(ICS_FLAG_BLAZON.into_iter()))
    }
}

impl Default for IcsFlags {
    fn default() -> Self {
        Self {}
    }
}

impl Code for IcsFlags {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity(text.len());
        for symbol in ICS_REGEX
            .captures_iter(text)
            .map(|cap| cap.get(0).unwrap().as_str())
        {
            // Ignore spaces
            if symbol == " " {
                continue;
            }

            match ICS_MAP.get_by_left(symbol) {
                Some(code) => out.push(*code),
                None => return Err(CodeError::invalid_input_group(symbol)),
            }
        }
        Ok(out.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();
        for code in text.split(" ") {
            output.push_str(symbol)
        }
        Ok(output)
    }
}

#[cfg(test)]
mod semaphore_tests {
    use super::*;

    const PLAINTEXT: &'static str = "TEST";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = IcsFlags::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = IcsFlags::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
