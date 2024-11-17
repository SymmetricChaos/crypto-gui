use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    W8,
    W16,
    W32,
    W64,
}

pub struct TwosComplement {
    pub byte_format: ByteFormat,
    pub spaced: bool,
    pub big_endian: bool,
    pub width: Width,
}

impl Default for TwosComplement {
    fn default() -> Self {
        Self {
            byte_format: ByteFormat::Binary,
            spaced: false,
            big_endian: true,
            width: Width::W32,
        }
    }
}

impl Code for TwosComplement {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        for group in text.split(",") {
            if group.trim().is_empty() {
                continue;
            }

            if self.big_endian {
                match self.width {
                    Width::W8 => {
                        let n = i8::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_be_bytes()));
                    }
                    Width::W16 => {
                        let n = i16::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_be_bytes()));
                    }
                    Width::W32 => {
                        let n = i32::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_be_bytes()));
                    }
                    Width::W64 => {
                        let n = i64::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_be_bytes()));
                    }
                };
            } else {
                match self.width {
                    Width::W8 => {
                        let n = i8::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_le_bytes()));
                    }
                    Width::W16 => {
                        let n = i16::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_le_bytes()));
                    }
                    Width::W32 => {
                        let n = i32::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_le_bytes()));
                    }
                    Width::W64 => {
                        let n = i64::from_str_radix(group.trim(), 10)
                            .map_err(|_| CodeError::invalid_input_group(group.trim()))?;
                        v.push(self.byte_format.byte_slice_to_text(&n.to_le_bytes()));
                    }
                };
            }
        }

        if self.spaced {
            Ok(v.join(", "))
        } else {
            Ok(v.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut v = Vec::new();

        if self.spaced {
            for group in text.split(",") {
                if self.big_endian {
                    match self.width {
                        Width::W8 => {
                            v.push(
                                self.byte_format
                                    .text_to_i8(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W16 => {
                            v.push(
                                self.byte_format
                                    .text_to_i16_be(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W32 => {
                            v.push(
                                self.byte_format
                                    .text_to_i32_be(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W64 => {
                            v.push(
                                self.byte_format
                                    .text_to_i64_be(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                    }
                } else {
                    match self.width {
                        Width::W8 => {
                            v.push(
                                self.byte_format
                                    .text_to_i8(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W16 => {
                            v.push(
                                self.byte_format
                                    .text_to_i16_le(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W32 => {
                            v.push(
                                self.byte_format
                                    .text_to_i32_le(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W64 => {
                            v.push(
                                self.byte_format
                                    .text_to_i64_le(group.trim())
                                    .map_err(|e| CodeError::Input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                    }
                }
            }
        } else {
            if self.big_endian {
                match self.width {
                    Width::W8 => {
                        v = self
                            .byte_format
                            .text_to_i8(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W16 => {
                        v = self
                            .byte_format
                            .text_to_i16_be(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W32 => {
                        v = self
                            .byte_format
                            .text_to_i32_be(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W64 => {
                        v = self
                            .byte_format
                            .text_to_i64_be(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                }
            } else {
                match self.width {
                    Width::W8 => {
                        v = self
                            .byte_format
                            .text_to_i8(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W16 => {
                        v = self
                            .byte_format
                            .text_to_i16_le(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W32 => {
                        v = self
                            .byte_format
                            .text_to_i32_le(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W64 => {
                        v = self
                            .byte_format
                            .text_to_i64_le(text)
                            .map_err(|e| CodeError::Input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                }
            }
        }

        Ok(v.iter().map(|n| n.to_string()).join(", "))
    }
}

#[cfg(test)]
mod twos_complement_tests {
    use super::*;

    const PLAINTEXT: &'static str = "-3, -2, -1, 0, 1, 2, 3";
    const ENCODEDTEXT: &'static str = "11111111111111111111111111111101111111111111111111111111111111101111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000001000000000000000000000000000000011";

    #[test]
    fn encode_test() {
        let code = TwosComplement::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = TwosComplement::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
