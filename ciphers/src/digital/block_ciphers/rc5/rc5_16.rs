use crate::{impl_block_cipher, impl_rc5};

impl_rc5!(Rc5_16, u16, 2, 16, 4, 0xb7e1, 0x9e37, u32, 12);

impl_block_cipher!(Rc5_16, 4);

#[cfg(test)]
mod rc5_tests {

    use utils::byte_formatting::hex_to_bytes_ltr;

    use crate::{
        digital::block_ciphers::block_cipher::{BCMode, BCPadding},
        Cipher,
    };

    use super::*;
    // Test vectors from
    // https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=fe22353a2b9b557d1130bf9ba9f1f73fe26359cd

    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "00010203";
        const CTEXT: &'static str = "23a8d72e";
        const KEY: &'static str = "0001020304050607";
        let mut cipher = Rc5_16::default();
        cipher.rounds = 16;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "00010203";
        const CTEXT: &'static str = "23a8d72e";
        const KEY: &'static str = "0001020304050607";
        let mut cipher = Rc5_16::default();
        cipher.rounds = 16;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test() {
        const PTEXT: &'static str = "00010203";
        const KEY: &'static str = "0001020304050607";
        let mut cipher = Rc5_16::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
