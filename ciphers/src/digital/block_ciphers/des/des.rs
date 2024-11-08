use crate::{
    digital::block_ciphers::{
        block_cipher::{BCMode, BCPadding, BlockCipher},
        des::des_functions::*,
    },
    impl_cipher_for_block_cipher, CipherError,
};
use utils::byte_formatting::{overwrite_bytes, ByteFormat};

pub struct Des {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    subkeys: [u64; 16],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Des {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            subkeys: [0; 16],
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

crate::block_cipher_builders! {Des, u64}

impl Des {
    // Key Scheduling Algorithm (key generation)
    pub fn ksa(&mut self, key: u64) -> Result<(), CipherError> {
        self.subkeys = des_ksa(key)?;
        Ok(())
    }
}

impl BlockCipher<8> for Des {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter() {
            b = round(b, *key);
        }
        let f = final_permutation(b.rotate_left(32));

        overwrite_bytes(bytes, &f.to_be_bytes());
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let block = u64::from_be_bytes(bytes.try_into().unwrap());
        let mut b = initial_permutation(block);
        for key in self.subkeys.iter().rev() {
            b = round(b, *key);
        }
        let f = final_permutation(b.rotate_left(32));
        overwrite_bytes(bytes, &f.to_be_bytes());
    }
}

impl_cipher_for_block_cipher!(Des, 8);

#[cfg(test)]
mod des_tests {

    use rand::{thread_rng, Rng};
    use strum::IntoEnumIterator;

    use crate::Cipher;

    use super::*;

    #[test]
    fn test_encrypt_decrypt_block() {
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();

        let mut bytes = 0x4E6F772069732074_u64.to_be_bytes();

        cipher.encrypt_block(&mut bytes);
        assert_eq!(bytes, 0x3FA40E8A984D4815_u64.to_be_bytes());

        cipher.decrypt_block(&mut bytes);
        assert_eq!(bytes, 0x4E6F772069732074_u64.to_be_bytes());
    }

    #[test]
    fn basic_test_encrypt_decrypt() {
        let mut cipher = Des::default();
        let mut rng = thread_rng();

        let k = rng.gen();
        match cipher.ksa(k) {
            Ok(_) => (),
            Err(_) => panic!("error with ksa for key: {}", k),
        }
        for mode in BCMode::iter() {
            for padding in BCPadding::iter() {
                cipher.mode = mode;
                cipher.padding = padding;

                const PTEXT: &'static str = "4e6f772069732074";

                let ctext = cipher.encrypt(PTEXT).unwrap();
                let dtext = cipher.decrypt(&ctext).unwrap();
                assert_eq!(PTEXT, dtext, "{:?} {:?}", padding, mode);
            }
        }
    }

    #[test]
    fn test_encrypt_decrypt_ecb() {
        let mut cipher = Des::default();
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
        let mut cipher = Des::default();
        cipher.ksa(0x0123456789ABCDEF).unwrap();
        cipher.mode = BCMode::Ctr;

        const PTEXT: &'static str = "4e6f772069732074";

        let ctext = cipher.encrypt(PTEXT).unwrap();

        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(PTEXT, dtext);
    }
}
