use utils::byte_formatting::{make_u64s_be, u64s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

use super::select_z_bit;

macro_rules! simon128 {
    ($name:ident, $key_words:literal, $rounds:literal, $z_string:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u128,
            pub subkeys: [u64; $rounds],
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

        crate::block_cipher_builders! {$name, u128}

        impl $name {
            pub fn ksa(&mut self, bytes: [u8; $key_words * 8]) {
                let key = make_u64s_be::<$key_words>(&bytes);
                self.generate_subkeys(key);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 8]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_64(&mut self, key: [u64; $key_words]) {
                self.generate_subkeys(key);
            }

            pub fn with_key_64(mut self, key: [u64; $key_words]) -> Self {
                self.ksa_64(key);
                self
            }

            pub fn generate_subkeys(&mut self, key: [u64; $key_words]) {
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
                        ^ (select_z_bit($z_string, bit_idx) as u64);
                }

                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u64s_be::<2>(bytes);

                for k in self.subkeys {
                    let t = y;
                    // L_i+1 = R_i
                    y = x;

                    // R_i+1 = L_i xor f(R_i)
                    x = t ^ super::round!(x, k);
                }

                u64s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u64s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    let t = x;
                    // L_i+1 = R_i
                    x = y;

                    // R_i+1 = L_i xor f(R_i)
                    y = t ^ super::round!(y, k);
                }

                u64s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

simon128!(Simon128_128, 2, 68, 2);
crate::impl_cipher_for_block_cipher!(Simon128_128, 16);
simon128!(Simon128_192, 3, 69, 3);
crate::impl_cipher_for_block_cipher!(Simon128_192, 16);
simon128!(Simon128_256, 4, 72, 4);
crate::impl_cipher_for_block_cipher!(Simon128_256, 16);

crate::test_block_cipher!(
    test_128_128, Simon128_128::default().with_key([0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x63, 0x73, 0x65, 0x64, 0x20, 0x73, 0x72, 0x65, 0x6c, 0x6c, 0x65, 0x76, 0x61, 0x72, 0x74, 0x20],
    [0x49, 0x68, 0x1b, 0x1e, 0x1e, 0x54, 0xfe, 0x3f, 0x65, 0xaa, 0x83, 0x2a, 0xf8, 0x4e, 0x0b, 0xbc];
    test_128_196, Simon128_192::default().with_key([0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x20, 0x65, 0x72, 0x65, 0x68, 0x74, 0x20, 0x6e, 0x65, 0x68, 0x77, 0x20, 0x65, 0x62, 0x69, 0x72],
    [0xc4, 0xac, 0x61, 0xef, 0xfc, 0xdc, 0x0d, 0x4f, 0x6c, 0x9c, 0x8d, 0x6e, 0x25, 0x97, 0xb8, 0x5b];
    test_128_256, Simon128_256::default().with_key([0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]),
    [0x74, 0x20, 0x6e, 0x69, 0x20, 0x6d, 0x6f, 0x6f, 0x6d, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x69],
    [0x8d, 0x2b, 0x55, 0x79, 0xaf, 0xc8, 0xa3, 0xa0, 0x3b, 0xf7, 0x2a, 0x87, 0xef, 0xe7, 0xb8, 0x68];
);
