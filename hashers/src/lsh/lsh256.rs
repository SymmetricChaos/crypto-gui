use super::lsh256_consts::{ALPHA, BETA, GAMMA, LSH_256_256_IV, SC, STATE_WORDS};
use crate::traits::StatefulHasher;

macro_rules! mix {
    (x: ident, y: ident, j: ident, l: ident) => {
        x = x.wrapping_add(y);
        x = x.rotate_left(ALPHA[j % 2]);
        x = x.wrapping_add(SC[STATE_WORDS * j + l]);
        y = y.wrapping_add(x);
        y = y.rotate_left(BETA[j % 2]);
        x = x.wrapping_add(y);
        y = y.rotate_left(GAMMA[l]);
    };
}

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

impl Lsh256_256 {
    pub fn init() -> Self {
        Self::default()
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

crate::stateful_hash_tests!(
    test_256_256, Lsh256_256::init(), b"abc",
    "f7c53ba4034e708e74fba42e55997ca5126bb7623688f85342f73732";
);
