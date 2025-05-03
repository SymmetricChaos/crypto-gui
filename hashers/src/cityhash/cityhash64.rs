// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use super::helpers::{hash128_64, hash64_0_to_16, hash64_17_to_32, hash64_33_to_64, hash64_65, P2};
use crate::traits::{ResettableHasher, StatefulHasher};

pub struct CityHash64 {
    buffer: Vec<u8>,
    pub seeds: Option<[u64; 2]>,
}

impl CityHash64 {
    pub fn init(seeds: Option<[u64; 2]>) -> Self {
        Self {
            seeds,
            buffer: Vec::new(),
        }
    }

    pub fn init_with_seed(seed: u64) -> Self {
        Self {
            seeds: Some([P2, seed]),
            buffer: Vec::new(),
        }
    }

    pub fn init_unseeded() -> Self {
        Self {
            seeds: None,
            buffer: Vec::new(),
        }
    }
}

impl StatefulHasher for CityHash64 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        let h = match self.buffer.len() {
            0..=16 => hash64_0_to_16(&self.buffer),
            17..=32 => hash64_17_to_32(&self.buffer),
            33..=64 => hash64_33_to_64(&self.buffer),
            _ => hash64_65(&self.buffer),
        };
        if let Some([s0, s1]) = self.seeds {
            hash128_64(h.wrapping_sub(s0), s1)
        } else {
            h
        }
        .to_be_bytes()
        .to_vec()
    }
}

impl ResettableHasher for CityHash64 {
    fn finalize_and_reset(&mut self) -> Vec<u8> {
        let out = {
            let h = match self.buffer.len() {
                0..=16 => hash64_0_to_16(&self.buffer),
                17..=32 => hash64_17_to_32(&self.buffer),
                33..=64 => hash64_33_to_64(&self.buffer),
                _ => hash64_65(&self.buffer),
            };
            if let Some([s0, s1]) = self.seeds {
                hash128_64(h.wrapping_sub(s0), s1)
            } else {
                h
            }
            .to_be_bytes()
            .to_vec()
        };
        self.buffer.clear();
        out
    }
}
