use utils::byte_formatting::{fill_u16s_be, u16s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

const J: usize = 0;
const M: usize = 4;
const ROUNDS: u16 = 32;

pub struct Simon32_64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u32,
    key: [u16; M],
}

impl Default for Simon32_64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
            iv: Default::default(),
            key: [0; M],
        }
    }
}

impl Simon32_64 {
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

    pub fn generate_subkeys(&self) -> [u16; ROUNDS as usize] {
        let mut subkeys = [0; ROUNDS as usize];

        // First four subkeys are just the key itself
        for i in 0..M {
            subkeys[i] = self.key[i]
        }

        for i in M..ROUNDS as usize {
            let mut t = subkeys[i - 1].rotate_right(3);
            if M == 4 {
                t ^= subkeys[i - 3];
            }
            t ^= t.rotate_right(1);
            let bit_idx = (i - M) % 62;

            subkeys[i] = !(subkeys[i - M]) ^ t ^ (select_z_bit(J, bit_idx) as u16) ^ 3
        }

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
            y = t ^ super::round!(y, k);
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
            y = t ^ super::round!(y, k);
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }
}

crate::impl_cipher_for_block_cipher!(Simon32_64, 4);

crate::test_block_cipher!(
    Simon32_64::default().with_key([0x19, 0x18, 0x11, 0x10, 0x09, 0x08, 0x01, 0x00]), test_32_64,
    [0x65, 0x65, 0x68, 0x77],
    [0xc6, 0x9b, 0xe9, 0xbb];
);
