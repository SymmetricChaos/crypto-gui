use utils::byte_formatting::ByteFormat;

use crate::Cipher;

use super::{BlockCipherMode, BlockCipherPadding};

pub const ONE: u32 = 0xffff;
pub const FUYI: u32 = 0x10000;
pub const MAXIM: u32 = 0x10001;
pub const N_SUBKEYS: usize = 52;
pub const ROUNDS: usize = 8;

// Original paper
// https://link.springer.com/chapter/10.1007/3-540-46877-3_35
// Implementation in Rust
// https://docs.rs/idea/latest/src/idea/lib.rs.html
pub struct Idea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub ctr: u64,
    subkeys_enc: [u16; N_SUBKEYS],
    subkeys_dec: [u16; N_SUBKEYS],
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Idea {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            ctr: 0,
            subkeys_enc: [0u16; N_SUBKEYS],
            subkeys_dec: [0u16; N_SUBKEYS],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Idea {
    pub fn subkeys_enc(&self) -> &[u16; N_SUBKEYS] {
        &self.subkeys_enc
    }

    pub fn subkeys_dec(&self) -> &[u16; N_SUBKEYS] {
        &self.subkeys_dec
    }

    pub fn ksa(&mut self, key: &[u16; 8]) {
        // There are six subkeys used in each of the eight rounds and then four additional subkeys

        for (i, k) in key.iter().enumerate() {
            self.subkeys_enc[i] = *k;
        }

        for i in 8..N_SUBKEYS {
            if (i + 2) % 8 == 0 {
                self.subkeys_enc[i] =
                    (self.subkeys_enc[i - 7] << 9) ^ (self.subkeys_enc[i - 14] >> 7)
            } else if (i + 1) % 8 == 0 {
                self.subkeys_enc[i] =
                    (self.subkeys_enc[i - 15] << 9) ^ (self.subkeys_enc[i - 14] >> 7)
            } else {
                self.subkeys_enc[i] =
                    (self.subkeys_enc[i - 7] << 9) ^ (self.subkeys_enc[i - 6] >> 7)
            }
        }

        // Calculate the inverse keys
        let mut k = ROUNDS * 6;
        for i in 0..=ROUNDS {
            let j = i * 6;
            let l = k - j;

            // Not sure why the Rust implementation I checked had this not matching the paper
            self.subkeys_dec[j] = Self::mul_inv(self.subkeys_enc[l]);
            self.subkeys_dec[j + 1] = Self::mul_inv(self.subkeys_enc[l + 1]);
            self.subkeys_dec[j + 2] = Self::add_inv(self.subkeys_enc[l + 2]);
            self.subkeys_dec[j + 3] = Self::add_inv(self.subkeys_enc[l + 3]);
        }

        k = (ROUNDS - 1) * 6;
        for i in 0..ROUNDS {
            let j = i * 6;
            let l = k - j;
            self.subkeys_dec[j + 4] = self.subkeys_enc[l + 4];
            self.subkeys_dec[j + 5] = self.subkeys_enc[l + 5];
        }
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

    // Additive inverse modulo 2^16
    fn add_inv(a: u16) -> u16 {
        ((FUYI - (u32::from(a))) & ONE) as u16
    }

    pub fn encrypt_block(&self, block: u64, keys: &[u16; N_SUBKEYS]) -> u64 {
        let mut x1 = (block >> 48) as u16;
        let mut x2 = (block >> 32) as u16;
        let mut x3 = (block >> 16) as u16;
        let mut x4 = (block >> 0) as u16;

        for r in 0..ROUNDS {
            let j = r * 6;
            x1 = Self::mul(x1, keys[j]);
            x2 = Self::mul(x2, keys[j + 1]);
            x3 = Self::add(x3, keys[j + 2]);
            x4 = Self::add(x4, keys[j + 3]);
            let kk = Self::mul(keys[j + 4], x1 ^ x3);
            let t1 = Self::mul(keys[j + 5], Self::add(kk, x1 ^ x3));
            let t2 = Self::add(kk, t1);
            let a = x1 ^ t1;
            x1 = x3 ^ t1;
            x3 = a;
            let a = x2 ^ t2;
            x2 = x4 ^ t2;
            x4 = a;
        }
        x1 = Self::mul(x1, keys[48 + 0]);
        x2 = Self::mul(x2, keys[48 + 1]);
        x3 = Self::add(x3, keys[48 + 2]);
        x4 = Self::add(x4, keys[48 + 3]);

        (x1 as u64) << 48 | (x2 as u64) << 32 | (x3 as u64) << 16 | (x4 as u64)
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
        let mut cipher = Idea::default();
        cipher.ksa(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 1024, 1536, 2048, 2560, 3072, 3584, 4096, 512, 16, 20, 24,
                28, 32, 4, 8, 12, 10240, 12288, 14336, 16384, 2048, 4096, 6144, 8192, 112, 128, 16,
                32, 48, 64, 80, 96, 0, 8192, 16384, 24576, 32768, 40960, 49152, 57345, 128, 192,
                256, 320
            ],
            cipher.subkeys_enc()
        );

        assert_eq!(
            &[
                65025, 43350, 65280, 65216, 49152, 57345, 65533, 21843, 32768, 24576, 0, 8192,
                42326, 64513, 65456, 65440, 16, 32, 21835, 65529, 65424, 65408, 2048, 4096, 13101,
                43686, 51200, 49152, 8, 12, 19115, 53834, 65504, 65532, 16, 20, 43670, 28069,
                61440, 65024, 2048, 2560, 18725, 57345, 64512, 64000, 5, 6, 1, 32769, 65533, 65532
            ],
            cipher.subkeys_dec()
        );
    }
}
