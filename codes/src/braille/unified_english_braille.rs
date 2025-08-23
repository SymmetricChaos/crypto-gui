use super::{
    unified_english_braille_parser::decode_g1_braille,
    unified_english_braille_parser_inv::encode_g1_braille,
};
use crate::{
    braille::unified_english_braille_maps::{LETTERS, LETTERS_BRAILLE, LETTERS_UPPER},
    traits::Code,
};
use itertools::izip;
use utils::errors::GeneralError;

pub struct UnifiedEnglishBraille {}

impl Default for UnifiedEnglishBraille {
    fn default() -> Self {
        Self {}
    }
}

impl UnifiedEnglishBraille {
    pub fn alphabet_triples() -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
        izip!(
            LETTERS_BRAILLE.into_iter(),
            LETTERS.into_iter(),
            LETTERS_UPPER.into_iter()
        )
    }
}

impl Code for UnifiedEnglishBraille {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        Ok(encode_g1_braille(text))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        Ok(decode_g1_braille(text))
    }
}
