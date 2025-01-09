// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use super::helpers::{hash32_0_to_4, hash32_13_to_24, hash32_25, hash32_5_to_12};
use crate::traits::{ResettableHasher, SimpleHasher, StatefulHasher};

pub struct CityHash32 {
    buffer: Vec<u8>,
}

impl CityHash32 {
    pub fn init() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl SimpleHasher for CityHash32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=4 => hash32_0_to_4(bytes),
            5..=12 => hash32_5_to_12(bytes),
            13..=24 => hash32_13_to_24(bytes),
            _ => hash32_25(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }
}

impl StatefulHasher for CityHash32 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        match self.buffer.len() {
            0..=4 => hash32_0_to_4(&self.buffer),
            5..=12 => hash32_5_to_12(&self.buffer),
            13..=24 => hash32_13_to_24(&self.buffer),
            _ => hash32_25(&self.buffer),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::stateful_hash_helpers!();
}

impl ResettableHasher for CityHash32 {
    fn finalize_and_reset(&mut self) -> Vec<u8> {
        let out = match self.buffer.len() {
            0..=4 => hash32_0_to_4(&self.buffer),
            5..=12 => hash32_5_to_12(&self.buffer),
            13..=24 => hash32_13_to_24(&self.buffer),
            _ => hash32_25(&self.buffer),
        }
        .to_be_bytes()
        .to_vec();
        self.buffer.clear();
        out
    }
}
