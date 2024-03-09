use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

// https://github.com/BLAKE3-team/BLAKE3
// https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf
// https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs

const OUT_LEN: usize = 32;
const KEY_LEN: usize = 32;
const BLOCK_LEN: usize = 64;
const CHUNK_LEN: usize = 1024;

const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

// Initialization vector, sqrt of the first eight primes
const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

pub fn mix(v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(16);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(12);

    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(8);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(7);
}

fn full_round(work: &mut [u32; 16], block: &[u32; 16]) {
    mix(work, 0, 4, 8, 12, block[0], block[1]);
    mix(work, 1, 5, 9, 13, block[2], block[3]);
    mix(work, 2, 6, 10, 14, block[4], block[5]);
    mix(work, 3, 7, 11, 15, block[6], block[7]);

    mix(work, 0, 5, 10, 15, block[8], block[9]);
    mix(work, 1, 6, 11, 12, block[10], block[11]);
    mix(work, 2, 7, 8, 13, block[12], block[13]);
    mix(work, 3, 4, 9, 14, block[14], block[15]);

    // println!("Working Vector at [{i}]:\n{work:016x?}\n");
}

fn permute_words(v: &mut [u32; 16]) {
    let mut permuted = [0; 16];
    for i in 0..16 {
        permuted[i] = v[MSG_PERMUTATION[i]];
    }
    *v = permuted;
}

// Up to 64 bytes
fn create_block(bytes: &[u8]) -> [u32; 16] {
    let mut k = [0u32; 16];
    for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
        *elem = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    k
}
// https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf
// Blocks of 64 bytes (512 bits) are compressed as a sequence of u32s
pub fn compress_block(
    chain: &[u32; 8],
    block: &[u32; 16],
    initialization_vector: &[u32; 8],
    counter: u64,
    block_length: u32,
    flags: u32,
) -> [u32; 16] {
    // create the working vector
    let mut work = [
        chain[0],
        chain[1],
        chain[2],
        chain[3],
        chain[4],
        chain[5],
        chain[6],
        chain[7],
        initialization_vector[0],
        initialization_vector[1],
        initialization_vector[2],
        initialization_vector[3],
        counter as u32,
        (counter >> 32) as u32,
        block_length,
        flags,
    ];

    // Seven full rounds with a permutation between each
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);
    permute_words(&mut work);
    full_round(&mut work, &block);

    // mix the working vector and the chain
    for i in 0..8 {
        work[i] ^= work[i];
        work[i + 8] ^= chain[i];
    }
    work
}

pub struct Chunk {
    chain: [u32; 8],
    chunk_counter: u64,
    block: [u8; BLOCK_LEN],
    block_len: usize,
    blocks_compressed: usize,
    initialization_vector: [u32; 8],
    flags: u32,
}

impl Chunk {
    fn len(&self) -> usize {
        BLOCK_LEN & self.blocks_compressed + self.block_len
    }

    fn start_flag(&self) -> u32 {
        if self.blocks_compressed == 0 {
            CHUNK_START
        } else {
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {
            if self.block_len == BLOCK_LEN {
                let mut block = create_block(&self.block);
                self.chain = compress_block(
                    &self.chain,
                    &block,
                    &self.initialization_vector,
                    self.chunk_counter,
                    BLOCK_LEN as u32,
                    self.flags,
                )[0..8]
                    .try_into()
                    .unwrap();
            }
        }
    }
}

pub struct Blake3 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u8; 32], // optional 256-bit key
    pub keyed_hash: bool,
    // pub derive_key: bool, // ignoring this for now
}

impl Default for Blake3 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: [0; 32],
            keyed_hash: false,
            // derive_key: false, // ignoring this for now
        }
    }
}

impl Blake3 {
    pub fn create_initialization_vector(&self) -> [u32; 8] {
        if self.keyed_hash {
            let mut k = [0u32; 8];
            for (elem, chunk) in k.iter_mut().zip(self.key.chunks_exact(4)).take(8) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }
            k
        } else {
            Self::IV
        }
    }

    // Up to 1024 bytes
    fn create_chunk(bytes: &[u8]) -> [u32; 16] {
        let mut k = [0u32; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
            *elem = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake3 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut chain = Self::IV.clone();

        let initialization_vector = self.create_initialization_vector();

        let mut counter: u64 = 0;

        // Divide the input into chunks of 1024 bytes
        let mut chunks = bytes.chunks_exact(1024);

        chain
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .take(32)
            .collect_vec()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod blake3_tests {
    use super::*;
}
