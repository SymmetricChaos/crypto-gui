use utils::byte_formatting::ByteFormat;

use crate::{
    digital::block_ciphers::{
        block_cipher::{none_padding, BlockCipher, BCMode, BCPadding},
        des::des::Des,
    },
    Cipher, CipherError,
};

use super::des_functions::*;

pub struct TripleDes {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    subkeys: [[u64; 16]; 3],
    pub ctr: u64,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for TripleDes {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            subkeys: [[0; 16]; 3],
            ctr: 0,
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl TripleDes {
    pub fn ksa(&mut self, keys: [u64; 3]) -> Result<(), CipherError> {
        let mut temp = [[0u64; 16]; 3];
        for (i, key) in keys.into_iter().enumerate() {
            temp[i] = des_ksa(key)?;
        }
        self.subkeys = temp;
        Ok(())
    }

    fn encrypt_with_subkey(&self, block: u64, i: usize) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys[i].iter() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }

    fn decrypt_with_subkey(&self, block: u64, i: usize) -> u64 {
        let mut b = initial_permutation(block);
        for key in self.subkeys[i].iter().rev() {
            b = round(b, *key);
        }
        final_permutation((b << 32) | (b >> 32))
    }
}

impl BlockCipher<8> for TripleDes {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let b = self.encrypt_with_subkey(block, 2);
        let b = self.decrypt_with_subkey(b, 1);
        let b = self.encrypt_with_subkey(b, 0);
        for (plaintext, ciphertext) in bytes.iter_mut().zip(b.to_be_bytes().iter()) {
            *plaintext = *ciphertext
        }
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let b = self.decrypt_with_subkey(block, 0);
        let b = self.encrypt_with_subkey(b, 1);
        let b = self.decrypt_with_subkey(b, 2);
        for (ciphertext, plaintext) in bytes.iter_mut().zip(b.to_be_bytes().iter()) {
            *ciphertext = *plaintext
        }
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl Cipher for TripleDes {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            self.padding.add_padding(&mut bytes, Des::BLOCKSIZE)?;
        }

        match self.mode {
            BCMode::Ecb => self.encrypt_ecb(&mut bytes),
            BCMode::Ctr => self.encrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.encrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            if self.padding == BCPadding::None {
                none_padding(&mut bytes, Des::BLOCKSIZE)?
            };
        }

        match self.mode {
            BCMode::Ecb => self.decrypt_ecb(&mut bytes),
            BCMode::Ctr => self.decrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.decrypt_cbc(&mut bytes, self.iv.to_be_bytes()),
        };

        if self.mode.padded() {
            self.padding.strip_padding(&mut bytes, Des::BLOCKSIZE)?;
        }

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}

#[cfg(test)]
mod des_tests {

    use super::*;

    // #[test]
    // fn test_encypt_decrypt_block() {
    //     let mut cipher = TripleDes::default();
    //     cipher
    //         .ksa([0x0123456789ABCDEF, 0x0101010101010101, 0x1010101010101010])
    //         .unwrap();

    //     let pblock = 0x4E6F772069732074;

    //     let cblock = cipher.encrypt_block(pblock);
    //     let dblock = cipher.decrypt_block(cblock);
    //     assert_eq!(dblock, pblock);
    // }

    // #[test]
    // fn test_encypt_ecb() {
    //     let mut cipher = TripleDes::default();
    //     cipher.ksa(0x0123456789ABCDEF).unwrap();
    //     cipher.mode = BlockCipherMode::Ecb;
    //     cipher.padding = BlockCipherPadding::None;

    //     const PTEXT: &'static str = "4e6f772069732074";
    //     const CTEXT: &'static str = "3fa40e8a984d4815";

    //     let ctext = cipher.encrypt(PTEXT).unwrap();
    //     assert_eq!(CTEXT, ctext);

    //     let dtext = cipher.decrypt(&ctext).unwrap();
    //     assert_eq!(PTEXT, dtext);
    // }

    // use rand::{thread_rng, Rng};
    use rand::{thread_rng, Rng};
    #[test]
    fn basic_test_encrypt_decrypt() {
        let mut cipher = TripleDes::default();
        let mut rng = thread_rng();

        let k = rng.gen();
        match cipher.ksa(k) {
            Ok(_) => (),
            Err(_) => panic!("error with ksa for key: {:?}", k),
        }
        for mode in BCMode::variants() {
            for padding in BCPadding::variants() {
                cipher.mode = mode;
                cipher.padding = padding;

                const PTEXT: &'static str = "4e6f772069732074";

                let ctext = cipher.encrypt(PTEXT).unwrap();
                let dtext = cipher.decrypt(&ctext).unwrap();
                assert_eq!(PTEXT, dtext, "{:?} {:?}", padding, mode);
            }
        }
    }
}
