use utils::byte_formatting::make_u64s_be;

use crate::traits::StatefulHasher;

use super::{padded_bytes_256, padded_bytes_320, padded_bytes_64, Variant, C, ROTS};

pub struct Ascon {
    hash_len: usize,
    state: [u64; 5],
    buffer: Vec<u8>,
    variant: Variant,
}

impl Ascon {
    // 256-bit hash
    pub fn init_hash() -> Self {
        Self {
            hash_len: 32,
            state: [
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ],
            buffer: Vec::new(),
            variant: Variant::Hash,
        }
    }

    // 256-bit hash
    pub fn init_hasha() -> Self {
        Self {
            hash_len: 32,
            state: [
                0x01470194fc6528a6,
                0x738ec38ac0adffa7,
                0x2ec8e3296c76384c,
                0xd6f6a54d7f52377d,
                0xa13c42a223be8d87,
            ],
            buffer: Vec::new(),
            variant: Variant::Hasha,
        }
    }

    // 128-bit MAC
    pub fn init_mac(key: [u8; 16]) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len: 16,
            state: [0x80808c0000000080, key[0], key[1], 0, 0],
            buffer: Vec::new(),
            variant: Variant::Mac,
        };
        h.rounds_12();
        h
    }

    // 128-bit MAC
    pub fn init_maca(key: [u8; 16]) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len: 16,
            state: [0x80808c0400000080, key[0], key[1], 0, 0],
            buffer: Vec::new(),
            variant: Variant::Maca,
        };
        h.rounds_12();
        h
    }

    // Arbitrary length PRF
    pub fn init_prf(key: [u8; 16], hash_len: usize) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len,
            state: [0x80808c0000000000, key[0], key[1], 0, 0],
            buffer: Vec::new(),
            variant: Variant::Prf,
        };
        h.rounds_12();
        h
    }

    // Arbitrary length PRF
    pub fn init_prfa(key: [u8; 16], hash_len: usize) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len,
            state: [0x80808c0400000000, key[0], key[1], 0, 0],
            buffer: Vec::new(),
            variant: Variant::Prfa,
        };
        h.rounds_12();
        h
    }

    // Arbitrary length XOF
    pub fn init_xof(hash_len: usize) -> Self {
        Self {
            hash_len,
            state: [
                0xb57e273b814cd416,
                0x2b51042562ae2420,
                0x66a3a7768ddf2218,
                0x5aad0a7a8153650c,
                0x4f3e0e32539493b6,
            ],
            buffer: Vec::new(),
            variant: Variant::Xof,
        }
    }

    // Arbitrary length XOF
    pub fn init_xofa(hash_len: usize) -> Self {
        Self {
            hash_len,
            state: [
                0x44906568b77b9832,
                0xcd8d6cae53455532,
                0xf7b5212756422129,
                0x246885e1de0d225b,
                0xa8cb5ce33449973f,
            ],
            buffer: Vec::new(),
            variant: Variant::Xofa,
        }
    }

    pub fn hash(bytes: &[u8]) -> Vec<u8> {
        let mut h = Self::init_hash();
        h.update(bytes);
        h.finalize()
    }

    pub fn hasha(bytes: &[u8]) -> Vec<u8> {
        let mut h = Self::init_hasha();
        h.update(bytes);
        h.finalize()
    }

    pub fn mac(bytes: &[u8], key: [u8; 16]) -> Vec<u8> {
        let mut h = Self::init_mac(key);
        h.update(bytes);
        h.finalize()
    }

    pub fn maca(bytes: &[u8], key: [u8; 16]) -> Vec<u8> {
        let mut h = Self::init_maca(key);
        h.update(bytes);
        h.finalize()
    }

    pub fn xof(bytes: &[u8], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_xof(hash_len);
        h.update(bytes);
        h.finalize()
    }

    pub fn xofa(bytes: &[u8], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_xofa(hash_len);
        h.update(bytes);
        h.finalize()
    }

    pub fn prf(bytes: &[u8], key: [u8; 16], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_prf(key, hash_len);
        h.update(bytes);
        h.finalize()
    }

    pub fn prfa(bytes: &[u8], key: [u8; 16], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_prfa(key, hash_len);
        h.update(bytes);
        h.finalize()
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

    pub fn rounds(&mut self, n: usize) {
        match n {
            6 => self.rounds_6(),
            8 => self.rounds_8(),
            12 => self.rounds_12(),
            _ => unreachable!("only 6, 8, and 12 should be possible"),
        }
    }

    pub fn transform(&mut self, i: usize) {
        // round constant
        self.state[2] ^= C[i];
        // substitution
        self.sbox();
        // linear diffusion
        self.linear_diffusor();
    }

    // The sbox works across words
    // It effectively take the nth bit of each word, interprets it as a 5-bit word, then substitutes it
    pub fn sbox(&mut self) {
        self.state[0] ^= self.state[4];
        self.state[4] ^= self.state[3];
        self.state[2] ^= self.state[1];

        let mut t = self.state.clone();
        for i in 0..5 {
            t[i] ^= !self.state[(i + 1) % 5] & self.state[(i + 2) % 5];
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
            self.state[i] ^=
                self.state[i].rotate_right(ROTS[i].0) ^ self.state[i].rotate_right(ROTS[i].1);
        }
    }

    pub fn absorb(&mut self) {
        let rate = self.variant.rate();
        let mut mlen = self.buffer.len();
        let mut ptr = 0;
        match self.variant {
            Variant::Hash | Variant::Hasha | Variant::Xof | Variant::Xofa => {
                while mlen >= rate {
                    self.state[0] ^= padded_bytes_64(&self.buffer[ptr..ptr + rate]);
                    self.rounds(self.variant.a());
                    ptr += rate;
                    mlen -= rate;
                }
            }

            Variant::Mac | Variant::Prf => {
                while mlen >= rate {
                    let [x0, x1, x2, x3] = padded_bytes_256(&self.buffer[ptr..ptr + rate]);
                    self.state[0] ^= x0;
                    self.state[1] ^= x1;
                    self.state[2] ^= x2;
                    self.state[3] ^= x3;
                    self.rounds(self.variant.a());
                    ptr += rate;
                    mlen -= rate;
                }
            }
            Variant::Maca | Variant::Prfa => {
                while mlen >= rate {
                    let [x0, x1, x2, x3, x4] = padded_bytes_320(&self.buffer[ptr..ptr + rate]);
                    self.state[0] ^= x0;
                    self.state[1] ^= x1;
                    self.state[2] ^= x2;
                    self.state[3] ^= x3;
                    self.state[4] ^= x4;
                    self.rounds(self.variant.a());
                    ptr += rate;
                    mlen -= rate;
                }
            }
        }
        self.buffer = self.buffer[ptr..].to_vec();
    }

    pub fn absorb_final_chunk(&mut self) {
        match self.variant {
            Variant::Hash | Variant::Hasha | Variant::Xof | Variant::Xofa => {
                self.state[0] ^= padded_bytes_64(&self.buffer);
                self.rounds_12();
            }

            Variant::Mac | Variant::Prf => {
                let [x0, x1, x2, x3] = padded_bytes_256(&self.buffer);
                self.state[0] ^= x0;
                self.state[1] ^= x1;
                self.state[2] ^= x2;
                self.state[3] ^= x3;
                self.state[4] ^= 1;
            }

            Variant::Maca | Variant::Prfa => {
                let [x0, x1, x2, x3, x4] = padded_bytes_320(&self.buffer);
                self.state[0] ^= x0;
                self.state[1] ^= x1;
                self.state[2] ^= x2;
                self.state[3] ^= x3;
                self.state[4] ^= x4;
                self.state[5] ^= 1;
            }
        }
    }

    fn squeeze(&mut self, output: &mut Vec<u8>) {
        match self.variant {
            Variant::Hash | Variant::Hasha | Variant::Xof | Variant::Xofa => {
                output.extend(self.state[0].to_be_bytes());
                self.rounds(self.variant.b());
            }
            Variant::Mac | Variant::Maca | Variant::Prf | Variant::Prfa => {
                output.extend(self.state[0].to_be_bytes());
                output.extend(self.state[1].to_be_bytes());
                self.rounds(self.variant.b());
            }
        }
    }
}

impl StatefulHasher for Ascon {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        self.absorb();
    }

    // Absorb the last block and squeeze the output
    fn finalize(mut self) -> Vec<u8> {
        self.absorb_final_chunk();
        let mut output = Vec::with_capacity(self.hash_len);

        while output.len() < self.hash_len {
            self.squeeze(&mut output);
        }

        output.truncate(self.hash_len);
        output
    }

    crate::stateful_hash_helpers!();
}
