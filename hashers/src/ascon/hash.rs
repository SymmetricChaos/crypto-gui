use utils::byte_formatting::make_u64s_be;

use crate::traits::StatefulHasher;

use super::{state::AsconState, Variant};

pub struct Ascon {
    hash_len: usize,
    state: AsconState,
    buffer: Vec<u8>,
    variant: Variant,
}

impl Ascon {
    pub fn init(hash_len: usize, state: [u64; 5], variant: Variant) -> Self {
        Self {
            hash_len,
            state: AsconState(state),
            buffer: Vec::new(),
            variant,
        }
    }

    // 256-bit hash
    pub fn init_hash() -> Self {
        Self {
            hash_len: 32,
            state: AsconState([
                0xee9398aadb67f03d,
                0x8bb21831c60f1002,
                0xb48a92db98d5da62,
                0x43189921b8f8e3e8,
                0x348fa5c9d525e140,
            ]),
            buffer: Vec::new(),
            variant: Variant::Hash,
        }
    }

    // 256-bit hash
    pub fn init_hasha() -> Self {
        Self {
            hash_len: 32,
            state: AsconState([
                0x01470194fc6528a6,
                0x738ec38ac0adffa7,
                0x2ec8e3296c76384c,
                0xd6f6a54d7f52377d,
                0xa13c42a223be8d87,
            ]),
            buffer: Vec::new(),
            variant: Variant::Hasha,
        }
    }

    // 128-bit MAC
    pub fn init_mac(key: [u8; 16]) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len: 16,
            state: AsconState([0x80808c0000000080, key[0], key[1], 0, 0]),
            buffer: Vec::new(),
            variant: Variant::Mac,
        };
        h.state.rounds_12();
        h
    }

    // 128-bit MAC
    pub fn init_maca(key: [u8; 16]) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len: 16,
            state: AsconState([0x80808c0400000080, key[0], key[1], 0, 0]),
            buffer: Vec::new(),
            variant: Variant::Maca,
        };
        h.state.rounds_12();
        h
    }

    // Arbitrary length PRF
    pub fn init_prf(key: [u8; 16], hash_len: usize) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len,
            state: AsconState([0x80808c0000000000, key[0], key[1], 0, 0]),
            buffer: Vec::new(),
            variant: Variant::Prf,
        };
        h.state.rounds_12();
        h
    }

    // Arbitrary length PRF
    pub fn init_prfa(key: [u8; 16], hash_len: usize) -> Self {
        let key: [u64; 2] = make_u64s_be(&key);
        let mut h = Self {
            hash_len,
            state: AsconState([0x80808c0400000000, key[0], key[1], 0, 0]),
            buffer: Vec::new(),
            variant: Variant::Prfa,
        };
        h.state.rounds_12();
        h
    }

    // Arbitrary length XOF
    pub fn init_xof(hash_len: usize) -> Self {
        Self {
            hash_len,
            state: AsconState([
                0xb57e273b814cd416,
                0x2b51042562ae2420,
                0x66a3a7768ddf2218,
                0x5aad0a7a8153650c,
                0x4f3e0e32539493b6,
            ]),
            buffer: Vec::new(),
            variant: Variant::Xof,
        }
    }

    // Arbitrary length XOF
    pub fn init_xofa(hash_len: usize) -> Self {
        Self {
            hash_len,
            state: AsconState([
                0x44906568b77b9832,
                0xcd8d6cae53455532,
                0xf7b5212756422129,
                0x246885e1de0d225b,
                0xa8cb5ce33449973f,
            ]),
            buffer: Vec::new(),
            variant: Variant::Xofa,
        }
    }

    // Helper function to immediately produce a 256-bit hash
    pub fn hash(bytes: &[u8]) -> Vec<u8> {
        let mut h = Self::init_hash();
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce a 256-bit hash with the faster settings
    pub fn hasha(bytes: &[u8]) -> Vec<u8> {
        let mut h = Self::init_hasha();
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce a 128-bit Message Authentication Code
    pub fn mac(bytes: &[u8], key: [u8; 16]) -> Vec<u8> {
        let mut h = Self::init_mac(key);
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce a 128-bit Message Authentication Code with the faster settings
    pub fn maca(bytes: &[u8], key: [u8; 16]) -> Vec<u8> {
        let mut h = Self::init_maca(key);
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce an output of arbitrary length from the eXtensible Output Function
    pub fn xof(bytes: &[u8], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_xof(hash_len);
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce an output of arbitrary length from the eXtensible Output Function with the faster settings
    pub fn xofa(bytes: &[u8], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_xofa(hash_len);
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce an output of arbitrary length from the Pseudo Random Function
    pub fn prf(bytes: &[u8], key: [u8; 16], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_prf(key, hash_len);
        h.update(bytes);
        h.finalize()
    }

    // Helper function to immediately produce an output of arbitrary length from the Pseudo Random Function with the faster settings
    pub fn prfa(bytes: &[u8], key: [u8; 16], hash_len: usize) -> Vec<u8> {
        let mut h = Self::init_prfa(key, hash_len);
        h.update(bytes);
        h.finalize()
    }

    pub fn absorb(&mut self) {
        match self.variant {
            Variant::Hash | Variant::Hasha | Variant::Xof | Variant::Xofa => {
                self.state
                    .absorb_64_partial(&mut self.buffer, self.variant.a());
            }

            Variant::Mac | Variant::Prf => {
                self.state
                    .absorb_256_partial(&mut self.buffer, self.variant.a());
            }
            Variant::Maca | Variant::Prfa => {
                self.state
                    .absorb_320_partial(&mut self.buffer, self.variant.a());
            }
        }
    }

    fn squeeze(&mut self, output: &mut Vec<u8>) {
        match self.variant {
            Variant::Hash | Variant::Hasha | Variant::Xof | Variant::Xofa => {
                self.state.absorb_64_final(&mut self.buffer);
                *output = self.state.squeeze_64(self.hash_len, self.variant.a())
            }
            Variant::Mac | Variant::Prf => {
                self.state.absorb_256_final(&mut self.buffer);
                *output = self.state.squeeze_128(self.hash_len, 12)
            }
            Variant::Maca | Variant::Prfa => {
                self.state
                    .absorb_320_final(&mut self.buffer, self.variant.a());
                *output = self.state.squeeze_128(self.hash_len, 12)
            }
        }
    }
}

impl StatefulHasher for Ascon {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        self.absorb();
        fn update(&mut self, mut bytes: &[u8]) {
            crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
                self.absorb();
            });
        }
    }

    // Absorb the last block and squeeze the output
    fn finalize(mut self) -> Vec<u8> {
        let mut output = Vec::new();
        self.squeeze(&mut output);
        output
    }

    crate::stateful_hash_helpers!();
}
