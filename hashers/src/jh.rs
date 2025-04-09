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
        nibbles[i] = t[i << 1];
        nibbles[i + 128] = t[(i << 1) + 1];
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
        round_constant[i] = t[i << 1];
        round_constant[i + 32] = t[(i << 1) + 1];
    }

    // Permutation Phi6
    for i in (32..64).step_by(2) {
        round_constant.swap(i, i + 1);
    }
}

pub struct Jh {
    state: [u64; 16],
    hash_len: usize,
    buffer: Vec<u8>,
    bits_taken: u128,
}

impl Default for Jh {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Jh {
    fn init(hash_len: usize) -> Self {
        Self {
            state: todo!(),
            hash_len,
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }

    pub fn init_224() -> Self {
        Self::init(224)
    }

    pub fn init_256() -> Self {
        Self::init(256)
    }

    pub fn init_384() -> Self {
        Self::init(384)
    }

    pub fn init_512() -> Self {
        Self::init(512)
    }
}

impl StatefulHasher for Jh {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            self.bits_taken += 512;
            todo!()
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bits_taken += self.buffer.len() as u128 * 8;
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00)
        }
        self.buffer.extend(self.bits_taken.to_be_bytes());
        todo!()
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
    "";
);
