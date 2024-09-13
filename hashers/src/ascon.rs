use crate::traits::ClassicHasher;
use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

fn padded_bytes_to_u64_be(bytes: &[u8]) -> u64 {
    if bytes.len() > 8 {
        panic!("input block was too large")
    } else if bytes.len() == 8 {
        u64::from_be_bytes(bytes.try_into().unwrap())
    } else {
        let mut word_bytes: [u8; 8] = [0; 8];
        for (word_byte, input_byte) in word_bytes.iter_mut().zip(bytes.iter()) {
            *word_byte = *input_byte;
        }
        word_bytes[bytes.len()] = 0x80;
        u64::from_be_bytes(word_bytes)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    AsconHash,
    AsconHasha,
    AsconXof,
    AsconXofa,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsconHash => write!(f, "Ascon-Hash"),
            Self::AsconHasha => write!(f, "Ascon-Hasha"),
            Self::AsconXof => write!(f, "Ascon-XOF"),
            Self::AsconXofa => write!(f, "Ascon-XOFa"),
        }
    }
}

impl Variant {
    pub fn initialize(&self) -> AsconState {
        match self {
            Variant::AsconHash => AsconState([
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ]),
            Variant::AsconHasha => AsconState([
                0x01470194fc6528a6,
                0x738ec38ac0adffa7,
                0x2ec8e3296c76384c,
                0xd6f6a54d7f52377d,
                0xa13c42a223be8d87,
            ]),
            Variant::AsconXof => AsconState([
                0xb57e273b814cd416,
                0x2b51042562ae2420,
                0x66a3a7768ddf2218,
                0x5aad0a7a8153650c,
                0x4f3e0e32539493b6,
            ]),
            Variant::AsconXofa => AsconState([
                0x44906568b77b9832,
                0xcd8d6cae53455532,
                0xf7b5212756422129,
                0x246885e1de0d225b,
                0xa8cb5ce33449973f,
            ]),
        }
    }
}

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone)]
pub struct AsconState([u64; 5]);

// Shortcut indexing
impl std::ops::Index<usize> for AsconState {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for AsconState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl AsconState {
    const RATE: usize = 8; // number of bytes absorbed at a time

    // Initial state for Ascon-Hash
    pub fn initialize(iv: u64) -> Self {
        let mut out = Self([iv, 0, 0, 0, 0]);
        out.rounds_12();
        out
    }

    pub fn rounds_12(&mut self) {
        for i in 0..12 {
            self.transform(i as usize);
        }
    }

    pub fn rounds_8(&mut self) {
        for i in 0..8 {
            self.transform((i + 4) as usize);
        }
    }

    pub fn rounds_6(&mut self) {
        for i in 0..6 {
            self.transform((i + 6) as usize);
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

        let mut t = self.clone();
        for i in 0..5 {
            t[i] ^= !self[(i + 1) % 5] & self[(i + 2) % 5];
        }

        t[1] ^= t[0];
        t[0] ^= t[4];
        t[3] ^= t[2];
        t[2] = !t[2];

        *self = t;
    }

    // This diffuses bits within each word of state
    pub fn linear_diffusor(&mut self) {
        for i in 0..5 {
            self[i] ^= self[i].rotate_right(ROTS[i].0) ^ self[i].rotate_right(ROTS[i].1);
        }
    }

    pub fn absorb(&mut self, message: &[u8], variant: Variant) {
        let rate = Self::RATE;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            self[0] ^= padded_bytes_to_u64_be(&message[ptr..ptr + rate]);
            ptr += rate;
            mlen -= rate;
            match variant {
                Variant::AsconHash | Variant::AsconXof => self.rounds_12(),
                Variant::AsconHasha | Variant::AsconXofa => self.rounds_8(),
            }
        }
        // Absorb the last padded block
        self[0] ^= padded_bytes_to_u64_be(&message[ptr..]);
        self.rounds_12();
    }

    pub fn squeeze(&mut self, hash_len: usize, variant: Variant) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            output.extend_from_slice(&self[0].to_be_bytes());
            match variant {
                Variant::AsconHash | Variant::AsconXof => self.rounds_12(),
                Variant::AsconHasha | Variant::AsconXofa => self.rounds_8(),
            }
        }

        output.truncate(hash_len);
        output
    }
}

pub struct Ascon {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: usize,
    pub variant: Variant,
}

impl Default for Ascon {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHash,
        }
    }
}

impl Ascon {
    pub fn ascon_hash() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHash,
        }
    }
    pub fn ascon_hasha() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
            variant: Variant::AsconHasha,
        }
    }
    pub fn ascon_xof(hash_len: usize) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len,
            variant: Variant::AsconXof,
        }
    }
    pub fn ascon_xofa(hash_len: usize) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len,
            variant: Variant::AsconXofa,
        }
    }
}

impl ClassicHasher for Ascon {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = self.variant.initialize();
        state.absorb(&bytes, self.variant);
        state.squeeze(self.hash_len, self.variant)
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod ascon_tests {
    use super::*;

    #[test]
    fn test_initialization_hash() {
        assert_eq!(
            AsconState::initialize(0x00400c0000000100).0,
            Variant::AsconHash.initialize().0
        )
    }

    #[test]
    fn test_initialization_xof() {
        assert_eq!(
            AsconState::initialize(0x00400c0000000000).0,
            Variant::AsconXof.initialize().0
        )
    }
}

crate::basic_hash_tests!(
    Ascon::ascon_hash(), ascon_0, "",
    "7346bc14f036e87ae03d0997913088f5f68411434b3cf8b54fa796a80d251f91";
    Ascon::ascon_hash(), ascon_1, "00",
    "8dd446ada58a7740ecf56eb638ef775f7d5c0fd5f0c2bbbdfdec29609d3c43a2";
    Ascon::ascon_hash(), ascon_2, "0001",
    "f77ca13bf89146d3254f1cfb7eddba8fa1bf162284bb29e7f645545cf9e08424";
    Ascon::ascon_hash(), ascon_7, "000102030405",
    "9c52142852beb6654907cc23cc5b171075d411ca80082aafd7dd0d09ba0bba1d";
    Ascon::ascon_hash(), ascon_8, "00010203040506",
    "dd409ccc0c60cd7f474c0beed1e1cd48140ad45d5136dc5fda5ebe283df8d3f6";
    Ascon::ascon_hash(), ascon_1025, "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
    "2eb89744de7f9a6f47d53db756bb2f67b127da96762a1c47a5d7bfc1f7273f5c";


    Ascon::ascon_hasha(), ascon_a_0, "",
    "aecd027026d0675f9de7a8ad8ccf512db64b1edcf0b20c388a0c7cc617aaa2c4";
    Ascon::ascon_hasha(), ascon_a_1, "00",
    "5a55f0367763d334a3174f9c17fa476eb9196a22f10daf29505633572e7756e4";
    Ascon::ascon_hasha(), ascon_a_2, "0001",
    "4243fd3b872e1ed4013711382cba032fecb4147d840ddf8436172ac62d129bc4";
    Ascon::ascon_hasha(), ascon_a_7, "000102030405",
    "c9832114b471fb2024f736c4ef3ff1802850ced13abd8a2f75cfa1f9d19490e2";
    Ascon::ascon_hasha(), ascon_a_8, "00010203040506",
    "6b6ad8a90eab00dccc182df1cec764e706461e76d303863728b8590b772e9082";
    Ascon::ascon_hasha(), ascon_a_1025, "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
    "14f6a0c1e5751733955b820ca67bc89bb7eb7014c88caeb5f380d75eed484fe9";
);
