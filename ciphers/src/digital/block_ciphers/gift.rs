use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

// https://github.com/RustCrypto/block-ciphers/tree/master/gift/src
// https://eprint.iacr.org/2020/412.pdf

// const SBOX: [u8; 16] = [
//     0x1, 0xa, 0x4, 0xc, 0x6, 0xf, 0x3, 0x9, 0x2, 0xd, 0xb, 0x7, 0x5, 0x0, 0x8, 0xe,
// ];

// const SBOX_INV: [u8; 16] = [
//     0xd, 0x0, 0x8, 0x6, 0x2, 0xc, 0x4, 0xb, 0xe, 0x7, 0x1, 0xa, 0x3, 0x9, 0xf, 0x5,
// ];

// const PERM64: [[usize; 16]; 4] = [
//     [0, 12, 8, 4, 1, 13, 9, 5, 2, 14, 10, 6, 3, 15, 11, 7],
//     [4, 0, 12, 8, 5, 1, 13, 9, 6, 2, 14, 10, 7, 3, 15, 11],
//     [8, 4, 0, 12, 9, 5, 1, 13, 10, 6, 2, 14, 11, 7, 3, 15],
//     [12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3],
// ];

// const PERM128: [[usize; 32]; 4] = [
//     [
//         0, 24, 16, 8, 1, 25, 17, 9, 2, 26, 18, 10, 3, 27, 19, 11, 4, 28, 20, 12, 5, 29, 21, 13, 6,
//         30, 22, 14, 7, 31, 23, 15,
//     ],
//     [
//         8, 0, 24, 16, 9, 1, 25, 17, 10, 2, 26, 18, 11, 3, 27, 19, 12, 4, 28, 20, 13, 5, 29, 21, 14,
//         6, 30, 22, 15, 7, 31, 23,
//     ],
//     [
//         16, 8, 0, 24, 17, 9, 1, 25, 18, 10, 2, 26, 19, 11, 3, 27, 20, 12, 4, 28, 21, 13, 5, 29, 22,
//         14, 6, 30, 23, 15, 7, 31,
//     ],
//     [
//         24, 16, 8, 0, 25, 17, 9, 1, 26, 18, 10, 2, 27, 19, 11, 3, 28, 20, 12, 4, 29, 21, 13, 5, 30,
//         22, 14, 6, 31, 23, 15, 7,
//     ],
// ];

const RC: [u32; 40] = [
    0x10000008, 0x80018000, 0x54000002, 0x01010181, 0x8000001f, 0x10888880, 0x6001e000, 0x51500002,
    0x03030180, 0x8000002f, 0x10088880, 0x60016000, 0x41500002, 0x03030080, 0x80000027, 0x10008880,
    0x4001e000, 0x11500002, 0x03020180, 0x8000002b, 0x10080880, 0x60014000, 0x01400002, 0x02020080,
    0x80000021, 0x10000080, 0x0001c000, 0x51000002, 0x03010180, 0x8000002e, 0x10088800, 0x60012000,
    0x40500002, 0x01030080, 0x80000006, 0x10008808, 0xc001a000, 0x14500002, 0x01020181, 0x8000001a,
];

// Software optimized implementation of the GIFT-128 SBOX from the original paper
fn sbox(s0: &mut u32, s1: &mut u32, s2: &mut u32, s3: &mut u32) {
    *s1 ^= *s0 & *s2;
    *s0 ^= *s1 & *s3;
    *s2 ^= *s0 | *s1;
    *s3 ^= *s2;
    *s1 ^= *s3;
    *s3 ^= 0xffffffff;
    *s2 ^= *s0 & *s1;
}

fn sbox_inv(s0: &mut u32, s1: &mut u32, s2: &mut u32, s3: &mut u32) {
    *s2 ^= *s3 & *s1;
    *s0 ^= 0xffffffff;
    *s1 ^= *s0;
    *s0 ^= *s2;
    *s2 ^= *s3 | *s1;
    *s3 ^= *s1 & *s0;
    *s1 ^= *s3 & *s2;
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

// swap the bits in the word masked by M, with the bits of the word masked by (M << N)
// In the paper this is just swampmove with a == b but that's not feasible in Rust
fn swapmove_single(a: &mut u32, mask: u32, n: u8) {
    let tmp = (*a ^ (*a >> n)) & mask;
    *a ^= tmp;
    *a ^= tmp << n;
}

// load the bytes of input into the block that will be operated on
// The purpose here is to arrange the bits so that the fixed-slice operations can be applied by the quintuple round
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

// GIFT-128 has 40 total rounds
pub fn quintuple_round(x: &mut [u32; 4], round_keys: &[u32], index: usize) {
    let [s0, s1, s2, s3] = x;
    sbox(s0, s1, s2, s3);
    *s3 = nibble_ror_1(s3);
    *s1 = nibble_ror_2(s1);
    *s2 = nibble_ror_3(s2);
    *s1 ^= round_keys[index + 0];
    *s2 ^= round_keys[index + 1];
    *s0 ^= RC[index + 0];

    sbox(s3, s1, s2, s0);
    *s3 = half_ror_4(s3);
    *s1 = half_ror_8(s1);
    *s2 = half_ror_12(s2);
    *s1 ^= round_keys[index + 2];
    *s2 ^= round_keys[index + 3];
    *s3 ^= RC[index + 1];

    sbox(s0, s1, s2, s3);
    *s2 = s2.rotate_right(16);
    *s3 = s3.rotate_right(16);
    swapmove_single(s1, 0x55555555, 1);
    swapmove_single(s2, 0x00005555, 1);
    swapmove_single(s3, 0x55550000, 1);
    *s1 ^= round_keys[index + 4];
    *s2 ^= round_keys[index + 5];
    *s0 ^= RC[index + 2];

    sbox(s3, s1, s2, s0);
    *s2 = byte_ror_2(s2);
    *s1 = byte_ror_4(s1);
    *s0 = byte_ror_6(s0);
    *s1 ^= round_keys[index + 6];
    *s2 ^= round_keys[index + 7];
    *s3 ^= RC[index + 3];

    sbox(s0, s1, s2, s3);
    *s2 = s2.rotate_right(8);
    *s1 = s1.rotate_right(16);
    *s3 = s3.rotate_right(24);
    *s1 ^= round_keys[index + 8];
    *s2 ^= round_keys[index + 9];
    *s3 ^= RC[index + 4];

    std::mem::swap(s0, s3);
}

pub fn quintuple_round_inv(x: &mut [u32; 4], round_keys: &[u32], index: usize) {
    let [s0, s1, s2, s3] = x;
    std::mem::swap(s0, s3);

    *s1 ^= round_keys[index + 8];
    *s2 ^= round_keys[index + 9];
    *s3 ^= RC[index + 4];
    *s3 = s3.rotate_right(8);
    *s1 = s1.rotate_right(16);
    *s2 = s2.rotate_right(24);
    sbox_inv(s3, s1, s2, s0);

    *s1 ^= round_keys[index + 6];
    *s2 ^= round_keys[index + 7];
    *s3 ^= RC[index + 3];
    *s0 = byte_ror_2(s0);
    *s1 = byte_ror_4(s1);
    *s2 = byte_ror_6(s2);
    sbox_inv(s0, s1, s2, s3);

    *s1 ^= round_keys[index + 4];
    *s2 ^= round_keys[index + 5];
    *s0 ^= RC[index + 2];
    swapmove_single(s1, 0x55555555, 1);
    swapmove_single(s2, 0x00005555, 1);
    swapmove_single(s3, 0x55550000, 1);
    *s2 = s2.rotate_right(16);
    *s3 = s3.rotate_right(16);
    sbox_inv(s3, s1, s2, s0);

    *s1 ^= round_keys[index + 2];
    *s2 ^= round_keys[index + 3];
    *s3 ^= RC[index + 1];
    *s2 = half_ror_4(s2);
    *s1 = half_ror_8(s1);
    *s3 = half_ror_12(s3);
    sbox_inv(s0, s1, s2, s3);

    *s1 ^= round_keys[index + 0];
    *s2 ^= round_keys[index + 1];
    *s0 ^= RC[index + 0];
    *s3 = nibble_ror_1(s3);
    *s1 = nibble_ror_2(s1);
    *s2 = nibble_ror_3(s2);
    sbox_inv(s3, s1, s2, s0);
}

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
    pub mode: BCMode,
    pub padding: BCPadding,
    pub round_keys: [u32; 80],
}

crate::block_cipher_builders! {Gift128, u128}

impl Default for Gift128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
            round_keys: [0; 80],
        }
    }
}

impl Gift128 {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        self.round_keys = [0; 80];
        self.round_keys[0] = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        self.round_keys[1] = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        self.round_keys[2] = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        self.round_keys[3] = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for Gift128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 4];
        pack(&mut v, bytes);
        for i in (0..40).step_by(5) {
            quintuple_round(&mut v, &self.round_keys, i);
        }
        unpack(&v, bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 4];
        pack(&mut v, bytes);
        for i in 0..40 {}
        unpack(&v, bytes);
    }
    crate::block_cipher_getters!();
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
