use crate::{
    ecc::{bits_from_bitstring, check_bitstring, Bit},
    errors::CodeError,
    traits::Code,
};

use nalgebra::{ArrayStorage, SMatrix, Vector};

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

// impl HammingCode {
//     pub fn total_bits(&self) -> u32 {
//         2_u32.pow(self.parity_bits) - 1
//     }

//     pub fn data_bits(&self) -> u32 {
//         self.total_bits() - self.parity_bits
//     }
// }

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

        todo!("match on self.parity bits")
    }
}

#[cfg(test)]
mod hamming_tests {
    use nalgebra::Vector4;

    use super::*;

    #[test]
    fn show_matrix() {
        println!("{:?}", GEN_4_7);
        let prod = Vector4::from([1, 0, 1, 1]).transpose() * (&GEN_4_7);
        println!("{:?}", prod);
    }

    #[test]
    fn encode() {
        let code = HammingCode::default();
        println!("{}", code.encode("1011").unwrap());
    }
}
