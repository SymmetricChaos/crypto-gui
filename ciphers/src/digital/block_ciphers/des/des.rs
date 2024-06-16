use crate::{
    digital::block_ciphers::{
        bit_padding, des::des_functions::*, none_padding, strip_bit_padding, BlockCipherMode,
        BlockCipherPadding,
    },
    Cipher, CipherError,
};
use utils::byte_formatting::ByteFormat;

pub struct Des {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    subkeys: [u64; 16],
    pub ctr: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Des {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            subkeys: [0; 16],
            ctr: 0,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl Des {
    // Key Scheduling Algorithm (key generation)
    pub fn ksa(&mut self, key: u64) -> Result<(), CipherError> {
        test_des_key(key)?;
        des_ksa(&mut self.subkeys, key);
        Ok(())
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter().rev() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    pub fn encrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        assert!(bytes.len() % 8 == 0);
        let mut out = Vec::with_capacity(bytes.len());

        for plaintext in bytes.chunks_exact(8) {
            let ciphertext = self.encrypt_block(u64::from_be_bytes(plaintext.try_into().unwrap()));
            out.extend_from_slice(&ciphertext.to_be_bytes());
        }

        Ok(out)
    }

    pub fn decrypt_ecb(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        assert!(bytes.len() % 8 == 0);
        let mut out = Vec::with_capacity(bytes.len());

        for ciphertext in bytes.chunks_exact(8) {
            let plaintext = self.decrypt_block(u64::from_be_bytes(ciphertext.try_into().unwrap()));
            out.extend_from_slice(&plaintext.to_be_bytes());
        }

        Ok(out)
    }

    pub fn encrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut ctr = self.ctr;
        let mut out = Vec::with_capacity(bytes.len());

        for plaintext in bytes.chunks_exact(8) {
            let keytext = self.encrypt_block(ctr).to_le_bytes();

            for (k, p) in keytext.into_iter().zip(plaintext.iter()) {
                out.push(k ^ p)
            }

            ctr = ctr.wrapping_add(1);
        }

        Ok(out)
    }

    pub fn decrypt_ctr(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        self.encrypt_ctr(bytes)
    }
}

impl Cipher for Des {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut bytes, 8)?,
            BlockCipherPadding::Bit => bit_padding(&mut bytes, 8),
        };

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
            none_padding(&mut bytes, 8)?
        };

        let mut out = match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes)?,
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes)?,
            BlockCipherMode::Cbc => return Err(CipherError::state("CBC mode not implemented")),
        };

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut out, 8)?,
            BlockCipherPadding::Bit => strip_bit_padding(&mut out)?,
        };

        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod des_tests {

    use super::*;

    #[test]
    fn test_encypt_block() {
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF);

        let cblock = cipher.encrypt_block(0x4E6F772069732074);
        assert_eq!(cblock, 0x3FA40E8A984D4815);

        let dblock = cipher.decrypt_block(0x3FA40E8A984D4815);
        assert_eq!(dblock, 0x4E6F772069732074);
    }

    #[test]
    fn test_encypt_ecb() {
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF);
        cipher.mode = BlockCipherMode::Ecb;
        cipher.padding = BlockCipherPadding::None;

        const PTEXT: &'static str = "4e6f772069732074";
        const CTEXT: &'static str = "3fa40e8a984d4815";

        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(CTEXT, ctext);

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }
}
