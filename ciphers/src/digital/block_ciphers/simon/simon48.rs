use super::select_z_bit;
use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use num::traits::ToBytes;
use utils::byte_formatting::ByteFormat;
use utils::other_word_sizes::{make_u24s_be, u24s_to_bytes_be, U24, U48};

macro_rules! simon48 {
    ($name:ident, $key_words:literal, $rounds:literal, $z_string:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: U48,
            pub subkeys: [U24; $rounds],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: Default::default(),
                    padding: Default::default(),
                    iv: Default::default(),
                    subkeys: [U24::default(); $rounds],
                }
            }
        }

        impl $name {
            pub fn input(mut self, input: utils::byte_formatting::ByteFormat) -> Self {
                self.input_format = input;
                self
            }

            pub fn output(mut self, output: utils::byte_formatting::ByteFormat) -> Self {
                self.output_format = output;
                self
            }

            pub fn padding(
                mut self,
                padding: crate::digital::block_ciphers::block_cipher::BCPadding,
            ) -> Self {
                self.padding = padding;
                self
            }

            pub fn mode(
                mut self,
                mode: crate::digital::block_ciphers::block_cipher::BCMode,
            ) -> Self {
                self.mode = mode;
                self
            }

            pub fn iv(mut self, iv: u64) -> Self {
                self.iv = U48::new(iv);
                self
            }

            pub fn ksa(&mut self, bytes: [u8; $key_words * 3]) {
                let key = make_u24s_be::<$key_words>(&bytes);
                self.generate_subkeys(key);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 3]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_32(&mut self, key: [u32; $key_words]) {
                self.generate_subkeys(key.map(|w| U24::new(w)));
            }

            pub fn with_key_32(mut self, key: [u32; $key_words]) -> Self {
                self.ksa_32(key);
                self
            }

            pub fn generate_subkeys(&mut self, key: [U24; $key_words]) {
                let mut subkeys = [U24::new(0); $rounds as usize];

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
                        ^ U24::new(!3)
                        ^ t
                        ^ U24::new(select_z_bit($z_string, bit_idx) as u32);
                }

                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<6> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys {
                    let t = y;
                    // L_i+1 = R_i
                    y = x;

                    // R_i+1 = L_i xor f(R_i)
                    x = t ^ super::round!(y, k);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    let t = x;
                    // L_i+1 = R_i
                    x = y;

                    // R_i+1 = L_i xor f(R_i)
                    y = t ^ super::round!(y, k);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

simon48!(Simon48_72, 3, 36, 0);
crate::impl_cipher_for_block_cipher!(Simon48_72, 6);
simon48!(Simon48_96, 4, 36, 1);
crate::impl_cipher_for_block_cipher!(Simon48_96, 6);

crate::test_block_cipher!(
    test_48_72, Simon48_72::default().with_key([0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x61, 0x20, 0x67, 0x6e, 0x69, 0x6c],
    [0xda, 0xe5, 0xac, 0x29, 0x2c, 0xac];
    test_48_96, Simon48_96::default().with_key([0x1a, 0x19, 0x18, 0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x72, 0x69, 0x63, 0x20, 0x64, 0x6e],
    [0x6e, 0x06, 0xa5, 0xac, 0xf1, 0x56];
);
