use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_block_cipher;
use std::{
    cmp::max,
    ops::{BitXor, Shl},
};
use utils::byte_formatting::{overwrite_bytes, ByteFormat};

const P64: u64 = 0xb7e151628aed2a6b;
const Q64: u64 = 0x9e3779b97f4a7c15;

pub fn bytes_to_words(s: &[u8]) -> [u64; 2] {
    [
        u64::from_le_bytes(s[..8].try_into().unwrap()),
        u64::from_le_bytes(s[8..16].try_into().unwrap()),
    ]
}

pub fn words_to_bytes(s: &[u64]) -> [u8; 16] {
    let mut out = [0; 16];
    let (left, right) = out.split_at_mut(8);
    left.copy_from_slice(&s[0].to_le_bytes());
    right.copy_from_slice(&s[1].to_le_bytes());
    out
}

pub struct Rc5_64 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u64>,
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc5_64 {
    fn default() -> Self {
        Self {
            rounds: 20,
            state: Vec::new(),
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Rc5_64 {
    const WORD_SIZE: u32 = 64; // w parameter from specification, word size in bits, half the block size

    pub fn state_size(&self) -> usize {
        2 * (self.rounds + 1)
    }

    pub fn ksa(&mut self, key: &[u8]) {
        assert!(
            key.len() < 256,
            "RC5 key is limited to 255 bytes, which is enough for anybody"
        );

        let u = 8; // bytes in a word
        let b = key.len(); // bytes in the key
        let c = max(b.div_ceil(u), 1); // words in the key
        let t = self.state_size(); // words in the state
        let mut l = vec![0_u64; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i].into())
        }

        let mut s = vec![0; t];
        s[0] = P64;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(Q64)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(t, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b))
                .rotate_left((a.wrapping_add(b) as u32) % Self::WORD_SIZE);
            b = l[j];
            i = (i + 1) % t;
            j = (j + 1) % c;
        }

        self.state = s;
    }
}

impl BlockCipher<16> for Rc5_64 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        block[0] = block[0].wrapping_add(self.state[0]);
        block[1] = block[1].wrapping_add(self.state[1]);

        for i in 1..=self.rounds {
            block[0] = block[0]
                .bitxor(block[1])
                .rotate_left((block[1] as u32) % Self::WORD_SIZE)
                .wrapping_add(self.state[2 * i]);
            block[1] = block[1]
                .bitxor(block[0])
                .rotate_left((block[0] as u32) % Self::WORD_SIZE)
                .wrapping_add(self.state[(2 * i) + 1])
        }
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        for i in (1..=self.rounds).rev() {
            block[1] = block[1]
                .wrapping_sub(self.state[(2 * i) + 1])
                .rotate_right((block[0] as u32) % Self::WORD_SIZE)
                .bitxor(block[0]);
            block[0] = block[0]
                .wrapping_sub(self.state[2 * i])
                .rotate_right((block[1] as u32) % Self::WORD_SIZE)
                .bitxor(block[1]);
        }

        block[0] = block[0].wrapping_sub(self.state[0]);
        block[1] = block[1].wrapping_sub(self.state[1]);
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }
}

impl_block_cipher!(Rc5_64, 16);

#[cfg(test)]
mod rc5_tests {

    use utils::byte_formatting::hex_to_bytes_ltr;

    use crate::Cipher;

    use super::*;
    // Test vectors from
    // https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=fe22353a2b9b557d1130bf9ba9f1f73fe26359cd
    // https://datatracker.ietf.org/doc/html/draft-krovetz-rc6-rc5-vectors-00#section-4
    #[test]
    fn encrypt_test() {
        const PTEXT: &'static str = "000102030405060708090a0b0c0d0e0f";
        const CTEXT: &'static str = "a46772820edbce0235abea32ae7178da";
        const KEY: &'static str = "000102030405060708090a0b0c0d0e0f1011121314151617";
        let mut cipher = Rc5_64::default();
        cipher.rounds = 24;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        const PTEXT: &'static str = "000102030405060708090a0b0c0d0e0f";
        const CTEXT: &'static str = "a46772820edbce0235abea32ae7178da";
        const KEY: &'static str = "000102030405060708090a0b0c0d0e0f1011121314151617";
        let mut cipher = Rc5_64::default();
        cipher.rounds = 24;
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn basic_encrypt_decrypt_test() {
        const PTEXT: &'static str = "00000000000000000000000000000000";
        const KEY: &'static str = "000000000000000000000000000000000000000000000000";
        let mut cipher = Rc5_64::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
