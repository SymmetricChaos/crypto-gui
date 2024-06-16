pub mod aes;
pub mod blowfish;
pub mod blowfish_arrays;
pub mod des;
pub mod feal;
pub mod idea;
pub mod rc5;
pub mod tea;
pub mod xtea;

use crate::CipherError;

pub trait BlockCipher {
    fn encrypt_block(&self, bytes: &mut [u8]);
    fn decrypt_block(&self, bytes: &mut [u8]);
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherMode {
    Ecb,
    Ctr,
    Cbc,
}

impl BlockCipherMode {
    /// Is a padding rule needed?
    pub fn padded(&self) -> bool {
        match self {
            BlockCipherMode::Ecb => true,
            BlockCipherMode::Ctr => false,
            BlockCipherMode::Cbc => true,
        }
    }
}

impl Default for BlockCipherMode {
    fn default() -> Self {
        BlockCipherMode::Ecb
    }
}

pub fn ecb_encrypt(cipher: &dyn BlockCipher, bytes: &mut Vec<u8>, block_size: u32) {
    // Padding should have been used before bytes are given or an error thrown to the user
    assert!(bytes.len() % (block_size as usize) == 0);

    for plaintext in bytes.chunks_mut(block_size as usize) {
        cipher.encrypt_block(plaintext);
    }
}

pub fn ecb_decrypt(cipher: &dyn BlockCipher, bytes: &mut Vec<u8>, block_size: u32) {
    // Padding should have been used before bytes are given or an error thrown to the user
    assert!(bytes.len() % (block_size as usize) == 0);

    for ciphertext in bytes.chunks_mut(block_size as usize) {
        cipher.decrypt_block(ciphertext);
    }
}

// pub fn ctr_mode(cipher: &dyn BlockCipher, bytes: &mut Vec<u8>) {}
// pub fn cbc_mode(cipher: &dyn BlockCipher, bytes: &mut Vec<u8>) {}

// macro_rules! feistel {
//     () => {

//     };
// }

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherPadding {
    None,
    Bit, // add the byte 0x80, then add 0x00 bytes until the block size (in bytes) is reached
         // equivalently add a single 1 bit then append 0 bits until the block size (in bytes) is reached
}

impl Default for BlockCipherPadding {
    fn default() -> Self {
        BlockCipherPadding::Bit
    }
}

pub fn none_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    if bytes.len() % block_size as usize != 0 {
        Err(CipherError::Input(format!(
            "encrypted data must be in chunks of {} bytes",
            block_size
        )))
    } else {
        Ok(())
    }
}

pub fn bit_padding(bytes: &mut Vec<u8>, block_size: u32) {
    bytes.push(0x80);
    while bytes.len() % block_size as usize != 0 {
        bytes.push(0x00)
    }
}

pub fn strip_bit_padding(bytes: &mut Vec<u8>) -> Result<(), CipherError> {
    loop {
        let p = bytes.pop();
        if p == Some(0x00) {
            continue;
        } else if p == Some(0x80) || p == None {
            return Ok(());
        } else {
            return Err(CipherError::Input(format!(
                "bit padding was invalid, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
}
