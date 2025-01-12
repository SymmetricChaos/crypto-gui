use utils::byte_formatting::ByteFormat;

use crate::{errors::CodeError, traits::Code};

pub enum BcdVariant {
    V8421,
    V7421,
    Aiken,
    Excess3,
    Gray,
}

impl BcdVariant {
    fn array(&self) -> [u8; 10] {
        match self {
            BcdVariant::V8421 => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9],
            BcdVariant::V7421 => [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xA],
            BcdVariant::Aiken => [0x0, 0x1, 0x2, 0x3, 0x4, 0xB, 0xC, 0xD, 0xE, 0xF],
            BcdVariant::Excess3 => [0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC],
            BcdVariant::Gray => [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xC, 0xD],
        }
    }
}

pub struct BinaryCodedDecimal {
    variant: BcdVariant,
    packed: bool,
    formatting: ByteFormat,
}

impl Default for BinaryCodedDecimal {
    fn default() -> Self {
        Self {
            variant: BcdVariant::V8421,
            packed: true,
            formatting: ByteFormat::Hex,
        }
    }
}

impl Code for BinaryCodedDecimal {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        let arr = self.variant.array();
        let mut bytes = Vec::new();
        if !self.packed {
            for digit in text.chars() {
                match digit {
                    '0'..='9' => bytes.push(arr[(digit as u32 - 48) as usize]),
                    _ => return Err(CodeError::invalid_input_char(digit)),
                }
            }
        } else {
            todo!()
        }

        Ok(self.formatting.byte_slice_to_text(&bytes))
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }
}
