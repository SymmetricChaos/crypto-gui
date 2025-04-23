use utils::byte_formatting::{fill_u64s_le, make_u64s_le};

use super::{octo_round_256, Tweak};
use crate::{skein::FIRST, traits::StatefulHasher};

const BLOCK_WORDS: usize = 4;
const BLOCK_BYTES: usize = BLOCK_WORDS * 8;
const STATE_WORDS: usize = 4;
const STATE_BYTES: usize = STATE_WORDS * 8;
const KEY_WORDS: usize = 4;
const KEY_BYTES: usize = KEY_WORDS * 8;

const ROUNDS: usize = 72;
const SUBKEYS: usize = ROUNDS / 4 + 1;
const N_OCTO_ROUNDS: usize = ROUNDS / 4;

pub fn create_subkeys(
    subkeys: &mut [[u64; KEY_WORDS]; SUBKEYS],
    ex_key: &[u64; KEY_WORDS + 1],
    tweak: &Tweak,
) {
    let ex_tweak = tweak.extended();

    // The inner loop allows this to be reused for other key sizes
    for k in 0..SUBKEYS {
        for i in 0..KEY_WORDS {
            subkeys[k][i] = ex_key[(k + i) % (KEY_WORDS + 1)];
            if i == KEY_WORDS - 3 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[k % 3]);
            } else if i == KEY_WORDS - 2 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[(k + 1) % 3]);
            } else if i == KEY_WORDS - 1 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(k as u64);
            }
        }
    }
}

pub struct Skein256 {
    chain: [u64; STATE_WORDS],
    ex_key: [u64; KEY_WORDS + 1],
    tweak: Tweak,
    buffer: Vec<u8>,
}

impl Default for Skein256 {
    fn default() -> Self {
        Self::init_128([0; KEY_BYTES])
    }
}

impl Skein256 {
    fn init(iv: [u64; KEY_WORDS], key: [u8; KEY_BYTES], tweak: Tweak) -> Self {
        let key: [u64; KEY_WORDS] = make_u64s_le(&key);
        let mut ex_key = [0; KEY_WORDS + 1];
        ex_key[4] = crate::skein::C240;
        for i in 0..KEY_WORDS {
            ex_key[i] = key[i];
            ex_key[4] ^= key[i];
        }

        let mut cfg = [0; BLOCK_BYTES];
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

    pub fn init_128(key: [u8; KEY_BYTES]) -> Self {
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

    pub fn init_160(key: [u8; KEY_BYTES]) -> Self {
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

    pub fn init_224(key: [u8; KEY_BYTES]) -> Self {
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

    pub fn init_256(key: [u8; KEY_BYTES]) -> Self {
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

    fn encrypt_block(&self, block: &mut [u64; BLOCK_WORDS], subkeys: &[[u64; KEY_WORDS]; SUBKEYS]) {
        for r in 0..((N_OCTO_ROUNDS) / 2) {
            octo_round_256(block, &subkeys[(2 * r)..][..2]);
        }

        for i in 0..4 {
            block[i] = block[i].wrapping_add(subkeys[N_OCTO_ROUNDS][i])
        }
    }
}

impl StatefulHasher for Skein256 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut block = [0; BLOCK_WORDS];
        let mut subkeys = [[0u64; KEY_WORDS]; SUBKEYS];
        crate::compression_routine!(self.buffer, bytes, BLOCK_BYTES, {
            self.tweak.increment(BLOCK_BYTES as u64);

            create_subkeys(&mut subkeys, &self.ex_key, &self.tweak);
            fill_u64s_le(&mut block, &self.buffer);

            self.encrypt_block(&mut block, &subkeys);

            for i in 0..STATE_WORDS {
                self.chain[i] ^= block[i];
            }

            // Turn off the first block identifier
            // Easier to do this each round than detect the first round
            self.tweak[1] &= !FIRST;
        });
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_256_empty, Skein256::init_256([0; KEY_BYTES]), b"",
    "c8877087da56e072870daa843f176e9453115929094c3a40c463a196c29bf7ba";
);
