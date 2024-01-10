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
        for i in 0..256 {
            arr[i] = i as u8;
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

    // pub fn encrypt_utf8(&self, text: &str) -> Result<String, CipherError> {
    //     let mut bytes = text.as_bytes().to_owned();
    //     self.encrypt_bytes_cloned(&mut bytes);
    //     Ok(bytes.iter().map(|byte| format!("{:0x}", byte)).collect())
    // }

    pub fn encrypt_hex(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?;
        self.encrypt_bytes_cloned(&mut bytes);
        Ok(bytes.iter().map(|byte| format!("{:02x}", byte)).collect())
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

#[cfg(test)]
mod rc4_tests {

    use utils::text_functions::bytes_to_hex;

    use super::*;

    const PLAINTEXT: &'static str = "Attack at dawn";
    const CIPHERTEXT: &'static str = "45a01f645fc35b383552544b9bf5";

    #[test]
    fn encrypt_test() {
        let mut cipher = Rc4::default();
        cipher.ksa("Secret".as_bytes());
        assert_eq!(
            cipher.encrypt(&bytes_to_hex(PLAINTEXT.as_bytes())).unwrap(),
            CIPHERTEXT
        )
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Rc4::default();
        cipher.ksa("Secret".as_bytes());
        let hex = cipher.decrypt(CIPHERTEXT).unwrap();
        let ptext = String::from_utf8(hex_to_bytes(&hex).unwrap()).unwrap();
        assert_eq!(ptext, PLAINTEXT)
    }
}
