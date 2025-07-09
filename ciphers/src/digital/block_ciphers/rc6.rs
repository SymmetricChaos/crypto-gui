use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crate::impl_cipher_for_block_cipher;
use std::{cmp::max, ops::Shl};
use utils::byte_formatting::ByteFormat;

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
const BLOCKSIZE: usize = 16;
const WORDSIZE: usize = 4;
const BLOCKWORDS: usize = BLOCKSIZE / WORDSIZE;
const ROUNDS: usize = 20;
const ROUND_KEYS: usize = (2 * ROUNDS) + 4;

pub fn bytes_to_words(s: &[u8]) -> [u32; BLOCKWORDS] {
    [
        u32::from_le_bytes(s[..4].try_into().unwrap()),
        u32::from_le_bytes(s[4..8].try_into().unwrap()),
        u32::from_le_bytes(s[8..12].try_into().unwrap()),
        u32::from_le_bytes(s[12..16].try_into().unwrap()),
    ]
}

pub fn words_to_bytes(s: &[u32]) -> [u8; BLOCKSIZE] {
    let mut out = [0; BLOCKSIZE];
    out[..4].copy_from_slice(&s[0].to_le_bytes());
    out[4..8].copy_from_slice(&s[1].to_le_bytes());
    out[8..12].copy_from_slice(&s[2].to_le_bytes());
    out[12..16].copy_from_slice(&s[3].to_le_bytes());
    out
}

struct Rc6 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub round_keys: [u32; ROUND_KEYS],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc6 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            round_keys: [0; ROUND_KEYS],
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Rc6 {
    pub fn with_key_128(mut self, bytes: &[u8]) -> Self {
        self.ksa_128(bytes);
        self
    }

    pub fn ksa_128(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 16);
        let c = max(key.len().div_ceil(WORDSIZE), 1); // number of words in the key
        let mut l = vec![0_u32; c];
        for i in (0..key.len()).rev() {
            l[i / WORDSIZE] = (l[i / WORDSIZE].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let mut s = [0; ROUND_KEYS];
        s[0] = P32;
        for i in 1..ROUND_KEYS {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(ROUND_KEYS, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
            b = l[j];
            i = (i + 1) % ROUND_KEYS;
            j = (j + 1) % c;
        }

        self.round_keys = s;
    }

    pub fn with_key_192(mut self, bytes: &[u8]) -> Self {
        self.ksa_192(bytes);
        self
    }

    pub fn ksa_192(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 24);
        todo!()
    }

    pub fn with_key_256(mut self, bytes: &[u8]) -> Self {
        self.ksa_256(bytes);
        self
    }

    pub fn ksa_256(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 32);
        todo!()
    }
}

impl BlockCipher<BLOCKSIZE> for Rc6 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // B = B + S[0]
        // D = D + S[1]
        // for i = 1 to r do
        // {
        // 	t = (B * (2B + 1)) <<< lg w
        // 	u = (D * (2D + 1)) <<< lg w
        // 	A = ((A ^ t) <<< u) + S[2i]
        // 	C = ((C ^ u) <<< t) + S[2i + 1]
        // 	(A, B, C, D)  =  (B, C, D, A)
        // }
        // A = A + S[2r + 2]
        // C = C + S[2r + 3]
        let mut block = bytes_to_words(bytes);

        block[1] = block[1].wrapping_add(self.round_keys[0]);
        block[3] = block[3].wrapping_add(self.round_keys[1]);

        for i in 1..ROUNDS {
            let t = (block[1].wrapping_mul(2_u32.wrapping_mul(block[1].wrapping_add(1))))
                .rotate_left(5);
            let u = (block[3].wrapping_mul(2_u32.wrapping_mul(block[3].wrapping_add(1))))
                .rotate_left(5);
            block[0] = (block[0] ^ t)
                .rotate_left(u % 32)
                .wrapping_add(self.round_keys[2 * i]);
            block[2] = (block[2] ^ u)
                .rotate_left(t % 32)
                .wrapping_add(self.round_keys[2 * i + 1]);
            (block[0], block[1], block[2], block[3]) = (block[1], block[2], block[3], block[0]);
        }

        block[0] = block[0].wrapping_add(self.round_keys[2 * ROUNDS + 2]);
        block[2] = block[2].wrapping_add(self.round_keys[2 * ROUNDS + 3]);

        utils::byte_formatting::overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // C = C - S[2r + 3]
        // A = A - S[2r + 2]
        // for i = r downto 1 do
        // {
        // 	(A, B, C, D) = (D, A, B, C)
        // 	u = (D * (2D + 1)) <<< lg w
        // 	t = (B * (2B + 1)) <<< lg w
        // 	C = ((C - S[2i + 1]) >>> t) ^ u
        // 	A = ((A - S[2i]) >>> u) ^ t
        // }
        // D = D - S[1]
        // B = B - S[0]
        let mut block = bytes_to_words(bytes);

        block[2] = block[2].wrapping_sub(self.round_keys[2 * ROUNDS + 3]);
        block[0] = block[0].wrapping_sub(self.round_keys[2 * ROUNDS + 2]);

        for i in (1..ROUNDS).rev() {
            (block[0], block[1], block[2], block[3]) = (block[3], block[0], block[1], block[2]);

            let u = (block[3].wrapping_mul(2_u32.wrapping_mul(block[3].wrapping_add(1))))
                .rotate_left(5);
            let t = (block[1].wrapping_mul(2_u32.wrapping_mul(block[1].wrapping_add(1))))
                .rotate_left(5);
            block[2] = (block[2]
                .wrapping_sub(self.round_keys[2 * i + 1])
                .rotate_right(t % 32))
                ^ u;
            block[0] = (block[0]
                .wrapping_sub(self.round_keys[2 * i])
                .rotate_right(u % 32))
                ^ t;
        }

        block[3] = block[3].wrapping_sub(self.round_keys[1]);
        block[1] = block[1].wrapping_sub(self.round_keys[0]);

        utils::byte_formatting::overwrite_bytes(bytes, &words_to_bytes(&block));
    }

    crate::block_cipher_getters!();
}

impl_cipher_for_block_cipher!(Rc6, 16);

#[cfg(test)]
mod basic_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let mut cipher = Rc6::default();
        let arr: [u32; 44] = std::array::from_fn(|n| (n + 1) as u32);
        cipher.round_keys = arr;
        let mut msg: [u8; 16] = std::array::from_fn(|n| (n + 1) as u8);
        let orig = msg.clone();
        cipher.encrypt_block(&mut msg);
        cipher.decrypt_block(&mut msg);
        assert_eq!(msg, orig)
    }
}

crate::test_block_cipher!(
    test_1, Rc6::default().with_key_128(&[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x1A, 0xD5, 0x78, 0xA0, 0x2A, 0x08, 0x16, 0x28, 0x50, 0xA1, 0x5A, 0x15, 0x52, 0xA1, 0x7A, 0xD4];

    test_2, Rc6::default().with_key_128(&[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x91, 0x2E, 0x9C, 0xF1, 0x47, 0x30, 0x35, 0xA8, 0x44, 0x3A, 0x82, 0x49, 0x5C, 0x07, 0x30, 0xD3];
);
