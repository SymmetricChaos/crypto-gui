use std::fmt::Display;
use utils::math_functions::incr_array_ctr;

use crate::CipherError;

pub trait BlockCipher<const N: usize> {
    fn encrypt_block(&self, bytes: &mut [u8]);
    fn decrypt_block(&self, bytes: &mut [u8]);
    fn set_mode(&mut self, mode: BCMode);
    fn set_padding(&mut self, padding: BCPadding);
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
pub enum BCMode {
    Cbc,
    Ctr,
    Ecb,
}

impl BCMode {
    /// Is a padding rule needed?
    pub fn padded(&self) -> bool {
        match self {
            BCMode::Ecb => true,
            BCMode::Ctr => false,
            BCMode::Cbc => true,
        }
    }

    pub fn variants() -> [Self; 3] {
        [Self::Cbc, Self::Ctr, Self::Ecb]
    }
}

impl Default for BCMode {
    fn default() -> Self {
        BCMode::Ecb
    }
}

impl Display for BCMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BCMode::Cbc => write!(f, "CBC"),
            BCMode::Ctr => write!(f, "CTR"),
            BCMode::Ecb => write!(f, "ECB"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BCPadding {
    None,
    Bit, // add the byte 0x80, then add 0x00 bytes until the block size (in bytes) is reached
    // equivalently add a single 1 bit then append 0 bits until the block size (in bytes) is reached
    Pkcs,
    Ansi923,
}

impl Default for BCPadding {
    fn default() -> Self {
        BCPadding::Bit
    }
}

impl BCPadding {
    pub fn add_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BCPadding::None => none_padding(bytes, block_size),
            BCPadding::Bit => bit_padding(bytes, block_size),
            BCPadding::Pkcs => pkcs_padding(bytes, block_size),
            BCPadding::Ansi923 => ansi923_padding(bytes, block_size),
        }
    }

    pub fn strip_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BCPadding::None => strip_none_padding(bytes, block_size),
            BCPadding::Bit => strip_bit_padding(bytes),
            BCPadding::Pkcs => strip_pkcs_padding(bytes),
            BCPadding::Ansi923 => strip_ansi923_padding(bytes),
        }
    }

    pub fn variants() -> [Self; 4] {
        [Self::None, Self::Bit, Self::Pkcs, Self::Ansi923]
    }
}

impl Display for BCPadding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Bit => write!(f, "Bit"),
            Self::Pkcs => write!(f, "PKCS"),
            Self::Ansi923 => write!(f, "ANSI X9.23"),
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
                "invalid bit padding, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
}

pub fn pkcs_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    let n_padding = (block_size as usize - (bytes.len() % block_size as usize))
        .try_into()
        .unwrap();
    for _ in 0..n_padding {
        bytes.push(n_padding)
    }
    Ok(())
}

pub fn strip_pkcs_padding(bytes: &mut Vec<u8>) -> Result<(), CipherError> {
    let n_padding = *bytes.iter().last().ok_or(CipherError::input(
        "PKCS padded ciphertext cannot have zero length",
    ))?;
    for _ in 0..n_padding {
        let p = bytes.pop();
        if p == Some(n_padding) {
            continue;
        } else if p == None {
            return Err(CipherError::input(
                "invalid PKCS padding, ran out of ciphertext",
            ));
        } else {
            return Err(CipherError::Input(format!(
                "invalid PKCS padding, found byte {:02x} for ",
                p.unwrap()
            )));
        }
    }
    Ok(())
}

pub fn ansi923_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
    let n_padding = (block_size as usize - (bytes.len() % block_size as usize))
        .try_into()
        .unwrap();
    for _ in 0..(n_padding - 1) {
        bytes.push(0)
    }
    bytes.push(n_padding);
    Ok(())
}

pub fn strip_ansi923_padding(bytes: &mut Vec<u8>) -> Result<(), CipherError> {
    let n_padding = bytes.pop().ok_or(CipherError::input(
        "ANSI X9.23 padded ciphertext cannot have zero length",
    ))?;

    for _ in 0..(n_padding - 1) {
        let p = bytes.pop();
        if p == Some(0) {
            continue;
        } else if p == None {
            return Err(CipherError::input(
                "invalid ANSI X9.23 padding, ran out of ciphertext",
            ));
        } else {
            return Err(CipherError::Input(format!(
                "invalid ANSI X9.23 padding, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod padding_tests {

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
        strip_ansi923_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_ansi_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        ansi923_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x00, 0x00, 0x00, 0x04], bytes);
        strip_ansi923_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }
}
