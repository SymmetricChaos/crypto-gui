use super::{InputFormat, OutputFormat};
use crate::{Cipher, CipherError};
use std::cmp::max;
use utils::text_functions::hex_to_bytes;

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
// const P64: u64 = 0xb7e151628aed2a6b;
// const Q64: u64 = 0x9e3779b97f4a7c15;

pub struct Rc5 {
    pub output_format: OutputFormat,
    pub input_format: InputFormat,
    pub rounds: usize,
    pub state: Vec<u32>,
}

impl Default for Rc5 {
    fn default() -> Self {
        Self {
            rounds: 12,
            state: Vec::new(),
            output_format: OutputFormat::Hex,
            input_format: InputFormat::Hex,
        }
    }
}

impl Rc5 {
    pub fn ksa_32(&mut self, key: &[u8]) {
        assert!(
            key.len() < 256,
            "RC5 key is limited to 255 bytes, which is enough for anybody"
        );

        let b = key.len();
        let u = 4; // Bytes in a word
        let c = max(b.div_ceil(u), 1);
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].rotate_left(8)).wrapping_add(key[i] as u32)
        }

        let t = 2 * (self.rounds + 1);
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

    pub fn encrypt_block_32(&self, bytes: &[u8]) -> Vec<u8> {
        // Pad with zeroes. No padding rule is given by Rivest
        let mut input = bytes.to_vec();
        while input.len() % 8 != 0 {
            input.push(0)
        }

        let mut out = Vec::with_capacity(input.len());

        for block in input.chunks_exact(8) {
            let mut x = [0u32; 2];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_be_bytes(chunk.try_into().unwrap());
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
        out
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
            for b in x[0].to_le_bytes() {
                out.push(b)
            }
            for b in x[1].to_le_bytes() {
                out.push(b)
            }
        }
        Ok(out)
    }

    // pub fn encrypt_block_64(&self, bytes: &[u8]) {}
}

impl Cipher for Rc5 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = match self.input_format {
            InputFormat::Hex => {
                hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?
            }
            InputFormat::Utf8 => text.bytes().collect(),
        };
        let b = self.encrypt_block_32(&mut bytes);
        match self.output_format {
            OutputFormat::Hex => Ok(b.iter().map(|byte| format!("{:02x}", byte)).collect()),
            OutputFormat::Utf8 => Ok(String::from_utf8_lossy(&b).to_string()),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = match self.input_format {
            InputFormat::Hex => {
                hex_to_bytes(text).map_err(|_| CipherError::input("not valid hexcode"))?
            }
            InputFormat::Utf8 => text.bytes().collect(),
        };
        let b = self.decrypt_block_32(&mut bytes)?;
        match self.output_format {
            OutputFormat::Hex => Ok(b.iter().map(|byte| format!("{:02x}", byte)).collect()),
            OutputFormat::Utf8 => Ok(String::from_utf8_lossy(&b).to_string()),
        }
    }
}

#[cfg(test)]
mod rc5_tests {

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
    fn encrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
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
    fn decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
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
    fn encrypt_decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.ksa_32(&hex_to_bytes(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}