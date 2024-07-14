use crate::{
    digital::block_ciphers::{
        block_cipher::{none_padding, BCMode, BCPadding, BlockCipher},
        des::des_functions::*,
    },
    Cipher, CipherError,
};
use utils::byte_formatting::ByteFormat;

pub struct DesX {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub extra_keys: [u64; 2],
    subkeys: [u64; 16],
    pub ctr: u64,
    pub cbc: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for DesX {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            extra_keys: [0, 0],
            subkeys: [0; 16],
            ctr: 0,
            cbc: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl DesX {
    pub const BLOCKSIZE: u32 = 8;

    // Key Scheduling Algorithm (key generation)
    pub fn ksa(&mut self, key: u64) -> Result<(), CipherError> {
        self.subkeys = des_ksa(key)?;
        Ok(())
    }
}

impl BlockCipher<8> for DesX {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = u64::from_be_bytes(bytes.try_into().unwrap());
        block ^= self.extra_keys[0];
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter() {
            b = round(b, *key);
        }
        let mut f = final_permutation((b << 32) | (b >> 32));
        f ^= self.extra_keys[1];
        for (plaintext, ciphertext) in bytes.iter_mut().zip(f.to_be_bytes().iter()) {
            *plaintext = *ciphertext
        }
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = u64::from_be_bytes(bytes.try_into().unwrap());
        block ^= self.extra_keys[1];
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter().rev() {
            b = round(b, *key);
        }
        let mut f = final_permutation((b << 32) | (b >> 32));
        f ^= self.extra_keys[0];
        for (ciphertext, plaintext) in bytes.iter_mut().zip(f.to_be_bytes().iter()) {
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

impl Cipher for DesX {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        if self.mode.padded() {
            self.padding.add_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        match self.mode {
            BCMode::Ecb => self.encrypt_ecb(&mut bytes),
            BCMode::Ctr => self.encrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.encrypt_cbc(&mut bytes, self.cbc.to_be_bytes()),
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
                none_padding(&mut bytes, Self::BLOCKSIZE)?
            };
        }

        match self.mode {
            BCMode::Ecb => self.decrypt_ecb(&mut bytes),
            BCMode::Ctr => self.decrypt_ctr(&mut bytes, self.ctr.to_be_bytes()),
            BCMode::Cbc => self.decrypt_cbc(&mut bytes, self.cbc.to_be_bytes()),
        };

        if self.mode.padded() {
            self.padding.strip_padding(&mut bytes, Self::BLOCKSIZE)?;
        }

        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}

#[cfg(test)]
mod des_tests {

    use super::*;

    #[test]
    fn test_encrypt_decrypt_ecb() {
        let mut cipher = DesX::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;

        const PTEXT: &'static str = "4e6f772069732074";
        const CTEXT: &'static str = "3fa40e8a984d4815";

        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(CTEXT, ctext);

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }

    #[test]
    fn test_encrypt_decrypt_ctr() {
        let mut cipher = DesX::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();
        cipher.mode = BCMode::Ctr;

        const PTEXT: &'static str = "4e6f772069732074";

        let ctext = cipher.encrypt(PTEXT).unwrap();

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }
}

#[cfg(test)]
mod desx_tests {

    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn basic_test_encrypt_decrypt() {
        let mut cipher = DesX::default();
        let mut rng = thread_rng();

        let k = rng.gen();
        match cipher.ksa(k) {
            Ok(_) => (),
            Err(_) => panic!("error with ksa for key: {}", k),
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
