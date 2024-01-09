use utils::text_functions::hex_to_bytes;

use crate::{Cipher, CipherError};

pub enum ByteFormat {
    Hex,
    Base64,
}

pub struct Rc4 {
    pub arr: [u8; 256],
    pub i: u8,
    pub j: u8,
    pub byte_format: ByteFormat,
}

impl Default for Rc4 {
    fn default() -> Self {
        let mut arr = [0u8; 256];
        for i in 0..255 {
            arr[i as usize] = i;
        }
        Self {
            arr,
            i: 0,
            j: 0,
            byte_format: ByteFormat::Hex,
        }
    }
}

impl Rc4 {
    pub fn ksa(&mut self, key: &[u8]) {
        // Set array to identity permutation
        let mut arr = [0u8; 256];
        for n in 0..255 {
            arr[n as usize] = n;
        }
        // Perform 256 swaps
        let key_length = key.len();
        let mut j: u8 = 0;
        for n in 0..255 {
            j = j.wrapping_add(arr[n]).wrapping_add(key[n % key_length]);
            arr.swap(n, j as usize)
        }
        self.arr = arr;
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

    // pub fn encrypt_utf8(&self, text: &str) -> Result<String, CipherError> {
    //     let mut bytes = text.as_bytes().to_owned();
    //     self.encrypt_bytes_cloned(&mut bytes);
    //     Ok(bytes.iter().map(|byte| format!("{:0x}", byte)).collect())
    // }

    pub fn encrypt_hex(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?;
        self.encrypt_bytes_cloned(&mut bytes);
        Ok(bytes.iter().map(|byte| format!("{:0x}", byte)).collect())
    }
}

impl Cipher for Rc4 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt_hex(text)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt_hex(text)
    }
}
