use utils::byte_formatting::ByteFormat;

use crate::Cipher;

use super::{BlockCipherMode, BlockCipherPadding};

pub const ONE: u32 = 0xffff;
pub const FUYI: u32 = 0x10000;
pub const MAXIM: u32 = 0x10001;
pub const N_SUBKEYS: usize = 52;
pub const ROUNDS: usize = 8;

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
    pub fn ksa(self, key: &[u16; 8]) -> [u16; N_SUBKEYS] {
        // There are six subkeys used in each of the eight rounds and then four additional subkeys
        let mut subkeys = [0_u16; N_SUBKEYS];

        for (i, k) in key.iter().enumerate() {
            subkeys[i] = *k;
        }

        for i in 8..N_SUBKEYS {
            if (i + 2) % 8 == 0 {
                subkeys[i] = (subkeys[i - 7] << 9) ^ (subkeys[i - 14] >> 7)
            } else if (i + 1) % 8 == 0 {
                subkeys[i] = (subkeys[i - 15] << 9) ^ (subkeys[i - 14] >> 7)
            } else {
                subkeys[i] = (subkeys[i - 7] << 9) ^ (subkeys[i - 6] >> 7)
            }
        }

        subkeys
    }

    pub fn ksa_inv(&mut self, subkeys: &[u16; N_SUBKEYS]) -> [u16; N_SUBKEYS] {
        let mut subkeys = [0_u16; N_SUBKEYS];

        for i in 0..ROUNDS {}

        subkeys
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

    // Multiplicative inverse modulo 2^16+1
    fn mul_inv(a: u16) -> u16 {
        if a <= 1 {
            a
        } else {
            let mut x = u32::from(a);
            let mut y = MAXIM;
            let mut t0 = 1u32;
            let mut t1 = 0u32;
            loop {
                t1 += y / x * t0;
                y %= x;
                if y == 1 {
                    return (MAXIM - t1) as u16;
                }
                t0 += x / y * t1;
                x %= y;
                if x == 1 {
                    return t0 as u16;
                }
            }
        }
    }

    // Addition modulo 2^16
    fn add(a: u16, b: u16) -> u16 {
        a.wrapping_add(b)
    }

    // Subtraction modulo 2^16
    fn sub(a: u16, b: u16) -> u16 {
        a.wrapping_sub(b)
    }

    // 16-bit XOR, self inverse
    fn xor(a: u16, b: u16) -> u16 {
        a ^ b
    }

    pub fn encrypt_block(&self, block: u64, subkeys: &[u16; N_SUBKEYS]) {
        let mut x1 = (block >> 48) as u16;
        let mut x2 = (block >> 32) as u16;
        let mut x3 = (block >> 16) as u16;
        let mut x4 = (block >> 0) as u16;

        for r in 0..ROUNDS {
            let j = r * 6;
            x1 = Self::mul(x1, subkeys[j]);
            x2 = Self::mul(x2, subkeys[j + 1]);
            x3 = Self::add(x3, subkeys[j + 2]);
            x4 = Self::add(x4, subkeys[j + 3]);
        }
    }

    pub fn decrypt_block(&self, block: u64, subkeys: &[u16; N_SUBKEYS]) {}
}

impl Cipher for Idea {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod idea_tests {

    use super::*;

    #[test]
    fn subkey_test() {
        let cipher = Idea::default();
        let subkeys = cipher.ksa(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 1024, 1536, 2048, 2560, 3072, 3584, 4096, 512, 16, 20, 24,
                28, 32, 4, 8, 12, 10240, 12288, 14336, 16384, 2048, 4096, 6144, 8192, 112, 128, 16,
                32, 48, 64, 80, 96, 0, 8192, 16384, 24576, 32768, 40960, 49152, 57345, 128, 192,
                256, 320
            ],
            subkeys
        )
    }
}
