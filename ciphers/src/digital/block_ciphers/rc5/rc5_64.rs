crate::impl_rc5!(
    Rc5_64,
    u64,
    8,
    64,
    16,
    0xb7e151628aed2a6b,
    0x9e3779b97f4a7c15,
    u128,
    20
);

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
    // https://datatracker.ietf.org/doc/html/draft-krovetz-rc6-rc5-vectors-00#section-4
    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "000102030405060708090a0b0c0d0e0f";
        const CTEXT: &'static str = "a46772820edbce0235abea32ae7178da";
        let mut cipher = Rc5_64::default();
        cipher.rounds = 24;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("000102030405060708090a0b0c0d0e0f1011121314151617"));
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "000102030405060708090a0b0c0d0e0f";
        const CTEXT: &'static str = "a46772820edbce0235abea32ae7178da";
        let mut cipher = Rc5_64::default();
        cipher.rounds = 24;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("000102030405060708090a0b0c0d0e0f1011121314151617"));
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
