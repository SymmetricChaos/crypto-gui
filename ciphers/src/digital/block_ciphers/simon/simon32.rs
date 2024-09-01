use utils::byte_formatting::{fill_u16s_be, u16s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

pub struct Simon32_64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u32,
    key: [u16; 4],
}

impl Default for Simon32_64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
            iv: Default::default(),
            key: [0; 4],
        }
    }
}

impl Simon32_64 {
    const ROUNDS: u16 = 32;

    pub fn ksa(&mut self, bytes: [u8; 8]) {
        fill_u16s_be(&mut self.key, &bytes);
    }

    pub fn with_key(mut self, bytes: [u8; 8]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn ksa_16(&mut self, key: [u16; 4]) {
        self.key = key;
    }

    pub fn with_key_16(mut self, key: [u16; 4]) -> Self {
        self.ksa_16(key);
        self
    }

    // For encryption this can be done on the fly for each round
    pub fn generate_subkeys(&self) -> [u16; Self::ROUNDS as usize] {
        let mut subkeys = [0; Self::ROUNDS as usize];

        subkeys
    }
}

impl BlockCipher<4> for Simon32_64 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        let subkeys = self.generate_subkeys();

        for k in subkeys {
            let t = x;
            // L_i+1 = R_i
            x = y;

            // R_i+1 = L_i xor f(R_i)
            y = t ^ (y.rotate_left(1) & y.rotate_left(8)) ^ y.rotate_left(2) ^ k;
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        let subkeys = self.generate_subkeys();

        for k in subkeys.into_iter().rev() {
            let t = x;
            // L_i+1 = R_i
            x = y;

            // R_i+1 = L_i xor f(R_i)
            y = t ^ (y.rotate_left(1) & y.rotate_left(8)) ^ y.rotate_left(2) ^ k;
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }
}

crate::impl_cipher_for_block_cipher!(Simon32_64, 4);

// crate::test_block_cipher!(
//     Simon32_64::default().with_key([]), test_32_64,
//     [],
//     [];
// );
