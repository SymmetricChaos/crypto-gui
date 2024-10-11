use super::{
    ghash::Ghash,
    multiplication::{mul2, mul3},
    sbox::{sbox, sub_word},
};
use itertools::Itertools;

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

// Shift each row the state
pub fn shift_rows(state: &mut [u8]) {
    state[4..8].rotate_left(1);
    state[8..12].rotate_left(2);
    state[12..16].rotate_left(3);
}

// Mix each column using an invertible matrix multiplication
pub fn mix_columns(state: &mut [u8]) {
    mix_column(state, [0, 4, 8, 12]);
    mix_column(state, [1, 5, 9, 13]);
    mix_column(state, [2, 6, 10, 14]);
    mix_column(state, [3, 7, 11, 15]);
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
macro_rules! aes_gcm_methods {
    ($name: ident, $nk: literal, $nr: literal) => {
        pub struct $name {
            pub input_format: utils::byte_formatting::ByteFormat,
            pub output_format: utils::byte_formatting::ByteFormat,
            round_keys: [[u8; 16]; Self::NR + 1],
            pub ad: Vec<u8>,
            pub iv: u128,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: utils::byte_formatting::ByteFormat::Hex,
                    output_format: utils::byte_formatting::ByteFormat::Hex,
                    // key: [0; Self::NK],
                    round_keys: [[0u8; 16]; Self::NR + 1],
                    ad: Vec::new(),
                    iv: 0,
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
            pub fn ksa_u32(&mut self, key: [u32; Self::NK]) {
                // During expansion actions are on words of 32-bits
                let mut round_keys: Vec<u32> = Vec::new();

                round_keys.extend_from_slice(&key);

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

            pub fn with_key_u32(mut self, key: [u32; Self::NK]) -> Self {
                self.ksa_u32(key);
                self
            }

            pub fn ksa(&mut self, bytes: [u8; Self::NK * 4]) {
                let mut key = [0u32; Self::NK];
                utils::byte_formatting::fill_u32s_be(&mut key, &bytes);
                self.ksa_u32(key)
            }

            pub fn with_key(mut self, bytes: [u8; Self::NK * 4]) -> Self {
                self.ksa(bytes);
                self
            }

            pub fn with_iv(mut self, bytes: Vec<u8>) -> Self {
                // Recommended 96-bit IV
                if bytes.len() == 12 {
                    let mut iv = 1;
                    for i in 0..12 {
                        iv |= (bytes[i] as u128) << (120 - i * 8)
                    }
                    self.iv = iv;
                } else {
                    let mut h = [0; 16];
                    self.encrypt_block(&mut h);
                    let hasher = Ghash::default().h_bytes(h);
                    self.iv = u128::from_be_bytes(hasher.hash(&bytes).try_into().unwrap());
                }
                self
            }

            pub fn encrypt_block(&self, bytes: &mut [u8]) {
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

            pub fn encrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; 16]) {
                let mut ctr = ctr;

                for ptext in bytes.chunks_mut(16) {
                    // Encrypt the counter to create a mask
                    let mut mask = ctr;
                    self.encrypt_block(&mut mask);

                    // XOR the mask into the plaintext at the source, creating ciphertext
                    utils::byte_formatting::xor_into_bytes(ptext, &mask);

                    // Step the counter
                    utils::math_functions::incr_array_ctr(&mut ctr);
                }
            }

            pub fn decrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; 16]) {
                self.encrypt_ctr(bytes, ctr)
            }

            pub fn create_tag(&self, bytes: &[u8]) -> [u8; 16] {
                let mut c = self.iv.to_be_bytes();
                self.encrypt_block(&mut c);
                let mut h = [0; 16];
                self.encrypt_block(&mut h);

                let hasher = Ghash::default()
                    .c_bytes(c)
                    .h_bytes(h)
                    .ad_len(self.ad.len() as u64);

                let mut input = self.ad.clone();
                input.extend_from_slice(&bytes);
                hasher.hash(&input).try_into().unwrap()
            }
        }

        impl crate::Cipher for $name {
            fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|_| crate::CipherError::input("byte format error"))?;

                self.encrypt_ctr(&mut bytes, (self.iv + 1).to_be_bytes());

                let tag = self.create_tag(&bytes);

                bytes.extend_from_slice(&tag);

                Ok(self.output_format.byte_slice_to_text(&bytes))
            }

            fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|_| crate::CipherError::input("byte format error"))?;

                // Split the tag and the encrypted message
                let l = bytes.len() - 16;
                let (mut message_bytes, tag) = bytes.split_at_mut(l);

                if tag != self.create_tag(&message_bytes) {
                    return Err(crate::CipherError::input("message failed authentication"));
                }

                self.decrypt_ctr(&mut message_bytes, (self.iv + 1).to_be_bytes());

                Ok(self.output_format.byte_slice_to_text(&message_bytes))
            }
        }
    };
}

aes_gcm_methods!(AesGcm128, 4, 10);
aes_gcm_methods!(AesGcm192, 6, 12);
aes_gcm_methods!(AesGcm256, 8, 14);

#[cfg(test)]
mod aes_gcm_tests {

    use crate::Cipher;

    use super::*;

    fn ttb(s: &str) -> Vec<u8> {
        utils::byte_formatting::ByteFormat::Hex
            .text_to_bytes(s)
            .unwrap()
    }

    #[test]
    fn test_case_1() {
        let cipher = AesGcm128::default()
            .with_key(ttb("00000000000000000000000000000000").try_into().unwrap())
            .with_iv(vec![0; 12]);
        assert_eq!(
            "58e2fccefa7e3061367f1d57a4e7455a",
            cipher.encrypt("").unwrap()
        );
    }

    #[test]
    fn test_case_2() {
        let cipher = AesGcm128::default()
            .with_key(ttb("00000000000000000000000000000000").try_into().unwrap())
            .with_iv(vec![0; 12]);
        assert_eq!(
            "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf",
            cipher.encrypt("00000000000000000000000000000000").unwrap()
        );
    }

    #[test]
    fn test_case_3() {
        let cipher = AesGcm128::default()
            .with_key(ttb("feffe9928665731c6d6a8f9467308308").try_into().unwrap())
            .with_iv(ttb("cafebabefacedbaddecaf888"));
        assert_eq!(
            "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f59854d5c2af327cd64a62cf35abd2ba6fab4",
            cipher.encrypt("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255").unwrap()
        );
    }
}
