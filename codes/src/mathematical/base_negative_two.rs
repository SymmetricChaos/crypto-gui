use crate::traits::Code;
use itertools::Itertools;
use utils::{byte_formatting::ByteFormat, errors::GeneralError};

// fn encode_bn2_be(n: i32) -> u32 {
//     for byte in n.to_be_bytes() {

//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    W8,
    W16,
    W32,
    W64,
}

pub struct BaseNegativeTwo {
    pub byte_format: ByteFormat,
    pub spaced: bool,
    pub big_endian: bool,
    pub width: Width,
}

impl Default for BaseNegativeTwo {
    fn default() -> Self {
        Self {
            byte_format: ByteFormat::Binary,
            spaced: false,
            big_endian: true,
            width: Width::W32,
        }
    }
}

impl Code for BaseNegativeTwo {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        // let mut v = Vec::new();

        // for group in text.split(",") {
        //     if group.trim().is_empty() {
        //         continue;
        //     }

        //     if self.big_endian {
        //         match self.width {
        //             Width::W8 => encode_bn2_be!(i8, group, v, self.byte_format),
        //             Width::W16 => encode_bn2_be!(i16, group, v, self.byte_format),
        //             Width::W32 => encode_bn2_be!(i32, group, v, self.byte_format),
        //             Width::W64 => encode_bn2_be!(i64, group, v, self.byte_format),
        //         };
        //     } else {
        //         match self.width {
        //             Width::W8 => encode_bn2_le!(i8, group, v, self.byte_format),
        //             Width::W16 => encode_bn2_le!(i16, group, v, self.byte_format),
        //             Width::W32 => encode_bn2_le!(i32, group, v, self.byte_format),
        //             Width::W64 => encode_bn2_le!(i64, group, v, self.byte_format),
        //         };
        //     }
        // }

        // if self.spaced {
        //     Ok(v.join(", "))
        // } else {
        //     Ok(v.join(""))
        // }
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut v = Vec::new();

        if self.spaced {
            for group in text.split(",") {
                if self.big_endian {
                    match self.width {
                        Width::W8 => {
                            v.push(
                                self.byte_format
                                    .text_to_i8(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W16 => {
                            v.push(
                                self.byte_format
                                    .text_to_i16_be(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W32 => {
                            v.push(
                                self.byte_format
                                    .text_to_i32_be(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W64 => {
                            v.push(
                                self.byte_format
                                    .text_to_i64_be(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
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
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W16 => {
                            v.push(
                                self.byte_format
                                    .text_to_i16_le(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W32 => {
                            v.push(
                                self.byte_format
                                    .text_to_i32_le(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
                                    .to_string(),
                            );
                        }
                        Width::W64 => {
                            v.push(
                                self.byte_format
                                    .text_to_i64_le(group.trim())
                                    .map_err(|e| GeneralError::input(e.to_string()))?[0]
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
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W16 => {
                        v = self
                            .byte_format
                            .text_to_i16_be(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W32 => {
                        v = self
                            .byte_format
                            .text_to_i32_be(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W64 => {
                        v = self
                            .byte_format
                            .text_to_i64_be(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
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
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W16 => {
                        v = self
                            .byte_format
                            .text_to_i16_le(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W32 => {
                        v = self
                            .byte_format
                            .text_to_i32_le(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect_vec();
                    }
                    Width::W64 => {
                        v = self
                            .byte_format
                            .text_to_i64_le(text)
                            .map_err(|e| GeneralError::input(e.to_string()))?
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

    const PTEXT: &'static str = "-3, -2, -1, 0, 1, 2, 3";
    const ENCODEDTEXT: &'static str = "11111111111111111111111111111101111111111111111111111111111111101111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000001000000000000000000000000000000011";

    #[test]
    fn encode_test() {
        let code = BaseNegativeTwo::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = BaseNegativeTwo::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
    }
}
