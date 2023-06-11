use crate::{ecc::check_bitstring, errors::CodeError, traits::Code};
use lazy_static::lazy_static;
use nalgebra::{ArrayStorage, SMatrix, Vector, Vector3};
use utils::bits::Bit;

lazy_static! {
    pub static ref GEN_4_7_SYS: SMatrix<Bit, 4, 7> = SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [0, 1, 1, 1],
            [1, 0, 1, 1],
            [1, 1, 0, 1],
        ]
        .map(|i| i.map(|i| Bit::try_from(i).unwrap())),
    ));
    pub static ref CHK_4_7_SYS: SMatrix<Bit, 3, 7> = SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 1, 0],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]
        .map(|i| i.map(|i| Bit::try_from(i).unwrap())),
    ));
    pub static ref GEN_4_8_SYS: SMatrix<Bit, 4, 8> = SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [0, 1, 1, 1],
            [1, 0, 1, 1],
            [1, 1, 0, 1],
            [1, 1, 1, 0],
        ]
        .map(|i| i.map(|i| Bit::try_from(i).unwrap())),
    ));
}

pub struct HammingCode {
    pub extra_bit: bool,
    pub systemtic: bool,
}

impl Default for HammingCode {
    fn default() -> Self {
        Self {
            extra_bit: false,
            systemtic: true,
        }
    }
}

fn error_index_4_7(vec: Vector3<Bit>) -> Option<usize> {
    CHK_4_7_SYS.column_iter().position(|c| c == vec)
}

impl HammingCode {
    fn decode_4_7(text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        let mut buffer: Vec<Bit> = Vec::with_capacity(7);
        let mut out = String::new();
        for bit in text.chars() {
            // Unwrap justified by check_bitstring()
            buffer.push(Bit::try_from(bit).unwrap());

            if buffer.len() == 7 {
                let error_syndrome = *CHK_4_7_SYS * Vector::from(&buffer[..]);

                let location = error_index_4_7(error_syndrome);
                if let Some(idx) = location {
                    buffer[idx].flip();
                }
                for b in buffer.iter().take(4) {
                    out.push(b.as_char());
                }
                buffer.clear();
            }
        }
        Ok(out)
    }

    fn decode_4_8(text: &str) -> Result<String, CodeError> {
        check_bitstring(text)?;

        let mut buffer: Vec<Bit> = Vec::with_capacity(8);
        let mut out = String::new();
        for bit in text.chars() {
            // Unwrap justified by check_bitstring()
            buffer.push(Bit::try_from(bit).unwrap());

            if buffer.len() == 8 {
                let total_parity = buffer.iter().cloned().sum();

                let error_syndrome = *CHK_4_7_SYS * Vector::from(&buffer[0..7]);

                let location = error_index_4_7(error_syndrome);

                // If the total parity is zero
                match total_parity {
                    Bit::Zero =>
                    // If an error location is found there must be a two bit error
                    {
                        if location.is_some() {
                            return Err(CodeError::input("a two bit error was detected"));
                        } else {
                            // Otherwise the extra check bit was a single bit error
                        }
                    }
                    Bit::One =>
                    // If the error location is detected fix it
                    {
                        if let Some(idx) = location {
                            buffer[idx].flip();
                        } else {
                            // Otherwise the extra check bit was a single bit error
                        }
                    }
                }

                for b in buffer.iter().take(4) {
                    out.push(b.as_char());
                }
                buffer.clear();
            }
        }
        Ok(out)
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        // if self.check_bits > 6 || self.check_bits < 3 {
        //     return Err(CodeError::state(
        //         "only check_bits bits from 3 to 6 are supported",
        //     ));
        // }
        check_bitstring(text)?;

        let mut buffer: Vec<Bit> = Vec::with_capacity(4);
        let mut out = String::new();
        for bit in text.chars() {
            // Unwrap justified by check_bitstring()
            buffer.push(Bit::try_from(bit).unwrap());

            if buffer.len() == 4 {
                if self.extra_bit {
                    let s = Vector::from(buffer.clone()).transpose() * *GEN_4_8_SYS;
                    for b in s.into_iter() {
                        out.push(b.as_char());
                    }
                } else {
                    let s = Vector::from(buffer.clone()).transpose() * *GEN_4_7_SYS;
                    for b in s.into_iter() {
                        out.push(b.as_char());
                    }
                };

                buffer.clear();
            }
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        // if self.check_bits > 6 || self.check_bits < 3 {
        //     return Err(CodeError::state(
        //         "only check_bits bits from 3 to 6 are supported",
        //     ));
        // }
        match self.extra_bit {
            true => Self::decode_4_8(text),
            false => Self::decode_4_7(text),
        }
    }
}

#[cfg(test)]
mod hamming_tests {

    use super::*;

    #[test]
    fn encode() {
        let code = HammingCode::default();
        assert_eq!(
            code.encode("100100000001").unwrap(),
            "100110000000000001111"
        );
    }

    #[test]
    fn encode_extra_bit() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.encode("1011").unwrap(), "10110100");
    }

    #[test]
    fn decode() {
        let code = HammingCode::default();
        assert_eq!(code.decode("1011010").unwrap(), "1011");
    }

    #[test]
    fn decode_extra_bit() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.decode("10110100").unwrap(), "1011");
    }

    #[test]
    fn decode_err() {
        let code = HammingCode::default();
        assert_eq!(code.decode("0011010").unwrap(), "1011");
    }

    #[test]
    fn decode_single_err_extra_bit_1() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.decode("10110101").unwrap(), "1011");
    }

    #[test]
    fn decode_single_err_extra_bit_2() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.decode("10100100").unwrap(), "1011");
    }

    #[test]
    fn decode_double_err_extra_bit_1() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(
            code.decode("10110001").unwrap_err(),
            CodeError::input("a two bit error was detected")
        );
    }

    #[test]
    fn decode_double_err_extra_bit_2() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(
            code.decode("10000100").unwrap_err(),
            CodeError::input("a two bit error was detected")
        );
    }
}
