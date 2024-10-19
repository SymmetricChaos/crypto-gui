use itertools::Itertools;
use utils::byte_formatting::{fill_u32s_le, make_u32s_le, u32s_to_bytes_le, ByteFormat};

use super::{
    super::block_cipher::{BCMode, BCPadding, BlockCipher},
    functions::{mds_column_mult, mds_mult, pht, q, rs_mult, QORD},
};

macro_rules! TwoFish {
    ($name: ident, $key_bytes: literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub mode: BCMode,
            pub padding: BCPadding,
            pub iv: u128,
            pub subkeys: [u32; 40],
            pub sbox_key: [u8; 16],
            // sboxes: [[u32; 256]; 4],
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
                    // sboxes: [[0; 256]; 4],
                }
            }
        }

        crate::block_cipher_builders! {$name, u128}

        impl $name {
            const KEY_BYTES: usize = $key_bytes;
            const KEY_WORDS: usize = KEY_BYTES / 4;
            const K: usize = KEY_BYTES / 8; // Keylength in bits divided by 64
            const START: usize = (K + 2) - 4;

            fn h(&self, x: u32, list: &[u8], offset: usize) -> u32 {
                let mut y = x.to_le_bytes();
                if K == 4 {
                    y[0] = q(1, y[0]) ^ list[4 * (6 + offset + 0)];
                    y[1] = q(0, y[1]) ^ list[4 * (6 + offset + 1)];
                    y[2] = q(0, y[2]) ^ list[4 * (6 + offset + 2)];
                    y[3] = q(1, y[3]) ^ list[4 * (6 + offset + 3)];
                }

                if K >= 3 {
                    y[0] = q(1, y[0]) ^ list[4 * (4 + offset + 0)];
                    y[1] = q(1, y[1]) ^ list[4 * (4 + offset + 1)];
                    y[2] = q(0, y[2]) ^ list[4 * (4 + offset + 2)];
                    y[3] = q(0, y[3]) ^ list[4 * (4 + offset + 3)];
                }

                let a = 4 * (2 + offset);
                let b = 4 * offset;

                y[0] = q(1, q(0, q(0, y[0]) ^ list[a + 0]) ^ list[b + 0]);
                y[1] = q(0, q(1, q(1, y[1]) ^ list[a + 1]) ^ list[b + 1]);
                y[2] = q(1, q(1, q(0, y[2]) ^ list[a + 2]) ^ list[b + 2]);
                y[3] = q(0, q(1, q(1, y[3]) ^ list[a + 3]) ^ list[b + 3]);

                mds_mult(y)
            }

            fn g(&self, x: u32) -> u32 {
                let mut out: u32 = 0;
                for y in 0..4 {
                    let mut g = q(QORD[y][START], (x >> (8 * y)) as u8);

                    for z in START + 1..5 {
                        g ^= self.sbox_key[4 * (z - START - 1) + y];
                        g = q(QORD[y][z], g);
                    }

                    out ^= mds_column_mult(g, y);
                }
                out
            }

            fn f(&self, a: u32, b: u32, round: usize) -> (u32, u32) {
                let t0 = self.g(a);
                let t1 = self.g(b.rotate_left(8));
                let mut o = pht(t0, t1);
                o.0 = o.0.wrapping_add(self.subkeys[2 * round + 8]);
                o.1 = o.1.wrapping_add(self.subkeys[2 * round + 9]);
                o
            }

            pub fn ksa(&mut self, bytes: [u8; KEY_BYTES]) {
                let rho = 0x01010101_u32;

                for x in 0..20 {
                    let a = self.h(rho * (2 * x), &bytes, 0);
                    let b = self.h(rho * (2 * x + 1), &bytes, 1).rotate_left(8);
                    let v = a.wrapping_add(b);
                    self.subkeys[(2 * x) as usize] = v;
                    self.subkeys[(2 * x + 1) as usize] = (v.wrapping_add(b)).rotate_left(9);
                }

                for i in 0..K {
                    rs_mult(
                        &bytes[i * 8..i * 8 + 8],
                        &mut self.sbox_key[i * 4..(i + 1) * 4],
                    );
                }
            }

            pub fn ksa_u32(&mut self, key: [u32; Self::KEY_WORDS]) {}

            pub fn with_key(mut self, bytes: [u8; Self::KEY_BYTES]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn with_key_u32(mut self, bytes: [u32; Self::KEY_WORDS]) -> Self {
                self.ksa_u32(bytes);
                self
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                let mut block = make_u32s_le<4>(&mut block, bytes);

                // Input Whitening
                for i in 0..4 {
                    block[i] ^= self.subkeys[i]
                }

                for i in 0..8 {
                    let k = 4 * i + 8;

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
                for i in 4..8 {
                    block[i - 4] ^= self.subkeys[i];
                }

                u32s_to_bytes_le(bytes, &block);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut block = [0; 4];
                fill_u32s_le(&mut block, bytes);

                // Input Whitening
                for i in 4..8 {
                    block[i - 4] ^= self.subkeys[i]
                }

                for i in (0..8).rev() {
                    let k = 4 * i + 8;

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
    };
}

const KEY_BYTES: usize = 16;
const KEY_WORDS: usize = KEY_BYTES / 4;
const K: usize = KEY_BYTES / 8; // Keylength in bits divided by 64
const START: usize = (K + 2) - 4;

pub struct TwoFish128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u128,
    pub subkeys: [u32; 40],
    pub sbox_key: [u8; 16],
    // sboxes: [[u32; 256]; 4],
}

impl Default for TwoFish128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
            iv: 0,
            subkeys: [0; 40],
            sbox_key: [0; 16],
            // sboxes: [[0; 256]; 4],
        }
    }
}

crate::block_cipher_builders! {TwoFish128, u128}

impl TwoFish128 {
    // pub fn sbox(&self, n: u32, i: usize) -> u32 {
    //     self.sboxes[i][n as usize]
    // }

    fn h(&self, x: u32, list: &[u8], offset: usize) -> u32 {
        let mut y = x.to_le_bytes();
        if K == 4 {
            y[0] = q(1, y[0]) ^ list[4 * (6 + offset + 0)];
            y[1] = q(0, y[1]) ^ list[4 * (6 + offset + 1)];
            y[2] = q(0, y[2]) ^ list[4 * (6 + offset + 2)];
            y[3] = q(1, y[3]) ^ list[4 * (6 + offset + 3)];
        }

        if K >= 3 {
            y[0] = q(1, y[0]) ^ list[4 * (4 + offset + 0)];
            y[1] = q(1, y[1]) ^ list[4 * (4 + offset + 1)];
            y[2] = q(0, y[2]) ^ list[4 * (4 + offset + 2)];
            y[3] = q(0, y[3]) ^ list[4 * (4 + offset + 3)];
        }

        let a = 4 * (2 + offset);
        let b = 4 * offset;

        y[0] = q(1, q(0, q(0, y[0]) ^ list[a + 0]) ^ list[b + 0]);
        y[1] = q(0, q(1, q(1, y[1]) ^ list[a + 1]) ^ list[b + 1]);
        y[2] = q(1, q(1, q(0, y[2]) ^ list[a + 2]) ^ list[b + 2]);
        y[3] = q(0, q(1, q(1, y[3]) ^ list[a + 3]) ^ list[b + 3]);

        mds_mult(y)
    }

    fn g(&self, x: u32) -> u32 {
        let mut out: u32 = 0;
        for y in 0..4 {
            let mut g = q(QORD[y][START], (x >> (8 * y)) as u8);

            for z in START + 1..5 {
                g ^= self.sbox_key[4 * (z - START - 1) + y];
                g = q(QORD[y][z], g);
            }

            out ^= mds_column_mult(g, y);
        }
        out
    }

    fn f(&self, a: u32, b: u32, round: usize) -> (u32, u32) {
        let t0 = self.g(a);
        let t1 = self.g(b.rotate_left(8));
        let mut o = pht(t0, t1);
        o.0 = o.0.wrapping_add(self.subkeys[2 * round + 8]);
        o.1 = o.1.wrapping_add(self.subkeys[2 * round + 9]);
        o
    }

    pub fn ksa(&mut self, bytes: [u8; KEY_BYTES]) {
        let rho = 0x01010101_u32;

        for x in 0..20 {
            let a = self.h(rho * (2 * x), &bytes, 0);
            let b = self.h(rho * (2 * x + 1), &bytes, 1).rotate_left(8);
            let v = a.wrapping_add(b);
            self.subkeys[(2 * x) as usize] = v;
            self.subkeys[(2 * x + 1) as usize] = (v.wrapping_add(b)).rotate_left(9);
        }

        for i in 0..K {
            rs_mult(
                &bytes[i * 8..i * 8 + 8],
                &mut self.sbox_key[i * 4..(i + 1) * 4],
            );
        }
    }

    pub fn ksa_u32(&mut self, key: [u32; KEY_WORDS]) {}

    pub fn with_key(mut self, bytes: [u8; KEY_BYTES]) -> Self {
        self.ksa(bytes);
        self
    }

    pub fn with_key_u32(mut self, bytes: [u32; KEY_WORDS]) -> Self {
        self.ksa_u32(bytes);
        self
    }
}

impl BlockCipher<16> for TwoFish128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = make_u32s_le::<4>(bytes);

        // Input Whitening
        for i in 0..4 {
            block[i] ^= self.subkeys[i]
        }

        for i in 0..8 {
            let k = 4 * i + 8;

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
        for i in 4..8 {
            block[i - 4] ^= self.subkeys[i];
        }

        u32s_to_bytes_le(bytes, &block);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = [0; 4];
        fill_u32s_le(&mut block, bytes);

        // Input Whitening
        for i in 4..8 {
            block[i - 4] ^= self.subkeys[i]
        }

        for i in (0..8).rev() {
            let k = 4 * i + 8;

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

// crate::test_block_cipher!(

// )
