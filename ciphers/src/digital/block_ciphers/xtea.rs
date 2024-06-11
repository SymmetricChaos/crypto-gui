use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};

use super::{bit_padding, none_padding, strip_bit_padding, BlockCipherPadding};

pub struct Xtea {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 4],
    pub padding: BlockCipherPadding,
}

impl Default for Xtea {
    fn default() -> Self {
        Self {
            key: [0, 1, 2, 3],
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            padding: BlockCipherPadding::Bit,
        }
    }
}

impl Xtea {
    const DELTA: u32 = 0x9e3779b9;

    pub fn encrypt_block(&self, v: &mut [u32; 2]) {
        let mut sum: u32 = 0;
        for _ in 0..32 {
            v[0] = v[0].wrapping_add(
                (v[1] << 4)
                    ^ (v[1] >> 5).wrapping_add(v[1])
                    ^ sum.wrapping_add(self.key[(sum % 4) as usize]),
            );
            sum = sum.wrapping_add(Self::DELTA);
            v[1] = v[1].wrapping_add(
                (v[0] << 4)
                    ^ (v[0] >> 5).wrapping_add(v[0])
                    ^ sum.wrapping_add(self.key[((sum >> 11) % 4) as usize]),
            );
        }
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut input = bytes.to_vec();

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut input, 8)?,
            BlockCipherPadding::Bit => bit_padding(&mut input, 8),
        };

        let mut out = Vec::new();

        // Take 8 byte chunks
        for block in input.chunks_exact(8) {
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

    pub fn decrypt_block(&self, v: &mut [u32; 2]) {
        let mut sum: u32 = 0xC6EF3720;
        for _ in 0..32 {
            v[1] = v[1].wrapping_sub(
                (v[0] << 4)
                    ^ (v[0] >> 5).wrapping_add(v[0])
                    ^ sum.wrapping_add(self.key[((sum >> 11) % 4) as usize]),
            );
            sum = sum.wrapping_sub(Self::DELTA);
            v[0] = v[0].wrapping_sub(
                (v[1] << 4)
                    ^ (v[1] >> 5).wrapping_add(v[1])
                    ^ sum.wrapping_add(self.key[(sum % 4) as usize]),
            );
        }
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
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

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut out, 16)?,
            BlockCipherPadding::Bit => strip_bit_padding(&mut out)?,
        };

        Ok(out)
    }
}

impl Cipher for Xtea {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.encrypt_bytes(&mut bytes)?;
        Ok(self.output_format.byte_slice_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.decrypt_bytes(&mut bytes)?;
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod xtea_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let ptext = "0102030405060708";
        let cipher = Xtea::default();
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
