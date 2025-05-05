use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};
use num::traits::{ToBytes, WrappingAdd, WrappingSub};
use utils::byte_formatting::ByteFormat;
use utils::other_word_sizes::{make_u24s_be, u24s_to_bytes_be, U24, U48};

macro_rules! speck48 {
    ($name:ident, $key_words:literal, $rounds:literal) => {
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
                    subkeys: [U24::new(0); $rounds],
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
                self.generate_subkeys(key)
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

            // For encryption this can be done on the fly for each round
            pub fn generate_subkeys(&mut self, key: [U24; $key_words]) {
                let mut subkeys = [U24::new(0); $rounds as usize];
                let mut k = key;
                // let [mut a, mut b, mut c, mut d] = self.key;
                for i in 0..$rounds {
                    subkeys[i as usize] = k[$key_words - 1];
                    let mut tc = k[$key_words - 2];
                    let mut td = k[$key_words - 1];
                    tc = tc.rotate_right(8);
                    tc = tc.wrapping_add(&td);
                    tc ^= U24::new(i);
                    td = td.rotate_left(3);
                    td ^= tc;
                    k[0..$key_words - 1].rotate_right(1);
                    k[0] = tc;
                    k[$key_words - 1] = td;
                }
                self.subkeys = subkeys;
            }
        }

        impl BlockCipher<6> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys {
                    x = x.rotate_right(8);
                    x = x.wrapping_add(&y);
                    x ^= k;
                    y = y.rotate_left(3);
                    y ^= x;
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let [mut x, mut y] = make_u24s_be::<2>(bytes);

                for k in self.subkeys.into_iter().rev() {
                    y ^= x;
                    y = y.rotate_right(3);
                    x ^= k;
                    x = x.wrapping_sub(&y);
                    x = x.rotate_left(8);
                }

                u24s_to_bytes_be(bytes, &[x, y]);
            }

            crate::block_cipher_getters!();
        }
    };
}

speck48!(Speck48_72, 3, 22);
crate::impl_cipher_for_block_cipher!(Speck48_72, 6);
speck48!(Speck48_96, 4, 23);
crate::impl_cipher_for_block_cipher!(Speck48_96, 6);

crate::test_block_cipher!(
    test_64_96, Speck48_72::default().with_key([0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x20, 0x79, 0x6c, 0x6c, 0x61, 0x72],
    [0xc0, 0x49, 0xa5, 0x38, 0x5a, 0xdc];
    test_64_128, Speck48_96::default().with_key([0x1a, 0x19, 0x18, 0x12, 0x11, 0x10, 0x0a, 0x09, 0x08, 0x02, 0x01, 0x00]),
    [0x6d, 0x20, 0x73, 0x69, 0x68, 0x74],
    [0x73, 0x5e, 0x10, 0xb6, 0x44, 0x5d];
);
