use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, ByteFormat};

use crate::digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher};

macro_rules! speck64 {
    ($name:ident, $key_words:literal, $rounds:literal) => {
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

        crate::block_cipher_builders! {$name}

        impl $name {
            pub fn ksa(&mut self, bytes: [u8; $key_words * 4]) {
                let mut key = [0u32; $key_words];
                fill_u32s_be(&mut key, &bytes);
                self.generate_subkeys(key)
            }

            pub fn with_key(mut self, bytes: [u8; $key_words * 4]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn ksa_32(&mut self, key: [u32; $key_words]) {
                self.generate_subkeys(key)
            }

            pub fn with_key_32(mut self, key: [u32; $key_words]) -> Self {
                self.ksa_32(key);
                self
            }

            pub fn iv(mut self, iv: u64) -> Self {
                self.iv = iv;
                self
            }

            // For encryption this can be done on the fly for each round
            pub fn generate_subkeys(&mut self, key: [u32; $key_words]) {
                let mut subkeys = [0; $rounds as usize];
                let mut k = key;
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
                    super::enc!(x, y, k, 8, 3);
                }

                u32s_to_bytes_be(bytes, &[x, y]);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                // Make mutable variables from the working vector
                let mut v = [0u32; 2];
                fill_u32s_be(&mut v, bytes);
                let [mut x, mut y] = v;

                for k in self.subkeys.into_iter().rev() {
                    super::dec!(x, y, k, 8, 3);
                }

                u32s_to_bytes_be(bytes, &[x, y]);
            }
        }
    };
}

speck64!(Speck64_96, 3, 26);
crate::impl_cipher_for_block_cipher!(Speck64_96, 8);
speck64!(Speck64_128, 4, 27);
crate::impl_cipher_for_block_cipher!(Speck64_128, 8);

crate::test_block_cipher!(
    Speck64_96::default().with_key([0x13, 0x12, 0x11, 0x10, 0x0b, 0x0a, 0x09, 0x08, 0x03, 0x02, 0x01, 0x00]), test_64_96,
    [0x74, 0x61, 0x46, 0x20, 0x73, 0x6e, 0x61, 0x65],
    [0x9f, 0x79, 0x52, 0xec, 0x41, 0x75, 0x94, 0x6c];
    Speck64_128::default().with_key([0x1b, 0x1a, 0x19, 0x18, 0x13, 0x12, 0x11, 0x10, 0x0b, 0x0a, 0x09, 0x08, 0x03, 0x02, 0x01, 0x00]), test_64_128,
    [0x3b, 0x72, 0x65, 0x74, 0x74, 0x75, 0x43, 0x2d],
    [0x8c, 0x6f, 0xa5, 0x48, 0x45, 0x4e, 0x02, 0x8b];
);
