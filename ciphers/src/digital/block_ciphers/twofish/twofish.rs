use utils::byte_formatting::{make_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::{
    super::block_cipher::{BCMode, BCPadding, BlockCipher},
    functions::{mds_column_mult, mds_mult, q, rs_mult, QORD},
};

macro_rules! twofish {
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
                let mut out: u32 = 0;
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
                let rho = 0x01010101_u32;

                for x in 0..20 {
                    let a = self.h(rho * (2 * x), &bytes, 0);
                    let b = self.h(rho * (2 * x + 1), &bytes, 1).rotate_left(8);
                    let v = a.wrapping_add(b);
                    self.subkeys[(2 * x) as usize] = v;
                    self.subkeys[(2 * x + 1) as usize] = (v.wrapping_add(b)).rotate_left(9);
                }

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

                u32s_to_bytes_le(bytes, &block);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut block = make_u32s_le::<4>(bytes);

                // Input Whitening
                block[2] ^= self.subkeys[4];
                block[3] ^= self.subkeys[5];
                block[0] ^= self.subkeys[6];
                block[1] ^= self.subkeys[7];

                for i in (0..8).rev() {
                    let k = 4 * i + 8;

                    // Pseudo-Hadamard Transform is used here
                    let t1 = self.g(block[3].rotate_left(8));
                    let t0 = self.g(block[2]).wrapping_add(t1);
                    block[0] = (block[0] ^ (t0.wrapping_add(self.subkeys[k]))).rotate_right(1);
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 1]);
                    block[1] = block[1].rotate_left(1) ^ t2;

                    let t1 = self.g(block[1].rotate_left(8));
                    let t0 = self.g(block[0]).wrapping_add(t1);
                    block[2] = (block[2] ^ (t0.wrapping_add(self.subkeys[k + 2]))).rotate_right(1);
                    let t2 = t1.wrapping_add(t0).wrapping_add(self.subkeys[k + 3]);
                    block[3] = (block[3].rotate_left(1)) ^ t2;
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

twofish!(TwoFish128, 16);
twofish!(TwoFish192, 24);
twofish!(TwoFish256, 32);

#[cfg(test)]
mod twofish_tests {

    use super::*;

    fn ttb(s: &str) -> Vec<u8> {
        ByteFormat::Hex.text_to_bytes(s).unwrap()
    }

    #[test]
    fn ksa() {
        let cipher = TwoFish128::default()
            .with_key(ttb("00000000000000000000000000000000").try_into().unwrap());
        println!("{:08x?}", cipher.subkeys);
    }

    crate::test_block_cipher!(
        // 128-bit keys
        test1_128, TwoFish128::default().with_key(ttb("00000000000000000000000000000000").try_into().unwrap()),
        ttb("00000000000000000000000000000000"),
        ttb("9F589F5CF6122C32B6BFEC2F2AE8C35A");

        test2_128, TwoFish128::default().with_key(ttb("00000000000000000000000000000000").try_into().unwrap()),
        ttb("9F589F5CF6122C32B6BFEC2F2AE8C35A"),
        ttb("D491DB16E7B1C39E86CB086B789F5419");

        test3_128, TwoFish128::default().with_key(ttb("9F589F5CF6122C32B6BFEC2F2AE8C35A").try_into().unwrap()),
        ttb("D491DB16E7B1C39E86CB086B789F5419"),
        ttb("019F9809DE1711858FAAC3A3BA20FBC3");


        // 192-bit keys
        test1_192, TwoFish192::default().with_key(ttb("000000000000000000000000000000000000000000000000").try_into().unwrap()),
        ttb("00000000000000000000000000000000"),
        ttb("EFA71F788965BD4453F860178FC19101");

        test2_192, TwoFish192::default().with_key(ttb("000000000000000000000000000000000000000000000000").try_into().unwrap()),
        ttb("EFA71F788965BD4453F860178FC19101"),
        ttb("88B2B2706B105E36B446BB6D731A1E88");

        test3_192, TwoFish192::default().with_key(ttb("EFA71F788965BD4453F860178FC191010000000000000000").try_into().unwrap()),
        ttb("88B2B2706B105E36B446BB6D731A1E88"),
        ttb("39DA69D6BA4997D585B6DC073CA341B2");

        test4_194, TwoFish192::default().with_key(ttb("88B2B2706B105E36B446BB6D731A1E88EFA71F788965BD44").try_into().unwrap()),
        ttb("39DA69D6BA4997D585B6DC073CA341B2"),
        ttb("182B02D81497EA45F9DAACDC29193A65");


        // 256-bit keys
        test1_256, TwoFish256::default().with_key(ttb("0000000000000000000000000000000000000000000000000000000000000000").try_into().unwrap()),
        ttb("00000000000000000000000000000000"),
        ttb("57FF739D4DC92C1BD7FC01700CC8216F");

        test2_256, TwoFish256::default().with_key(ttb("0000000000000000000000000000000000000000000000000000000000000000").try_into().unwrap()),
        ttb("57FF739D4DC92C1BD7FC01700CC8216F"),
        ttb("D43BB7556EA32E46F2A282B7D45B4E0D");

        test3_256, TwoFish256::default().with_key(ttb("57FF739D4DC92C1BD7FC01700CC8216F00000000000000000000000000000000").try_into().unwrap()),
        ttb("D43BB7556EA32E46F2A282B7D45B4E0D"),
        ttb("90AFE91BB288544F2C32DC239B2635E6");

        test4_256, TwoFish256::default().with_key(ttb("D43BB7556EA32E46F2A282B7D45B4E0D57FF739D4DC92C1BD7FC01700CC8216F").try_into().unwrap()),
        ttb("90AFE91BB288544F2C32DC239B2635E6"),
        ttb("6CB4561C40BF0A9705931CB6D408E7FA");
    );
}
