use super::lsh256_consts::{
    ALPHA, BETA, BLOCK_WORDS, CHAIN_WORDS, CN_WORDS, GAMMA, LSH_256_224_IV, LSH_256_256_IV,
    PERM_SIGMA, PERM_TAU, SC, STEPS,
};
use crate::{lsh::lsh256_consts::BLOCK_BYTES, traits::StatefulHasher};
use utils::byte_formatting::fill_u32s_le;

// Probably more efficient to calculate this on the fly
fn message_expand(mb: &[u32; BLOCK_WORDS], arr: &mut [[u32; CHAIN_WORDS]; STEPS + 1]) {
    arr[0].copy_from_slice(&mb[0..16]);
    arr[1].copy_from_slice(&mb[16..32]);
    for j in 2..(STEPS + 1) {
        for l in 0..CHAIN_WORDS {
            arr[j][l] = arr[j - 1][l].wrapping_add(arr[j - 2][PERM_TAU[l]])
        }
    }
}

fn message_perm(x: [u32; CHAIN_WORDS]) -> [u32; CHAIN_WORDS] {
    let mut out = [0; CHAIN_WORDS];
    for (i, sigma) in PERM_SIGMA.into_iter().enumerate() {
        out[i] = x[sigma]
    }
    out
}

fn message_add(x: [u32; CHAIN_WORDS], y: [u32; CHAIN_WORDS]) -> [u32; CHAIN_WORDS] {
    let mut out = [0; CHAIN_WORDS];
    for i in 0..CHAIN_WORDS {
        out[i] = x[i] ^ y[i]
    }
    out
}

fn step(cv: &mut [u32; CHAIN_WORDS], m: [u32; CHAIN_WORDS], j: usize) {
    let mut vl: u32;
    let mut vr: u32;
    for l in 0..(CHAIN_WORDS / 2) {
        vl = cv[l] ^ m[l];
        vr = cv[l + 8] ^ m[l + 8];
        vl = vl.wrapping_add(vr).rotate_left(ALPHA[j % 2]) ^ SC[CN_WORDS * j + l];
        vr = vl.wrapping_add(vr).rotate_left(BETA[j % 2]);
        cv[l] = vl.wrapping_add(vr);
        cv[l + 8] = vr.rotate_left(GAMMA[l]);
    }
    *cv = message_perm(*cv);
}

fn compress(
    cv: &mut [u32; CHAIN_WORDS],
    mb: &[u32; BLOCK_WORDS],
    arr: &mut [[u32; CHAIN_WORDS]; STEPS + 1],
) {
    message_expand(mb, arr);
    for j in 0..STEPS {
        step(cv, arr[j], j);
    }
    *cv = message_add(*cv, arr[STEPS]);
}

pub struct Lsh256 {
    pub chain_value: [u32; CHAIN_WORDS],
    pub buffer: Vec<u8>,
    outlen: usize,
}

impl Default for Lsh256 {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Lsh256 {
    /// Initialize LSH-256-224
    pub fn init_224() -> Self {
        Self {
            chain_value: LSH_256_224_IV,
            buffer: Vec::new(),
            outlen: 28,
        }
    }

    /// Initialize LSH-256-256
    pub fn init_256() -> Self {
        Self {
            chain_value: LSH_256_256_IV,
            buffer: Vec::new(),
            outlen: 32,
        }
    }
}

impl StatefulHasher for Lsh256 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut arr = [[0_u32; CHAIN_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        crate::compression_routine!(self.buffer, bytes, BLOCK_BYTES, {
            fill_u32s_le(&mut mb, &self.buffer);
            compress(&mut self.chain_value, &mb, &mut arr);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        self.buffer.push(0x80);
        while self.buffer.len() % BLOCK_BYTES != 0 {
            self.buffer.push(0x00);
        }

        // Final compressions
        let mut arr = [[0_u32; CHAIN_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        fill_u32s_le(&mut mb, &self.buffer);
        compress(&mut self.chain_value, &mb, &mut arr);

        // Final output
        let mut h = [0; 8];
        for i in 0..8 {
            h[i] = self.chain_value[i] ^ self.chain_value[i + 8];
        }
        let mut out = Vec::with_capacity(32);
        for word in h {
            out.extend(word.to_le_bytes())
        }
        out.truncate(self.outlen);
        out
    }
}

crate::stateful_hash_tests!(
    test_256_224, Lsh256::init_224(), b"abc",
    "f7c53ba4034e708e74fba42e55997ca5126bb7623688f85342f73732";
    test_256_256, Lsh256::init_256(), b"abc",
    "5fbf365daea5446a7053c52b57404d77a07a5f48a1f7c1963a0898ba1b714741";
);
