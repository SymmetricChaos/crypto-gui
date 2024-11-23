use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

pub struct BeltHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for BeltHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl BeltHash {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for BeltHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1,
    BeltHash::default(),
    "INPUT",
    "OUTPUT";
);
