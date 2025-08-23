use crate::traits::Code;
use bimap::BiMap;
use utils::errors::GeneralError;

const ICS_MEANING: [&'static str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

const ICS_FLAG_BLAZON: [&'static str; 26] = [
    "Swallowtailed, per pale argent and azure.",
    "Swallowtailed, gules.",
    "Azure, a fess gules fimbriated argent.",
    "Or, a Spanish fess azure.",
    "Per fess azure and gules.",
    "Argent, a lozenge throughout gules.",
    "Paly of six or and azure.",
    "Per pale argent and gules.",
    "Or, a pellet.",
    "Azure, a fess argent.",
    "Per pale or and azure.",
    "Quarterly or and sable.",
    "Azure, a saltire argent.",
    "Chequy of sixteen azure and argent.",
    "Per bend gules and or.",
    "Azure, an inescutcheon argent.",
    "Or.",
    "Gules, a cross or.",
    "Argent, an inescutcheon azure.",
    "Tierced in pale gules, argent and azure.",
    "Quarterly gules and argent.",
    "Argent, a saltire gules.",
    "Azure, an inescutcheon gules fimbriated argent.",
    "Argent, a cross azure.",
    "Bendy sinister of ten or and gules.",
    "Per saltire or, sable, gules and azure.",
];

crate::lazy_regex!(
    ICS_BLAZON_REGEX, r"[A-Z][a-z ,]+\.";
    ICS_REGEX, r"[A-Z]| |.";
);

crate::lazy_bimap!(
    ICS_MAP: BiMap<&'static str, &'static str> = ICS_MEANING.into_iter().zip(ICS_FLAG_BLAZON.into_iter())
);

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
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
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
                None => return Err(GeneralError::invalid_input_group(symbol)),
            }
        }
        Ok(out.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::new();
        for code in ICS_BLAZON_REGEX
            .captures_iter(text)
            .map(|cap| cap.get(0).unwrap().as_str())
        {
            match ICS_MAP.get_by_right(code) {
                Some(code) => out.push_str(code),
                None => return Err(GeneralError::invalid_input_group(code)),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod ics_flag_tests {
    use super::*;

    const PLAINTEXT: &'static str = "EXAMPLE";
    const CODETEXT: &'static str = "Per fess azure and gules. Argent, a cross azure. Swallowtailed, per pale argent and azure. Azure, a saltire argent. Azure, an inescutcheon argent. Quarterly or and sable. Per fess azure and gules.";

    #[test]
    fn encrypt_test() {
        let code = IcsFlags::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = IcsFlags::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
