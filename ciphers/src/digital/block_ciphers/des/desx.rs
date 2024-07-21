use crate::{
    digital::block_ciphers::{
        block_cipher::{none_padding, BCMode, BCPadding, BlockCipher},
        des::des_functions::*,
    },
    impl_block_cipher, Cipher, CipherError,
};
use utils::byte_formatting::{overwrite_bytes, ByteFormat};

pub const BLOCKSIZE: u32 = 8;

pub struct DesX {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub extra_keys: [u64; 2],
    subkeys: [u64; 16],
    pub iv: u64,
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
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl DesX {
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
        overwrite_bytes(bytes, &f.to_be_bytes());
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
        overwrite_bytes(bytes, &f.to_be_bytes());
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl_block_cipher!(DesX);

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
