use utils::byte_formatting::fill_u64s_le;

use crate::traits::StatefulHasher;

use super::{octo_round_1024, Tweak, CFG, CFG_LEN, FINAL, FIRST, MSG, OUT};

const BLOCK_WORDS: usize = 16;
const BLOCK_BYTES: usize = BLOCK_WORDS * 8;
const KEY_WORDS: usize = 16;
const ROUNDS: usize = 80;
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
        octo_round_1024(block, &subkeys[(2 * r)..][..2]);
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

pub struct Skein1024 {
    chain: [u64; BLOCK_WORDS],
    ex_key: [u64; KEY_WORDS + 1],
    tweak: Tweak,
    buffer: Vec<u8>,
    hash_len: usize,
}

impl Default for Skein1024 {
    fn default() -> Self {
        Self::init_512()
    }
}

impl Skein1024 {
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

    pub fn init_384() -> Self {
        Self::init(
            [
                0x5102B6B8C1894A35,
                0xFEEBC9E3FE8AF11A,
                0x0C807F06E32BED71,
                0x60C13A52B41A91F6,
                0x9716D35DD4917C38,
                0xE780DF126FD31D3A,
                0x797846B6C898303A,
                0xB172C2A8B3572A3B,
                0xC9BC8203A6104A6C,
                0x65909338D75624F4,
                0x94BCC5684B3F81A0,
                0x3EBBF51E10ECFD46,
                0x2DF50F0BEEB08542,
                0x3B5A65300DBC6516,
                0x484B9CD2167BBCE1,
                0x2D136947D4CBAFEA,
            ],
            384,
        )
    }

    pub fn init_512() -> Self {
        Self::init(
            [
                0xCAEC0E5D7C1B1B18,
                0xA01B0E045F03E802,
                0x33840451ED912885,
                0x374AFB04EAEC2E1C,
                0xDF25A0E2813581F7,
                0xE40040938B12F9D2,
                0xA662D539C2ED39B6,
                0xFA8B85CF45D8C75A,
                0x8316ED8E29EDE796,
                0x053289C02E9F91B8,
                0xC3F8EF1D6D518B73,
                0xBDCEC3C4D5EF332E,
                0x549A7E5222974487,
                0x670708725B749816,
                0xB9CD28FBF0581BD1,
                0x0E2940B815804974,
            ],
            512,
        )
    }

    pub fn init_1024() -> Self {
        Self::init(
            [
                0xD593DA0741E72355,
                0x15B5E511AC73E00C,
                0x5180E5AEBAF2C4F0,
                0x03BD41D3FCBCAFAF,
                0x1CAEC6FD1983A898,
                0x6E510B8BCDD0589F,
                0x77E2BDFDC6394ADA,
                0xC11E1DB524DCB0A3,
                0xD6D14AF9C6329AB5,
                0x6A9B0BFC6EB67E0D,
                0x9243C60DCCFF1332,
                0x1A1F1DDE743F02D4,
                0x0996753C10ED0BB8,
                0x6572DD22F2B4969A,
                0x61FD3062D00A579A,
                0x1DE0536E8682E539,
            ],
            1024,
        )
    }
}

impl StatefulHasher for Skein1024 {
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
}
