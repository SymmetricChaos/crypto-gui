use utils::byte_formatting::{overwrite_bytes, ByteFormat};

use crate::{impl_block_cipher, Cipher, CipherError};
use std::{
    cmp::max,
    ops::{BitXor, Shl},
};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
// const P64: u64 = 0xb7e151628aed2a6b;
// const Q64: u64 = 0x9e3779b97f4a7c15;

pub fn bytes_to_words(s: &[u8]) -> [u32; 2] {
    [
        u32::from_le_bytes(s[..4].try_into().unwrap()),
        u32::from_le_bytes(s[4..8].try_into().unwrap()),
    ]
}

pub fn words_to_bytes(s: &[u32]) -> [u8; 8] {
    let mut out = [0; 8];
    let (left, right) = out.split_at_mut(4);
    left.copy_from_slice(&s[0].to_le_bytes());
    right.copy_from_slice(&s[1].to_le_bytes());
    out
}

const BLOCKSIZE: u32 = 8;

pub struct Rc5 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u32>,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc5 {
    fn default() -> Self {
        Self {
            rounds: 12,
            state: Vec::new(),
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
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

        let u = 4; // bytes in a word
        let b = key.len(); // bytes in the key
        let c = max(b.div_ceil(u), 1); // words in the key
        let t = self.state_size(); // words in the state
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i] as u32)
        }

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
}

impl BlockCipher<8> for Rc5 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        block[0] = block[0].wrapping_add(self.state[0]);
        block[1] = block[1].wrapping_add(self.state[1]);

        for i in 1..=self.rounds {
            block[0] = block[0]
                .bitxor(block[1])
                .rotate_left(block[1])
                .wrapping_add(self.state[2 * i]);
            block[1] = block[1]
                .bitxor(block[0])
                .rotate_left(block[0])
                .wrapping_add(self.state[(2 * i) + 1])
        }
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        for i in (1..=self.rounds).rev() {
            block[1] = block[1]
                .wrapping_sub(self.state[(2 * i) + 1])
                .rotate_right(block[0])
                .bitxor(block[0]);
            block[0] = block[0]
                .wrapping_sub(self.state[2 * i])
                .rotate_right(block[1])
                .bitxor(block[1]);
        }

        block[0] = block[0].wrapping_sub(self.state[0]);
        block[1] = block[1].wrapping_sub(self.state[1]);
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl_block_cipher!(Rc5);

#[cfg(test)]
mod rc5_tests {

    use utils::byte_formatting::hex_to_bytes_ltr;

    use super::*;
    // Test vectors from
    // https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=fe22353a2b9b557d1130bf9ba9f1f73fe26359cd
    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const CTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const CTEXT: &'static str = "f7c013ac5b2b8952";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test_2() {
        const PTEXT: &'static str = "21a5dbee154b8f6d";
        const KEY: &'static str = "915f4619be41b2516355a50110a9ce91";
        let mut cipher = Rc5::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa_32(&hex_to_bytes_ltr(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
