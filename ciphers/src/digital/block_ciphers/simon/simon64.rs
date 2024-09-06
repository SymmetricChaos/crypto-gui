use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

macro_rules! simon64 {
    ($name:ident, $key_words:literal, $rounds:literal, $z_string:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u64,
            pub subkeys: [u32; $rounds],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: Default::default(),
                    padding: Default::default(),
                    iv: Default::default(),
                    subkeys: [0; $rounds],
                }
            }
        }

        impl $name {
            pub fn ksa(&mut self, bytes: [u8; $key_words * 4]) {
                let mut key = [0; $key_words];
                fill_u32s_be(&mut key, &bytes);
                self.generate_subkeys(key);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 4]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_16(&mut self, key: [u32; $key_words]) {
                self.generate_subkeys(key);
            }

            pub fn with_key_16(mut self, key: [u32; $key_words]) -> Self {
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

            pub fn iv(mut self, iv: u64) -> Self {
                self.iv = iv;
                self
            }

            pub fn generate_subkeys(&mut self, key: [u32; $key_words]) {
                let mut subkeys = [0; $rounds as usize];

                // First four subkeys are just the key itself
                for i in 0..$key_words {
                    subkeys[$key_words - i - 1] = key[i]
                }

                for i in $key_words..$rounds as usize {
                    let mut t = subkeys[i - 1].rotate_right(3);
                    if $key_words == 4 {
                        t ^= subkeys[i - 3];
                    }
                    t ^= t.rotate_right(1);
                    let bit_idx = (i - $key_words) % 62;

                    subkeys[i] = (subkeys[i - $key_words])
                        ^ !3
                        ^ t
                        ^ (select_z_bit($z_string, bit_idx) as u32);
                }

                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<8> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let mut v = [0u32; 2];
                fill_u32s_be(&mut v, bytes);
                let [mut x, mut y] = v;

                for k in self.subkeys {
                    let t = y;
                    // L_i+1 = R_i
                    y = x;

                    // R_i+1 = L_i xor f(R_i)
                    x = t ^ super::round!(x, k);
                }

                u32s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let mut v = [0u32; 2];
                fill_u32s_be(&mut v, bytes);
                let [mut x, mut y] = v;

                for k in self.subkeys.into_iter().rev() {
                    let t = x;
                    // L_i+1 = R_i
                    x = y;

                    // R_i+1 = L_i xor f(R_i)
                    y = t ^ super::round!(y, k);
                }

                u32s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

simon64!(Simon64_96, 3, 42, 2);
crate::impl_cipher_for_block_cipher!(Simon64_96, 8);
simon64!(Simon64_128, 4, 44, 3);
crate::impl_cipher_for_block_cipher!(Simon64_128, 8);

crate::test_block_cipher!(
    Simon64_96::default().with_key([0x13, 0x12, 0x11, 0x10, 0x0b, 0x0a, 0x09, 0x08, 0x03, 0x02, 0x01, 0x00]), test_64_96,
    [0x6f, 0x72, 0x20, 0x67, 0x6e, 0x69, 0x6c, 0x63],
    [0x5c, 0xa2, 0xe2, 0x7f, 0x11, 0x1a, 0x8f, 0xc8];
    Simon64_128::default().with_key([0x1b, 0x1a, 0x19, 0x18, 0x13, 0x12, 0x11, 0x10, 0x0b, 0x0a, 0x09, 0x08, 0x03, 0x02, 0x01, 0x00]), test_64_128,
    [0x65, 0x6b, 0x69, 0x6c, 0x20, 0x64, 0x6e, 0x75],
    [0x44, 0xc8, 0xfc, 0x20, 0xb9, 0xdf, 0xa0, 0x7a];
);
