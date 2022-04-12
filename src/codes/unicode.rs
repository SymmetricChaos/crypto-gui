use itertools::Itertools;

use crate::errors::CodeError;

use super::Code;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnicodeMode {
    Utf8,
    Utf16,
    Utf32,
}
 
pub struct Unicode {
    pub mode: UnicodeMode,
}
 
impl Unicode {

}

impl Default for Unicode {
    fn default() -> Self {
        Unicode{ mode: UnicodeMode::Utf8 }
    }
}

impl Code for Unicode {
 
    fn encode(&self, text: &str) -> Result<String,CodeError> {
        match self.mode {
            UnicodeMode::Utf8 => Ok(text.bytes().map(|b| (format!("{:08b}",b))).join("")),
            UnicodeMode::Utf16 => todo!(),
            UnicodeMode::Utf32 => todo!(),
        }
    }
 
    fn decode(&self, text: &str) -> Result<String,CodeError> {
        match self.mode {
            UnicodeMode::Utf8 => todo!(),
            UnicodeMode::Utf16 => todo!(),
            UnicodeMode::Utf32 => todo!(),
        }
    }
}





#[cfg(test)]
mod ascii_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "The 素早い καφέ 🦊 ｊｕｍｐｓ over the lazy 🐶.";
    const CIPHERTEXT_UTF8: &'static str = "010101000110100001100101001000001110011110110100101000001110011010010111101010011110001110000001100001000010000011001110101110101100111010110001110011111000011011001110101011010010000011110000100111111010011010001010001000001110111110111101100010101110111110111101100101011110111110111101100011011110111110111101100100001110111110111101100100110010000001101111011101100110010101110010001000000111010001101000011001010010000001101100011000010111101001111001001000001111000010011111100100001011011000101110";

    #[test]
    fn encrypt_test_utf8() {
        let code = Unicode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_UTF8);
    }

    #[test]
    fn decrypt_test_utf8() {
        let code = Unicode::default();
        assert_eq!(code.decode(CIPHERTEXT_UTF8).unwrap(), PLAINTEXT);
    }

    // #[test]
    // fn encrypt_test_utf8() {
    //     let code = Unicode::default();
    //     assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    // }

    // #[test]
    // fn decrypt_test_utf8() {
    //     let code = Unicode::default();
    //     assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    // }


}
