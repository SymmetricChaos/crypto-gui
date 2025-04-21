use crate::digital::block_ciphers::block_cipher::BlockCipher;

crate::impl_rc5!(Rc5_16, u16, 2, 16, 4, 0xb7e1, 0x9e37, u32, 12);

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
        const PTEXT: &'static str = "00010203";
        const CTEXT: &'static str = "23a8d72e";
        let mut cipher = Rc5_16::default();
        cipher.rounds = 16;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("0001020304050607"));
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "00010203";
        const CTEXT: &'static str = "23a8d72e";
        let mut cipher = Rc5_16::default();
        cipher.rounds = 16;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex!("0001020304050607"));
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
