use utils::byte_formatting::{fill_u16s_be, u16s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

const J: usize = 0;
const KEY_WORDS: usize = 4;
const ROUNDS: usize = 32;

pub struct Simon32_64 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u32,
    pub subkeys: [u16; ROUNDS],
}

impl Default for Simon32_64 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            padding: Default::default(),
            iv: Default::default(),
            subkeys: [0; ROUNDS],
        }
    }
}

impl Simon32_64 {
    pub fn ksa(&mut self, bytes: [u8; KEY_WORDS * 2]) {
        let mut key = [0; KEY_WORDS];
        fill_u16s_be(&mut key, &bytes);
        self.generate_subkeys(key);
    }

    pub fn with_key(mut self, bytes: [u8; KEY_WORDS * 2]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn ksa_16(&mut self, key: [u16; KEY_WORDS]) {
        self.generate_subkeys(key);
    }

    pub fn with_key_16(mut self, key: [u16; KEY_WORDS]) -> Self {
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

    pub fn generate_subkeys(&mut self, key: [u16; KEY_WORDS]) {
        let mut subkeys = [0; ROUNDS as usize];

        // First four subkeys are just the key itself
        for i in 0..KEY_WORDS {
            subkeys[KEY_WORDS - i - 1] = key[i]
        }

        for i in KEY_WORDS..ROUNDS as usize {
            let mut t = subkeys[i - 1].rotate_right(3);
            if KEY_WORDS == 4 {
                t ^= subkeys[i - 3];
            }
            t ^= t.rotate_right(1);
            let bit_idx = (i - KEY_WORDS) % 62;

            subkeys[i] = (subkeys[i - KEY_WORDS]) ^ !3 ^ t ^ (select_z_bit(J, bit_idx) as u16);
        }

        self.subkeys = subkeys;
    }
}

impl BlockCipher<4> for Simon32_64 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        for k in self.subkeys {
            let t = y;
            // L_i+1 = R_i
            y = x;

            // R_i+1 = L_i xor f(R_i)
            x = t ^ super::round!(x, k);
        }

        u16s_to_bytes_be(bytes, &[x, y]);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        // Make mutable variables from the working vector
        let mut v = [0u16; 2];
        fill_u16s_be(&mut v, bytes);
        let [mut x, mut y] = v;

        for k in self.subkeys.into_iter().rev() {
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

#[cfg(test)]
mod simon_tests {

    use super::*;

    #[test]
    fn simon_key_expansion() {
        let cipher =
            Simon32_64::default().with_key([0x19, 0x18, 0x11, 0x10, 0x09, 0x08, 0x01, 0x00]);
        assert_eq!(
            [0x0100, 0x0908, 0x1110, 0x1918, 0x71C3, 0xB649, 0x56D4, 0xE070, 0xF15A, 0xC535],
            &cipher.subkeys[0..10]
        );
    }
}

crate::test_block_cipher!(
    Simon32_64::default().with_key([0x19, 0x18, 0x11, 0x10, 0x09, 0x08, 0x01, 0x00]), test_32_64,
    [0x65, 0x65, 0x68, 0x77],
    [0xc6, 0x9b, 0xe9, 0xbb];

);
