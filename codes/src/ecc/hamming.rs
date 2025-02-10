use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use nalgebra::{ArrayStorage, SMatrix, Vector, Vector3};
use std::sync::LazyLock;
use utils::bits::{bits_from_str, to_bit_array, Bit};

// Generator matrix with systemtic order
pub static GEN_4_7_SYS: LazyLock<SMatrix<Bit, 4, 7>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [0, 1, 1, 1],
            [1, 0, 1, 1],
            [1, 1, 0, 1],
        ]
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

// Generator with the commonn mixed bit order
pub static GEN_4_7_MIX: LazyLock<SMatrix<Bit, 4, 7>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 1, 0, 1],
            [1, 0, 1, 1],
            [1, 0, 0, 0],
            [0, 1, 1, 1],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ]
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

pub static CHK_4_7_SYS: LazyLock<SMatrix<Bit, 3, 7>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 1, 0],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

pub static CHK_4_7_MIX: LazyLock<SMatrix<Bit, 3, 7>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 0, 0],
            [0, 1, 0],
            [1, 1, 0],
            [0, 0, 1],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
        ]
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

pub static GEN_4_8_SYS: LazyLock<SMatrix<Bit, 4, 8>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
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
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

pub static GEN_4_8_MIX: LazyLock<SMatrix<Bit, 4, 8>> = LazyLock::new(|| {
    SMatrix::from_array_storage(ArrayStorage(
        [
            [1, 1, 0, 1],
            [1, 0, 1, 1],
            [1, 0, 0, 0],
            [0, 1, 1, 1],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [1, 1, 1, 0],
        ]
        .map(|i| to_bit_array(i).unwrap()),
    ))
});

pub struct HammingCode {
    pub extra_bit: bool,
    pub systematic: bool,
}

impl Default for HammingCode {
    fn default() -> Self {
        Self {
            extra_bit: false,
            systematic: true,
        }
    }
}

impl HammingCode {
    fn error_index_4_7(&self, vec: Vector3<Bit>) -> Option<usize> {
        match self.systematic {
            true => CHK_4_7_SYS.column_iter(),
            false => CHK_4_7_MIX.column_iter(),
        }
        .position(|c| c == vec)
    }

    fn decode_4_7(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % 7 != 0 {
            return Err(CodeError::Input(format!(
                "the input must have a length that is a multiple of 7",
            )));
        }

        let mut buffer: Vec<Bit> = Vec::with_capacity(7);
        let mut out = String::new();
        for chunk in &bits.into_iter().chunks(7) {
            for bit in chunk {
                buffer.push(bit);
            }

            if buffer.len() == 7 {
                let location = self.error_index_4_7(match self.systematic {
                    true => *CHK_4_7_SYS * Vector::from(&buffer[..]),
                    false => *CHK_4_7_MIX * Vector::from(&buffer[..]),
                });

                if let Some(idx) = location {
                    buffer[idx].flip();
                }
                match self.systematic {
                    true => buffer.iter().take(4).for_each(|b| out.push(char::from(b))),
                    false => {
                        out.push(char::from(&buffer[2]));
                        out.push(char::from(&buffer[4]));
                        out.push(char::from(&buffer[5]));
                        out.push(char::from(&buffer[6]));
                    }
                }

                buffer.clear();
            }
        }
        Ok(out)
    }

    fn decode_4_8(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % 8 != 0 {
            return Err(CodeError::Input(format!(
                "the input must have a length that is a multiple of 8",
            )));
        }

        let mut buffer: Vec<Bit> = Vec::with_capacity(8);
        let mut out = String::new();
        for chunk in &bits.into_iter().chunks(8) {
            for bit in chunk {
                buffer.push(bit);
            }

            let total_parity = buffer.iter().copied().fold(Bit::Zero, |a, b| a + b);

            let location = self.error_index_4_7(match self.systematic {
                true => *CHK_4_7_SYS * Vector::from(&buffer[..7]),
                false => *CHK_4_7_MIX * Vector::from(&buffer[..7]),
            });

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

            match self.systematic {
                true => buffer.iter().take(4).for_each(|b| out.push(char::from(b))),
                false => {
                    out.push(char::from(&buffer[2]));
                    out.push(char::from(&buffer[4]));
                    out.push(char::from(&buffer[5]));
                    out.push(char::from(&buffer[6]));
                }
            }
            buffer.clear();
        }
        Ok(out)
    }
}

impl Code for HammingCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let bits: Vec<Bit> = bits_from_str(text)
            .map_err(|e| CodeError::input(&e.to_string()))?
            .collect();

        if bits.len() % 4 != 0 {
            return Err(CodeError::Input(format!(
                "the input must have a length that is a multiple of 4",
            )));
        }

        let mut buffer: Vec<Bit> = Vec::with_capacity(4);
        let mut out = String::new();
        for chunk in &bits.into_iter().chunks(4) {
            for bit in chunk {
                buffer.push(bit);
            }

            let v = Vector::from(&buffer[..]).transpose();
            if self.extra_bit {
                match self.systematic {
                    true => v * *GEN_4_8_SYS,
                    false => v * *GEN_4_8_MIX,
                }
                .into_iter()
                .for_each(|b| out.push(char::from(*b)));
            } else {
                match self.systematic {
                    true => v * *GEN_4_7_SYS,
                    false => v * *GEN_4_7_MIX,
                }
                .into_iter()
                .for_each(|b| out.push(char::from(*b)));
            }

            buffer.clear();
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        match self.extra_bit {
            true => self.decode_4_8(text),
            false => self.decode_4_7(text),
        }
    }
}

#[cfg(test)]
mod hamming_tests {

    use super::*;

    // Default Settings (systemtic with no extra bit)
    #[test]
    fn encode() {
        let code = HammingCode::default();
        assert_eq!(
            code.encode("100100000001").unwrap(),
            "100110000000000001111"
        );
    }

    #[test]
    fn decode() {
        let code = HammingCode::default();
        assert_eq!(code.decode("1011010").unwrap(), "1011");
    }

    // Extra Bit
    #[test]
    fn encode_extra_bit() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.encode("1011").unwrap(), "10110100");
    }

    #[test]
    fn decode_extra_bit() {
        let mut code = HammingCode::default();
        code.extra_bit = true;
        assert_eq!(code.decode("10110100").unwrap(), "1011");
    }

    // Mixed Bit Order
    #[test]
    fn encode_mixed() {
        let mut code = HammingCode::default();
        code.systematic = false;
        assert_eq!(code.encode("1001").unwrap(), "0011001");
    }

    #[test]
    fn decode_mixed() {
        let mut code = HammingCode::default();
        code.systematic = false;
        assert_eq!(code.decode("0011001").unwrap(), "1001");
    }

    // Extra Bit and Mixed Bit Order
    #[test]
    fn encode_mixed_extra() {
        let mut code = HammingCode::default();
        code.systematic = false;
        code.extra_bit = true;
        assert_eq!(code.encode("1001").unwrap(), "00110011");
    }

    #[test]
    fn decode_mixed_extra() {
        let mut code = HammingCode::default();
        code.systematic = false;
        code.extra_bit = true;
        assert_eq!(code.decode("00110011").unwrap(), "1001");
    }

    // Errors
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
