use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FletcherhWidth {
    W16,
    W32,
    W64,
}

pub struct Fletcher {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub width: FletcherhWidth,
}

impl Default for Fletcher {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            width: FletcherhWidth::W32,
        }
    }
}

impl Fletcher {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn width(mut self, width: FletcherhWidth) -> Self {
        self.width = width;
        self
    }

    pub fn w16() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            width: FletcherhWidth::W16,
        }
    }

    pub fn w32() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            width: FletcherhWidth::W32,
        }
    }

    pub fn w64() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            width: FletcherhWidth::W64,
        }
    }
}

impl ClassicHasher for Fletcher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut c0: u32 = 0;
        let mut c1: u32 = 0;

        let m = match self.width {
            FletcherhWidth::W16 => u8::MAX as u32,
            FletcherhWidth::W32 => u16::MAX as u32,
            FletcherhWidth::W64 => u32::MAX,
        };

        for byte in bytes {
            c0 = (c0 + *byte as u32) % m;
            c1 = (c1 + c0 as u32) % m;
        }

        match self.width {
            FletcherhWidth::W16 => vec![c1 as u8, c0 as u8],
            FletcherhWidth::W32 => [c1 as u16, c0 as u16]
                .iter()
                .flat_map(|w| w.to_be_bytes())
                .collect(),
            FletcherhWidth::W64 => [c1, c0].iter().flat_map(|w| w.to_be_bytes()).collect(),
        }
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1,
    Fletcher::w16(),
    "abcde",
    "c8f0";

    test2,
    Fletcher::w16(),
    "abcdef",
    "2057";

    test3,
    Fletcher::w16(),
    "abcdefgh",
    "0627";
);
