use crate::{Cipher, CipherError};

use super::{InputFormat, OutputFormat};

pub struct Des {
    pub output_format: OutputFormat,
    pub input_format: InputFormat,
    pub state: Vec<u32>,
}

impl Default for Des {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Hex,
            input_format: InputFormat::Hex,
            state: Vec::new(),
        }
    }
}

fn delta_swap(a: u64, delta: u64, mask: u64) -> u64 {
    let b = (a ^ (a >> delta)) & mask;
    a ^ b ^ (b << delta)
}

impl Des {
    // Permuted choice
    pub fn pc1(mut key: u64) -> u64 {
        key = delta_swap(key, 2, 0x3333000033330000);
        key = delta_swap(key, 4, 0x0f0f0f0f00000000);
        key = delta_swap(key, 8, 0x009a000a00a200a8);
        key = delta_swap(key, 16, 0x00006c6c0000cccc);
        key = delta_swap(key, 1, 0x1045500500550550);
        key = delta_swap(key, 32, 0x00000000f0f0f5fa);
        key = delta_swap(key, 8, 0x00550055006a00aa);
        key = delta_swap(key, 2, 0x0000333330000300);
        key & 0xffffffffffffff00
    }

    pub fn initial_permutation() {}

    pub fn initial_permutation_inv() {}

    pub fn ksa(&mut self, key: u64) {
        todo!()
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "input length must be a multiple of 64 bits",
            ));
        };

        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "input length must be a multiple of 64 bits",
            ));
        };

        let mut out = Vec::with_capacity(bytes.len());

        Ok(out)
    }
}

impl Cipher for Des {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

#[cfg(test)]
mod des_tests {

    use super::*;
}
