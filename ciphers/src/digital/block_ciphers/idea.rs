use super::{none_padding, BlockCipherMode, BlockCipherPadding};
use crate::{Cipher, CipherError};
use utils::byte_formatting::ByteFormat;

pub const ONE: u32 = 0xffff;
pub const FUYI: u32 = 0x10000;
pub const MAXIM: u32 = 0x10001;
pub const N_SUBKEYS: usize = ROUNDS * 6 + 4; // there are six keys used in each of eight rounds, then four keys used in the finalization round
pub const ROUNDS: usize = 8;
pub const BLOCK_SIZE: usize = 8;

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
            self.subkeys_dec[j + 0] = Self::mul_inv(self.subkeys_enc[l + 0]);
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

    // Multiplication modulo 2^16+1 (sort of)
    fn mul(a: u16, b: u16) -> u16 {
        let x = u32::from(a);
        let y = u32::from(b);
        let mut r: i32;

        if x == 0 {
            r = (MAXIM - y) as i32;
        } else if y == 0 {
            r = (MAXIM - x) as i32;
        } else {
            let c: u32 = x * y;
            r = ((c & ONE) as i32) - ((c >> 16) as i32);
            if r < 0 {
                r += MAXIM as i32;
            }
        }

        (r & (ONE as i32)) as u16
    }

    // Multiplicative inverse modulo 2^16+1 (sort of)
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
    // Exhastive test shows that both versions below are identical
    fn add(a: u16, b: u16) -> u16 {
        // ((u32::from(a) + u32::from(b)) & ONE) as u16
        a.wrapping_add(b)
    }

    // Additive inverse modulo 2^16
    // Exhastive test shows that both versions below are identical
    fn add_inv(a: u16) -> u16 {
        // ((FUYI - (u32::from(a))) & ONE) as u16
        (u16::MAX - a).wrapping_add(1)
    }

    fn block_function(block: u64, keys: &[u16; N_SUBKEYS]) -> u64 {
        let mut x1 = (block >> 48) as u16;
        let mut x2 = (block >> 32) as u16;
        let mut x3 = (block >> 16) as u16;
        let mut x4 = (block >> 0) as u16;

        let mut kk: u16;
        let mut t1: u16;
        let mut t2: u16;
        let mut a: u16;

        for r in 0..ROUNDS {
            // println!("{x1:5?} {x2:5?} {x3:5?} {x4:5?}");
            let j = r * 6;
            x1 = Self::mul(x1, keys[j + 0]);
            x2 = Self::mul(x2, keys[j + 1]);
            x3 = Self::add(x3, keys[j + 2]);
            x4 = Self::add(x4, keys[j + 3]);
            kk = Self::mul(keys[j + 4], x1 ^ x3);
            t1 = Self::mul(keys[j + 5], Self::add(kk, x2 ^ x4));
            t2 = Self::add(kk, t1);
            a = x1 ^ t1;
            x1 = x3 ^ t1;
            x3 = a;
            a = x2 ^ t2;
            x2 = x4 ^ t2;
            x4 = a;
        }
        // println!("{x1:5?} {x2:5?} {x3:5?} {x4:5?}");
        x1 = Self::mul(x1, keys[48]);
        x2 = Self::mul(x2, keys[49]);
        x3 = Self::add(x3, keys[50]);
        x4 = Self::add(x4, keys[51]);
        // println!("{x1:5?} {x2:5?} {x3:5?} {x4:5?}");

        (x1 as u64) << 48 | (x2 as u64) << 32 | (x3 as u64) << 16 | (x4 as u64)
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        Self::block_function(block, self.subkeys_enc())
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
        Self::block_function(block, self.subkeys_dec())
    }

    pub fn encrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        assert!(bytes.len() % BLOCK_SIZE == 0);
        let mut out = Vec::with_capacity(bytes.len());

        // Take 8 byte chunks
        for block in bytes.chunks_exact(BLOCK_SIZE) {
            // Turn each chunk into a u64
            let x = u64::from_be_bytes(block.try_into().unwrap());

            // Push bytes to the output
            out.extend_from_slice(&self.encrypt_block(x).to_be_bytes());
        }

        Ok(out)
    }

    pub fn decrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        assert!(bytes.len() % 8 == 0);
        let mut out = Vec::with_capacity(bytes.len());

        // Take 8 byte chunks
        for block in bytes.chunks_exact(BLOCK_SIZE) {
            // Turn each chunk into a u64
            let x = u64::from_be_bytes(block.try_into().unwrap());

            // Push bytes to the output
            out.extend_from_slice(&self.decrypt_block(x).to_be_bytes());
        }

        Ok(out)
    }

    pub fn encrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut ctr = self.ctr;
        let mut out = Vec::with_capacity(bytes.len());

        // Take 8 byte chunks
        for block in bytes.chunks_exact(BLOCK_SIZE) {
            let mask = self.encrypt_block(ctr).to_be_bytes();

            for (byte, m) in block.iter().zip(mask.iter()) {
                out.push(byte ^ m)
            }

            ctr = ctr.wrapping_add(1);
        }

        Ok(out)
    }

    pub fn decrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        self.encrypt_ctr(bytes)
    }
}

impl Cipher for Idea {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            self.padding.add_padding(&mut bytes, BLOCK_SIZE as u32)?;
        }

        let out = match self.mode {
            BlockCipherMode::Ecb => self.encrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.encrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => return Err(CipherError::state("CBC mode not implemented")),
        };

        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.padding == BlockCipherPadding::None {
            none_padding(&mut bytes, BLOCK_SIZE as u32)?
        };

        let out = match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => return Err(CipherError::state("CBC mode not implemented")),
        };

        if self.mode.padded() {
            self.padding.strip_padding(&mut bytes, BLOCK_SIZE as u32)?;
        }

        Ok(self.output_format.byte_slice_to_text(&out))
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

    #[test]
    fn encrypt_decrypt_test() {
        let mut cipher = Idea::default();
        cipher.ksa(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let ptext = 0x0000_0001_0002_0003; // from 0 1 2 3
        let ctext = 0x3ffb311b0a44067b; // from 16379 12571 2628 1659
        let etext: u64 = cipher.encrypt_block(ptext);
        assert_eq!(ctext, etext);
        assert_eq!(ptext, cipher.decrypt_block(etext));
    }
}
