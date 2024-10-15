use utils::byte_formatting::{fill_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

// 4-bit SBOX for q0 function
const Q0: [[u8; 16]; 4] = [
    [0x8, 0x1, 0x7, 0xD, 0x6, 0xF, 0x3, 0x2, 0x0, 0xB, 0x5, 0x9, 0xE, 0xC, 0xA, 0x4],
    [0xE, 0xC, 0xB, 0x8, 0x1, 0x2, 0x3, 0x5, 0xF, 0x4, 0xA, 0x6, 0x7, 0x0, 0x9, 0xD],
    [0xB, 0xA, 0x5, 0xE, 0x6, 0xD, 0x9, 0x0, 0xC, 0x8, 0xF, 0x3, 0x2, 0x4, 0x7, 0x1],
    [0xD, 0x7, 0xF, 0x4, 0x1, 0x2, 0x6, 0xE, 0x9, 0xB, 0x3, 0x0, 0x8, 0x5, 0xC, 0xA],
];

// 4-bit SBOX for q1 function
const Q1: [[u8; 16]; 4] = [
    [0x2, 0x8, 0xB, 0xD, 0xF, 0x7, 0x6, 0xE, 0x3, 0x1, 0x9, 0x4, 0x0, 0xA, 0xC, 0x5],
    [0x1, 0xE, 0x2, 0xB, 0x4, 0xC, 0x3, 0x7, 0x6, 0xD, 0xA, 0x5, 0xF, 0x9, 0x0, 0x8],
    [0x4, 0xC, 0x7, 0x5, 0x1, 0x6, 0x9, 0xA, 0x0, 0xE, 0xD, 0x8, 0x2, 0xB, 0x3, 0xF],
    [0xB, 0x9, 0x5, 0x1, 0xC, 0x3, 0xD, 0xE, 0x6, 0x4, 0x7, 0xF, 0x2, 0x0, 0x8, 0xA],
];

// Each 4-bit nibble is rotated one bit toward the LSB (to the right)
fn nibble_ror_1(x: u8) -> u8 {
    (((x) >> 1) & 0x77) | (((x) & 0x11) << 3)
}

fn q0(n: u8) -> u8{
    let a0 = (n >> 4) & 15;
    let b0 = n & 15;
    let a1 = a0 ^ b0;
    let b1 = a0 ^ nibble_ror_1(b0) ^ ((8*a0) & 15);
    let a2 = Q0[0][a1 as usize];
    let b2 = Q0[1][b1 as usize];
    let a3 = a2 ^ b2;
    let b3 = a2 ^ nibble_ror_1(b2) ^ ((8*a2) & 15);
    let a4 = Q0[2][a3 as usize];
    let b4 = Q0[3][b3 as usize];
    (b4 << 4) | a4
}

fn q1() {
    let a0 = (n >> 4) & 15;
    let b0 = n & 15;
    let a1 = a0 ^ b0;
    let b1 = a0 ^ nibble_ror_1(b0) ^ ((8*a0) & 15);
    let a2 = Q1[0][a1 as usize];
    let b2 = Q1[1][b1 as usize];
    let a3 = a2 ^ b2;
    let b3 = a2 ^ nibble_ror_1(b2) ^ ((8*a2) & 15);
    let a4 = Q1[2][a3 as usize];
    let b4 = Q1[3][b3 as usize];
    (b4 << 4) | a4
}

// Pseudo-Hadamard Transform
fn pht(a: u32, b: u32) -> (u32,u32) {
    (
        a.wrapping_add(b)
        a.wrapping_add(b << 1)
    )
}

fn f(a: u32, b: u32, round: usize, subkeys: &[u32;40]) -> (u32,u32) {
    let t0 = g(a);
    let t1 = g(b.rotate_left(8));
    let mut o = pht(t0, t1);
    o.0 = o.0.wrapping_add(subkeys[2*round+8]);
    o.1 = o.1.wrapping_add(subkeys[2*round+9])
    o
}

fn g(n: u32) -> u32 {
    let mut x = n.to_le_bytes();

    todo!()
}



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
    const K: u32 = 2; // Keylength in bits divided by 64

    pub fn sbox(&self, n: u32, i: usize) -> u32 {
        self.sboxes[i][n as usize]
    }
    
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        let mut words = [0_u32; 4];
        fill_u32s_le(&mut word, bytes);
    }

    pub fn ksa_u32(&mut self, key: [u32; 4]) {}

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn with_key_u32(mut self, bytes: [u32; 4]) -> Self {
        self.ksa(bytes);
        self
    }

    fn h(x: u32, list: [u32; Self:K]) -> u32 {
        todo!()
    }
}

impl BlockCipher<16> for TwoFish128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        u32s_to_bytes_le(bytes, &block);
    }
}
