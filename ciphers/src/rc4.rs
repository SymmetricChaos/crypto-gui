use crate::{Cipher, CipherError};
use bimap::BiMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref IS_HEX_BYTES: Regex = Regex::new(r"^([0-9a-f][0-9a-f])*$").unwrap();
    pub static ref HEX: BiMap<String, u8> = (0..255).map(|n| (format!("{:02x}", n), n)).collect();
}

// A string containing hex characters converted into bytes
// "DEADBEEF" -> [222, 173, 190, 239]
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, CipherError> {
    let mut text: String = hex.split_whitespace().collect();
    text.make_ascii_lowercase();
    if !IS_HEX_BYTES.is_match(&text) {
        return Err(CipherError::input("not valid hex bytes"));
    } else {
        let mut out = Vec::new();
        for i in 0..(text.len() / 2) {
            let lo = i * 2;
            out.push(*HEX.get_by_left(&text[lo..lo + 2]).unwrap())
        }
        Ok(out)
    }
}

pub enum ByteOutputFormat {
    Hex,
    Base64,
}

pub struct Rc4 {
    pub arr: [u8; 256],
    pub i: u8,
    pub j: u8,
    pub output_format: ByteOutputFormat,
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
            output_format: ByteOutputFormat::Hex,
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

    pub fn bytes_then_reset(&mut self, n: usize) -> Vec<u8> {
        let arr = self.arr;
        let i = self.i;
        let j = self.j;

        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.next_byte());
        }

        self.arr = arr;
        self.i = i;
        self.j = j;

        out
    }

    pub fn encrypt_bytes(&mut self, bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
            *byte = *byte ^ self.next_byte()
        }
    }

    pub fn encrypt_utf8(&self, text: &str) {
        todo!()
    }

    pub fn encrypt_hex(&self, text: &str) {
        todo!()
    }
}

impl Cipher for Rc4 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let bytes = text.as_bytes();
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}
