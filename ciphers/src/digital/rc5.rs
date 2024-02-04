use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};
use std::{cmp::max, ops::Shl};

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
// const P64: u64 = 0xb7e151628aed2a6b;
// const Q64: u64 = 0x9e3779b97f4a7c15;

pub struct Rc5 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u32>,
}

impl Default for Rc5 {
    fn default() -> Self {
        Self {
            rounds: 12,
            state: Vec::new(),
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
        }
    }
}

impl Rc5 {
    pub fn state_size(&self) -> usize {
        2 * (self.rounds + 1)
    }

    pub fn ksa_32(&mut self, key: &[u8]) {
        assert!(
            key.len() < 256,
            "RC5 key is limited to 255 bytes, which is enough for anybody"
        );

        let u = 4; // Bytes in a word
        let b = key.len(); // Bytes in the key
        let c = max(b.div_ceil(u), 1); // number of words in the key
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let t = self.state_size();
        let mut s = vec![0; t];
        s[0] = P32;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(t, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
            b = l[j];
            i = (i + 1) % t;
            j = (j + 1) % c;
        }

        self.state = s;
    }

    // pub fn ksa_64(&self, key: &[u8]) {
    //     let b = key.len();
    //     let u = 8; // Bytes in a word
    //     let c = max(b.div_ceil(u), 1);
    //     let mut l = vec![0_u64; c];
    //     for i in (0..b).rev() {
    //         l[i / u] = (l[i / u].rotate_left(8)).wrapping_add(key[i] as u64)
    //     }

    //     let t = 2 * (self.rounds + 1);
    //     let mut s = vec![0; t];
    //     s[0] = P64;
    //     for i in 1..t {
    //         s[i] = s[i - 1].wrapping_add(Q64)
    //     }

    //     let mut i = 0;
    //     let mut j = 0;
    //     let mut a = 0;
    //     let mut b = 0;
    //     for _ in 0..(3 * max(t, c)) {
    //         s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
    //         a = s[i];
    //         l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
    //         b = l[j];
    //         i = (i + 1) % t;
    //         j = (j + 1) % c;
    //     }
    // }

    pub fn encrypt_block_32(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        // No padding rule is given by Rivest
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "decrypted data must be in chunks of 64 bits",
            ));
        }

        let mut out = Vec::with_capacity(bytes.len());

        for block in bytes.chunks_exact(8) {
            let mut x = [0u32; 2];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            x[0] = x[0].wrapping_add(self.state[0]);
            x[1] = x[1].wrapping_add(self.state[1]);

            for i in 1..=self.rounds {
                x[0] = (x[0] ^ x[1])
                    .rotate_left(x[1])
                    .wrapping_add(self.state[2 * i]);
                x[1] = (x[1] ^ x[0])
                    .rotate_left(x[0])
                    .wrapping_add(self.state[(2 * i) + 1])
            }

            out.extend_from_slice(&x[0].to_le_bytes());
            out.extend_from_slice(&x[1].to_le_bytes());
        }
        Ok(out)
    }

    pub fn decrypt_block_32(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() % 8 != 0 {
            return Err(CipherError::input(
                "decrypted data must be in chunks of 64 bits",
            ));
        }

        let mut out = Vec::with_capacity(bytes.len());

        for block in bytes.chunks_exact(8).rev() {
            let mut x = [0u32; 2];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            for i in (1..=self.rounds).rev() {
                x[1] = x[1]
                    .wrapping_sub(self.state[(2 * i) + 1])
                    .rotate_right(x[0])
                    ^ x[0];
                x[0] = x[0].wrapping_sub(self.state[2 * i]).rotate_right(x[1]) ^ x[1];
            }

            x[0] = x[0].wrapping_sub(self.state[0]);
            x[1] = x[1].wrapping_sub(self.state[1]);

            out.extend_from_slice(&x[0].to_le_bytes());
            out.extend_from_slice(&x[1].to_le_bytes());
        }
        Ok(out)
    }

    // pub fn encrypt_block_64(&self, bytes: &[u8]) {}
}

impl Cipher for Rc5 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.encrypt_block_32(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let out = self.decrypt_block_32(&mut bytes)?;
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod rc5_tests {

    use utils::text_functions::hex_to_bytes;

    use super::*;

    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
