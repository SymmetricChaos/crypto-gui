use super::lsh256_consts::{
    ALPHA, BETA, CV_WORDS, GAMMA, LSH_256_256_IV, MB_WORDS, PERM_SIGMA, SC,
};
use crate::traits::StatefulHasher;

macro_rules! mix {
    (x: ident, y: ident, j: ident, l: ident) => {
        x = x.wrapping_add(y);
        x = x.rotate_left(ALPHA[j % 2]);
        x = x.wrapping_add(SC[CV_WORDS * j + l]);
        y = y.wrapping_add(x);
        y = y.rotate_left(BETA[j % 2]);
        x = x.wrapping_add(y);
        y = y.rotate_left(GAMMA[l]);
    };
}

macro_rules! message_expand {
    () => {};
}

fn message_perm(x: [u32; CV_WORDS]) -> [u32; CV_WORDS] {
    let mut out = [0; CV_WORDS];
    for (i, sigma) in PERM_SIGMA.into_iter().enumerate() {
        out[i] = x[sigma]
    }
    out
}

fn message_add(x: [u32; CV_WORDS], y: [u32; CV_WORDS]) -> [u32; CV_WORDS] {
    let mut out = [0; CV_WORDS];
    for i in 0..CV_WORDS {
        out[i] = x[i] ^ y[i]
    }
    out
}

fn compress(cv: &mut [u32; CV_WORDS], mb: &Vec<u8>) -> [u32; CV_WORDS] {
    todo!()
}

pub struct Lsh256_256 {
    chain_value: [u32; CV_WORDS],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl Default for Lsh256_256 {
    fn default() -> Self {
        Self {
            chain_value: LSH_256_256_IV,
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }
}

impl Lsh256_256 {
    pub fn init() -> Self {
        Self::default()
    }
}

impl StatefulHasher for Lsh256_256 {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, MB_WORDS * 4, {
            self.bits_taken += 512;
            compress(&mut self.chain_value, &self.buffer);
        });
    }

    fn finalize(self) -> Vec<u8> {
        let mut h = [0; 8];
        for i in 0..8 {
            h[i] = self.chain_value[i] ^ self.chain_value[i + 8];
        }
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_256, Lsh256_256::init(), b"abc",
    "f7c53ba4034e708e74fba42e55997ca5126bb7623688f85342f73732";
);
