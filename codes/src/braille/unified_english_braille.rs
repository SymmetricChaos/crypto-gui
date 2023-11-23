use crate::{errors::CodeError, traits::Code};

use super::{
    unified_english_braille_parser::decode_g1_braille,
    unified_english_braille_parser_inv::encode_g1_braille,
};

pub struct UnifiedEnglishBraille {}

impl Default for UnifiedEnglishBraille {
    fn default() -> Self {
        Self {}
    }
}

impl Code for UnifiedEnglishBraille {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        Ok(encode_g1_braille(text))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        Ok(decode_g1_braille(text))
    }
}
