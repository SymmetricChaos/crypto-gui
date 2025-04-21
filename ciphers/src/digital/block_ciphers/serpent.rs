use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use hex_literal::hex;
use std::ops::Shl;
use utils::byte_formatting::{fill_u32s_le, make_u32s_le, u32s_to_bytes_le, ByteFormat};

pub const ROUNDS: usize = 32;
pub const FRAC: u32 = 0x9e3779b9;

// Serpents eight 4-bit sboxes and their inverses
pub const SBOX: [[u8; 16]; 8] = [
    [3, 8, 15, 1, 10, 6, 5, 11, 14, 13, 4, 2, 7, 0, 9, 12],
    [15, 12, 2, 7, 9, 0, 5, 10, 1, 11, 14, 8, 6, 13, 3, 4],
    [8, 6, 7, 9, 3, 12, 10, 15, 13, 1, 14, 4, 0, 11, 5, 2],
    [0, 15, 11, 8, 12, 9, 6, 3, 13, 1, 2, 4, 10, 7, 5, 14],
    [1, 15, 8, 3, 12, 0, 11, 6, 2, 5, 4, 10, 9, 14, 7, 13],
    [15, 5, 2, 11, 4, 10, 9, 12, 0, 3, 14, 8, 13, 6, 7, 1],
    [7, 2, 12, 5, 8, 4, 6, 11, 14, 9, 1, 15, 13, 3, 10, 0],
    [1, 13, 15, 0, 14, 8, 2, 11, 7, 4, 12, 10, 9, 3, 5, 6],
];

pub const SBOX_INV: [[u8; 16]; 8] = [
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
fn sbox(i: usize, nibble: u8) -> u8 {
    SBOX[i][nibble as usize]
}

// Apply a specific SBOX_INV, u8 should only use the lower 4 bits
fn sbox_inv(i: usize, nibble: u8) -> u8 {
    SBOX_INV[i][nibble as usize]
}

// Select one bit from a u32
fn get_bit(x: u32, i: usize) -> u8 {
    (x >> i) as u8 & 0x01
}

// Apply a sbox across the bits of four 32-bit words
fn sbox_bitslice(idx: usize, words: [u32; 4]) -> [u32; 4] {
    let mut out: [u32; 4] = [0; 4];
    for i in 0..32 {
        // Take bits across the words
        let nibble = get_bit(words[0], i)
            | get_bit(words[1], i) << 1
            | get_bit(words[2], i) << 2
            | get_bit(words[3], i) << 3;

        // Apply the sbox to the bits
        let s = sbox(idx % 8, nibble);

        // Push the transformed bits into the output
        for pos in 0..4 {
            out[pos] |= u32::from(get_bit(s as u32, pos)) << i;
        }
    }
    out
}

fn sbox_bitslice_inv(idx: usize, words: [u32; 4]) -> [u32; 4] {
    let mut out: [u32; 4] = [0; 4];
    for i in 0..32 {
        // Take bits across the words
        let nibble = get_bit(words[0], i)
            | get_bit(words[1], i) << 1
            | get_bit(words[2], i) << 2
            | get_bit(words[3], i) << 3;

        // Apply the sbox_inv to the bits
        let s = sbox_inv(idx % 8, nibble);

        // Push the transformed bits into the output
        for pos in 0..4 {
            out[pos] |= u32::from(get_bit(s as u32, pos)) << i;
        }
    }
    out
}

// Serpent's Linear Transformation and its inverse
fn lt(mut x: [u32; 4]) -> [u32; 4] {
    x[0] = x[0].rotate_left(13);
    x[2] = x[2].rotate_left(3);
    x[1] = x[1] ^ x[0] ^ x[2];
    x[3] = x[3] ^ x[2] ^ x[0].shl(3);
    x[1] = x[1].rotate_left(1);
    x[3] = x[3].rotate_left(7);
    x[0] = x[0] ^ x[1] ^ x[3];
    x[2] = x[2] ^ x[3] ^ x[1].shl(7);
    x[0] = x[0].rotate_left(5);
    x[2] = x[2].rotate_left(22);
    x
}

fn lt_inv(mut x: [u32; 4]) -> [u32; 4] {
    x[2] = x[2].rotate_right(22);
    x[0] = x[0].rotate_right(5);
    x[2] = x[2] ^ x[3] ^ x[1].shl(7); // note that this is still shift left
    x[0] = x[0] ^ x[1] ^ x[3];
    x[3] = x[3].rotate_right(7);
    x[1] = x[1].rotate_right(1);
    x[3] = x[3] ^ x[2] ^ x[0].shl(3); // note that this is still shift left
    x[1] = x[1] ^ x[0] ^ x[2];
    x[2] = x[2].rotate_right(3);
    x[0] = x[0].rotate_right(13);
    x
}

// Expand a key to 256 bits.
// Serpent accepts keys of any bit length from 128 to 256 bits.
// I will not bother since keys not given in bytes are rare.
fn expand_key(bytes: &[u8]) -> [u8; 32] {
    let mut ex = [0; 32];
    ex[..bytes.len()].copy_from_slice(bytes);
    if bytes.len() < 32 {
        ex[bytes.len()] = 0x01 // this is correct per test vectors, probably because of little endian view of the bytes
    }
    ex
}

// Generate the pre_keys that are used to generate the round keys
fn pre_keys(bytes: &[u8]) -> [u32; 132] {
    // Copy the expanded key into the start of the pre_key
    let mut pre_key: [u32; 140] = [0; 140];
    fill_u32s_le(&mut pre_key[0..8], &expand_key(bytes));
    // Fill the entire pre_key
    for i in 0..132 {
        let pos = i + 8;
        pre_key[pos] = (pre_key[pos - 8]
            ^ pre_key[pos - 5]
            ^ pre_key[pos - 3]
            ^ pre_key[pos - 1]
            ^ FRAC
            ^ i as u32)
            .rotate_left(11);
    }
    // Discard the first eight words
    pre_key[8..].try_into().unwrap()
}

// Convert the pre_keys into the actual round keys using the sboxes
fn round_keys(pre_keys: [u32; 132]) -> [[u32; 4]; ROUNDS + 1] {
    let mut t = [0; 132];
    for (idx, chunk) in pre_keys.chunks_exact(4).enumerate() {
        let s_idx = (ROUNDS + 3 - idx) % ROUNDS;
        for i in 0..32 {
            // Take bits across the words
            let nibble = get_bit(chunk[0], i)
                | get_bit(chunk[1], i) << 1
                | get_bit(chunk[2], i) << 2
                | get_bit(chunk[3], i) << 3;

            // Apply the sbox to the bits
            let s = sbox(s_idx % 8, nibble);

            // Modified schedule for where to push the bits, not the same as the bit sliced sbox
            for pos in 0..4 {
                t[4 * idx + pos] |= u32::from(get_bit(s as u32, pos)) << i;
            }
        }
    }
    let mut out = [[0; 4]; ROUNDS + 1];
    for (i, chunk) in t.chunks_exact(4).enumerate() {
        out[i].copy_from_slice(chunk);
    }
    out
}

fn xor_words(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let mut out = [0; 4];
    for i in 0..4 {
        out[i] = a[i] ^ b[i]
    }
    out
}

pub struct Serpent {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub round_keys: [[u32; 4]; ROUNDS + 1],
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Serpent {
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

crate::block_cipher_builders! {Serpent, u128}

impl Serpent {
    pub fn ksa_128(&mut self, bytes: [u8; 16]) {
        self.round_keys = round_keys(pre_keys(&bytes));
    }

    pub fn with_key_128(mut self, bytes: [u8; 16]) -> Self {
        self.ksa_128(bytes);
        self
    }

    pub fn ksa_192(&mut self, bytes: [u8; 24]) {
        self.round_keys = round_keys(pre_keys(&bytes));
    }

    pub fn with_key_192(mut self, bytes: [u8; 24]) -> Self {
        self.ksa_192(bytes);
        self
    }

    pub fn ksa_256(&mut self, bytes: [u8; 32]) {
        self.round_keys = round_keys(pre_keys(&bytes));
    }

    pub fn with_key_256(mut self, bytes: [u8; 32]) -> Self {
        self.ksa_256(bytes);
        self
    }

    pub fn ksa_u32(&mut self, words: &[u32]) {
        assert!(words.len() >= 4 && words.len() <= 8);
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        self.round_keys = round_keys(pre_keys(&bytes));
    }
}

impl BlockCipher<16> for Serpent {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = make_u32s_le::<4>(bytes);

        for i in 0..ROUNDS - 1 {
            let t = xor_words(block, self.round_keys[i]);
            let s = sbox_bitslice(i, t);
            block = lt(s);
        }

        let t = xor_words(block, self.round_keys[ROUNDS - 1]);
        let s = sbox_bitslice(ROUNDS - 1, t);
        block = xor_words(s, self.round_keys[ROUNDS]);

        u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = make_u32s_le::<4>(bytes);

        let s = xor_words(block, self.round_keys[ROUNDS]);
        let t = sbox_bitslice_inv(ROUNDS - 1, s);
        block = xor_words(t, self.round_keys[ROUNDS - 1]);

        for i in (0..ROUNDS - 1).rev() {
            let s = lt_inv(block);
            let t = sbox_bitslice_inv(i, s);
            block = xor_words(t, self.round_keys[i])
        }

        u32s_to_bytes_le(bytes, &block);
    }
}

crate::impl_cipher_for_block_cipher!(Serpent, 16);

// https://web.archive.org/web/20140617083036/http://www.cs.technion.ac.il/~biham/Reports/Serpent/
crate::test_block_cipher!(
    test_128_1, Serpent::default().with_key_128(hex!("80000000000000000000000000000000")),
    hex!("00000000000000000000000000000000"),
    hex!("264E5481EFF42A4606ABDA06C0BFDA3D");

    test_128_2, Serpent::default().with_key_128(hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF")),
    hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF"),
    hex!("AF39614E747B9331C38B797F527EBEA6");

    test_128_3, Serpent::default().with_key_128(hex!("2BD6459F82C5B300952C49104881FF48")),
    hex!("EA024714AD5C4D84EA024714AD5C4D84"),
    hex!("92D7F8EF2C36C53409F275902F06539F");

    test_192_1, Serpent::default().with_key_192(hex!("800000000000000000000000000000000000000000000000")),
    hex!("00000000000000000000000000000000"),
    hex!("9E274EAD9B737BB21EFCFCA548602689");

    test_192_2, Serpent::default().with_key_192(hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF")),
    hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF"),
    hex!("B91C5A6582A87D13A17E3B17842F3FCC");

    test_192_3, Serpent::default().with_key_192(hex!("2BD6459F82C5B300952C49104881FF482BD6459F82C5B300")),
    hex!("EA024714AD5C4D84EA024714AD5C4D84"),
    hex!("827B18C2678A239DFC5512842000E204");

    test_256_1, Serpent::default().with_key_256(hex!("8000000000000000000000000000000000000000000000000000000000000000")),
    hex!("00000000000000000000000000000000"),
    hex!("A223AA1288463C0E2BE38EBD825616C0");

    test_256_2, Serpent::default().with_key_256(hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF")),
    hex!("BFBFBFBFBFBFBFBFBFBFBFBFBFBFBFBF"),
    hex!("052BD61DFCCEBF17FDDBA5BBEB947613");

    test_256_3, Serpent::default().with_key_256(hex!("2BD6459F82C5B300952C49104881FF482BD6459F82C5B300952C49104881FF48")),
    hex!("EA024714AD5C4D84EA024714AD5C4D84"),
    hex!("3E507730776B93FDEA661235E1DD99F0");
);
