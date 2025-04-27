use utils::byte_formatting::fill_u64s_le;

use crate::{skein::FIRST, traits::StatefulHasher};

use super::{octo_round_512, Tweak, CFG, CFG_LEN, FINAL, MSG, OUT};

const BLOCK_WORDS: usize = 8;
const BLOCK_BYTES: usize = BLOCK_WORDS * 8;
const KEY_WORDS: usize = 8;
const ROUNDS: usize = 72;
const SUBKEYS: usize = ROUNDS / 4 + 1;

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

fn encrypt_block(block: &mut [u64; BLOCK_WORDS], subkeys: &[[u64; KEY_WORDS]; SUBKEYS]) {
    for r in 0..(ROUNDS / 8) {
        octo_round_512(block, &subkeys[(2 * r)..][..2]);
    }

    for i in 0..BLOCK_WORDS {
        block[i] = block[i].wrapping_add(subkeys[ROUNDS / 4][i])
    }
}

fn compress(
    bytes: &[u8],
    len: u64, // needed when a block is padded
    tweak: &mut Tweak,
    block: &mut [u64; BLOCK_WORDS],
    subkeys: &mut [[u64; KEY_WORDS]; SUBKEYS],
    chain: &mut [u64; BLOCK_WORDS],
) {
    debug_assert!(bytes.len() == BLOCK_BYTES);

    let mut ex_key = [0; KEY_WORDS + 1];
    ex_key[KEY_WORDS] = crate::skein::C240;
    for i in 0..KEY_WORDS {
        ex_key[i] = chain[i];
        ex_key[KEY_WORDS] ^= chain[i];
    }

    tweak.increment(len);
    create_subkeys(subkeys, &ex_key, &tweak);
    fill_u64s_le(block, &bytes);

    let mut temp = block.clone();

    encrypt_block(&mut temp, &subkeys);

    // Compress into the chain value
    for i in 0..BLOCK_WORDS {
        chain[i] = block[i] ^ temp[i];
    }

    // Turn off the first block identifier
    // Easier to do this each round than detect the first round
    tweak[1] &= !FIRST;
}

pub struct Skein512 {
    chain: [u64; BLOCK_WORDS],
    ex_key: [u64; KEY_WORDS + 1],
    tweak: Tweak,
    buffer: Vec<u8>,
    hash_len: usize,
}

impl Default for Skein512 {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Skein512 {
    fn init(iv: [u64; KEY_WORDS], hash_len: u64) -> Self {
        // Create the key
        let mut ex_key = [0; KEY_WORDS + 1];
        ex_key[KEY_WORDS] = crate::skein::C240;
        for i in 0..KEY_WORDS {
            ex_key[i] = iv[i];
            ex_key[KEY_WORDS] ^= iv[i];
        }

        // Create the config string
        let mut cfg = [0; BLOCK_BYTES];
        cfg[..8].copy_from_slice(&crate::skein::SCHEMA_VERSION.to_le_bytes());
        cfg[8..16].copy_from_slice(&hash_len.to_le_bytes());
        cfg[16..24].copy_from_slice(&crate::skein::TREE_INFO.to_le_bytes());

        // Create a new state ready for the config string
        let mut state = Self {
            chain: [0u64; BLOCK_WORDS],
            ex_key,
            tweak: Tweak::blank_with_flags(FIRST | CFG | FINAL),
            buffer: Vec::new(),
            hash_len: (hash_len / 8) as usize,
        };

        // Create the initial chain value
        compress(
            &cfg,
            CFG_LEN,
            &mut state.tweak,
            &mut [0; BLOCK_WORDS],
            &mut [[0u64; KEY_WORDS]; SUBKEYS],
            &mut state.chain,
        );

        // Set the tweak for processing the message
        state.tweak = Tweak::blank_with_flags(FIRST | MSG);

        state
    }

    pub fn init_128() -> Self {
        Self::init(
            [
                0xA8BC7BF36FBF9F52,
                0x1E9872CEBD1AF0AA,
                0x309B1790B32190D3,
                0xBCFBB8543F94805C,
                0x0DA61BCD6E31B11B,
                0x1A18EBEAD46A32E3,
                0xA2CC5B18CE84AA82,
                0x6982AB289D46982D,
            ],
            128,
        )
    }

    pub fn init_160() -> Self {
        Self::init(
            [
                0x28B81A2AE013BD91,
                0xC2F11668B5BDF78F,
                0x1760D8F3F6A56F12,
                0x4FB747588239904F,
                0x21EDE07F7EAF5056,
                0xD908922E63ED70B8,
                0xB8EC76FFECCB52FA,
                0x01A47BB8A3F27A6E,
            ],
            160,
        )
    }

    pub fn init_224() -> Self {
        Self::init(
            [
                0xCCD0616248677224,
                0xCBA65CF3A92339EF,
                0x8CCD69D652FF4B64,
                0x398AED7B3AB890B4,
                0x0F59D1B1457D2BD0,
                0x6776FE6575D4EB3D,
                0x99FBC70E997413E9,
                0x9E2CFCCFE1C41EF7,
            ],
            224,
        )
    }

    pub fn init_256() -> Self {
        Self::init(
            [
                0xCCD044A12FDB3E13,
                0xE83590301A79A9EB,
                0x55AEA0614F816E6F,
                0x2A2767A4AE9B94DB,
                0xEC06025E74DD7683,
                0xE7A436CDC4746251,
                0xC36FBAF9393AD185,
                0x3EEDBA1833EDFC13,
            ],
            256,
        )
    }

    pub fn init_384() -> Self {
        Self::init(
            [
                0xA3F6C6BF3A75EF5F,
                0xB0FEF9CCFD84FAA4,
                0x9D77DD663D770CFE,
                0xD798CBF3B468FDDA,
                0x1BC4A6668A0E4465,
                0x7ED7D434E5807407,
                0x548FC1ACD4EC44D6,
                0x266E17546AA18FF8,
            ],
            384,
        )
    }

    pub fn init_512() -> Self {
        Self::init(
            [
                0x4903ADFF749C51CE,
                0x0D95DE399746DF03,
                0x8FD1934127C79BCE,
                0x9A255629FF352CB1,
                0x5DB62599DF6CA7B0,
                0xEABE394CA9D5C3F4,
                0x991112C71A75B523,
                0xAE18A40B660FCC33,
            ],
            512,
        )
    }
}

impl StatefulHasher for Skein512 {
    fn update(&mut self, mut bytes: &[u8]) {
        let mut block = [0; BLOCK_WORDS];
        let mut subkeys = [[0u64; KEY_WORDS]; SUBKEYS];
        crate::compression_routine!(self.buffer, bytes, BLOCK_BYTES, {
            compress(
                &self.buffer,
                BLOCK_BYTES as u64,
                &mut self.tweak,
                &mut block,
                &mut subkeys,
                &mut self.chain,
            );
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        // Set final flag for the tweak
        self.tweak[1] |= FINAL;

        // The tweak accounts for making the final block different so only zeroes are needed for padding
        let f_len = self.buffer.len();
        if self.buffer.is_empty() {
            self.buffer.push(0x00);
        }
        while self.buffer.len() % BLOCK_BYTES != 0 {
            self.buffer.push(0x00);
        }

        // Compress the final block
        let mut block = [0; BLOCK_WORDS];
        let mut subkeys = [[0u64; KEY_WORDS]; SUBKEYS];
        compress(
            &self.buffer,
            f_len as u64,
            &mut self.tweak,
            &mut block,
            &mut subkeys,
            &mut self.chain,
        );

        // Now the Threefish cipher is run in CTR mode to produce the output
        // Notice that the flags and chain value are the same for each block
        let flags = FIRST | OUT | FINAL;

        block = [0; BLOCK_WORDS];
        subkeys = [[0u64; KEY_WORDS]; SUBKEYS];
        create_subkeys(&mut subkeys, &self.ex_key, &self.tweak);
        let chain = self.chain;
        let mut ctr = [0_u8; BLOCK_BYTES];

        let mut out = vec![0; self.hash_len];

        for (i, chunk) in out.chunks_mut(BLOCK_BYTES).enumerate() {
            self.chain = chain;
            ctr[..8].copy_from_slice(&(i as u64).to_le_bytes());

            compress(
                &ctr,
                8 as u64,
                &mut Tweak::blank_with_flags(flags),
                &mut block,
                &mut subkeys,
                &mut self.chain,
            );

            for (source, target) in self.chain.iter().zip(chunk.chunks_exact_mut(8)) {
                target.copy_from_slice(&source.to_le_bytes());
            }
        }

        out.to_vec()
    }

    crate::stateful_hash_helpers!();
}
