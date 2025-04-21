use utils::byte_formatting::{make_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::{
    super::block_cipher::{BCMode, BCPadding, BlockCipher},
    functions::{mds_column_mult, mds_mult, q, rs_mult, QORD},
};

macro_rules! Twofish {
    ($name: ident, $key_bytes: literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u128,
            pub subkeys: [u32; 40],
            pub sbox_key: [u8; 16],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    mode: BCMode::default(),
                    padding: BCPadding::default(),
                    iv: 0,
                    subkeys: [0; 40],
                    sbox_key: [0; 16],
                }
            }
        }

        crate::block_cipher_builders! {$name, u128}

        impl $name {
            const KEY_BYTES: usize = $key_bytes;
            const KEY_WORDS: usize = Self::KEY_BYTES / 4;
            const K: usize = Self::KEY_BYTES / 8;
            const START: usize = 4 - Self::K;

            fn h(&self, x: u32, key_bytes: &[u8], offset: usize) -> u32 {
                let mut y = x.to_le_bytes();
                // For 256-bit keys
                if Self::K == 4 {
                    y[0] = q(1, y[0]) ^ key_bytes[4 * (6 + offset) + 0];
                    y[1] = q(0, y[1]) ^ key_bytes[4 * (6 + offset) + 1];
                    y[2] = q(0, y[2]) ^ key_bytes[4 * (6 + offset) + 2];
                    y[3] = q(1, y[3]) ^ key_bytes[4 * (6 + offset) + 3];
                }

                // For both 192-bit and 256-bit keys
                if Self::K >= 3 {
                    y[0] = q(1, y[0]) ^ key_bytes[4 * (4 + offset) + 0];
                    y[1] = q(1, y[1]) ^ key_bytes[4 * (4 + offset) + 1];
                    y[2] = q(0, y[2]) ^ key_bytes[4 * (4 + offset) + 2];
                    y[3] = q(0, y[3]) ^ key_bytes[4 * (4 + offset) + 3];
                }

                // For all keys
                let a = 4 * (2 + offset);
                let b = 4 * offset;

                y[0] = q(1, q(0, q(0, y[0]) ^ key_bytes[a + 0]) ^ key_bytes[b + 0]);
                y[1] = q(0, q(0, q(1, y[1]) ^ key_bytes[a + 1]) ^ key_bytes[b + 1]);
                y[2] = q(1, q(1, q(0, y[2]) ^ key_bytes[a + 2]) ^ key_bytes[b + 2]);
                y[3] = q(0, q(1, q(1, y[3]) ^ key_bytes[a + 3]) ^ key_bytes[b + 3]);

                mds_mult(y)
            }

            fn g(&self, x: u32) -> u32 {
                let mut out = 0;
                for y in 0..4 {
                    let mut g = q(QORD[y][Self::START], (x >> (8 * y)) as u8);

                    for z in Self::START + 1..5 {
                        g ^= self.sbox_key[4 * (z - Self::START - 1) + y];
                        g = q(QORD[y][z], g);
                    }

                    out ^= mds_column_mult(g, y);
                }
                out
            }

            pub fn ksa(&mut self, bytes: [u8; Self::KEY_BYTES]) {
                let rho = 0x01010101;

                // Subkeys
                for x in 0..20 {
                    let a = self.h(rho * (2 * x), &bytes, 0);
                    let b = self.h(rho * (2 * x + 1), &bytes, 1).rotate_left(8);
                    let v = a.wrapping_add(b);
                    self.subkeys[(2 * x) as usize] = v;
                    self.subkeys[(2 * x + 1) as usize] = (v.wrapping_add(b)).rotate_left(9);
                }

                // Sbox keys
                for i in 0..Self::K {
                    rs_mult(
                        &bytes[i * 8..i * 8 + 8],
                        &mut self.sbox_key[i * 4..(i + 1) * 4],
                    );
                }
            }

            pub fn ksa_u32(&mut self, key: [u32; Self::KEY_WORDS]) {
                let mut bytes = [0; Self::KEY_BYTES];
                u32s_to_bytes_le(&mut bytes, &key);
                self.ksa(bytes);
            }

            pub fn with_key(mut self, bytes: [u8; Self::KEY_BYTES]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn with_key_u32(mut self, key: [u32; Self::KEY_WORDS]) -> Self {
                self.ksa_u32(key);
                self
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                let mut block = make_u32s_le::<4>(bytes);

                // Input Whitening
                // Using the first four key
                for i in 0..4 {
                    block[i] ^= self.subkeys[i]
                }

                // Eight quad rounds use the last 32 keys (the first eight are used in key whitening)
                for i in 0..8 {
                    let k = 4 * i + 8;

                    // Pseudo-Hadamard Transform is used here
                    let t1 = self.g(block[1].rotate_left(8));
                    let t0 = self.g(block[0]).wrapping_add(t1);
                    block[2] = (block[2] ^ (t0.wrapping_add(self.subkeys[k]))).rotate_right(1);
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 1]);
                    block[3] = block[3].rotate_left(1) ^ t2;

                    let t1 = self.g(block[3].rotate_left(8));
                    let t0 = self.g(block[2]).wrapping_add(t1);
                    block[0] = (block[0] ^ (t0.wrapping_add(self.subkeys[k + 2]))).rotate_right(1);
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 3]);
                    block[1] = (block[1].rotate_left(1)) ^ t2;
                }

                // Output Whitening
                // Using the second four key
                block[2] ^= self.subkeys[4];
                block[3] ^= self.subkeys[5];
                block[0] ^= self.subkeys[6];
                block[1] ^= self.subkeys[7];

                block.swap(2, 0);
                block.swap(3, 1);

                u32s_to_bytes_le(bytes, &block);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut block = make_u32s_le::<4>(bytes);

                block.swap(2, 0);
                block.swap(3, 1);

                // Input Whitening
                block[0] ^= self.subkeys[6];
                block[1] ^= self.subkeys[7];
                block[2] ^= self.subkeys[4];
                block[3] ^= self.subkeys[5];

                for i in (0..8).rev() {
                    let k = 4 * i + 8;

                    // Pseudo-Hadamard Transform is used here
                    let t1 = self.g(block[3].rotate_left(8));
                    let t0 = self.g(block[2]).wrapping_add(t1);
                    block[0] = block[0].rotate_left(1) ^ (t0.wrapping_add(self.subkeys[k + 2]));
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 3]);
                    block[1] = (block[1] ^ t2).rotate_right(1);

                    let t1 = self.g(block[1].rotate_left(8));
                    let t0 = self.g(block[0]).wrapping_add(t1);
                    block[2] = block[2].rotate_left(1) ^ (t0.wrapping_add(self.subkeys[k]));
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 1]);
                    block[3] = (block[3] ^ t2).rotate_right(1);
                }

                // Output Whitening
                for i in 0..4 {
                    block[i] ^= self.subkeys[i]
                }

                u32s_to_bytes_le(bytes, &block);
            }
        }

        crate::impl_cipher_for_block_cipher!($name, 16);
    };
}

Twofish!(Twofish128, 16);
Twofish!(Twofish192, 24);
Twofish!(Twofish256, 32);

#[cfg(test)]
mod twofish_tests {

    use super::*;
    use hex_literal::hex;

    #[test]
    fn ksa_128() {
        let cipher = Twofish128::default().with_key(hex!("00000000000000000000000000000000"));
        assert_eq!(&[0; 8][..], &cipher.sbox_key[0..8]);
        assert_eq!(
            [
                0x52C54DDE, 0x11F0626D, 0x7CAC9D4A, 0x4D1B4AAA, 0xB7B83A10, 0x1E7D0BEB, 0xEE9C341F,
                0xCFE14BE4, 0xF98FFEF9, 0x9C5B3C17, 0x15A48310, 0x342A4D81, 0x424D89FE, 0xC14724A7,
                0x311B834C, 0xFDE87320, 0x3302778F, 0x26CD67B4, 0x7A6C6362, 0xC2BAF60E, 0x3411B994,
                0xD972C87F, 0x84ADB1EA, 0xA7DEE434, 0x54D2960F, 0xA2F7CAA8, 0xA6B8FF8C, 0x8014C425,
                0x6A748D1C, 0xEDBAF720, 0x928EF78C, 0x0338EE13, 0x9949D6BE, 0xC8314176, 0x07C07D68,
                0xECAE7EA7, 0x1FE71844, 0x85C05C89, 0xF298311E, 0x696EA672,
            ],
            cipher.subkeys,
        );
    }

    #[test]
    fn ksa_192() {
        let cipher = Twofish192::default()
            .with_key(hex!("0123456789ABCDEFFEDCBA98765432100011223344556677"));
        assert_eq!(
            &[0xf2, 0xf6, 0x9f, 0xb8, 0x4b, 0xbc, 0x55, 0xb2, 0x61, 0x10, 0x66, 0x45,][..],
            &cipher.sbox_key[0..12],
        );
        assert_eq!(
            [
                0x38394A24, 0xC36D1175, 0xE802528F, 0x219BFEB4, 0xB9141AB4, 0xBD3E70CD, 0xAF609383,
                0xFD36908A, 0x03EFB931, 0x1D2EE7EC, 0xA7489D55, 0x6E44B6E8, 0x714AD667, 0x653AD51F,
                0xB6315B66, 0xB27C05AF, 0xA06C8140, 0x9853D419, 0x4016E346, 0x8D1C0DD4, 0xF05480BE,
                0xB6AF816F, 0x2D7DC789, 0x45B7BD3A, 0x57F8A163, 0x2BEFDA69, 0x26AE7271, 0xC2900D79,
                0xED323794, 0x3D3FFD80, 0x5DE68E49, 0x9C3D2478, 0xDF326FE3, 0x5911F70D, 0xC229F13B,
                0xB1364772, 0x4235364D, 0x0CEC363A, 0x57C8DD1F, 0x6A1AD61E,
            ],
            cipher.subkeys,
        );
    }

    #[test]
    fn ksa_256() {
        let cipher = Twofish256::default().with_key(hex!(
            "0123456789ABCDEFFEDCBA987654321000112233445566778899AABBCCDDEEFF"
        ));
        assert_eq!(
            &[
                0xf2, 0xf6, 0x9f, 0xb8, 0x4b, 0xbc, 0x55, 0xb2, 0x61, 0x10, 0x66, 0x45, 0xf7, 0x47,
                0x44, 0x8e,
            ][..],
            &cipher.sbox_key[0..16],
        );
        assert_eq!(
            [
                0x5EC769BF, 0x44D13C60, 0x76CD39B1, 0x16750474, 0x349C294B, 0xEC21F6D6, 0x4FBD10B4,
                0x578DA0ED, 0xC3479695, 0x9B6958FB, 0x6A7FBC4E, 0x0BF1830B, 0x61B5E0FB, 0xD78D9730,
                0x7C6CF0C4, 0x2F9109C8, 0xE69EA8D1, 0xED99BDFF, 0x35DC0BBD, 0xA03E5018, 0xFB18EA0B,
                0x38BD43D3, 0x76191781, 0x37A9A0D3, 0x72427BEA, 0x911CC0B8, 0xF1689449, 0x71009CA9,
                0xB6363E89, 0x494D9855, 0x590BBC63, 0xF95A28B5, 0xFB72B4E1, 0x2A43505C, 0xBFD34176,
                0x5C133D12, 0x3A9247F7, 0x9A3331DD, 0xEE7515E6, 0xF0D54DCD,
            ],
            cipher.subkeys,
        );
    }

    crate::test_block_cipher!(
        // 128-bit keys
        test1_128, Twofish128::default().with_key(hex!("00000000000000000000000000000000")),
        hex!("00000000000000000000000000000000"),
        hex!("9F589F5CF6122C32B6BFEC2F2AE8C35A");

        test2_128, Twofish128::default().with_key(hex!("00000000000000000000000000000000")),
        hex!("9F589F5CF6122C32B6BFEC2F2AE8C35A"),
        hex!("D491DB16E7B1C39E86CB086B789F5419");

        test3_128, Twofish128::default().with_key(hex!("9F589F5CF6122C32B6BFEC2F2AE8C35A")),
        hex!("D491DB16E7B1C39E86CB086B789F5419"),
        hex!("019F9809DE1711858FAAC3A3BA20FBC3");


        // 192-bit keys
        test1_192, Twofish192::default().with_key(hex!("000000000000000000000000000000000000000000000000")),
        hex!("00000000000000000000000000000000"),
        hex!("EFA71F788965BD4453F860178FC19101");

        test2_192, Twofish192::default().with_key(hex!("000000000000000000000000000000000000000000000000")),
        hex!("EFA71F788965BD4453F860178FC19101"),
        hex!("88B2B2706B105E36B446BB6D731A1E88");

        test3_192, Twofish192::default().with_key(hex!("EFA71F788965BD4453F860178FC191010000000000000000")),
        hex!("88B2B2706B105E36B446BB6D731A1E88"),
        hex!("39DA69D6BA4997D585B6DC073CA341B2");

        test4_194, Twofish192::default().with_key(hex!("88B2B2706B105E36B446BB6D731A1E88EFA71F788965BD44")),
        hex!("39DA69D6BA4997D585B6DC073CA341B2"),
        hex!("182B02D81497EA45F9DAACDC29193A65");


        // 256-bit keys
        test1_256, Twofish256::default().with_key(hex!("0000000000000000000000000000000000000000000000000000000000000000")),
        hex!("00000000000000000000000000000000"),
        hex!("57FF739D4DC92C1BD7FC01700CC8216F");

        test2_256, Twofish256::default().with_key(hex!("0000000000000000000000000000000000000000000000000000000000000000")),
        hex!("57FF739D4DC92C1BD7FC01700CC8216F"),
        hex!("D43BB7556EA32E46F2A282B7D45B4E0D");

        test3_256, Twofish256::default().with_key(hex!("57FF739D4DC92C1BD7FC01700CC8216F00000000000000000000000000000000")),
        hex!("D43BB7556EA32E46F2A282B7D45B4E0D"),
        hex!("90AFE91BB288544F2C32DC239B2635E6");

        test4_256, Twofish256::default().with_key(hex!("D43BB7556EA32E46F2A282B7D45B4E0D57FF739D4DC92C1BD7FC01700CC8216F")),
        hex!("90AFE91BB288544F2C32DC239B2635E6"),
        hex!("6CB4561C40BF0A9705931CB6D408E7FA");
    );
}
