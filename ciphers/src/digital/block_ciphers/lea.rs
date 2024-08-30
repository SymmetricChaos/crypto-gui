use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

// Key schedule constants
const KSC: [u32; 8] = [
    0xc3efe9db, 0x44626b02, 0x79e27c8a, 0x78df30ec, 0x715ea49e, 0xc785da0a, 0xe04ef22a, 0xe5c40957,
];

macro_rules! lea_struct {
    ($name: ident, $key_words: literal, $num_rounds: literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u128,
            subkeys: [[u32; 6]; $num_rounds],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    iv: 0,
                    subkeys: [[0u32; 6]; $num_rounds],
                    mode: Default::default(),
                    padding: Default::default(),
                }
            }
        }

        impl $name {
            pub fn ksa(&mut self, key: [u32; $key_words]) {
                let mut key = key;
                if $key_words == 4 {
                    for i in 0..$num_rounds {
                        key[0] = key[0]
                            .wrapping_add(KSC[i % 4].rotate_left((i + 0) as u32))
                            .rotate_left(1);
                        key[1] = key[1]
                            .wrapping_add(KSC[i % 4].rotate_left((i + 1) as u32))
                            .rotate_left(3);
                        key[2] = key[2]
                            .wrapping_add(KSC[i % 4].rotate_left((i + 2) as u32))
                            .rotate_left(6);
                        key[3] = key[3]
                            .wrapping_add(KSC[i % 4].rotate_left((i + 3) as u32))
                            .rotate_left(11);
                        self.subkeys[i] = [key[0], key[1], key[2], key[1], key[3], key[1]];
                    }
                }
                if $key_words == 6 {
                    for i in 0..$num_rounds {
                        key[0] = key[0]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 0) as u32))
                            .rotate_left(1);
                        key[1] = key[1]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 1) as u32))
                            .rotate_left(3);
                        key[2] = key[2]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 2) as u32))
                            .rotate_left(6);
                        key[3] = key[3]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 3) as u32))
                            .rotate_left(11);
                        key[4] = key[4]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 4) as u32))
                            .rotate_left(13);
                        key[5] = key[5]
                            .wrapping_add(KSC[i % 6].rotate_left((i + 5) as u32))
                            .rotate_left(17);
                        self.subkeys[i] = [key[0], key[1], key[2], key[3], key[4], key[5]];
                    }
                }
                if $key_words == 8 {
                    for i in 0..$num_rounds {
                        key[(6 * i + 0) % 8] = key[(6 * i + 0) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 0) as u32))
                            .rotate_left(1);
                        key[(6 * i + 1) % 8] = key[(6 * i + 1) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 1) as u32))
                            .rotate_left(3);
                        key[(6 * i + 2) % 8] = key[(6 * i + 2) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 2) as u32))
                            .rotate_left(6);
                        key[(6 * i + 3) % 8] = key[(6 * i + 3) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 3) as u32))
                            .rotate_left(11);
                        key[(6 * i + 4) % 8] = key[(6 * i + 4) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 4) as u32))
                            .rotate_left(13);
                        key[(6 * i + 5) % 8] = key[(6 * i + 5) % 8]
                            .wrapping_add(KSC[i % 8].rotate_left((i + 5) as u32))
                            .rotate_left(17);
                        self.subkeys[i] = [
                            key[(6 * i + 0) % 8],
                            key[(6 * i + 1) % 8],
                            key[(6 * i + 2) % 8],
                            key[(6 * i + 3) % 8],
                            key[(6 * i + 4) % 8],
                            key[(6 * i + 5) % 8],
                        ];
                    }
                }
            }

            pub fn with_ksa(mut self, key: [u32; $key_words]) -> Self {
                self.ksa(key);
                self
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                let mut v = [0u32; 4];
                for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
                    *elem = u32::from_be_bytes(chunk.try_into().unwrap());
                }
                for s in self.subkeys {
                    let t0 = (v[0] ^ s[0]).wrapping_add(v[1] ^ s[1]).rotate_left(9);
                    let t1 = (v[1] ^ s[2]).wrapping_add(v[2] ^ s[3]).rotate_right(5);
                    let t2 = (v[2] ^ s[4]).wrapping_add(v[3] ^ s[5]).rotate_right(3);
                    v[3] = v[0];
                    v[0] = t0;
                    v[1] = t1;
                    v[2] = t2;
                }
                utils::byte_formatting::overwrite_bytes(
                    bytes,
                    &utils::byte_formatting::u32_4_to_u8_16(v),
                );
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut v = [0u32; 4];
                for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
                    *elem = u32::from_be_bytes(chunk.try_into().unwrap());
                }
                for s in self.subkeys.into_iter().rev() {
                    let t1 = v[0].rotate_right(9).wrapping_sub(v[0] ^ s[0]) ^ s[1];
                    let t2 = v[1].rotate_left(5).wrapping_sub(v[1] ^ s[2]) ^ s[3];
                    let t3 = v[2].rotate_left(3).wrapping_sub(v[2] ^ s[4]) ^ s[5];
                    v[0] = v[3];
                    v[1] = t1;
                    v[2] = t2;
                    v[3] = t3;
                }
                utils::byte_formatting::overwrite_bytes(
                    bytes,
                    &utils::byte_formatting::u32_4_to_u8_16(v),
                );
            }
        }
    };
}

lea_struct!(Lea128, 4, 24);
crate::impl_cipher_for_block_cipher!(Lea128, 16);

lea_struct!(Lea192, 6, 28);
crate::impl_cipher_for_block_cipher!(Lea192, 16);

lea_struct!(Lea256, 8, 32);
crate::impl_cipher_for_block_cipher!(Lea256, 16);

crate::test_block_cipher!(

    // Lea128::default().with_ksa([0x0f1e2d3c_u32, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0].map(|n| n.swap_bytes())), test_128,
    Lea128::default().with_ksa([0x0f1e2d3c_u32, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0]), test_128,
    [0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x1a,0x1b,0x1c,0x1d,0x1e,0x1f],
    [0x9f,0xc8,0x4e,0x35,0x28,0xc6,0xc6,0x18,0x55,0x32,0xc7,0xa7,0x04,0x64,0x8b,0xfd];

    // Lea192::default().with_ksa([0x0f1e2d3c, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0, 0xf0e1d2c3, 0xb4a59687].map(|n| n.swap_bytes())), test_192,
    Lea192::default().with_ksa([0x0f1e2d3c, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0, 0xf0e1d2c3, 0xb4a59687]), test_192,
    [0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2a,0x2b,0x2c,0x2d,0x2e,0x2f],
    [0x6f,0xb9,0x5e,0x32,0x5a,0xad,0x1b,0x87,0x8c,0xdc,0xf5,0x35,0x76,0x74,0xc6,0xf2];

    // Lea256::default().with_ksa([0x0f1e2d3c,0x4b5a6978,0x8796a5b4,0xc3d2e1f0,0xf0e1d2c3,0xb4a59687,0x78695a4b,0x3c2d1e0f].map(|n| n.swap_bytes())), test_256,
    Lea256::default().with_ksa([0x0f1e2d3c,0x4b5a6978,0x8796a5b4,0xc3d2e1f0,0xf0e1d2c3,0xb4a59687,0x78695a4b,0x3c2d1e0f]), test_256,
    [0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x3a,0x3b,0x3c,0x3d,0x3e,0x3f],
    [0xd6,0x51,0xaf,0xf6,0x47,0xb1,0x89,0xc1,0x3a,0x89,0x00,0xca,0x27,0xf9,0xe1,0x97];
);
