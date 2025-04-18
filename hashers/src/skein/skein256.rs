use crate::{skein::C240, traits::StatefulHasher};

pub struct Skein256 {
    pub key: [u64; Self::WORDS],
    pub tweak: [u64; 2],
}

impl Default for Skein256 {
    fn default() -> Self {
        Self {
            key: [0; Self::WORDS],
            tweak: [0; 2],
        }
    }
}

impl Skein256 {
    const WORDS: usize = 4;
    const ROUNDS: usize = 72;

    pub fn init128() -> Self {
        todo!()
    }

    pub fn init160() -> Self {
        todo!()
    }

    pub fn init224() -> Self {
        todo!()
    }

    pub fn init256() -> Self {
        todo!()
    }

    fn ksa(&self) -> [[u64; 4]; 19] {
        // XOR together all the key words and the C240 constant
        let knw = self.key.into_iter().fold(C240, |acc, w| acc ^ w);
        let t2 = self.tweak[0] ^ self.tweak[1];
        todo!()
    }

    // fn encrypt_block(
    //     mut a: u64,
    //     mut b: u64,
    //     mut c: u64,
    //     mut d: u64,
    //     round_keys: [[u64; 4]; 19],
    // ) -> (u64, u64, u64, u64) {
    //     // First round key
    //     a = a.wrapping_add(round_keys[0][0]);
    //     b = b.wrapping_add(round_keys[0][1]);
    //     c = c.wrapping_add(round_keys[0][2]);
    //     d = d.wrapping_add(round_keys[0][3]);

    //     for i in 1..(Self::ROUNDS / 4 + 1) {
    //         (a, b, c, d) = four_rounds(a, b, c, d, round_keys[i]);
    //     }

    //     (a, b, c, d)
    // }

    // Unique Block Iteration
    // Incorporates the tweak information for each block to make each block and each mode unique
    fn ubi() {}
}

impl StatefulHasher for Skein256 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_256_empty, Skein256::init256(), b"",
    "c8877087da56e072870daa843f176e9453115929094c3a40c463a196c29bf7ba";

);
