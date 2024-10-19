use itertools::Itertools;
use utils::byte_formatting::{fill_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::{
    super::block_cipher::{BCMode, BCPadding, BlockCipher},
    functions::{mds_mult, pht, q0, q1},
};

fn h(x: u32, list: &[u8], offset: usize) -> u32 {
    let mut y = x.to_le_bytes();
    if K == 4 {
        y[0] = q1(y[0]) ^ list[4 * (6 + offset + 0)];
        y[1] = q0(y[1]) ^ list[4 * (6 + offset + 1)];
        y[2] = q0(y[2]) ^ list[4 * (6 + offset + 2)];
        y[3] = q1(y[3]) ^ list[4 * (6 + offset + 3)];
    }

    if K >= 3 {
        y[0] = q1(y[0]) ^ list[4 * (4 + offset + 0)];
        y[1] = q1(y[1]) ^ list[4 * (4 + offset + 1)];
        y[2] = q0(y[2]) ^ list[4 * (4 + offset + 2)];
        y[3] = q0(y[3]) ^ list[4 * (4 + offset + 3)];
    }

    let a = 4 * (2 + offset);
    let b = 4 * offset;

    y[0] = q1(q0(q0(y[0]) ^ list[a + 0]) ^ list[b + 0]);
    y[1] = q0(q1(q1(y[1]) ^ list[a + 1]) ^ list[b + 1]);
    y[2] = q1(q1(q0(y[2]) ^ list[a + 2]) ^ list[b + 2]);
    y[3] = q0(q1(q1(y[3]) ^ list[a + 3]) ^ list[b + 3]);

    mds_mult(y)
}

fn g(n: u32) -> u32 {
    let mut out = 0;
    let mut x = n.to_le_bytes();

    for i in 0..4 {}

    out
}

fn f(a: u32, b: u32, round: usize, subkeys: &[u32; 40]) -> (u32, u32) {
    let t0 = g(a);
    let t1 = g(b.rotate_left(8));
    let mut o = pht(t0, t1);
    o.0 = o.0.wrapping_add(subkeys[2 * round + 8]);
    o.1 = o.1.wrapping_add(subkeys[2 * round + 9]);
    o
}

const KEY_BYTES: usize = 16;
const KEY_WORDS: usize = KEY_BYTES / 4;
const K: usize = 2; // Keylength in bits divided by 64
const START: usize = 2;

pub struct TwoFish128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u128,
    pub subkeys: [u32; 40],
    sboxes: [[u32; 256]; 4],
}

impl Default for TwoFish128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
            iv: 0,
            subkeys: [0; 40],
            sboxes: [[0; 256]; 4],
        }
    }
}

crate::block_cipher_builders! {TwoFish128, u128}

impl TwoFish128 {
    pub fn sbox(&self, n: u32, i: usize) -> u32 {
        self.sboxes[i][n as usize]
    }

    pub fn ksa(&mut self, bytes: [u8; KEY_BYTES]) {
        let mut words = [0_u32; 4];
        fill_u32s_le(&mut words, &bytes);
        let rho = 0x1010101_u32;

        // Tale the even and odd words respectively
        let m_e = words.into_iter().step_by(2).collect_vec();
        let m_o = words.into_iter().skip(1).step_by(2).collect_vec();
    }

    pub fn ksa_u32(&mut self, key: [u32; KEY_WORDS]) {}

    pub fn with_key(mut self, bytes: [u8; KEY_BYTES]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn with_key_u32(mut self, bytes: [u32; KEY_WORDS]) -> Self {
        self.ksa_u32(bytes);
        self
    }
}

impl BlockCipher<16> for TwoFish128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        // Input Whitening
        for i in 0..4 {
            block[i] ^= self.subkeys[i]
        }

        for i in 0..8 {}

        // Output Whitening
        for i in 4..8 {
            block[i - 4] ^= self.subkeys[i]
        }

        u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        // Input Whitening
        for i in 0..4 {
            block[i] ^= self.subkeys[i]
        }

        for i in (0..8).rev() {}

        // Output Whitening
        for i in 4..8 {
            block[i - 4] ^= self.subkeys[i]
        }

        u32s_to_bytes_le(bytes, &block);
    }
}

// crate::test_block_cipher!(

// )
