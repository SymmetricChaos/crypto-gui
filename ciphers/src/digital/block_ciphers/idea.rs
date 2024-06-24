use utils::{byte_formatting::ByteFormat, math_functions::mul_inv};

use crate::Cipher;

use super::{BlockCipherMode, BlockCipherPadding};

pub const ONE: u32 = 0xffff;
pub const FUYI: u32 = 0x10000;
pub const MAXIM: u32 = 0x10001;

// https://link.springer.com/chapter/10.1007/3-540-46877-3_35
pub struct Idea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub ctr: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Idea {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            ctr: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Idea {
    pub fn ksa(&mut self, key: u128) -> [u16; 16] {
        todo!()
    }

    // Multiplication modulo 2^16+1, accomplished by swapping 0x0000 and 0xffff.
    fn mul(a: u16, b: u16) -> u16 {
        let x = if a == 0 { ONE } else { u32::from(a) };
        let y = if b == 0 { ONE } else { u32::from(b) };

        let t = (x * y) % MAXIM;

        if t == ONE {
            0
        } else {
            t as u16
        }
    }

    // fn mul_inv(a: u16) -> u16 {
    //     // Can use .unwrap() because 2^16+1 is prime
    //     let i = mul_inv(&u32::from(a), &MAXIM).unwrap();
    // }

    // Addition modulo 2^16
    fn add(a: u16, b: u16) -> u16 {
        a.wrapping_add(b)
    }

    // Subtraction modulo 2^16
    fn sub(a: u16, b: u16) -> u16 {
        a.wrapping_sub(b)
    }

    // 16-bit XOR
    fn xor(a: u16, b: u16) -> u16 {
        a ^ b
    }

    pub fn encrypt_block(&self, block: u64) {}
    pub fn decrypt_block(&self, block: u64) {}
}

impl Cipher for Idea {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
