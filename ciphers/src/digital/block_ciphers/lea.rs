use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

// Key schedule constants
const KSC: [u32; 8] = [
    0xc3efe9db, 0x44626b02, 0x79e27c8a, 0x78df30ec, 0x715ea49e, 0xc785da0a, 0xe04ef22a, 0xe5c40957,
];

pub fn u32_4_to_u8_16(s: [u32; 4]) -> [u8; 16] {
    let a = s[0].to_le_bytes();
    let b = s[1].to_le_bytes();
    let c = s[2].to_le_bytes();
    let d = s[3].to_le_bytes();
    let mut out = [0; 16];
    for i in 0..4 {
        out[i] = a[i];
        out[i + 4] = b[i];
        out[i + 8] = c[i];
        out[i + 12] = d[i];
    }
    out
}

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
                let mut key = key.map(|n| n.to_be());
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
                // println!("{:08x?}", self.subkeys[0]);
                // println!("{:08x?}", self.subkeys[$num_rounds - 1]);
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
                    *elem = u32::from_le_bytes(chunk.try_into().unwrap());
                }
                for s in self.subkeys {
                    // println!("{:08x?}", v);
                    let t = v;
                    v[0] = (t[0] ^ s[0]).wrapping_add(t[1] ^ s[1]).rotate_left(9);
                    v[1] = (t[1] ^ s[2]).wrapping_add(t[2] ^ s[3]).rotate_right(5);
                    v[2] = (t[2] ^ s[4]).wrapping_add(t[3] ^ s[5]).rotate_right(3);
                    v[3] = t[0];
                }
                utils::byte_formatting::overwrite_bytes(bytes, &u32_4_to_u8_16(v));
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut v = [0u32; 4];
                for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
                    *elem = u32::from_le_bytes(chunk.try_into().unwrap());
                }
                for s in self.subkeys.into_iter().rev() {
                    let t = v;
                    v[0] = t[3];
                    v[1] = t[0].rotate_right(9).wrapping_sub(v[0] ^ s[0]) ^ s[1];
                    v[2] = t[1].rotate_left(5).wrapping_sub(v[1] ^ s[2]) ^ s[3];
                    v[3] = t[2].rotate_left(3).wrapping_sub(v[2] ^ s[4]) ^ s[5];
                }
                utils::byte_formatting::overwrite_bytes(bytes, &u32_4_to_u8_16(v));
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

#[cfg(test)]
mod lea_tests {

    use super::*;
    #[test]
    fn ksa() {
        let mut cipher = Lea128::default();
        cipher.ksa([0x0f1e2d3c_u32, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0].map(|n| n.to_be()));
        let test_subkeys: [[u32; 6]; 24] = [
            [
                0x003a0fd4, 0x02497010, 0x194f7db1, 0x02497010, 0x090d0883, 0x02497010,
            ],
            [
                0x11fdcbb1, 0x9e98e0c8, 0x18b570cf, 0x9e98e0c8, 0x9dc53a79, 0x9e98e0c8,
            ],
            [
                0xf30f7bb5, 0x6d6628db, 0xb74e5dad, 0x6d6628db, 0xa65e46d0, 0x6d6628db,
            ],
            [
                0x74120631, 0xdac9bd17, 0xcd1ecf34, 0xdac9bd17, 0x540f76f1, 0xdac9bd17,
            ],
            [
                0x662147db, 0xc637c47a, 0x46518932, 0xc637c47a, 0x23269260, 0xc637c47a,
            ],
            [
                0xe4dd5047, 0xf694285e, 0xe1c2951d, 0xf694285e, 0x8ca5242c, 0xf694285e,
            ],
            [
                0xbaf8e5ca, 0x3e936cd7, 0x0fc7e5b1, 0x3e936cd7, 0xf1c8fa8c, 0x3e936cd7,
            ],
            [
                0x5522b80c, 0xee22ca78, 0x8a6fa8b3, 0xee22ca78, 0x65637b74, 0xee22ca78,
            ],
            [
                0x8a19279e, 0x6fb40ffe, 0x85c5f092, 0x6fb40ffe, 0x92cc9f25, 0x6fb40ffe,
            ],
            [
                0x9dde584c, 0xcb00c87f, 0x4780ad66, 0xcb00c87f, 0xe61b5dcb, 0xcb00c87f,
            ],
            [
                0x4fa10466, 0xf728e276, 0xd255411b, 0xf728e276, 0x656839ad, 0xf728e276,
            ],
            [
                0x9250d058, 0x51bd501f, 0x1cb40dae, 0x51bd501f, 0x1abf218d, 0x51bd501f,
            ],
            [
                0x21dd192d, 0x77c644e2, 0xcabfaa45, 0x77c644e2, 0x681c207d, 0x77c644e2,
            ],
            [
                0xde7ac372, 0x9436afd0, 0x10331d80, 0x9436afd0, 0xf326fe98, 0x9436afd0,
            ],
            [
                0xfb3ac3d4, 0x93df660e, 0x2f65d8a3, 0x93df660e, 0xdf92e761, 0x93df660e,
            ],
            [
                0x27620087, 0x265ef76e, 0x4fb29864, 0x265ef76e, 0x2656ed1a, 0x265ef76e,
            ],
            [
                0x227b88ec, 0xd0b3fa6f, 0xc86a08fd, 0xd0b3fa6f, 0xa864cba9, 0xd0b3fa6f,
            ],
            [
                0xf1002361, 0xe5e85fc3, 0x1f0b0408, 0xe5e85fc3, 0x488e7ac4, 0xe5e85fc3,
            ],
            [
                0xc65415d5, 0x51e176b6, 0xeca88bf9, 0x51e176b6, 0xedb89ece, 0x51e176b6,
            ],
            [
                0x9b6fb99c, 0x0548254b, 0x8de9f7c2, 0x0548254b, 0xb6b4d146, 0x0548254b,
            ],
            [
                0x7257f134, 0x06051a42, 0x36bcef01, 0x06051a42, 0xb649d524, 0x06051a42,
            ],
            [
                0xa540fb03, 0x34b196e6, 0xf7c80dad, 0x34b196e6, 0x71bc7dc4, 0x34b196e6,
            ],
            [
                0x8fbee745, 0xcf744123, 0x907c0a60, 0xcf744123, 0x8215ec35, 0xcf744123,
            ],
            [
                0x0bf6adba, 0xdf69029d, 0x5b72305a, 0xdf69029d, 0xcb47c19f, 0xdf69029d,
            ],
        ];

        for (generated_subkey, correct_subkey) in
            cipher.subkeys.into_iter().zip(test_subkeys.into_iter())
        {
            assert_eq!(correct_subkey, generated_subkey)
        }
    }
}

crate::test_block_cipher!(

    Lea128::default().with_ksa([0x0f1e2d3c, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0]), test_128,
    [0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x1a,0x1b,0x1c,0x1d,0x1e,0x1f],
    [0x9f,0xc8,0x4e,0x35,0x28,0xc6,0xc6,0x18,0x55,0x32,0xc7,0xa7,0x04,0x64,0x8b,0xfd];

    Lea192::default().with_ksa([0x0f1e2d3c, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0, 0xf0e1d2c3, 0xb4a59687]), test_192,
    [0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2a,0x2b,0x2c,0x2d,0x2e,0x2f],
    [0x6f,0xb9,0x5e,0x32,0x5a,0xad,0x1b,0x87,0x8c,0xdc,0xf5,0x35,0x76,0x74,0xc6,0xf2];

    Lea256::default().with_ksa([0x0f1e2d3c, 0x4b5a6978, 0x8796a5b4, 0xc3d2e1f0, 0xf0e1d2c3, 0xb4a59687, 0x78695a4b, 0x3c2d1e0f]), test_256,
    [0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x3a,0x3b,0x3c,0x3d,0x3e,0x3f],
    [0xd6,0x51,0xaf,0xf6,0x47,0xb1,0x89,0xc1,0x3a,0x89,0x00,0xca,0x27,0xf9,0xe1,0x97];
);
