use crate::{skein::C240, traits::StatefulHasher};

const WORDS: usize = 4;
const ROUNDS: usize = 72;

pub struct Skein256 {
    state: [u64; WORDS],
    key: [u64; WORDS],
    tweak: [u64; 2],
    bytes_taken: u64,
}

impl Default for Skein256 {
    fn default() -> Self {
        Self::init_128()
    }
}

impl Skein256 {
    fn init(iv: [u64; WORDS]) -> Self {
        Self {
            state: iv,
            key: todo!(),
            tweak: todo!(),
            bytes_taken: 0,
        }
    }

    pub fn init_128() -> Self {
        Self::init([
            0xE1111906964D7260,
            0x883DAAA77C8D811C,
            0x10080DF491960F7A,
            0xCCF7DDE5B45BC1C2,
        ])
    }

    pub fn init_160() -> Self {
        Self::init([
            0x1420231472825E98,
            0x2AC4E9A25A77E590,
            0xD47A58568838D63E,
            0x2DD2E4968586AB7D,
        ])
    }

    pub fn init_224() -> Self {
        Self::init([
            0xC6098A8C9AE5EA0B,
            0x876D568608C5191C,
            0x99CB88D7D7F53884,
            0x384BDDB1AEDDB5DE,
        ])
    }

    pub fn init_256() -> Self {
        Self::init([
            0xFC9DA860D048B449,
            0x2FCA66479FA7D833,
            0xB33BC3896656840F,
            0x6A54E920FDE8DA69,
        ])
    }

    fn ksa(&self) -> [[u64; 4]; 19] {
        // XOR together all the key words and the C240 constant
        let knw = self.key.into_iter().fold(C240, |acc, w| acc ^ w);
        let t2 = self.tweak[0] ^ self.tweak[1];
        todo!()
    }

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
    test_256_256_empty, Skein256::init_256(), b"",
    "c8877087da56e072870daa843f176e9453115929094c3a40c463a196c29bf7ba";

);
