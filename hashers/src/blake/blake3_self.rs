// BLAKE3 but implemented myself rather than just copying the reference

use crate::{blake_double_round, traits::StatefulHasher};
use std::cmp::min;
use utils::byte_formatting::make_u32s_le;

// https://github.com/BLAKE3-team/BLAKE3
// https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf
// https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs

const OUT_LEN: usize = 32;
const KEY_LEN: usize = 32;

// Each chunk of 1024 bytes (256 words) is divided up into blocks of 64 bytes (16 words).
// Chunks are arranged into a tree structure while blocks are a simple array within each chunk
const BLOCK_LEN: usize = 64;
const CHUNK_LEN: usize = 1024;

// Bitflags that can be set for chunks
const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

// Same IV as BLAKE2s, sqrt of the first eight primes
const IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

const ROTS: [u32; 4] = [16, 12, 8, 7];
const WORD_ORDER: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn permute(m: &mut [u32; 16]) {
    let mut permuted = [0; 16];
    for i in 0..16 {
        permuted[i] = m[MSG_PERMUTATION[i]];
    }
    *m = permuted;
}

// The compression function.
// Compresses a block into a chaining value.
fn compress(
    chaining_value: &[u32; 8],
    block_words: &[u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
) -> [u32; 16] {
    let mut state = [
        chaining_value[0],
        chaining_value[1],
        chaining_value[2],
        chaining_value[3],
        chaining_value[4],
        chaining_value[5],
        chaining_value[6],
        chaining_value[7],
        IV[0],
        IV[1],
        IV[2],
        IV[3],
        counter as u32,
        (counter >> 32) as u32,
        block_len,
        flags,
    ];
    let mut block = *block_words;

    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);
    permute(&mut block);
    blake_double_round!(&mut state, &block, ROTS, WORD_ORDER);

    for i in 0..8 {
        state[i] ^= state[i + 8];
        state[i + 8] ^= chaining_value[i];
    }
    state
}

fn first_8_words(compression_output: [u32; 16]) -> [u32; 8] {
    compression_output[0..8].try_into().unwrap()
}

// A Chunk is similar to Blake2
// Each chunk is made of up to 1024 bytes divided into blocks of 64 butes
// The last block may be shorter than 64 bytes but can only be empty if the whole input is empty
// If the last block is less than 64 bytes it is padded with zeroes

pub struct Chunk {
    chaining_value: [u32; 8],
    chunk_counter: u64,
    flags: u32,
    blocks_compressed: u8,
}

impl Chunk {
    fn new(key_words: [u32; 8], chunk_counter: u64, flags: u32) -> Self {
        Self {
            chaining_value: key_words,
            chunk_counter,
            flags,
            blocks_compressed: 0,
        }
    }

    // Create the flag with CHUNK_START set if on the first block
    fn start_flag(&self) -> u32 {
        if self.blocks_compressed == 0 {
            CHUNK_START
        } else {
            0
        }
    }
}

impl StatefulHasher for Chunk {
    fn update(&mut self, bytes: &[u8]) {
        // while !input.is_empty() {
        //     // If the block buffer is full, compress it and clear it. More
        //     // input is coming, so this compression is not CHUNK_END.
        //     if self.block_len as usize == BLOCK_LEN {
        //         let block_words = make_u32s_le::<16>(&self.block);
        //         self.chaining_value = first_8_words(compress(
        //             &self.chaining_value,
        //             &block_words,
        //             self.chunk_counter,
        //             BLOCK_LEN as u32,
        //             self.flags | self.start_flag(), // if this is the first block the start flag if set for compress
        //         ));
        //         self.blocks_compressed += 1;
        //         self.block = [0; BLOCK_LEN];
        //         self.block_len = 0;
        //     }

        //     // Copy input bytes into the block buffer.
        //     let want = BLOCK_LEN - self.block_len as usize;
        //     let take = min(want, input.len());
        //     self.block[self.block_len as usize..][..take].copy_from_slice(&input[..take]);
        //     self.block_len += take as u8;
        //     input = &input[take..];
        // }
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Blake3Mode {
    Hash,
    KeyedHash,
    DeriveKey,
}

pub struct Blake3 {
    key_words: [u32; 8],
    flags: u32,
}

impl Blake3 {
    fn new_internal(key_words: [u32; 8], flags: u32) -> Self {
        Self { key_words, flags }
    }

    /// Construct a new Hasher for the regular hash function.
    pub fn new() -> Self {
        Self {
            key_words: [0u32; 8],
            flags: 0,
        }
    }

    /// Construct a new Hasher for the keyed hash function.
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        // The same as Self::new() but with the key material instead of the default IV and they KEYED_HASH mode set
        Self {
            key_words: make_u32s_le(key),
            flags: KEYED_HASH,
        }
    }

    /// Construct a new `Hasher` for the key derivation function. The context
    /// string should be hardcoded, globally unique, and application-specific.
    pub fn new_derive_key(context: &str) -> Self {
        // The context is converted into a IV by hashing it in the DERIVE_KEY_CONTEXT mode
        let mut context_hasher = Self::new_internal(IV, DERIVE_KEY_CONTEXT);
        context_hasher.update(context.as_bytes());
        let context_key = context_hasher.finalize();
        // The hasher used in DERIVE_KEY_MATERIAL mode
        Self::new_internal(make_u32s_le::<8>(&context_key), DERIVE_KEY_MATERIAL)
    }
}

impl StatefulHasher for Blake3 {
    fn update(&mut self, bytes: &[u8]) {
        // while !input.is_empty() {
        //     // If the current chunk is complete, finalize it and reset the
        //     // chunk state. More input is coming, so this chunk is not ROOT.
        //     if self.chunk_state.len() == CHUNK_LEN {
        //         let chunk_cv = self.chunk_state.output().chaining_value();
        //         let total_chunks = self.chunk_state.chunk_counter + 1;
        //         self.add_chunk_chaining_value(chunk_cv, total_chunks);
        //         self.chunk_state = ChunkState::new(self.key_words, total_chunks, self.flags);
        //     }

        //     // Compress input bytes into the current chunk state.
        //     let want = CHUNK_LEN - self.chunk_state.len();
        //     let take = min(want, input.len());
        //     self.chunk_state.update(&input[..take]);
        //     input = &input[take..];
        // }
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        // Starting with the Output from the current chunk, compute all the
        // parent chaining values along the right edge of the tree, until we
        // have the root Output.
        // let mut output = self.chunk_state.output();
        // let mut parent_nodes_remaining = self.cv_stack_len as usize;
        // while parent_nodes_remaining > 0 {
        //     parent_nodes_remaining -= 1;
        //     output = parent_output(
        //         self.cv_stack[parent_nodes_remaining],
        //         output.chaining_value(),
        //         self.key_words,
        //         self.flags,
        //     );
        // }
        // output.root_output_bytes(out_slice);
        todo!()
    }

    crate::stateful_hash_helpers!();
}
