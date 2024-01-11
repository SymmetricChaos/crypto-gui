use utils::text_functions::hex_to_bytes;

use crate::{Cipher, CipherError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Hex,
    Utf8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputFormat {
    Hex,
    Utf8,
}

pub struct Rc4 {
    pub arr: [u8; 256],
    pub i: u8,
    pub j: u8,
    pub output_format: OutputFormat,
    pub input_format: InputFormat,
}

impl Default for Rc4 {
    fn default() -> Self {
        let mut arr = [0u8; 256];
        for i in 0..256 {
            arr[i] = i as u8;
        }
        Self {
            arr,
            i: 0,
            j: 0,
            output_format: OutputFormat::Hex,
            input_format: InputFormat::Utf8,
        }
    }
}

impl Rc4 {
    pub fn ksa(&mut self, key: &[u8]) {
        // Set array to identity permutation
        for n in 0..256 {
            self.arr[n] = n as u8;
        }
        // Perform 256 swaps
        let mut j: u8 = 0;
        for (i, k) in (0..256).zip(key.iter().cycle()) {
            j = j.wrapping_add(self.arr[i]).wrapping_add(*k);
            self.arr.swap(i, j as usize)
        }
        self.i = 0;
        self.j = 0;
    }

    pub fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.arr[self.i as usize]);
        self.arr.swap(self.i as usize, self.j as usize);
        let t = self.arr[self.i as usize].wrapping_add(self.arr[self.j as usize]);
        self.arr[t as usize]
    }

    pub fn encrypt_bytes_cloned(&self, bytes: &mut [u8]) {
        let mut arr = self.arr;
        let mut i = self.i;
        let mut j = self.j;

        for byte in bytes.iter_mut() {
            i = i.wrapping_add(1);
            j = j.wrapping_add(arr[i as usize]);
            arr.swap(i as usize, j as usize);
            let t = arr[i as usize].wrapping_add(arr[j as usize]);
            *byte = *byte ^ arr[t as usize]
        }
    }

    pub fn encrypt_bytes(&mut self, bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
            *byte = *byte ^ self.next_byte()
        }
    }
}

impl Cipher for Rc4 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = match self.input_format {
            InputFormat::Hex => {
                hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?
            }
            InputFormat::Utf8 => text.bytes().collect(),
        };
        self.encrypt_bytes_cloned(&mut bytes);
        match self.output_format {
            OutputFormat::Hex => Ok(bytes.iter().map(|byte| format!("{:02x}", byte)).collect()),
            OutputFormat::Utf8 => Ok(String::from_utf8_lossy(&bytes).to_string()),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt(text)
    }
}

#[cfg(test)]
mod rc4_tests {

    use super::*;

    const PLAINTEXT: &'static str = "Attack at dawn";
    const CIPHERTEXT: &'static str = "45a01f645fc35b383552544b9bf5";

    #[test]
    fn encrypt_test() {
        let mut cipher = Rc4::default();
        cipher.ksa("Secret".as_bytes());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT)
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Rc4::default();
        cipher.ksa("Secret".as_bytes());
        cipher.input_format = InputFormat::Hex;
        cipher.output_format = OutputFormat::Utf8;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT)
    }
}
