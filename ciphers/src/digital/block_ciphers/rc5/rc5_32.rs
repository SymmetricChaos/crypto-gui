crate::impl_rc5!(Rc5_32, u32, 4, 32, 8, 0xb7e15163, 0x9e3779b9, u64, 12);

#[cfg(test)]
mod rc5_tests {

    use super::*;
    use crate::{
        digital::block_ciphers::block_cipher::{BCMode, BCPadding},
        Cipher,
    };
    use hex_literal::hex;

    // Test vectors from
    // https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=fe22353a2b9b557d1130bf9ba9f1f73fe26359cd
    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("00000000000000000000000000000000"));
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("00000000000000000000000000000000"));
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("00000000000000000000000000000000"));
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("915f4619be41b2516355a50110a9ce91"));
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("915f4619be41b2516355a50110a9ce91"));
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        let mut cipher = Rc5_32::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("915f4619be41b2516355a50110a9ce91"));
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
