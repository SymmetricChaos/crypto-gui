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

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherPadding {
    None,
    Bit, // add the byte 0x80, then add 0x00 bytes until the block size (in bytes) is reached
    // equivalently add a single 1 bit then append 0 bits until the block size (in bytes) is reached
    Pkcs,
}

impl Default for BlockCipherPadding {
    fn default() -> Self {
        BlockCipherPadding::Bit
    }
}

impl BlockCipherPadding {
    fn add_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BlockCipherPadding::None => none_padding(bytes, block_size),
            BlockCipherPadding::Bit => bit_padding(bytes, block_size),
            BlockCipherPadding::Pkcs => pkcs_padding(bytes, block_size),
        }
    }

    fn strip_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BlockCipherPadding::None => strip_none_padding(bytes, block_size),
            BlockCipherPadding::Bit => strip_bit_padding(bytes),
            BlockCipherPadding::Pkcs => strip_pkcs_padding(bytes),
        }
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

pub fn strip_none_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    if bytes.len() % block_size as usize != 0 {
        Err(CipherError::Input(format!(
            "encrypted data must be in chunks of {} bytes",
            block_size
        )))
    } else {
        Ok(())
    }
}

pub fn bit_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    bytes.push(0x80);
    while bytes.len() % block_size as usize != 0 {
        bytes.push(0x00)
    }
    Ok(())
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

pub fn pkcs_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    let n_padding = (block_size as usize - (bytes.len() % block_size as usize))
        .try_into()
        .unwrap();
    while bytes.len() % block_size as usize != 0 {
        bytes.push(n_padding)
    }
    Ok(())
}

pub fn strip_pkcs_padding(bytes: &mut Vec<u8>) -> Result<(), CipherError> {
    let n_padding = *bytes
        .iter()
        .last()
        .ok_or(CipherError::input("ciphertext has zero length"))?;
    for _ in 0..n_padding {
        let p = bytes.pop();
        if p == Some(n_padding) {
            continue;
        } else if p == None {
            return Err(CipherError::input("invalid padding, ran out of ciphertext"));
        } else {
            return Err(CipherError::Input(format!(
                "PKCS padding was invalid, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod idea_tests {

    use super::*;

    #[test]
    fn test_bit_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        bit_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x80, 0x00, 0x00, 0x00], bytes);
        strip_bit_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_pkcs_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        pkcs_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x04, 0x04, 0x04, 0x04], bytes);
        strip_pkcs_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }
}
