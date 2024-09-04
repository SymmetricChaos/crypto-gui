use utils::byte_formatting::{fill_u16s_be, u16s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Speck32_64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u32,
    pub subkeys: [u16; Self::ROUNDS as usize],
}

impl Default for Speck32_64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
            iv: Default::default(),
            subkeys: [0; Self::ROUNDS as usize],
        }
    }
}

impl Speck32_64 {
    const ROUNDS: u16 = 22;

    pub fn ksa(&mut self, bytes: [u8; 8]) {
        let mut key = [0u16; 4];
        fill_u16s_be(&mut key, &bytes);
        self.generate_subkeys(key)
    }

    pub fn with_key(mut self, bytes: [u8; 8]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn ksa_16(&mut self, key: [u16; 4]) {
        self.generate_subkeys(key)
    }

    pub fn with_key_16(mut self, key: [u16; 4]) -> Self {
        self.ksa_16(key);
        self
    }

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn padding(mut self, padding: BCPadding) -> Self {
        self.padding = padding;
        self
    }

    pub fn mode(mut self, mode: BCMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn iv(mut self, iv: u32) -> Self {
        self.iv = iv;
        self
    }

    // For encryption this could be done on the fly for each round
    pub fn generate_subkeys(&mut self, key: [u16; 4]) {
        let mut subkeys = [0; Self::ROUNDS as usize];
        let [mut a, mut b, mut c, mut d] = key;
        for i in 0..Self::ROUNDS {
            subkeys[i as usize] = d;
            let mut t = c;
            super::enc!(t, d, i, 7, 2);
            c = b;
            b = a;
            a = t;
        }
        self.subkeys = subkeys;
    }
}

impl BlockCipher<4> for Speck32_64 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        for k in self.subkeys {
            super::enc!(x, y, k, 7, 2);
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        for k in self.subkeys.into_iter().rev() {
            super::dec!(x, y, k, 7, 2);
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }
}

crate::impl_cipher_for_block_cipher!(Speck32_64, 4);

crate::test_block_cipher!(
    Speck32_64::default().with_key([0x19, 0x18, 0x11, 0x10, 0x09, 0x08, 0x01, 0x00]), test_32_64,
    [0x65, 0x74, 0x69, 0x4c],
    [0xa8, 0x68, 0x42, 0xf2];
);
