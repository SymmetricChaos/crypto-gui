use utils::byte_formatting::xor_into_bytes_strict;

use crate::traits::StatefulHasher;

const BLOCK_LEN: usize = 64;

const RC0: [u8; 64] = [
    0x6, 0xa, 0x0, 0x9, 0xe, 0x6, 0x6, 0x7, 0xf, 0x3, 0xb, 0xc, 0xc, 0x9, 0x0, 0x8, 0xb, 0x2, 0xf,
    0xb, 0x1, 0x3, 0x6, 0x6, 0xe, 0xa, 0x9, 0x5, 0x7, 0xd, 0x3, 0xe, 0x3, 0xa, 0xd, 0xe, 0xc, 0x1,
    0x7, 0x5, 0x1, 0x2, 0x7, 0x7, 0x5, 0x0, 0x9, 0x9, 0xd, 0xa, 0x2, 0xf, 0x5, 0x9, 0x0, 0xb, 0x0,
    0x6, 0x6, 0x7, 0x3, 0x2, 0x2, 0xa,
];

const SBOXES: [[u8; 16]; 2] = [
    [9, 0, 4, 11, 13, 12, 3, 15, 1, 10, 2, 6, 7, 5, 8, 14],
    [3, 12, 6, 13, 5, 7, 1, 9, 15, 2, 0, 4, 11, 10, 14, 8],
];

// Round function
// R8
pub fn round(nibbles: &mut [u8; 256], round_constant: [u8; 64]) {
    let mut t = [0u8; 256];
    for i in 0..256 {
        let select = ((round_constant[i >> 2] >> (3 - (i & 3))) & 1) as usize;
        t[i] = SBOXES[select][nibbles[i] as usize];
    }

    // Apply MDS matrix
    for i in (0..256).step_by(2) {
        t[i + 1] ^= (((t[i]) << 1) ^ ((t[i]) >> 3) ^ (((t[i]) >> 2) & 2)) & 0xf;
        t[i] ^= (((t[i + 1]) << 1) ^ ((t[i + 1]) >> 3) ^ (((t[i + 1]) >> 2) & 2)) & 0xf;
    }

    // Permutation Pi8
    for i in (0..256).step_by(4) {
        t.swap(i + 2, i + 3);
    }

    // Permutation P'8
    for i in 0..128 {
        nibbles[i] = t[2 * i];
        nibbles[i + 128] = t[(2 * i) + 1];
    }

    // Permutation Phi8
    for i in (128..256).step_by(2) {
        nibbles.swap(i, i + 1);
    }
}

// Generate the next round constant
pub fn next_rc(round_constant: &mut [u8; 64]) {
    let mut t = [0; 64];
    for i in 0..64 {
        t[i] = SBOXES[0][round_constant[i] as usize];
    }

    for i in (0..64).step_by(2) {
        t[i + 1] ^= (((t[i]) << 1) ^ ((t[i]) >> 3) ^ (((t[i]) >> 2) & 2)) & 0xf;
        t[i] ^= (((t[i + 1]) << 1) ^ ((t[i + 1]) >> 3) ^ (((t[i + 1]) >> 2) & 2)) & 0xf;
    }

    // Permutation Pi6
    for i in (0..64).step_by(4) {
        t.swap(i + 2, i + 3);
    }

    // Permutation P'6
    for i in 0..32 {
        round_constant[i] = t[2 * i];
        round_constant[i + 32] = t[(2 * i) + 1];
    }

    // Permutation Phi6
    for i in (32..64).step_by(2) {
        round_constant.swap(i, i + 1);
    }
}

// Group bits into nibbles
fn grouping(state: &[u8; 128], nibbles: &mut [u8; 256]) {
    let mut t = [0; 256];
    for i in 0..256 {
        let t0 = ((state[i / 8] >> (7 - (i & 7))) & 1) as u8;
        let t1 = ((state[(i + 256) / 8] >> (7 - (i & 7))) & 1) as u8;
        let t2 = ((state[(i + 512) / 8] >> (7 - (i & 7))) & 1) as u8;
        let t3 = ((state[(i + 768) / 8] >> (7 - (i & 7))) & 1) as u8;
        t[i] = (t0 << 3) | (t1 << 2) | (t2 << 1) | (t3 << 0);
    }
    // Direct translation of reference but this shouldn't need an extra step
    for i in 0..128 {
        nibbles[i * 2] = t[i];
        nibbles[(i * 2) + 1] = t[i + 128];
    }
}

// Degroup the nibbles into bytes
fn degrouping(state: &mut [u8; 128], nibbles: &[u8; 256]) {
    let mut t = [0; 256];

    for i in 0..128 {
        t[i] = nibbles[i * 2];
        t[i + 128] = nibbles[(i * 2) + 1];
    }
    for i in 0..256 {
        let t0 = (t[i] >> 3) & 1;
        let t1 = (t[i] >> 2) & 1;
        let t2 = (t[i] >> 1) & 1;
        let t3 = (t[i] >> 0) & 1;
        state[i * 8] |= t0 << (7 - (i & 7));
        state[(i + 256) * 8] |= t1 << (7 - (i & 7));
        state[(i + 512) * 8] |= t2 << (7 - (i & 7));
        state[(i + 768) * 8] |= t3 << (7 - (i & 7));
    }
}

// E8
fn bijective_function(state: &mut [u8; 128]) {
    let mut rc = RC0;
    let mut nibbles = [0; 256];
    grouping(&state, &mut nibbles);
    for _ in 0..41 {
        round(&mut nibbles, rc);
        next_rc(&mut rc);
    }
    degrouping(state, &nibbles);
}

// F8
fn compress(state: &mut [u8; 128], input: &[u8]) {
    xor_into_bytes_strict(&mut state[..64], &input);
    bijective_function(state);
    xor_into_bytes_strict(&mut state[64..], &input);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JhHashLen {
    L224,
    L256,
    L384,
    L512,
}

pub struct Jh {
    state: [u8; 128],
    hash_len: JhHashLen,
    buffer: Vec<u8>,
    bits_taken: u128,
}

impl Default for Jh {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Jh {
    fn init(hash_len: JhHashLen) -> Self {
        Self {
            state: todo!(),
            hash_len,
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }

    pub fn init_224() -> Self {
        Self::init(JhHashLen::L224)
    }

    pub fn init_256() -> Self {
        Self::init(JhHashLen::L256)
    }

    pub fn init_384() -> Self {
        Self::init(JhHashLen::L384)
    }

    pub fn init_512() -> Self {
        Self::init(JhHashLen::L512)
    }
}

impl StatefulHasher for Jh {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            self.bits_taken += 512;
            compress(&mut self.state, &self.buffer);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bits_taken += self.buffer.len() as u128 * 8;
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00)
        }
        self.buffer.extend(self.bits_taken.to_be_bytes());

        // There can be multiple final blocks after padding
        for chunk in self.buffer.chunks_exact(64) {
            compress(&mut self.state, &chunk);
        }

        match self.hash_len {
            JhHashLen::L224 => self.state[100..].to_vec(),
            JhHashLen::L256 => self.state[96..].to_vec(),
            JhHashLen::L384 => self.state[80..].to_vec(),
            JhHashLen::L512 => self.state[64..].to_vec(),
        }
    }

    crate::stateful_hash_helpers!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_generation() {
        let mut rc = RC0;
        for _ in 0..41 {
            next_rc(&mut rc);
        }
        assert_eq!(
            [
                0x7, 0x3, 0xb, 0xd, 0x6, 0x9, 0x7, 0x8, 0xc, 0x5, 0x9, 0xf, 0x2, 0xb, 0x2, 0x1,
                0x9, 0x4, 0x4, 0x9, 0xb, 0x3, 0x6, 0x7, 0x7, 0x0, 0xf, 0xb, 0x3, 0x1, 0x3, 0xf,
                0xb, 0xe, 0x2, 0xd, 0xa, 0x2, 0x8, 0xf, 0x6, 0xb, 0x0, 0x4, 0x2, 0x7, 0x5, 0xf,
                0x0, 0x7, 0x1, 0xa, 0x1, 0xb, 0x1, 0x9, 0x3, 0xd, 0xd, 0xe, 0x2, 0x0, 0x7, 0x2
            ],
            rc
        )
    }
}

crate::stateful_hash_tests!(
    test_1, Jh::init_256(), b"",
    "46e64619c18bb0a92a5e87185a47eef83ca747b8fcc8e1412921357e326df434";
    test_2, Jh::init_256(), b"The quick brown fox jumps over the lazy dog",
    "6a049fed5fc6874acfdc4a08b568a4f8cbac27de933496f031015b38961608a0";

);
