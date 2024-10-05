use std::ops::{Shl, Shr};

use utils::byte_formatting::{fill_u32s_le, ByteFormat};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

const ROUNDS: usize = 32;
const FRAC: u32 = 0x9e3779b9;

// Serpents eight 4-bit sboxes and their inverses
const SBOX: [[u8; 16]; 8] = [
    [3, 8, 15, 1, 10, 6, 5, 11, 14, 13, 4, 2, 7, 0, 9, 12],
    [15, 12, 2, 7, 9, 0, 5, 10, 1, 11, 14, 8, 6, 13, 3, 4],
    [8, 6, 7, 9, 3, 12, 10, 15, 13, 1, 14, 4, 0, 11, 5, 2],
    [0, 15, 11, 8, 12, 9, 6, 3, 13, 1, 2, 4, 10, 7, 5, 14],
    [1, 15, 8, 3, 12, 0, 11, 6, 2, 5, 4, 10, 9, 14, 7, 13],
    [15, 5, 2, 11, 4, 10, 9, 12, 0, 3, 14, 8, 13, 6, 7, 1],
    [7, 2, 12, 5, 8, 4, 6, 11, 14, 9, 1, 15, 13, 3, 10, 0],
    [1, 13, 15, 0, 14, 8, 2, 11, 7, 4, 12, 10, 9, 3, 5, 6],
];

const SBOX_INV: [[u8; 16]; 8] = [
    [13, 3, 11, 0, 10, 6, 5, 12, 1, 14, 4, 7, 15, 9, 8, 2],
    [5, 8, 2, 14, 15, 6, 12, 3, 11, 4, 7, 9, 1, 13, 10, 0],
    [12, 9, 15, 4, 11, 14, 1, 2, 0, 3, 6, 13, 5, 8, 10, 7],
    [0, 9, 10, 7, 11, 14, 6, 13, 3, 5, 12, 2, 4, 8, 15, 1],
    [5, 0, 8, 3, 10, 9, 7, 14, 2, 12, 11, 6, 4, 15, 13, 1],
    [8, 15, 2, 9, 4, 1, 13, 14, 11, 6, 5, 3, 7, 12, 10, 0],
    [15, 10, 1, 13, 5, 3, 6, 0, 4, 9, 14, 7, 2, 12, 8, 11],
    [3, 0, 6, 13, 9, 14, 15, 8, 5, 12, 11, 7, 10, 1, 4, 2],
];

// Apply a specific SBOX, u8 should only use the lower 4 bits
pub fn sbox(i: usize, nibble: u8) -> u8 {
    SBOX[i][nibble as usize]
}

// Apply a specific SBOX_INV, u8 should only use the lower 4 bits
pub fn sbox_inv(i: usize, nibble: u8) -> u8 {
    SBOX_INV[i][nibble as usize]
}

// Select one bit from a u32
fn get_bit(x: u32, i: usize) -> u8 {
    (x >> i) as u8 & 0x01
}

// Apply a sbox across the bits of four 32-bit words
pub fn sbox_bitslice(idx: usize, words: [u32; 4]) -> [u32; 4] {
    let mut out: [u32; 4] = [0; 4];
    for i in 0..32 {
        // Take bits across the words
        let slice = get_bit(words[0], i)
            | get_bit(words[1], i) << 1
            | get_bit(words[2], i) << 2
            | get_bit(words[3], i) << 4;

        // Apply the sbox to the bits
        let s = sbox(idx, slice);

        // Push the transformed bits into the output
        for pos in 0..4 {
            out[pos] |= u32::from(get_bit(s as u32, pos)) << i;
        }
    }
    out
}

pub fn sbox_bitslice_inv(idx: usize, words: [u32; 4]) -> [u32; 4] {
    let mut out: [u32; 4] = [0; 4];
    for i in 0..32 {
        // Take bits across the words
        let slice = get_bit(words[0], i)
            | get_bit(words[1], i) << 1
            | get_bit(words[2], i) << 2
            | get_bit(words[3], i) << 4;

        // Apply the sbox to the bits
        let s = sbox_inv(idx, slice);

        // Push the transformed bits into the output
        for pos in 0..4 {
            out[pos] |= u32::from(get_bit(s as u32, pos)) << i;
        }
    }
    out
}

// Serpent's Linear Transformation and its inverse
pub fn lt(x: &mut [u32; 4]) {
    x[0] = x[0].rotate_left(13);
    x[2] = x[2].rotate_left(3);
    x[1] = x[1] ^ x[0] ^ x[2];
    x[3] = x[3] ^ x[3] ^ x[0].shl(3);
    x[1] = x[1].rotate_left(1);
    x[3] = x[3].rotate_left(7);
    x[0] = x[0] ^ x[1] ^ x[3];
    x[2] = x[2] ^ x[3] ^ x[1].shl(7);
    x[0] = x[0].rotate_left(5);
    x[2] = x[2].rotate_left(22);
}

pub fn lt_inv(x: &mut [u32; 4]) {
    x[2] = x[2].rotate_right(22);
    x[0] = x[0].rotate_right(5);
    x[2] = x[2] ^ x[3] ^ x[1].shr(7);
    x[0] = x[0] ^ x[1] ^ x[3];
    x[3] = x[3].rotate_right(7);
    x[1] = x[1].rotate_right(1);
    x[3] = x[3] ^ x[3] ^ x[0].shr(3);
    x[1] = x[1] ^ x[0] ^ x[2];
    x[2] = x[2].rotate_right(3);
    x[0] = x[0].rotate_right(13);
}

// Expand a key to 256 bits.
// Serpent accepts keys of any bit length from 0 to 256 bits.
// I will not bother keys not given in bytes are rare.
pub fn expand_key(bytes: &[u8]) -> [u8; 32] {
    let mut ex = [0; 32];
    ex[..bytes.len()].copy_from_slice(bytes);
    if bytes.len() < 32 {
        ex[bytes.len()] = 0x80
    }
    ex
}

// Generate the prekeys that are used to generate the round keys
pub fn pre_keys(bytes: &[u8]) -> [u32; 132] {
    // Created the expanded list of words
    let mut ex_words = [0u32; 8];
    fill_u32s_le(&mut ex_words, &expand_key(bytes));
    // Copy that list into the start of the pre_key
    let mut pre_key: [u32; 140] = [0; 140];
    // Fill the entire pre_key
    pre_key[..8].copy_from_slice(&ex_words);
    for i in 0..132 {
        pre_key[i + 8] =
            (pre_key[i] ^ pre_key[i + 3] ^ pre_key[i + 5] ^ pre_key[i + 7] ^ FRAC ^ i as u32)
                .rotate_left(11);
    }
    // Discard the first eight words
    pre_key[8..].try_into().unwrap()
}

pub fn round_keys(pre_keys: [u32; 132]) -> [[u32; 4]; ROUNDS + 1] {
    let mut out = [[0; 4]; ROUNDS + 1];
    for (i, chunk) in pre_keys.chunks_exact(4).enumerate() {
        let words: [u32; 4] = chunk.try_into().unwrap();
        out[i] = sbox_bitslice((ROUNDS + 3 - i) % ROUNDS, words);
    }
    out
}

pub struct Serpent128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub round_keys: [[u32; 4]; ROUNDS + 1],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Serpent128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            round_keys: [[0; 4]; ROUNDS + 1],
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

crate::block_cipher_builders! {Serpent128, u128}

impl Serpent128 {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        self.round_keys = round_keys(pre_keys(&bytes));
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for Serpent128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // let mut block = [0; 4];
        // fill_u32s_le(&mut block, bytes);

        // u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // let mut block = [0; 4];
        // fill_u32s_le(&mut block, bytes);

        // u32s_to_bytes_le(bytes, &block);
    }
}
