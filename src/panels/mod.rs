use crate::ciphers::Cipher;

pub mod caesar_panel;
pub mod cipher_windows;
pub mod affine_panel;
pub mod substitution_panel;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}


fn run_cipher(mode: &mut Mode, cipher: &dyn Cipher, plaintext: &mut String, ciphertext: &mut String) {
    if *mode == Mode::Encrypt {
        match cipher.encrypt(plaintext) {
            Ok(text) => *ciphertext = text ,
            Err(e) => *ciphertext = String::from(e),
        }
    } else {
        match cipher.decrypt(ciphertext) {
            Ok(text) => *plaintext = text ,
            Err(e) => *plaintext = String::from(e),
        }
    }
}