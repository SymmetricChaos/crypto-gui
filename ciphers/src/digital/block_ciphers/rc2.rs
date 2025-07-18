use utils::byte_formatting::{overwrite_bytes, ByteFormat};

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

const PI: [u8; 256] = [
    0xd9, 0x78, 0xf9, 0xc4, 0x19, 0xdd, 0xb5, 0xed, 0x28, 0xe9, 0xfd, 0x79, 0x4a, 0xa0, 0xd8, 0x9d,
    0xc6, 0x7e, 0x37, 0x83, 0x2b, 0x76, 0x53, 0x8e, 0x62, 0x4c, 0x64, 0x88, 0x44, 0x8b, 0xfb, 0xa2,
    0x17, 0x9a, 0x59, 0xf5, 0x87, 0xb3, 0x4f, 0x13, 0x61, 0x45, 0x6d, 0x8d, 0x09, 0x81, 0x7d, 0x32,
    0xbd, 0x8f, 0x40, 0xeb, 0x86, 0xb7, 0x7b, 0x0b, 0xf0, 0x95, 0x21, 0x22, 0x5c, 0x6b, 0x4e, 0x82,
    0x54, 0xd6, 0x65, 0x93, 0xce, 0x60, 0xb2, 0x1c, 0x73, 0x56, 0xc0, 0x14, 0xa7, 0x8c, 0xf1, 0xdc,
    0x12, 0x75, 0xca, 0x1f, 0x3b, 0xbe, 0xe4, 0xd1, 0x42, 0x3d, 0xd4, 0x30, 0xa3, 0x3c, 0xb6, 0x26,
    0x6f, 0xbf, 0x0e, 0xda, 0x46, 0x69, 0x07, 0x57, 0x27, 0xf2, 0x1d, 0x9b, 0xbc, 0x94, 0x43, 0x03,
    0xf8, 0x11, 0xc7, 0xf6, 0x90, 0xef, 0x3e, 0xe7, 0x06, 0xc3, 0xd5, 0x2f, 0xc8, 0x66, 0x1e, 0xd7,
    0x08, 0xe8, 0xea, 0xde, 0x80, 0x52, 0xee, 0xf7, 0x84, 0xaa, 0x72, 0xac, 0x35, 0x4d, 0x6a, 0x2a,
    0x96, 0x1a, 0xd2, 0x71, 0x5a, 0x15, 0x49, 0x74, 0x4b, 0x9f, 0xd0, 0x5e, 0x04, 0x18, 0xa4, 0xec,
    0xc2, 0xe0, 0x41, 0x6e, 0x0f, 0x51, 0xcb, 0xcc, 0x24, 0x91, 0xaf, 0x50, 0xa1, 0xf4, 0x70, 0x39,
    0x99, 0x7c, 0x3a, 0x85, 0x23, 0xb8, 0xb4, 0x7a, 0xfc, 0x02, 0x36, 0x5b, 0x25, 0x55, 0x97, 0x31,
    0x2d, 0x5d, 0xfa, 0x98, 0xe3, 0x8a, 0x92, 0xae, 0x05, 0xdf, 0x29, 0x10, 0x67, 0x6c, 0xba, 0xc9,
    0xd3, 0x00, 0xe6, 0xcf, 0xe1, 0x9e, 0xa8, 0x2c, 0x63, 0x16, 0x01, 0x3f, 0x58, 0xe2, 0x89, 0xa9,
    0x0d, 0x38, 0x34, 0x1b, 0xab, 0x33, 0xff, 0xb0, 0xbb, 0x48, 0x0c, 0x5f, 0xb9, 0xb1, 0xcd, 0x2e,
    0xc5, 0xf3, 0xdb, 0x47, 0xe5, 0xa5, 0x9c, 0x77, 0x0a, 0xa6, 0x20, 0x68, 0xfe, 0x7f, 0xc1, 0xad,
];

fn mix_round(state: &mut [u16; 4], key: &[u16], j: &mut usize) {
    state[0] = state[0]
        .wrapping_add(key[*j])
        .wrapping_add(state[3] & state[2])
        .wrapping_add(!state[3] & state[1]);
    state[0] = state[0].rotate_left(1);
    *j += 1;

    state[1] = state[1]
        .wrapping_add(key[*j])
        .wrapping_add(state[0] & state[3])
        .wrapping_add(!state[0] & state[2]);
    state[1] = state[1].rotate_left(2);
    *j += 1;

    state[2] = state[2]
        .wrapping_add(key[*j])
        .wrapping_add(state[1] & state[0])
        .wrapping_add(!state[1] & state[3]);
    state[2] = state[2].rotate_left(3);
    *j += 1;

    state[3] = state[3]
        .wrapping_add(key[*j])
        .wrapping_add(state[2] & state[1])
        .wrapping_add(!state[2] & state[0]);
    state[3] = state[3].rotate_left(5);
    *j += 1;
}

fn mix_round_inv(state: &mut [u16; 4], key: &[u16], j: &mut usize) {
    // We subtract first because we will start with j = 64
    *j -= 1;
    state[3] = state[3].rotate_right(5);
    state[3] = state[3]
        .wrapping_sub(key[*j])
        .wrapping_sub(state[2] & state[1])
        .wrapping_sub(!state[2] & state[0]);

    *j -= 1;
    state[2] = state[2].rotate_right(3);
    state[2] = state[2]
        .wrapping_sub(key[*j])
        .wrapping_sub(state[1] & state[0])
        .wrapping_sub(!state[1] & state[3]);

    *j -= 1;
    state[1] = state[1].rotate_right(2);
    state[1] = state[1]
        .wrapping_sub(key[*j])
        .wrapping_sub(state[0] & state[3])
        .wrapping_sub(!state[0] & state[2]);

    *j -= 1;
    state[0] = state[0].rotate_right(1);
    state[0] = state[0]
        .wrapping_sub(key[*j])
        .wrapping_sub(state[3] & state[2])
        .wrapping_sub(!state[3] & state[1]);
}

fn mash_round(state: &mut [u16; 4], key: &[u16]) {
    state[0] = state[0].wrapping_add(key[state[3] as usize & 63]);
    state[1] = state[1].wrapping_add(key[state[0] as usize & 63]);
    state[2] = state[2].wrapping_add(key[state[1] as usize & 63]);
    state[3] = state[3].wrapping_add(key[state[2] as usize & 63]);
}

fn mash_round_inv(state: &mut [u16; 4], key: &[u16]) {
    state[3] = state[3].wrapping_sub(key[state[2] as usize & 63]);
    state[2] = state[2].wrapping_sub(key[state[1] as usize & 63]);
    state[1] = state[1].wrapping_sub(key[state[0] as usize & 63]);
    state[0] = state[0].wrapping_sub(key[state[3] as usize & 63]);
}

pub struct Rc2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub effective_bits: usize,
    pub round_keys: [u16; 64],
    pub mode: BCMode,
    pub padding: BCPadding,
}

crate::block_cipher_builders! {Rc2, u64}

impl Default for Rc2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            effective_bits: 64,
            round_keys: [0; 64],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Rc2 {
    // Must be invoked before the key schedule to change it
    pub fn with_effective_bits(mut self, effective_bits: usize) -> Self {
        self.effective_bits = effective_bits;
        self
    }

    pub fn effective_bytes(&self) -> usize {
        (self.effective_bits + 7) / 8
    }

    pub fn mask(&self) -> usize {
        let t8 = self.effective_bytes();
        255 % 2_usize.pow((8 + self.effective_bits - 8 * t8) as u32)
    }

    pub fn ksa(&mut self, bytes: &[u8]) {
        let t = bytes.len();
        let t8 = self.effective_bytes();
        let tm = self.mask();
        let mut l = [0_u8; 128];
        overwrite_bytes(&mut l[0..t], &bytes);
        for i in t..128 {
            l[i] = PI[l[i - 1].wrapping_add(l[i - t]) as usize]
        }
        l[128 - t8] = PI[l[128 - t8] as usize & tm];
        for i in (0..128 - t8).rev() {
            l[i] = PI[(l[i + 1] ^ l[i + t8]) as usize]
        }
        utils::byte_formatting::fill_u16s_le(&mut self.round_keys, &l);
    }

    pub fn with_key(mut self, bytes: &[u8]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for Rc2 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = utils::byte_formatting::make_u16s_le::<4>(bytes);

        let mut j = 0;
        // Five rounds
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);

        mash_round(&mut v, &self.round_keys);

        // Six rounds
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);

        mash_round(&mut v, &self.round_keys);

        // Five rounds
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);
        mix_round(&mut v, &self.round_keys, &mut j);

        utils::byte_formatting::u16s_to_bytes_le(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = utils::byte_formatting::make_u16s_le::<4>(bytes);

        let mut j = 64;
        // Five rounds
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);

        mash_round_inv(&mut v, &self.round_keys);

        // Six rounds
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);

        mash_round_inv(&mut v, &self.round_keys);

        // Five rounds
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);
        mix_round_inv(&mut v, &self.round_keys, &mut j);

        utils::byte_formatting::u16s_to_bytes_le(bytes, &v);
    }

    crate::block_cipher_getters!();
}

crate::impl_cipher_for_block_cipher!(Rc2, 8);

crate::test_block_cipher!(
    test_1, Rc2::default().with_key(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0xeb, 0xb7, 0x73, 0xf9, 0x93, 0x27, 0x8e, 0xff];

    test_2, Rc2::default().with_key(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
    [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    [0x27, 0x8b, 0x27, 0xe4, 0x2e, 0x2f, 0x0d, 0x49];

    test_3, Rc2::default().with_key(&[0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    [0x30, 0x64, 0x9e, 0xdf, 0x9b, 0xe7, 0xd2, 0xc2];

    test_4, Rc2::default().with_key(&[0x88]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x61, 0xa8, 0xa2, 0x44, 0xad, 0xac, 0xcc, 0xf0];

    test_5, Rc2::default().with_key(&[0x88, 0xbc, 0xa9, 0x0e, 0x90, 0x87, 0x5a, 0x7f, 0x0f, 0x79, 0xc3, 0x84, 0x62, 0x7b, 0xaf, 0xb2]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x1a, 0x80, 0x7d, 0x27, 0x2b, 0xbe, 0x5d, 0xb1];

    test_6, Rc2::default().with_effective_bits(128).with_key(&[0x88, 0xbc, 0xa9, 0x0e, 0x90, 0x87, 0x5a, 0x7f, 0x0f, 0x79, 0xc3, 0x84, 0x62, 0x7b, 0xaf, 0xb2]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x22, 0x69, 0x55, 0x2a, 0xb0, 0xf8, 0x5c, 0xa6];

    test_7, Rc2::default().with_effective_bits(129).with_key(&[0x88, 0xbc, 0xa9, 0x0e, 0x90, 0x87, 0x5a, 0x7f, 0x0f, 0x79, 0xc3, 0x84, 0x62, 0x7b, 0xaf, 0xb2, 0x16, 0xf8, 0x0a, 0x6f, 0x85, 0x92, 0x05, 0x84, 0xc4, 0x2f, 0xce, 0xb0, 0xbe, 0x25, 0x5d, 0xaf, 0x1e]),
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x5b, 0x78, 0xd3, 0xa4, 0x3d, 0xff, 0xf1, 0xf1];
);
