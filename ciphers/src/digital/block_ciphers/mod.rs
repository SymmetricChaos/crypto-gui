pub mod aes;
pub mod blowfish;
pub mod blowfish_arrays;
pub mod des;
pub mod feal;
pub mod idea;
pub mod rc5;
pub mod tea;
pub mod xtea;

use crypto_bigint::generic_array::GenericArray;
use num::{BigUint, One};
use utils::math_functions::incr_array_ctr;

use crate::CipherError;

pub trait BlockCipher<const N: usize> {
    fn encrypt_block(&self, bytes: &mut [u8]);
    fn decrypt_block(&self, bytes: &mut [u8]);
    fn set_mode(&mut self, mode: BlockCipherMode);
    fn set_padding(&mut self, padding: BlockCipherPadding);
    fn encrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for plaintext in bytes.chunks_mut(N) {
            self.encrypt_block(plaintext);
        }
    }
    fn decrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for plaintext in bytes.chunks_mut(N) {
            self.decrypt_block(plaintext);
        }
    }

    fn encrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        let mut ctr = ctr;

        for plaintext in bytes.chunks_mut(N) {
            // Encrypt the counter to create a mask
            let mut mask = ctr;
            self.encrypt_block(&mut mask);
            // XOR the mask into the plaintext at the source, creating ciphertext
            for (key_byte, ptext) in mask.iter().zip(plaintext.iter_mut()) {
                *ptext ^= key_byte
            }
            incr_array_ctr(&mut ctr);
        }
    }

    fn decrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        let mut ctr = ctr;

        for plaintext in bytes.chunks_mut(N) {
            // Encrypt the counter to create a mask
            let mut mask = ctr;
            self.encrypt_block(&mut mask);
            // XOR the mask into the plaintext at the source, creating ciphertext
            for (key_byte, ptext) in mask.iter().zip(plaintext.iter_mut()) {
                *ptext ^= key_byte
            }
            incr_array_ctr(&mut ctr);
        }
    }

    fn encrypt_cbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for source in bytes.chunks_mut(N) {
            // XOR the plaintext into the previous ciphertext (or the IV), creating a mixed array
            for (c, b) in chain.iter_mut().zip(source.iter()) {
                *c ^= b;
            }

            // Encrypt the mixed value, producing ciphertext
            self.encrypt_block(&mut chain);

            // Overwrite plaintext at source with the ciphertext
            for (ctext, source) in chain
                .iter()
                .map(|w| w.to_be_bytes())
                .flatten()
                .zip(source.iter_mut())
            {
                *source = ctext
            }
        }
    }

    fn decrypt_cbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for source in bytes.chunks_mut(N) {
            // Decrypt the ciphertext at the source to get the plaintext XORed with the previous chain value
            let mut mixed = source.to_vec();
            self.decrypt_block(&mut mixed);

            // XOR the current chain value into the mixed text
            for (c, b) in mixed.iter_mut().zip(chain.iter()) {
                *c ^= b;
            }

            // Store the ciphertext as the next chain value before it gets overwritten
            chain = source.try_into().unwrap();

            // The overwrite ciphertext at source with the plaintext
            for (ptext, source) in mixed.into_iter().zip(source.iter_mut()) {
                *source = ptext
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
