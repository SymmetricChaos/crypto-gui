use crate::{Cipher, CipherError};
use utils::byte_formatting::ByteFormat;

use super::BlockCipherMode;

pub struct Tea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 4],
    pub ctr: u64,
    pub mode: BlockCipherMode,
}

impl Default for Tea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            ctr: 0,
            mode: BlockCipherMode::ECB,
        }
    }
}

impl Tea {
    const DELTA: u32 = 0x9e3779b9;

    // Encrypt a block.
    pub fn encrypt_block(&self, v: &mut [u32; 2]) {
        let mut sum: u32 = 0;
        for _ in 0..32 {
            sum = sum.wrapping_add(Self::DELTA);
            v[0] = v[0].wrapping_add(
                ((v[1] << 4).wrapping_add(self.key[0]))
                    ^ (v[1].wrapping_add(sum))
                    ^ ((v[1] >> 5).wrapping_add(self.key[1])),
            );
            v[1] = v[1].wrapping_add(
                ((v[0] << 4).wrapping_add(self.key[2]))
                    ^ (v[0].wrapping_add(sum))
                    ^ ((v[0] >> 5).wrapping_add(self.key[3])),
            );
        }
    }

    // Encrypt in CTR mode.
    pub fn encrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        // No padding rule is given
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "encrypted data must be in chunks of 64 bits",
            ));
        }
        let mut out = Vec::new();
        let mut ctr = self.ctr;

        // Take 8 byte chunks
        for block in bytes.chunks_exact(8) {
            // Encrypt the counter
            let mut b = [(ctr >> 32) as u32, ctr as u32];
            self.encrypt_block(&mut b);

            // Save the values
            let mut mask = Vec::with_capacity(8);
            mask.extend_from_slice(&b[0].to_be_bytes());
            mask.extend_from_slice(&b[1].to_be_bytes());

            // XOR the bytes of the plaintext with the masking bytes
            for (byte, m) in block.iter().zip(mask.iter()) {
                out.push(byte ^ m)
            }

            // Increment
            ctr = ctr.wrapping_add(1);
        }

        Ok(out)
    }

    // Encrypt in ECB mode.
    pub fn encrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        // No padding rule is given
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "encrypted data must be in chunks of 64 bits",
            ));
        }
        let mut out = Vec::new();

        // Take 8 byte chunks
        for block in bytes.chunks_exact(8) {
            // Turn each chunk into a pair of u32
            let mut x = [0u32; 2];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_be_bytes(chunk.try_into().unwrap());
            }

            // Encrypt that pair
            self.encrypt_block(&mut x);

            // Push bytes to the output
            out.extend_from_slice(&x[0].to_be_bytes());
            out.extend_from_slice(&x[1].to_be_bytes());
        }

        Ok(out)
    }

    // Decrypt a block
    pub fn decrypt_block(&self, v: &mut [u32; 2]) {
        let mut sum: u32 = 0xC6EF3720;
        for _ in 0..32 {
            v[1] = v[1].wrapping_sub(
                ((v[0] << 4).wrapping_add(self.key[2]))
                    ^ (v[0].wrapping_add(sum))
                    ^ ((v[0] >> 5).wrapping_add(self.key[3])),
            );
            v[0] = v[0].wrapping_sub(
                ((v[1] << 4).wrapping_add(self.key[0]))
                    ^ (v[1].wrapping_add(sum))
                    ^ ((v[1] >> 5).wrapping_add(self.key[1])),
            );
            sum = sum.wrapping_sub(Self::DELTA);
        }
    }

    // Decrypt in CTR mode. Identical to encrypt as CTR mode operates as a stream cipher
    pub fn decrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        self.encrypt_ctr(bytes)
    }

    // Decrypt in ECB mode.
    pub fn decrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        // No padding rule is given
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "decrypted data must be in chunks of 64 bits",
            ));
        }
        let mut out = Vec::new();

        for block in bytes.chunks_exact(8) {
            let mut x = [0u32; 2];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_be_bytes(chunk.try_into().unwrap());
            }

            self.decrypt_block(&mut x);

            out.extend_from_slice(&x[0].to_be_bytes());
            out.extend_from_slice(&x[1].to_be_bytes());
        }

        Ok(out)
    }
}

impl Cipher for Tea {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = match self.mode {
            BlockCipherMode::ECB => self.encrypt_ecb(&mut bytes)?,
            BlockCipherMode::CTR => self.encrypt_ctr(&mut bytes)?,
        };
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = match self.mode {
            BlockCipherMode::ECB => self.decrypt_ecb(&mut bytes)?,
            BlockCipherMode::CTR => self.decrypt_ctr(&mut bytes)?,
        };
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod tea_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = Tea::default();
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    // #[test]
    // fn encrypt_test() {
    //     const PTEXT: &'static str = "0000000000000000";
    //     const CTEXT: &'static str = "21a5dbee154b8f6d";
    //     const KEY: &'static str = "00000000000000000000000000000000";
    //     let mut cipher = Tea::default();
    //     assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    // }

    // #[test]
    // fn decrypt_test() {
    //     const PTEXT: &'static str = "0000000000000000";
    //     const CTEXT: &'static str = "21a5dbee154b8f6d";
    //     const KEY: &'static str = "00000000000000000000000000000000";
    //     let mut cipher = Tea::default();
    //     assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    // }

    // #[test]
    // fn encrypt_test_2() {
    //     const PTEXT: &'static str = "21a5dbee154b8f6d";
    //     const CTEXT: &'static str = "f7c013ac5b2b8952";
    //     const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
    //     let mut cipher = Tea::default();
    //     cipher.ksa_32(&hex_to_bytes_be(KEY).unwrap());
    //     assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    // }

    // #[test]
    // fn decrypt_test_2() {
    //     const PTEXT: &'static str = "21a5dbee154b8f6d";
    //     const CTEXT: &'static str = "f7c013ac5b2b8952";
    //     const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
    //     let mut cipher = Tea::default();
    //     cipher.ksa_32(&hex_to_bytes_be(KEY).unwrap());
    //     assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    // }

    // #[test]
    // fn encrypt_decrypt_test_2() {
    //     const PTEXT: &'static str = "21a5dbee154b8f6d";
    //     const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
    //     let mut cipher = Tea::default();
    //     cipher.ksa_32(&hex_to_bytes_be(KEY).unwrap());
    //     let ctext = cipher.encrypt(PTEXT).unwrap();
    //     assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    // }
}