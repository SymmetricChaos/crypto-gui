use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_cipher_for_block_cipher;
use std::{cmp::max, ops::Shl};
use utils::byte_formatting::ByteFormat;

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
const BLOCKSIZE: usize = 16;
const WORDSIZE: usize = 4;
const ROUNDS: usize = 20;
const STATE_SIZE: usize = (2 * ROUNDS) + 4;

pub fn bytes_to_words(s: &[u8]) -> [u32; 2] {
    [
        u32::from_le_bytes(s[..4].try_into().unwrap()),
        u32::from_le_bytes(s[4..8].try_into().unwrap()),
    ]
}

pub fn words_to_bytes(s: &[u32]) -> [u8; BLOCKSIZE] {
    let mut out = [0; BLOCKSIZE];
    let (left, right) = out.split_at_mut(WORDSIZE);
    left.copy_from_slice(&s[0].to_le_bytes());
    right.copy_from_slice(&s[1].to_le_bytes());
    out
}

struct Rc6 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub state: [u32; STATE_SIZE],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc6 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            state: [0; STATE_SIZE],
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Rc6 {
    pub fn ksa_128(&mut self, key: &[u8]) {
        let b = key.len(); // Bytes in the key
        let c = max(b.div_ceil(WORDSIZE), 1); // number of words in the key
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / WORDSIZE] = (l[i / WORDSIZE].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let mut s = [0; STATE_SIZE];
        s[0] = P32;
        for i in 1..STATE_SIZE {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(STATE_SIZE, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
            b = l[j];
            i = (i + 1) % STATE_SIZE;
            j = (j + 1) % c;
        }

        self.state = s;
    }
}

impl BlockCipher<BLOCKSIZE> for Rc6 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes_to_words(bytes);
    }

    crate::block_cipher_getters!();
}

impl_cipher_for_block_cipher!(Rc6, 16);
