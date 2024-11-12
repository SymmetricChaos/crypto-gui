use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::{
    make_u16s_be, make_u16s_le, u16s_to_bytes_be, u16s_to_bytes_le, ByteFormat,
};

const FTABLE: [u8; 256] = [
    0xa3, 0xd7, 0x09, 0x83, 0xf8, 0x48, 0xf6, 0xf4, 0xb3, 0x21, 0x15, 0x78, 0x99, 0xb1, 0xaf, 0xf9,
    0xe7, 0x2d, 0x4d, 0x8a, 0xce, 0x4c, 0xca, 0x2e, 0x52, 0x95, 0xd9, 0x1e, 0x4e, 0x38, 0x44, 0x28,
    0x0a, 0xdf, 0x02, 0xa0, 0x17, 0xf1, 0x60, 0x68, 0x12, 0xb7, 0x7a, 0xc3, 0xe9, 0xfa, 0x3d, 0x53,
    0x96, 0x84, 0x6b, 0xba, 0xf2, 0x63, 0x9a, 0x19, 0x7c, 0xae, 0xe5, 0xf5, 0xf7, 0x16, 0x6a, 0xa2,
    0x39, 0xb6, 0x7b, 0x0f, 0xc1, 0x93, 0x81, 0x1b, 0xee, 0xb4, 0x1a, 0xea, 0xd0, 0x91, 0x2f, 0xb8,
    0x55, 0xb9, 0xda, 0x85, 0x3f, 0x41, 0xbf, 0xe0, 0x5a, 0x58, 0x80, 0x5f, 0x66, 0x0b, 0xd8, 0x90,
    0x35, 0xd5, 0xc0, 0xa7, 0x33, 0x06, 0x65, 0x69, 0x45, 0x00, 0x94, 0x56, 0x6d, 0x98, 0x9b, 0x76,
    0x97, 0xfc, 0xb2, 0xc2, 0xb0, 0xfe, 0xdb, 0x20, 0xe1, 0xeb, 0xd6, 0xe4, 0xdd, 0x47, 0x4a, 0x1d,
    0x42, 0xed, 0x9e, 0x6e, 0x49, 0x3c, 0xcd, 0x43, 0x27, 0xd2, 0x07, 0xd4, 0xde, 0xc7, 0x67, 0x18,
    0x89, 0xcb, 0x30, 0x1f, 0x8d, 0xc6, 0x8f, 0xaa, 0xc8, 0x74, 0xdc, 0xc9, 0x5d, 0x5c, 0x31, 0xa4,
    0x70, 0x88, 0x61, 0x2c, 0x9f, 0x0d, 0x2b, 0x87, 0x50, 0x82, 0x54, 0x64, 0x26, 0x7d, 0x03, 0x40,
    0x34, 0x4b, 0x1c, 0x73, 0xd1, 0xc4, 0xfd, 0x3b, 0xcc, 0xfb, 0x7f, 0xab, 0xe6, 0x3e, 0x5b, 0xa5,
    0xad, 0x04, 0x23, 0x9c, 0x14, 0x51, 0x22, 0xf0, 0x29, 0x79, 0x71, 0x7e, 0xff, 0x8c, 0x0e, 0xe2,
    0x0c, 0xef, 0xbc, 0x72, 0x75, 0x6f, 0x37, 0xa1, 0xec, 0xd3, 0x8e, 0x62, 0x8b, 0x86, 0x10, 0xe8,
    0x08, 0x77, 0x11, 0xbe, 0x92, 0x4f, 0x24, 0xc5, 0x32, 0x36, 0x9d, 0xcf, 0xf3, 0xa6, 0xbb, 0xac,
    0x5e, 0x6c, 0xa9, 0x13, 0x57, 0x25, 0xb5, 0xe3, 0xbd, 0xa8, 0x3a, 0x01, 0x05, 0x59, 0x2a, 0x46,
];

fn g(x: u16, ctr: u16, k: &[u8; 10]) -> u16 {
    let p = (ctr * 4) as usize;
    let [g1, g2] = x.to_be_bytes();
    let g3 = FTABLE[(g2 ^ k[(p + 0) % 10]) as usize] ^ g1;
    let g4 = FTABLE[(g3 ^ k[(p + 1) % 10]) as usize] ^ g2;
    let g5 = FTABLE[(g4 ^ k[(p + 2) % 10]) as usize] ^ g3;
    let g6 = FTABLE[(g5 ^ k[(p + 3) % 10]) as usize] ^ g4;
    u16::from_be_bytes([g5, g6])
}

fn g_inv(x: u16, ctr: u16, k: &[u8; 10]) -> u16 {
    let p = (ctr * 4) as usize;
    let [g5, g6] = x.to_be_bytes();
    let g4 = FTABLE[(g5 ^ k[(p + 3) % 10]) as usize] ^ g6;
    let g3 = FTABLE[(g4 ^ k[(p + 2) % 10]) as usize] ^ g5;
    let g2 = FTABLE[(g3 ^ k[(p + 1) % 10]) as usize] ^ g4;
    let g1 = FTABLE[(g2 ^ k[(p + 0) % 10]) as usize] ^ g3;
    u16::from_be_bytes([g1, g2])
}

fn a(x: [u16; 4], ctr: u16, k: &[u8; 10]) -> [u16; 4] {
    [x[3] ^ g(x[0], ctr, k) ^ ctr, g(x[0], ctr, k), x[1], x[2]]
}

fn a_inv(x: [u16; 4], ctr: u16, k: &[u8; 10]) -> [u16; 4] {
    [g_inv(x[1], ctr, k), x[2], x[3], x[0] ^ x[1] ^ ctr]
}

fn b(x: [u16; 4], ctr: u16, k: &[u8; 10]) -> [u16; 4] {
    [x[3], g(x[0], ctr, k), x[0] ^ x[1] ^ ctr, x[2]]
}

fn b_inv(x: [u16; 4], ctr: u16, k: &[u8; 10]) -> [u16; 4] {
    [
        g_inv(x[1], ctr, k),
        x[2] ^ g_inv(x[1], ctr, k) ^ ctr,
        x[3],
        x[0],
    ]
}

pub struct Skipjack {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub key: [u8; 10],
}

impl Default for Skipjack {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
            key: [0; 10],
        }
    }
}

crate::block_cipher_builders! {Skipjack, u64}

impl Skipjack {
    pub fn ksa(&mut self, bytes: [u8; 10]) {
        self.key = bytes;
    }

    pub fn with_key(mut self, bytes: [u8; 10]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Skipjack {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = make_u16s_be::<4>(bytes);

        // Eight A rounds
        // note ctr starts at 1
        v = a(v, 1, &self.key);
        v = a(v, 2, &self.key);
        v = a(v, 3, &self.key);
        v = a(v, 4, &self.key);
        v = a(v, 5, &self.key);
        v = a(v, 6, &self.key);
        v = a(v, 7, &self.key);
        v = a(v, 8, &self.key);

        // Eight B rounds
        v = b(v, 9, &self.key);
        v = b(v, 10, &self.key);
        v = b(v, 11, &self.key);
        v = b(v, 12, &self.key);
        v = b(v, 13, &self.key);
        v = b(v, 14, &self.key);
        v = b(v, 15, &self.key);
        v = b(v, 16, &self.key);

        // Eight A rounds
        v = a(v, 17, &self.key);
        v = a(v, 18, &self.key);
        v = a(v, 19, &self.key);
        v = a(v, 20, &self.key);
        v = a(v, 21, &self.key);
        v = a(v, 22, &self.key);
        v = a(v, 23, &self.key);
        v = a(v, 24, &self.key);

        // Eight B rounds
        v = b(v, 25, &self.key);
        v = b(v, 26, &self.key);
        v = b(v, 27, &self.key);
        v = b(v, 28, &self.key);
        v = b(v, 29, &self.key);
        v = b(v, 30, &self.key);
        v = b(v, 31, &self.key);
        v = b(v, 32, &self.key);

        v.reverse();

        u16s_to_bytes_le(bytes, &v);
    }
    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = make_u16s_le::<4>(bytes);

        v.reverse();

        v = b_inv(v, 32, &self.key);
        v = b_inv(v, 31, &self.key);
        v = b_inv(v, 30, &self.key);
        v = b_inv(v, 29, &self.key);
        v = b_inv(v, 28, &self.key);
        v = b_inv(v, 27, &self.key);
        v = b_inv(v, 26, &self.key);
        v = b_inv(v, 25, &self.key);

        v = a_inv(v, 24, &self.key);
        v = a_inv(v, 23, &self.key);
        v = a_inv(v, 22, &self.key);
        v = a_inv(v, 21, &self.key);
        v = a_inv(v, 20, &self.key);
        v = a_inv(v, 19, &self.key);
        v = a_inv(v, 18, &self.key);
        v = a_inv(v, 17, &self.key);

        v = b_inv(v, 16, &self.key);
        v = b_inv(v, 15, &self.key);
        v = b_inv(v, 14, &self.key);
        v = b_inv(v, 13, &self.key);
        v = b_inv(v, 12, &self.key);
        v = b_inv(v, 11, &self.key);
        v = b_inv(v, 10, &self.key);
        v = b_inv(v, 9, &self.key);

        v = a_inv(v, 8, &self.key);
        v = a_inv(v, 7, &self.key);
        v = a_inv(v, 6, &self.key);
        v = a_inv(v, 5, &self.key);
        v = a_inv(v, 4, &self.key);
        v = a_inv(v, 3, &self.key);
        v = a_inv(v, 2, &self.key);
        v = a_inv(v, 1, &self.key);

        u16s_to_bytes_be(bytes, &v);
    }
}

crate::impl_cipher_for_block_cipher!(Skipjack, 8);

crate::test_block_cipher!(
    test_1, Skipjack::default().with_key([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
    [0x9a, 0x90, 0xbc, 0x0B, 0x75, 0xc7, 0x37, 0x03];
);
