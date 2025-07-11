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
const NUM_ROUND_KEYS: usize = (2 * ROUNDS) + 4;

fn bytes_to_words(s: &[u8]) -> [u32; BLOCKWORDS] {
    [
        u32::from_le_bytes(s[..4].try_into().unwrap()),
        u32::from_le_bytes(s[4..8].try_into().unwrap()),
        u32::from_le_bytes(s[8..12].try_into().unwrap()),
        u32::from_le_bytes(s[12..16].try_into().unwrap()),
    ]
}

fn words_to_bytes(s: &[u32]) -> [u8; BLOCKSIZE] {
    let mut out = [0; BLOCKSIZE];
    out[..4].copy_from_slice(&s[0].to_le_bytes());
    out[4..8].copy_from_slice(&s[1].to_le_bytes());
    out[8..12].copy_from_slice(&s[2].to_le_bytes());
    out[12..16].copy_from_slice(&s[3].to_le_bytes());
    out
}

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
macro_rules! enc_round {
    ($i: literal, $a: ident, $b: ident, $c: ident, $d: ident, $round_keys: ident) => {
        let t = $b
            .wrapping_mul($b.wrapping_add($b).wrapping_add(1))
            .rotate_left(5);
        let u = $d
            .wrapping_mul($d.wrapping_add($d).wrapping_add(1))
            .rotate_left(5);
        $a = ($a ^ t).rotate_left(u).wrapping_add($round_keys[2 * $i]);
        $c = ($c ^ u)
            .rotate_left(t)
            .wrapping_add($round_keys[2 * $i + 1]);
    };
}

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
macro_rules! dec_round {
    ($i: literal, $a: ident, $b: ident, $c: ident, $d: ident, $round_keys: ident) => {
        let u = $d
            .wrapping_mul($d.wrapping_add($d).wrapping_add(1))
            .rotate_left(5);
        let t = $b
            .wrapping_mul($b.wrapping_add($b).wrapping_add(1))
            .rotate_left(5);
        $c = $c.wrapping_sub($round_keys[2 * $i + 1]).rotate_right(t) ^ u;
        $a = $a.wrapping_sub($round_keys[2 * $i]).rotate_right(u) ^ t;
    };
}

struct Rc6 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub round_keys: [u32; NUM_ROUND_KEYS],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc6 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            round_keys: [0; NUM_ROUND_KEYS],
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
        let key_words = 4; // number of words in the key

        let mut s = [0; NUM_ROUND_KEYS];
        s[0] = P32;
        for i in 1..NUM_ROUND_KEYS {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut l = [0_u32; 4];
        for i in (0..key.len()).rev() {
            l[i / WORDSIZE] = (l[i / WORDSIZE].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        let v = 3 * max(NUM_ROUND_KEYS, key_words);
        for _ in 1..(v + 1) {
            a = s[i].wrapping_add(a).wrapping_add(b).rotate_left(3);
            s[i] = a;
            b = l[j]
                .wrapping_add(a)
                .wrapping_add(b)
                .rotate_left(a.wrapping_add(b));
            l[j] = b;

            i = (i + 1) % NUM_ROUND_KEYS;
            j = (j + 1) % key_words;
        }

        self.round_keys = s;
    }

    pub fn with_key_192(mut self, bytes: &[u8]) -> Self {
        self.ksa_192(bytes);
        self
    }

    pub fn ksa_192(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 24);
        let key_words = 6; // number of words in the key

        let mut s = [0; NUM_ROUND_KEYS];
        s[0] = P32;
        for i in 1..NUM_ROUND_KEYS {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut l = [0_u32; 6];
        for i in (0..key.len()).rev() {
            l[i / WORDSIZE] = (l[i / WORDSIZE].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        let v = 3 * max(NUM_ROUND_KEYS, key_words);
        for _ in 1..(v + 1) {
            a = s[i].wrapping_add(a).wrapping_add(b).rotate_left(3);
            s[i] = a;
            b = l[j]
                .wrapping_add(a)
                .wrapping_add(b)
                .rotate_left(a.wrapping_add(b));
            l[j] = b;

            i = (i + 1) % NUM_ROUND_KEYS;
            j = (j + 1) % key_words;
        }

        self.round_keys = s;
    }

    pub fn with_key_256(mut self, bytes: &[u8]) -> Self {
        self.ksa_256(bytes);
        self
    }

    pub fn ksa_256(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 32);
        let key_words = 8; // number of words in the key

        let mut s = [0; NUM_ROUND_KEYS];
        s[0] = P32;
        for i in 1..NUM_ROUND_KEYS {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut l = [0_u32; 8];
        for i in (0..key.len()).rev() {
            l[i / WORDSIZE] = (l[i / WORDSIZE].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        let v = 3 * max(NUM_ROUND_KEYS, key_words);
        for _ in 1..(v + 1) {
            a = s[i].wrapping_add(a).wrapping_add(b).rotate_left(3);
            s[i] = a;
            b = l[j]
                .wrapping_add(a)
                .wrapping_add(b)
                .rotate_left(a.wrapping_add(b));
            l[j] = b;

            i = (i + 1) % NUM_ROUND_KEYS;
            j = (j + 1) % key_words;
        }

        self.round_keys = s;
    }
}

impl BlockCipher<BLOCKSIZE> for Rc6 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let [mut a, mut b, mut c, mut d] = bytes_to_words(bytes);

        let keys = &self.round_keys;

        b = b.wrapping_add(keys[0]);
        d = d.wrapping_add(keys[1]);

        enc_round!(1, a, b, c, d, keys);
        enc_round!(2, b, c, d, a, keys);
        enc_round!(3, c, d, a, b, keys);
        enc_round!(4, d, a, b, c, keys);

        enc_round!(5, a, b, c, d, keys);
        enc_round!(6, b, c, d, a, keys);
        enc_round!(7, c, d, a, b, keys);
        enc_round!(8, d, a, b, c, keys);

        enc_round!(9, a, b, c, d, keys);
        enc_round!(10, b, c, d, a, keys);
        enc_round!(11, c, d, a, b, keys);
        enc_round!(12, d, a, b, c, keys);

        enc_round!(13, a, b, c, d, keys);
        enc_round!(14, b, c, d, a, keys);
        enc_round!(15, c, d, a, b, keys);
        enc_round!(16, d, a, b, c, keys);

        enc_round!(17, a, b, c, d, keys);
        enc_round!(18, b, c, d, a, keys);
        enc_round!(19, c, d, a, b, keys);
        enc_round!(20, d, a, b, c, keys);

        a = a.wrapping_add(keys[2 * ROUNDS + 2]);
        c = c.wrapping_add(keys[2 * ROUNDS + 3]);

        println!("{:08x?} {:08x?} {:08x?} {:08x?} ", a, b, c, d);

        utils::byte_formatting::overwrite_bytes(bytes, &words_to_bytes(&[a, b, c, d]));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let [mut a, mut b, mut c, mut d] = bytes_to_words(bytes);

        let keys = &self.round_keys;
        c = c.wrapping_sub(keys[2 * ROUNDS + 3]);
        a = a.wrapping_sub(keys[2 * ROUNDS + 2]);

        dec_round!(20, d, a, b, c, keys);
        dec_round!(19, c, d, a, b, keys);
        dec_round!(18, b, c, d, a, keys);
        dec_round!(17, a, b, c, d, keys);

        dec_round!(16, d, a, b, c, keys);
        dec_round!(15, c, d, a, b, keys);
        dec_round!(14, b, c, d, a, keys);
        dec_round!(13, a, b, c, d, keys);

        dec_round!(12, d, a, b, c, keys);
        dec_round!(11, c, d, a, b, keys);
        dec_round!(10, b, c, d, a, keys);
        dec_round!(9, a, b, c, d, keys);

        dec_round!(8, d, a, b, c, keys);
        dec_round!(7, c, d, a, b, keys);
        dec_round!(6, b, c, d, a, keys);
        dec_round!(5, a, b, c, d, keys);

        dec_round!(4, d, a, b, c, keys);
        dec_round!(3, c, d, a, b, keys);
        dec_round!(2, b, c, d, a, keys);
        dec_round!(1, a, b, c, d, keys);

        d = d.wrapping_sub(keys[1]);
        b = b.wrapping_sub(keys[0]);

        utils::byte_formatting::overwrite_bytes(bytes, &words_to_bytes(&[a, b, c, d]));
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

    test_1, Rc6::default().with_key_128(&
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x8f, 0xc3, 0xa5, 0x36, 0x56, 0xb1, 0xf7, 0x78, 0xc1, 0x29, 0xdf, 0x4e, 0x98, 0x48, 0xa4, 0x1e];

    test_2, Rc6::default().with_key_128(&
    [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x1A, 0xD5, 0x78, 0xA0, 0x2A, 0x08, 0x16, 0x28, 0x50, 0xA1, 0x5A, 0x15, 0x52, 0xA1, 0x7A, 0xD4];

    test_3, Rc6::default().with_key_128(&
    [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x91, 0x2E, 0x9C, 0xF1, 0x47, 0x30, 0x35, 0xA8, 0x44, 0x3A, 0x82, 0x49, 0x5C, 0x07, 0x30, 0xD3];


    test_4, Rc6::default().with_key_128(&
    [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x12, 0x23, 0x34, 0x45, 0x56, 0x67, 0x78]),
    [0x02, 0x13, 0x24, 0x35, 0x46, 0x57, 0x68, 0x79, 0x8a, 0x9b, 0xac, 0xbd, 0xce, 0xdf, 0xe0, 0xf1],
    [0x52, 0x4e, 0x19, 0x2f, 0x47, 0x15, 0xc6, 0x23, 0x1f, 0x51, 0xf6, 0x36, 0x7e, 0xa4, 0x3f, 0x18];
);
