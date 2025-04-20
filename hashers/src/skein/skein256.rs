use utils::byte_formatting::fill_u64s_le;

use super::Tweak;
use crate::traits::StatefulHasher;
use std::num::Wrapping as W;

const WORDS: usize = 4;
const BLOCK_SIZE: usize = WORDS * 8;
const ROUNDS: usize = 72;
const N_OCTO_ROUNDS: usize = ROUNDS / 4;

pub struct Skein256 {
    chain: [W<u64>; WORDS],
    ex_key: [W<u64>; WORDS + 1],
    tweak: Tweak,
    buffer: Vec<u8>,
}

impl Default for Skein256 {
    fn default() -> Self {
        Self::init_128([0; WORDS])
    }
}

impl Skein256 {
    fn init(iv: [W<u64>; WORDS], key: [W<u64>; WORDS], tweak: Tweak) -> Self {
        let mut ex_key = [W(0); WORDS + 1];
        ex_key[4] = W(crate::skein::C240);
        for i in 0..WORDS {
            ex_key[i] = key[i];
            ex_key[4].0 ^= key[i].0;
        }

        let mut cfg = [0; BLOCK_SIZE];
        cfg[..8].copy_from_slice(&crate::skein::SCHEMA_VERSION.to_le_bytes());
        cfg[8..16].copy_from_slice(&128_u64.to_le_bytes());
        cfg[16..24].copy_from_slice(&crate::skein::TREE_INFO.to_le_bytes());

        Self {
            chain: iv,
            ex_key,
            tweak,
            buffer: Vec::new(),
        }
    }

    pub fn init_128(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                W(0xE1111906964D7260),
                W(0x883DAAA77C8D811C),
                W(0x10080DF491960F7A),
                W(0xCCF7DDE5B45BC1C2),
            ],
            key.map(|k| W(k)),
            Tweak::new(),
        )
    }

    pub fn init_160(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                W(0x1420231472825E98),
                W(0x2AC4E9A25A77E590),
                W(0xD47A58568838D63E),
                W(0x2DD2E4968586AB7D),
            ],
            key.map(|k| W(k)),
            Tweak::new(),
        )
    }

    pub fn init_224(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                W(0xC6098A8C9AE5EA0B),
                W(0x876D568608C5191C),
                W(0x99CB88D7D7F53884),
                W(0x384BDDB1AEDDB5DE),
            ],
            key.map(|k| W(k)),
            Tweak::new(),
        )
    }

    pub fn init_256(key: [u64; WORDS]) -> Self {
        Self::init(
            [
                W(0xFC9DA860D048B449),
                W(0x2FCA66479FA7D833),
                W(0xB33BC3896656840F),
                W(0x6A54E920FDE8DA69),
            ],
            key.map(|k| W(k)),
            Tweak::new(),
        )
    }
}

impl StatefulHasher for Skein256 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut block = [0; WORDS];
        crate::compression_routine!(self.buffer, bytes, BLOCK_SIZE, {
            self.tweak.increment(BLOCK_SIZE as u64);
            fill_u64s_le(&mut block, &self.buffer);
            todo!("compression function")
        });
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
