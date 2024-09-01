use utils::byte_formatting::{fill_u64s_be, u64s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

macro_rules! speck128 {
    ($name:ident, $key_words:literal, $rounds:literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u128,
            key: [u64; $key_words],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: Default::default(),
                    padding: Default::default(),
                    iv: Default::default(),
                    key: [0; $key_words],
                }
            }
        }

        impl $name {
            pub fn ksa(&mut self, bytes: [u8; $key_words * 8]) {
                fill_u64s_be(&mut self.key, &bytes);
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 8]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_64(&mut self, key: [u64; $key_words]) {
                self.key = key;
            }

            pub fn with_key_64(mut self, key: [u64; $key_words]) -> Self {
                self.ksa_64(key);
                self
            }

            // For encryption this can be done on the fly for each round
            pub fn generate_subkeys(&self) -> [u64; $rounds as usize] {
                let mut subkeys = [0; $rounds as usize];
                let mut k = self.key;
                // let [mut a, mut b, mut c, mut d] = self.key;
                for i in 0..$rounds {
                    subkeys[i as usize] = k[$key_words - 1];
                    let mut tc = k[$key_words - 2];
                    let mut td = k[$key_words - 1];
                    super::enc!(tc, td, i, 8, 3);
                    k[0..$key_words - 1].rotate_right(1);
                    k[0] = tc;
                    k[$key_words - 1] = td;
                }
                subkeys
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let mut v = [0u64; 2];
                fill_u64s_be(&mut v, bytes);
                let [mut x, mut y] = v;

                let subkeys = self.generate_subkeys();

                for k in subkeys {
                    super::enc!(x, y, k, 8, 3);
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
                    super::dec!(x, y, k, 8, 3);
                }

                u64s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

speck128!(Speck128_128, 2, 32);
crate::impl_cipher_for_block_cipher!(Speck128_128, 16);
speck128!(Speck128_192, 3, 33);
crate::impl_cipher_for_block_cipher!(Speck128_192, 16);
speck128!(Speck128_256, 4, 34);
crate::impl_cipher_for_block_cipher!(Speck128_256, 16);

crate::test_block_cipher!(
    Speck128_128::default().with_key([0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]), test_128_128,
    [0x6c, 0x61, 0x76, 0x69, 0x75, 0x71, 0x65, 0x20, 0x74, 0x69, 0x20, 0x65, 0x64, 0x61, 0x6d, 0x20],
    [0xa6, 0x5d, 0x98, 0x51, 0x79, 0x78, 0x32, 0x65, 0x78, 0x60, 0xfe, 0xdf, 0x5c, 0x57, 0x0d, 0x18];
    Speck128_192::default().with_key([0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]), test_128_196,
    [0x72, 0x61, 0x48, 0x20, 0x66, 0x65, 0x69, 0x68, 0x43, 0x20, 0x6f, 0x74, 0x20, 0x74, 0x6e, 0x65],
    [0x1b, 0xe4, 0xcf, 0x3a, 0x13, 0x13, 0x55, 0x66, 0xf9, 0xbc, 0x18, 0x5d, 0xe0, 0x3c, 0x18, 0x86];
    Speck128_256::default().with_key([0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x000]), test_128_256,
    [0x65, 0x73, 0x6f, 0x68, 0x74, 0x20, 0x6e, 0x49, 0x20, 0x2e, 0x72, 0x65, 0x6e, 0x6f, 0x6f, 0x70],
    [0x41, 0x09, 0x01, 0x04, 0x05, 0xc0, 0xf5, 0x3e, 0x4e, 0xee, 0xb4, 0x8d, 0x9c, 0x18, 0x8f, 0x43];
);
