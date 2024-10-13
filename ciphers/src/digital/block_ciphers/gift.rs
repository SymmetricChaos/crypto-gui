use utils::byte_formatting::ByteFormat;

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

// const SBOX: [u8; 16] = [
//     0x1, 0xa, 0x4, 0xc, 0x6, 0xf, 0x3, 0x9, 0x2, 0xd, 0xb, 0x7, 0x5, 0x0, 0x8, 0xe,
// ];

// const SBOX_INV: [u8; 16] = [
//     0xd, 0x0, 0x8, 0x6, 0x2, 0xc, 0x4, 0xb, 0xe, 0x7, 0x1, 0xa, 0x3, 0x9, 0xf, 0x5,
// ];

const RC: [u8; 48] = [
    0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3E, 0x3D, 0x3B, 0x37, 0x2F, 0x1E, 0x3C, 0x39, 0x33, 0x27, 0x0E,
    0x1D, 0x3A, 0x35, 0x2B, 0x16, 0x2C, 0x18, 0x30, 0x21, 0x02, 0x05, 0x0B, 0x17, 0x2E, 0x1C, 0x38,
    0x31, 0x23, 0x06, 0x0D, 0x1B, 0x36, 0x2D, 0x1A, 0x34, 0x29, 0x12, 0x24, 0x08, 0x11, 0x22, 0x04,
];

// Software optimized implementation of the GIFT-128 SBOX from the original paper
fn sbox(x: &mut [u32; 4]) {
    x[1] ^= x[0] & x[2];
    x[0] ^= x[1] & x[3];
    x[2] ^= x[0] | x[1];
    x[3] ^= x[2];
    x[1] ^= x[3];
    x[3] ^= 0xffffffff;
    x[2] ^= x[0] & x[1];
}

fn sbox_inv(x: &mut [u32; 4]) {
    x[2] ^= x[3] & x[1];
    x[0] ^= 0xffffffff;
    x[1] ^= x[0];
    x[0] ^= x[2];
    x[2] ^= x[3] | x[1];
    x[3] ^= x[1] & x[0];
    x[1] ^= x[3] & x[2];
}

// Each 4-bit nibble is rotated one bit toward the LSB (to the right)
fn nibble_ror_1(x: &u32) -> u32 {
    (((x) >> 1) & 0x77777777) | (((x) & 0x11111111) << 3)
}

fn nibble_ror_2(x: &u32) -> u32 {
    (((x) >> 2) & 0x33333333) | (((x) & 0x33333333) << 2)
}

fn nibble_ror_3(&x: &u32) -> u32 {
    (((x) >> 3) & 0x11111111) | (((x) & 0x77777777) << 1)
}

// Each 8-bit byte is rotated two bits toward the LSB (to the right)
fn byte_ror_2(x: &u32) -> u32 {
    (((x) >> 2) & 0x3f3f3f3f) | (((x) & 0x03030303) << 6)
}

fn byte_ror_4(x: &u32) -> u32 {
    (((x) >> 4) & 0x0f0f0f0f) | (((x) & 0x0f0f0f0f) << 4)
}

fn byte_ror_6(x: &u32) -> u32 {
    (((x) >> 6) & 0x03030303) | (((x) & 0x3f3f3f3f) << 2)
}

// Each 16-bit half-word is rotated four bits toward the LSB (to the right)
fn half_ror_4(&x: &u32) -> u32 {
    (((x) >> 4) & 0x0fff0fff) | (((x) & 0x000f000f) << 12)
}

fn half_ror_8(x: &u32) -> u32 {
    (((x) >> 8) & 0x00ff00ff) | (((x) & 0x00ff00ff) << 8)
}

fn half_ror_12(&x: &u32) -> u32 {
    (((x) >> 12) & 0x000f000f) | (((x) & 0x0fff0fff) << 4)
}

// swap the bits in B masked by M, with the bits in A masked by (M << N)
fn swapmove(a: &mut u32, b: &mut u32, mask: u32, n: u8) {
    let tmp = (*b ^ (*a >> n)) & mask;
    *b ^= tmp;
    *a ^= tmp << n;
}

// swap the bits in the word masked by M, with the bits in of the word masked by (M << N)
// In the paper this is just swampmode with a == b but that's not feasible in Rust
fn swapmove_single(a: &mut u32, mask: u32, n: u8) {
    let tmp = (*a ^ (*a >> n)) & mask;
    *a ^= tmp;
    *a ^= tmp << n;
}

// load the bytes of input into the block that will be operated on
fn pack(block: &mut [u32; 4], bytes: &[u8]) {
    let mut s0 = u32::from_be_bytes([bytes[6], bytes[7], bytes[14], bytes[15]]);
    let mut s1 = u32::from_be_bytes([bytes[4], bytes[5], bytes[12], bytes[13]]);
    let mut s2 = u32::from_be_bytes([bytes[2], bytes[3], bytes[10], bytes[11]]);
    let mut s3 = u32::from_be_bytes([bytes[0], bytes[1], bytes[8], bytes[9]]);

    swapmove_single(&mut s0, 0x0a0a0a0a, 3);
    swapmove_single(&mut s0, 0x00cc00cc, 6);
    swapmove_single(&mut s1, 0x0a0a0a0a, 3);
    swapmove_single(&mut s1, 0x00cc00cc, 6);
    swapmove_single(&mut s2, 0x0a0a0a0a, 3);
    swapmove_single(&mut s2, 0x00cc00cc, 6);
    swapmove_single(&mut s3, 0x0a0a0a0a, 3);
    swapmove_single(&mut s3, 0x00cc00cc, 6);

    swapmove(&mut s0, &mut s1, 0x000f000f, 4);
    swapmove(&mut s0, &mut s2, 0x000f000f, 8);
    swapmove(&mut s0, &mut s3, 0x000f000f, 12);
    swapmove(&mut s1, &mut s2, 0x00f000f0, 4);
    swapmove(&mut s1, &mut s3, 0x00f000f0, 8);
    swapmove(&mut s2, &mut s3, 0x0f000f00, 4);

    *block = [s0, s1, s2, s3];
}

// overwrite some bytes with the contents of the block
fn unpack(block: &[u32; 4], bytes: &mut [u8]) {
    let (mut s0, mut s1, mut s2, mut s3) = (block[0], block[1], block[2], block[3]);

    swapmove(&mut s2, &mut s3, 0x0f000f00, 4);
    swapmove(&mut s1, &mut s3, 0x00f000f0, 8);
    swapmove(&mut s1, &mut s2, 0x00f000f0, 4);
    swapmove(&mut s0, &mut s3, 0x000f000f, 12);
    swapmove(&mut s0, &mut s2, 0x000f000f, 8);
    swapmove(&mut s0, &mut s1, 0x000f000f, 4);
    swapmove_single(&mut s3, 0x00cc00cc, 6);
    swapmove_single(&mut s3, 0x0a0a0a0a, 3);
    swapmove_single(&mut s2, 0x00cc00cc, 6);
    swapmove_single(&mut s2, 0x0a0a0a0a, 3);
    swapmove_single(&mut s1, 0x00cc00cc, 6);
    swapmove_single(&mut s1, 0x0a0a0a0a, 3);
    swapmove_single(&mut s0, 0x00cc00cc, 6);
    swapmove_single(&mut s0, 0x0a0a0a0a, 3);

    bytes[0] = (s3 >> 24) as u8;
    bytes[1] = ((s3 >> 16) & 0xff) as u8;
    bytes[2] = (s2 >> 24) as u8;
    bytes[3] = ((s2 >> 16) & 0xff) as u8;
    bytes[4] = (s1 >> 24) as u8;
    bytes[5] = ((s1 >> 16) & 0xff) as u8;
    bytes[6] = (s0 >> 24) as u8;
    bytes[7] = ((s0 >> 16) & 0xff) as u8;
    bytes[8] = ((s3 >> 8) & 0xff) as u8;
    bytes[9] = (s3 & 0xff) as u8;
    bytes[10] = ((s2 >> 8) & 0xff) as u8;
    bytes[11] = (s2 & 0xff) as u8;
    bytes[12] = ((s1 >> 8) & 0xff) as u8;
    bytes[13] = (s1 & 0xff) as u8;
    bytes[14] = ((s0 >> 8) & 0xff) as u8;
    bytes[15] = (s0 & 0xff) as u8;
}

fn sub_cells() {}

fn perm_bits() {}

// pub struct Gift64 {
//     pub input_format: ByteFormat,
//     pub output_format: ByteFormat,
//     pub iv: u64,
//     pub round_keys: [u32; 28],
//     pub mode: BCMode,
//     pub padding: BCPadding,
// }

// crate::block_cipher_builders! {Gift64, u64}

// impl Default for Gift64 {
//     fn default() -> Self {
//         Self {
//             input_format: ByteFormat::Hex,
//             output_format: ByteFormat::Hex,
//             iv: 0,
//             round_keys: [0; 28],
//             mode: Default::default(),
//             padding: Default::default(),
//         }
//     }
// }

// impl Gift64 {
//     pub fn ksa(&mut self, bytes: [u8; 16]) {}

//     pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
//         self.ksa(bytes);
//         self
//     }
// }

// impl BlockCipher<8> for Gift64 {
//     fn encrypt_block(&self, bytes: &mut [u8]) {
//         for i in 0..28 {}
//     }

//     fn decrypt_block(&self, bytes: &mut [u8]) {
//         for i in 0..28 {}
//     }
// }

// crate::impl_cipher_for_block_cipher!(Gift64, 8);

pub struct Gift128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u128,
    pub round_keys: [u32; 80],
    pub mode: BCMode,
    pub padding: BCPadding,
}

crate::block_cipher_builders! {Gift128, u128}

impl Default for Gift128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            round_keys: [0; 80],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Gift128 {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        let mut rk = [0; 80];
        rk[0] = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        rk[1] = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        rk[2] = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        rk[3] = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn round(x: &mut [u32; 4], rk: [u32; 2], rc: u32) {
        sbox(x);
        x[3] = nibble_ror_1(&x[3]);
        x[1] = nibble_ror_2(&x[1]);
        x[2] = nibble_ror_3(&x[2]);
        x[1] ^= rk[0];
        x[2] ^= rk[1];
        x[0] ^= rc;
    }

    pub fn round_inv(x: &mut [u32; 4], rk: [u32; 2], rc: u32) {
        sbox_inv(x);
    }
}

impl BlockCipher<16> for Gift128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 4];
        pack(&mut v, bytes);
        for i in 0..40 {}
        unpack(&v, bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 4];
        pack(&mut v, bytes);
        for i in 0..40 {}
        unpack(&v, bytes);
    }
}

crate::impl_cipher_for_block_cipher!(Gift128, 16);

// #[cfg(test)]
// mod gift_tests {

//     use super::*;

// }

crate::test_block_cipher!(
    test_1, Gift128::default().with_key([0;16]),
    [0; 16],
    [0xcd, 0x0b, 0xd7, 0x38, 0x38, 0x8a, 0xd3, 0xf6, 0x68, 0xb1, 0x5a, 0x36, 0xce, 0xb6, 0xff, 0x92];

    test_2, Gift128::default().with_key([0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]),
    [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10],
    [0x84, 0x22, 0x24, 0x1a, 0x6d, 0xbf, 0x5a, 0x93, 0x46, 0xaf, 0x46, 0x84, 0x09, 0xee, 0x01, 0x52];

    test_3, Gift128::default().with_key([0xd0, 0xf5, 0xc5, 0x9a, 0x77, 0x00, 0xd3, 0xe7, 0x99, 0x02, 0x8f, 0xa9, 0xf9, 0x0a, 0xd8, 0x37]),
    [0xe3, 0x9c, 0x14, 0x1f, 0xa5, 0x7d, 0xba, 0x43, 0xf0, 0x8a, 0x85, 0xb6, 0xa9, 0x1f, 0x86, 0xc1],
    [0x13, 0xed, 0xe6, 0x7c, 0xbd, 0xcc, 0x3d, 0xbf, 0x40, 0x0a, 0x62, 0xd6, 0x97, 0x72, 0x65, 0xea];
);
