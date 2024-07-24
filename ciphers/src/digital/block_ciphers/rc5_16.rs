use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_block_cipher;
use std::{
    cmp::max,
    ops::{BitXor, Shl},
};
use utils::byte_formatting::{overwrite_bytes, ByteFormat};

const P16: u16 = 0xb7e1;
const Q16: u16 = 0x9e37;

pub fn bytes_to_words(s: &[u8]) -> [u16; 2] {
    [
        u16::from_le_bytes(s[..2].try_into().unwrap()),
        u16::from_le_bytes(s[2..4].try_into().unwrap()),
    ]
}

pub fn words_to_bytes(s: &[u16]) -> [u8; 4] {
    let mut out = [0; 4];
    let (left, right) = out.split_at_mut(2);
    left.copy_from_slice(&s[0].to_le_bytes());
    right.copy_from_slice(&s[1].to_le_bytes());
    out
}

pub struct Rc5_16 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u16>,
    pub iv: u32,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc5_16 {
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

impl Rc5_16 {
    const WORD_SIZE: u32 = 16; // w parameter from specification, word size in bits, half the block size

    pub fn state_size(&self) -> usize {
        2 * (self.rounds + 1)
    }

    pub fn ksa(&mut self, key: &[u8]) {
        assert!(
            key.len() < 256,
            "RC5 key is limited to 255 bytes, which is enough for anybody"
        );

        let u = 2; // bytes in a word
        let b = key.len(); // bytes in the key
        let c = max(b.div_ceil(u), 1); // words in the key
        let t = self.state_size(); // words in the state
        let mut l = vec![0_u16; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i].into())
        }

        let mut s = vec![0; t];
        s[0] = P16;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(Q16)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(t, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b))
                .rotate_left(a.wrapping_add(b) as u32 % Self::WORD_SIZE);
            b = l[j];
            i = (i + 1) % t;
            j = (j + 1) % c;
        }

        self.state = s;
    }
}

impl BlockCipher<4> for Rc5_16 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        block[0] = block[0].wrapping_add(self.state[0]);
        block[1] = block[1].wrapping_add(self.state[1]);

        for i in 1..=self.rounds {
            block[0] = block[0]
                .bitxor(block[1])
                .rotate_left(block[1] as u32 % Self::WORD_SIZE)
                .wrapping_add(self.state[2 * i]);
            block[1] = block[1]
                .bitxor(block[0])
                .rotate_left(block[0] as u32 % Self::WORD_SIZE)
                .wrapping_add(self.state[(2 * i) + 1])
        }
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
        for i in (1..=self.rounds).rev() {
            block[1] = block[1]
                .wrapping_sub(self.state[(2 * i) + 1])
                .rotate_right(block[0] as u32 % Self::WORD_SIZE)
                .bitxor(block[0]);
            block[0] = block[0]
                .wrapping_sub(self.state[2 * i])
                .rotate_right(block[1] as u32 % Self::WORD_SIZE)
                .bitxor(block[1]);
        }

        block[0] = block[0].wrapping_sub(self.state[0]);
        block[1] = block[1].wrapping_sub(self.state[1]);
        overwrite_bytes(bytes, &words_to_bytes(&block));
    }
}

impl_block_cipher!(Rc5_16, 4);

#[cfg(test)]
mod rc5_tests {

    use utils::byte_formatting::hex_to_bytes_ltr;

    use crate::Cipher;

    use super::*;
    // Test vectors from
    // https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=fe22353a2b9b557d1130bf9ba9f1f73fe26359cd

    #[test]
    fn basic_encrypt_decrypt_test() {
        const PTEXT: &'static str = "0000000000000000";
        const KEY: &'static str = "00000000000000000000000000000000";
        let mut cipher = Rc5_16::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::None;
        cipher.ksa(&hex_to_bytes_ltr(KEY).unwrap());
        let ctext = cipher.encrypt(PTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), PTEXT);
    }
}
