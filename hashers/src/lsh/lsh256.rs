use utils::byte_formatting::{fill_u32s_be, fill_u32s_le};

use super::lsh256_consts::{
    ALPHA, BETA, CN_WORDS, CV_WORDS, GAMMA, LSH_256_256_IV, MB_WORDS, PERM_SIGMA, PERM_TAU, SC,
    STEPS,
};
use crate::{lsh::lsh256_consts::MB_BYTES, traits::StatefulHasher};

fn message_expand(mb: &[u32; MB_WORDS], arr: &mut [[u32; CV_WORDS]; STEPS + 1]) {
    arr[0].copy_from_slice(&mb[0..16]);
    arr[1].copy_from_slice(&mb[16..32]);
    for i in 2..STEPS {
        for l in 0..16 {
            arr[i][l] = arr[i - 1][l].wrapping_add(arr[i - 2][PERM_TAU[l]])
        }
    }
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

fn step(t: &mut [u32; CV_WORDS], m: [u32; CV_WORDS], j: usize) {
    *t = message_add(*t, m);
    for l in 0..(CV_WORDS / 2) {
        t[l] = t[l].wrapping_add(t[l + 8]);
        t[l] = t[l].rotate_left(ALPHA[j % 2]);
        t[l] = t[l].wrapping_add(SC[CN_WORDS * j + l]);
        t[l + 8] = t[l + 8].wrapping_add(t[l]);
        t[l + 8] = t[l + 8].rotate_left(BETA[j % 2]);
        t[l] = t[l].wrapping_add(t[l + 8]);
        t[l + 8] = t[l + 8].rotate_left(GAMMA[l]);
    }
    *t = message_perm(*t);
}

fn compress(
    cv: &mut [u32; CV_WORDS],
    mb: &[u32; MB_WORDS],
    arr: &mut [[u32; CV_WORDS]; STEPS + 1],
) {
    message_expand(mb, arr);
    let mut t = cv.clone();
    for j in 0..STEPS {
        step(&mut t, arr[j], j);
    }
    *cv = message_add(t, arr[STEPS]);
}

pub struct Lsh256_256 {
    chain_value: [u32; CV_WORDS],
    buffer: Vec<u8>,
    // bits_taken: u64,
}

impl Default for Lsh256_256 {
    fn default() -> Self {
        Self {
            chain_value: LSH_256_256_IV,
            buffer: Vec::new(),
            // bits_taken: 0,
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
        let mut arr = [[0_u32; CV_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        crate::compression_routine!(self.buffer, bytes, MB_BYTES, {
            fill_u32s_le(&mut mb, &self.buffer);
            compress(&mut self.chain_value, &mb, &mut arr);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        self.buffer.push(0x80);
        while self.buffer.len() % MB_BYTES != 0 {
            self.buffer.push(0x00);
        }
        let mut arr = [[0_u32; CV_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        fill_u32s_le(&mut mb, &self.buffer);
        compress(&mut self.chain_value, &mb, &mut arr);

        let mut h = [0; 8];
        for i in 0..8 {
            h[i] = self.chain_value[i] ^ self.chain_value[i + 8];
        }
        let mut out = Vec::with_capacity(32 * 8);
        for word in h {
            out.extend(word.to_le_bytes())
        }
        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_256, Lsh256_256::init(), b"abc",
    "5fbf365daea5446a7053c52b57404d77a07a5f48a1f7c1963a0898ba1b714741";
);
