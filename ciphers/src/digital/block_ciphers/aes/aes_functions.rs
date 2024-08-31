use itertools::Itertools;

use super::{
    multiplication::{mul11, mul13, mul14, mul2, mul3, mul9},
    sbox::{inv_sbox, sbox},
};

// Rotate a 32-bit word by 8-bits
pub fn rot_word(n: u32) -> u32 {
    n.rotate_left(8)
}

// When the subkeys are created by the key scheduke they are [u32;4] but need to be [u8;16] to xor into the block
pub fn sub_key_to_bytes(key: [u32; 4]) -> [u8; 16] {
    key.into_iter()
        .map(|w| w.to_be_bytes())
        .flatten()
        .collect_vec()
        .try_into()
        .unwrap()
}

pub fn sub_key_slice_to_bytes(key: &[u32]) -> [u8; 16] {
    key.into_iter()
        .map(|w| w.to_be_bytes())
        .flatten()
        .collect_vec()
        .try_into()
        .unwrap()
}

// The internal state of AES is shown as a grid of bytes in column major order.
// This swaps array positions to transpose the bytes and put them in this order
// A (faster?) alternative would be to change the block transformation instead
pub fn transpose_state(state: &mut [u8]) {
    for (idx, orig) in [(1, 4), (2, 8), (3, 12), (6, 9), (7, 13), (11, 14)].into_iter() {
        state.swap(orig, idx)
    }
}

// Perform the SBOX substititon to all bytes in the state
pub fn sub_bytes(state: &mut [u8]) {
    for byte in state {
        *byte = sbox(*byte)
    }
}

// Inverse of above
pub fn inv_sub_bytes(state: &mut [u8]) {
    for byte in state {
        *byte = inv_sbox(*byte)
    }
}

// Shift each row the state
pub fn shift_rows(state: &mut [u8]) {
    state[4..8].rotate_left(1);
    state[8..12].rotate_left(2);
    state[12..16].rotate_left(3);
}

// Inverse of above
pub fn inv_shift_rows(state: &mut [u8]) {
    state[4..8].rotate_right(1);
    state[8..12].rotate_right(2);
    state[12..16].rotate_right(3);
}

// Mix each column using an invertible matrix multiplication
pub fn mix_columns(state: &mut [u8]) {
    mix_column(state, [0, 4, 8, 12]);
    mix_column(state, [1, 5, 9, 13]);
    mix_column(state, [2, 6, 10, 14]);
    mix_column(state, [3, 7, 11, 15]);
}

// Inverse of above
pub fn inv_mix_columns(state: &mut [u8]) {
    inv_mix_column(state, [0, 4, 8, 12]);
    inv_mix_column(state, [1, 5, 9, 13]);
    inv_mix_column(state, [2, 6, 10, 14]);
    inv_mix_column(state, [3, 7, 11, 15]);
}

// Perform the matrix multiplication.
// The scalar additions are XOR
// The scalar multiplications looked up from tables
pub fn mix_column(state: &mut [u8], idxs: [usize; 4]) {
    let a = state[idxs[0]];
    let b = state[idxs[1]];
    let c = state[idxs[2]];
    let d = state[idxs[3]];
    state[idxs[0]] = mul2(a) ^ mul3(b) ^ c ^ d;
    state[idxs[1]] = a ^ mul2(b) ^ mul3(c) ^ d;
    state[idxs[2]] = a ^ b ^ mul2(c) ^ mul3(d);
    state[idxs[3]] = mul3(a) ^ b ^ c ^ mul2(d);
}

// Inverse of above
pub fn inv_mix_column(state: &mut [u8], idxs: [usize; 4]) {
    let a = state[idxs[0]];
    let b = state[idxs[1]];
    let c = state[idxs[2]];
    let d = state[idxs[3]];
    state[idxs[0]] = mul14(a) ^ mul11(b) ^ mul13(c) ^ mul9(d);
    state[idxs[1]] = mul9(a) ^ mul14(b) ^ mul11(c) ^ mul13(d);
    state[idxs[2]] = mul13(a) ^ mul9(b) ^ mul14(c) ^ mul11(d);
    state[idxs[3]] = mul11(a) ^ mul13(b) ^ mul9(c) ^ mul14(d);
}

// XOR the round key into the state column by column
// This operation is its own inverse
pub fn add_round_key(state: &mut [u8], round_key: &[u8]) {
    // Key is added column by column
    for (idx, key) in [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15]
        .into_iter()
        .zip(round_key)
    {
        state[idx] ^= key
    }
}

pub fn print_aes_state(state: &[u8]) {
    for line in state.chunks_exact(4) {
        println!("{:02x?}", line)
    }
}

pub const RC: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1b000000, 0x36000000,
];

#[macro_export]
macro_rules! aes_methods {
    ($name: ident, $nk: literal, $nr: literal) => {
        pub struct $name {
            pub input_format: ByteFormat,
            pub output_format: ByteFormat,
            pub key: [u32; Self::NK],
            round_keys: [[u8; 16]; Self::NR + 1],
            pub iv: u128,
            pub mode: BCMode,
            pub padding: BCPadding,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: ByteFormat::Hex,
                    output_format: ByteFormat::Hex,
                    key: [0; Self::NK],
                    round_keys: [[0u8; 16]; Self::NR + 1],
                    iv: 0,
                    mode: BCMode::default(),
                    padding: BCPadding::default(),
                }
            }
        }

        impl $name {
            /// Number of 32-bit words in key.
            const NK: usize = $nk;
            /// Number of rounds.
            const NR: usize = $nr;
            /// Number of columns in the state. Fixed at 4 for all NIST versions.
            const NB: usize = 4;

            // Create the round keys
            pub fn ksa(&mut self) {
                // During expansion actions are on words of 32-bits
                let mut round_keys: Vec<u32> = Vec::new();

                round_keys.extend_from_slice(&self.key);

                for i in Self::NK..((Self::NR + 1) * Self::NB) {
                    let mut t = round_keys[i - 1];
                    if i % Self::NK == 0 {
                        t = sub_word(rot_word(t))
                            ^ crate::digital::block_ciphers::aes::aes_functions::RC
                                [(i / Self::NK) - 1];
                    } else if Self::NK > 6 && i % Self::NK == 4 {
                        t = sub_word(t);
                    }
                    round_keys.push(round_keys[i - Self::NK] ^ t);
                }

                for (i, chunk) in round_keys.chunks(4).enumerate() {
                    self.round_keys[i] = sub_key_slice_to_bytes(chunk)
                }
            }
        }

        impl BlockCipher<16> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                transpose_state(bytes);
                // Initial round key
                add_round_key(bytes, &self.round_keys[0]);

                // Main NR
                for i in 1..Self::NR {
                    sub_bytes(bytes);
                    shift_rows(bytes);
                    mix_columns(bytes);
                    add_round_key(bytes, &self.round_keys[i]);
                }

                // Finalization round
                sub_bytes(bytes);
                shift_rows(bytes);
                add_round_key(bytes, &self.round_keys[Self::NR]);
                transpose_state(bytes);
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                transpose_state(bytes);
                // Initial round key
                add_round_key(bytes, &self.round_keys[Self::NR]);

                // Main NR
                for i in (1..Self::NR).rev() {
                    inv_shift_rows(bytes);
                    inv_sub_bytes(bytes);
                    add_round_key(bytes, &self.round_keys[i]);
                    inv_mix_columns(bytes);
                }

                // Finalization round
                inv_shift_rows(bytes);
                inv_sub_bytes(bytes);
                add_round_key(bytes, &self.round_keys[0]);
                transpose_state(bytes);
            }
        }
    };
}
