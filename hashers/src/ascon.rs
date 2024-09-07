use utils::{
    byte_formatting::{u64s_to_bytes_be, ByteFormat},
    padding::bit_padding,
};

fn bytes_to_u64_be(bytes: &[u8]) -> Vec<u64> {
    assert!(
        bytes.len() % 8 == 0,
        "must have a length that is a multiple of eight bytes"
    );
    let output_len = bytes.len() / 8;
    let mut out = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let mut word_bits: [u8; 8] = Default::default();
        word_bits.copy_from_slice(&bytes[(i * 8)..(i * 8 + 8)]);
        out.push(u64::from_be_bytes(word_bits));
    }
    out
}

use crate::traits::ClassicHasher;

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone)]
pub struct AsconHashState {
    state: [u64; 5],
}

// Shortcut indexing
impl std::ops::Index<usize> for AsconHashState {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}

impl std::ops::IndexMut<usize> for AsconHashState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.state[index]
    }
}

// Default to precomputed state
impl Default for AsconHashState {
    fn default() -> Self {
        Self {
            state: [
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ],
        }
    }
}

impl AsconHashState {
    const A: usize = 12; // initialization rounds
    const RATE: usize = 8; // number of bytes absorbed at a time

    pub fn rounds_a(&mut self) {
        for i in 0..Self::A {
            self.transform(i);
        }
    }

    pub fn transform(&mut self, i: usize) {
        // round constant
        self[2] ^= C[i];
        // substitution
        self.sbox();
        // linear diffusion
        self.linear_diffusor();
    }

    // The sbox works across words
    // It effectively take the nth bit of each word, interprets it as a 5-bit word, then substitutes it
    pub fn sbox(&mut self) {
        self[0] ^= self[4];
        self[4] ^= self[3];
        self[2] ^= self[1];

        let mut t = self.state.clone();
        for i in 0..5 {
            t[i] ^= !self[(i + 1) % 5] & self[(i + 2) % 5];
        }

        t[1] ^= t[0];
        t[0] ^= t[4];
        t[3] ^= t[2];
        t[2] = !t[2];

        self.state = t;
    }

    // This diffuses bits within each word of state
    pub fn linear_diffusor(&mut self) {
        for i in 0..5 {
            self[i] ^= self[i].rotate_right(ROTS[i].0) ^ self[i].rotate_right(ROTS[i].1);
        }
    }

    pub fn absorb(&mut self, message: &[u8]) {
        assert!(
            message.len() % Self::RATE == 0,
            "message length in bytes must be a multiple of {}",
            Self::RATE
        );
        let words = bytes_to_u64_be(message);

        for word in words {
            self[0] ^= word;
            self.rounds_a();
        }
    }

    pub fn squeeze(&mut self, hash_len: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            output.extend_from_slice(&self[0].to_be_bytes());
            self.rounds_a();
        }

        output.truncate(hash_len);
        output
    }
}

pub struct AsconHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: usize,
}

impl Default for AsconHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
        }
    }
}

impl ClassicHasher for AsconHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();
        bit_padding(&mut input, 8).expect("somehow padding failed");
        // println!("{:02x?}", input);

        let mut state = AsconHashState::default();
        state.absorb(&input);
        state.squeeze(self.hash_len)
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ascon_tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut state = AsconHashState::default();
        state.state = [0x00400c0000000100, 0, 0, 0, 0];
        state.rounds_a();
        assert_eq!(
            [
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140
            ],
            state.state
        )
    }
}

crate::basic_hash_tests!(
    AsconHash::default(), ascon_0, "",
    "7346bc14f036e87ae03d0997913088f5f68411434b3cf8b54fa796a80d251f91";
    AsconHash::default(), ascon_1, "00",
    "8dd446ada58a7740ecf56eb638ef775f7d5c0fd5f0c2bbbdfdec29609d3c43a2";
    AsconHash::default(), ascon_2, "0001",
    "f77ca13bf89146d3254f1cfb7eddba8fa1bf162284bb29e7f645545cf9e08424";
    AsconHash::default(), ascon_7, "000102030405",
    "9c52142852beb6654907cc23cc5b171075d411ca80082aafd7dd0d09ba0bba1d";
    AsconHash::default(), ascon_8, "00010203040506",
    "dd409ccc0c60cd7f474c0beed1e1cd48140ad45d5136dc5fda5ebe283df8d3f6";
    AsconHash::default(), ascon_1025, "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
    "2eb89744de7f9a6f47d53db756bb2f67b127da96762a1c47a5d7bfc1f7273f5c";

);
