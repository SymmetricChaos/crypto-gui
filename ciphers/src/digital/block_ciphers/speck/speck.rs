use utils::byte_formatting::{
    fill_u64s_be, fill_u64s_le, u64s_to_bytes_be, u64s_to_bytes_le, ByteFormat,
};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

pub fn enc(x: &mut u64, y: &mut u64, k: u64) {
    *x = x.rotate_right(8);
    *x = x.wrapping_add(*y);
    *x ^= k;
    *y = y.rotate_left(3);
    *y ^= *x;
}

pub fn dec(x: &mut u64, y: &mut u64, k: u64) {
    *y ^= *x;
    *y = y.rotate_right(3);
    *x ^= k;
    *x = x.wrapping_sub(*y);
    *x = x.rotate_left(8);
}

pub struct Speck128_128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u128,
    key: [u64; 2],
}

impl Default for Speck128_128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
            iv: Default::default(),
            key: [0, 0],
        }
    }
}

impl Speck128_128 {
    const ROUNDS: u64 = 32;

    pub fn ksa(&mut self, bytes: [u8; 16]) {
        fill_u64s_be(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn ksa_64(&mut self, key: [u64; 2]) {
        self.key = key;
    }

    pub fn with_key_64(mut self, key: [u64; 2]) -> Self {
        self.ksa_64(key);
        self
    }

    // For encryption this can be done on the fly for each round
    pub fn generate_subkeys(&self) -> [u64; Self::ROUNDS as usize] {
        let mut subkeys = [0; Self::ROUNDS as usize];
        let [mut a, mut b] = self.key;
        for i in 0..Self::ROUNDS {
            subkeys[i as usize] = b;
            enc(&mut a, &mut b, i);
        }
        subkeys
    }
}

impl BlockCipher<16> for Speck128_128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u64; 2];
        fill_u64s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        let subkeys = self.generate_subkeys();

        for k in subkeys {
            enc(&mut x, &mut y, k);
        }

        u64s_to_bytes_be(bytes, &[x, y]);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u64; 2];
        fill_u64s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        let subkeys = self.generate_subkeys();

        for k in subkeys.into_iter().rev() {
            dec(&mut x, &mut y, k);
        }

        u64s_to_bytes_be(bytes, &[x, y]);
    }
}

crate::impl_cipher_for_block_cipher!(Speck128_128, 16);

crate::test_block_cipher!(
    Speck128_128::default().with_key([0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]), test_128_128,
    [0x6c, 0x61, 0x76, 0x69, 0x75, 0x71, 0x65, 0x20, 0x74, 0x69, 0x20, 0x65, 0x64, 0x61, 0x6d, 0x20],
    [0xa6, 0x5d, 0x98, 0x51, 0x79, 0x78, 0x32, 0x65, 0x78, 0x60, 0xfe, 0xdf, 0x5c, 0x57, 0x0d, 0x18];
);
