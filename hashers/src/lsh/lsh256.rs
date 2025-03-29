use super::lsh256_consts::{ALPHA, BETA, LSH_256_256_IV, STATE_WORDS};
use crate::traits::StatefulHasher;

pub struct Lsh256_256 {
    state: [u32; STATE_WORDS],
}

impl Default for Lsh256_256 {
    fn default() -> Self {
        Self {
            state: LSH_256_256_IV,
        }
    }
}

impl StatefulHasher for Lsh256_256 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        let mut h = [0; 8];
        for i in 0..8 {
            h[i] = self.state[i] ^ self.state[i + 8];
        }
        todo!()
    }

    fn update_multiple(&mut self, bytes: &[&[u8]]) {
        todo!()
    }

    fn update_and_finalize(self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    fn update_multiple_and_finalize(self, bytes: &[&[u8]]) -> Vec<u8> {
        todo!()
    }
}
