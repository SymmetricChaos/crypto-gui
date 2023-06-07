use crate::{ecc::check_bitstring, errors::CodeError, traits::Code};

use nalgebra::{ArrayStorage, SMatrix, Vector, Vector3};

const GEN_4_7: SMatrix<u8, 4, 7> = SMatrix::from_array_storage(ArrayStorage([
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
    [1, 1, 0, 1],
    [1, 0, 1, 1],
    [0, 1, 1, 1],
]));

const CHK_4_7: SMatrix<u8, 3, 7> = SMatrix::from_array_storage(ArrayStorage([
    [1, 1, 0],
    [1, 0, 1],
    [0, 1, 1],
    [1, 1, 1],
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
]));

pub struct HammingCode {
    // pub parity_bits: u32,
}

impl Default for HammingCode {
    fn default() -> Self {
        Self {}
    }
}

impl HammingCode {
    //     pub fn total_bits(&self) -> u32 {
    //         2_u32.pow(self.parity_bits) - 1
    //     }

    //     pub fn data_bits(&self) -> u32 {
    //         self.total_bits() - self.parity_bits
    //     }

    fn error_index(&self, vec: Vector3<u8>) -> Option<usize> {
        CHK_4_7.column_iter().position(|c| c == vec)
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        // if self.parity_bits > 6 || self.parity_bits < 2 {
        //     return Err(CodeError::state(
        //         "only parity bits from 2 to 6 are supported",
        //     ));
        // }
        check_bitstring(text)?;

        let mut buffer: Vec<u8> = Vec::with_capacity(4);
        let mut out = String::new();
        for bit in text.chars() {
            if bit == '0' {
                buffer.push(0);
            } else if bit == '1' {
                buffer.push(1);
            } else {
                unreachable!("characters other than 0 and 1 should be filtered out")
            }

            if buffer.len() == 4 {
                let s = Vector::from(buffer.clone()).transpose() * GEN_4_7;
                for b in s.into_iter() {
                    match b % 2 {
                        0 => out.push('0'),
                        1 => out.push('1'),
                        _ => unreachable!("only 0 and 1 can occur"),
                    }
                }
                buffer.clear();
            }
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // if self.parity_bits > 6 || self.parity_bits < 2 {
        //     return Err(CodeError::state(
        //         "only parity bits from 2 to 6 are supported",
        //     ));
        // }
        check_bitstring(text)?;

        let mut buffer: Vec<u8> = Vec::with_capacity(7);
        let mut out = String::new();
        for bit in text.chars() {
            if bit == '0' {
                buffer.push(0);
            } else if bit == '1' {
                buffer.push(1);
            } else {
                unreachable!("characters other than 0 and 1 should be filtered out")
            }

            if buffer.len() == 7 {
                let mut error_syndrome = CHK_4_7 * Vector::from(buffer.clone());
                error_syndrome.apply(|x| *x = *x % 2);
                dbg!(error_syndrome);
                let location = self.error_index(error_syndrome);
                dbg!(location);
                dbg!(&buffer);
                if let Some(idx) = location {
                    buffer[idx] ^= 1;
                }
                dbg!(&buffer);
                for b in buffer.iter().take(4) {
                    match b % 2 {
                        0 => out.push('0'),
                        1 => out.push('1'),
                        _ => unreachable!("only 0 and 1 can occur"),
                    }
                }
                buffer.clear();
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod hamming_tests {

    use super::*;

    #[test]
    fn encode_simple() {
        let code = HammingCode::default();
        assert_eq!(code.encode("1011").unwrap(), "1011010");
    }

    #[test]
    fn decode_simple() {
        let code = HammingCode::default();
        assert_eq!(code.decode("1011010").unwrap(), "1011");
    }

    #[test]
    fn decode_simple_err() {
        let code = HammingCode::default();
        assert_eq!(code.decode("0011010").unwrap(), "1011");
    }
}
