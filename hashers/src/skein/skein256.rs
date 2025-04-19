use super::Tweak;
use crate::{skein::C240, traits::StatefulHasher};
use std::num::Wrapping as W;

const WORDS: usize = 4;
const ROUNDS: usize = 72;
const N_OCTO_ROUNDS: usize = ROUNDS / 4;

pub struct Skein256 {
    chain: [W<u64>; WORDS],
    ex_key: [W<u64>; WORDS + 1],
    tweak: Tweak,
}

impl Default for Skein256 {
    fn default() -> Self {
        Self::init_128([0; WORDS])
    }
}

impl Skein256 {
    fn init(iv: [u64; WORDS], key: [u64; WORDS], tweak: Tweak) -> Self {
        let mut ex_key = [W(0); WORDS + 1];
        ex_key[4] = W(crate::skein::C240);
        for i in 0..WORDS {
            ex_key[i] = W(key[i]);
            ex_key[4].0 ^= key[i];
        }
        Self {
            chain: iv.map(|n| W(n)),
            ex_key,
            tweak,
        }
    }

    pub fn init_128(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                0xE1111906964D7260,
                0x883DAAA77C8D811C,
                0x10080DF491960F7A,
                0xCCF7DDE5B45BC1C2,
            ],
            key,
            Tweak::new(),
        )
    }

    pub fn init_160(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                0x1420231472825E98,
                0x2AC4E9A25A77E590,
                0xD47A58568838D63E,
                0x2DD2E4968586AB7D,
            ],
            key,
            Tweak::new(),
        )
    }

    pub fn init_224(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                0xC6098A8C9AE5EA0B,
                0x876D568608C5191C,
                0x99CB88D7D7F53884,
                0x384BDDB1AEDDB5DE,
            ],
            key,
            Tweak::new(),
        )
    }

    pub fn init_256(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                0xFC9DA860D048B449,
                0x2FCA66479FA7D833,
                0xB33BC3896656840F,
                0x6A54E920FDE8DA69,
            ],
            key,
            Tweak::new(),
        )
    }
}

impl StatefulHasher for Skein256 {
    fn update(&mut self, mut bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_256_empty, Skein256::init_256([0;WORDS]), b"",
    "c8877087da56e072870daa843f176e9453115929094c3a40c463a196c29bf7ba";

);
