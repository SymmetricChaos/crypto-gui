use super::lsh512_consts::{
    ALPHA, BETA, BLOCK_BYTES, BLOCK_WORDS, CHAIN_WORDS, CN_WORDS, GAMMA, LSH_512_224_IV,
    LSH_512_256_IV, LSH_512_384_IV, LSH_512_512_IV, PERM_SIGMA, PERM_TAU, SC, STEPS,
};
use crate::traits::StatefulHasher;
use utils::byte_formatting::fill_u64s_le;

// Probably more efficient to calculate this on the fly
fn message_expand(mb: &[u64; BLOCK_WORDS], arr: &mut [[u64; CHAIN_WORDS]; STEPS + 1]) {
    arr[0].copy_from_slice(&mb[0..16]);
    arr[1].copy_from_slice(&mb[16..32]);
    for j in 2..(STEPS + 1) {
        for l in 0..CHAIN_WORDS {
            arr[j][l] = arr[j - 1][l].wrapping_add(arr[j - 2][PERM_TAU[l]])
        }
    }
}

fn message_perm(x: [u64; CHAIN_WORDS]) -> [u64; CHAIN_WORDS] {
    let mut out = [0; CHAIN_WORDS];
    for (i, sigma) in PERM_SIGMA.into_iter().enumerate() {
        out[i] = x[sigma]
    }
    out
}

fn message_add(x: [u64; CHAIN_WORDS], y: [u64; CHAIN_WORDS]) -> [u64; CHAIN_WORDS] {
    let mut out = [0; CHAIN_WORDS];
    for i in 0..CHAIN_WORDS {
        out[i] = x[i] ^ y[i]
    }
    out
}

fn step(cv: &mut [u64; CHAIN_WORDS], m: [u64; CHAIN_WORDS], j: usize) {
    let mut vl: u64;
    let mut vr: u64;

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
    cv: &mut [u64; CHAIN_WORDS],
    mb: &[u64; BLOCK_WORDS],
    arr: &mut [[u64; CHAIN_WORDS]; STEPS + 1],
) {
    message_expand(mb, arr);
    for j in 0..STEPS {
        step(cv, arr[j], j);
    }
    *cv = message_add(*cv, arr[STEPS]);
}

pub struct Lsh512 {
    pub chain_value: [u64; CHAIN_WORDS],
    pub buffer: Vec<u8>,
    outlen: usize,
}

impl Default for Lsh512 {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Lsh512 {
    /// Initialize LSH-512-224
    pub fn init_224() -> Self {
        Self {
            chain_value: LSH_512_224_IV,
            buffer: Vec::new(),
            outlen: 28,
        }
    }

    /// Initialize LSH-512-256
    pub fn init_256() -> Self {
        Self {
            chain_value: LSH_512_256_IV,
            buffer: Vec::new(),
            outlen: 32,
        }
    }

    /// Initialize LSH-512-384
    pub fn init_384() -> Self {
        Self {
            chain_value: LSH_512_384_IV,
            buffer: Vec::new(),
            outlen: 48,
        }
    }

    /// Initialize LSH-512-512
    pub fn init_512() -> Self {
        Self {
            chain_value: LSH_512_512_IV,
            buffer: Vec::new(),
            outlen: 64,
        }
    }
}

impl StatefulHasher for Lsh512 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut arr = [[0_u64; CHAIN_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        crate::compression_routine!(self.buffer, bytes, BLOCK_BYTES, {
            fill_u64s_le(&mut mb, &self.buffer);
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
        let mut arr = [[0_u64; CHAIN_WORDS]; STEPS + 1];
        let mut mb = [0; 32];
        fill_u64s_le(&mut mb, &self.buffer);
        compress(&mut self.chain_value, &mb, &mut arr);

        // Final output
        let mut h = [0; 8];
        for i in 0..8 {
            h[i] = self.chain_value[i] ^ self.chain_value[i + 8];
        }
        let mut out = Vec::with_capacity(64);
        for word in h {
            out.extend(word.to_le_bytes())
        }
        out.truncate(self.outlen);
        out
    }
}

crate::stateful_hash_tests!(
    test_512_224, Lsh512::init_224(), b"abc",
    "d1683234513ec5698394571ead128a8cd5373e97661ba20dcf89e489";
    test_512_256, Lsh512::init_256(), b"abc",
    "cd892310532602332b613f1ec11a6962fca61ea09ecffcd4bcf75858d802edec";
    test_512_384, Lsh512::init_384(), b"abc",
    "5f344efaa0e43ccd2e5e194d6039794b4fb431f10fb4b65fd45e9da4ecde0f27b66e8dbdfa47252e0d0b741bfd91f9fe";
    test_512_512, Lsh512::init_512(), b"abc",
    "a3d93cfe60dc1aacdd3bd4bef0a6985381a396c7d49d9fd177795697c3535208b5c57224bef21084d42083e95a4bd8eb33e869812b65031c428819a1e7ce596d";
);
