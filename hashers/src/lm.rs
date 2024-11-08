use core::panic;
use utils::byte_formatting::ByteFormat;

use crate::{
    auxiliary::des_functions::{expand_56_to_64, Des},
    errors::HasherError,
    traits::ClassicHasher,
};

// derived from the ASCII string "KGS!@#$%"
pub const LM_WORD: u64 = 0x4B47532140232425;

pub struct Lm {
    pub output_format: ByteFormat,
}

impl Default for Lm {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
        }
    }
}

impl Lm {
    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for Lm {
    /// This should not be called directly as LM is not meant to encrypt arbitrary bytes
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if !bytes.is_ascii() {
            panic!("LM accepts only ASCII characters")
        }
        if bytes.len() > 14 {
            panic!("LM only accepts inputs of up to 14 bytes")
        }

        let mut input = bytes.to_vec();
        while input.len() < 14 {
            input.push(0);
        }
        let mut cipher = Des::default();
        let mut out = Vec::with_capacity(14);

        let k1 = expand_56_to_64(input[0..7].try_into().unwrap());
        let k2 = expand_56_to_64(input[7..14].try_into().unwrap());

        cipher.ksa(k1);
        out.extend(cipher.encrypt_block(LM_WORD).to_be_bytes());
        cipher.ksa(k2);
        out.extend(cipher.encrypt_block(LM_WORD).to_be_bytes());

        out
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if !text.is_ascii() {
            return Err(HasherError::general("LM accepts only ASCII characters"));
        }

        let mut input = text.to_uppercase();
        while input.len() < 14 {
            input.push('\0')
        }

        let mut bytes = ByteFormat::Utf8
            .text_to_bytes(&input)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

crate::basic_hash_tests!(
    test1, Lm::default(), "PassWord", "e52cac67419a9a224a3b108f3fa6cb6d";
);
