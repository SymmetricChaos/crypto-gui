use crate::{auxiliary::snefru_arrays::SBOXES, traits::StatefulHasher};
use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be};

// https://link.springer.com/article/10.1007/BF00203968

const INPUT_BLOCK_SIZE: usize = 16;
const MASK: usize = INPUT_BLOCK_SIZE - 1;
const ROTATE: [u32; 4] = [16, 8, 16, 24];
const MIN_SECURITY: u32 = 2;
const MAX_SECURITY: u32 = 16;

// Output length in bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnefruOutputSize {
    W128,
    W256,
}

impl SnefruOutputSize {
    pub fn output_block_size(&self) -> usize {
        match self {
            SnefruOutputSize::W128 => 4,
            SnefruOutputSize::W256 => 8,
        }
    }

    pub fn output_block_size_bytes(&self) -> usize {
        self.output_block_size() * 4
    }

    pub fn chunk_size(&self) -> usize {
        INPUT_BLOCK_SIZE - self.output_block_size()
    }

    pub fn chunk_size_bytes(&self) -> usize {
        self.chunk_size() * 4
    }
}

// The compression function is based on a block cipher created for purpose
pub fn compress(state: &mut [u32], security_level: u32, output_size: usize) {
    let mut block = state.to_owned();
    // println!("start\n15: {:08x?}", block[15]);
    for index in 0..security_level {
        for byte_in_word in 0..4 {
            for i in 0..INPUT_BLOCK_SIZE {
                let next = (i + 1) & MASK;
                let last = (i + MASK) & MASK;
                let entry = 2 * index as usize + ((i / 2) & 1);
                // if index == 0 && byte_in_word == 0 {
                //     println!("{last} {i} {next} {entry}");
                // }
                let sbox_entry = SBOXES[entry][(block[i as usize] & 0xff) as usize];
                block[next] ^= sbox_entry;
                block[last] ^= sbox_entry;
            }
            // println!("after rounds\n15: {:08x?}", block[15]);
            let shift = ROTATE[byte_in_word];
            for word in block.iter_mut() {
                *word = word.rotate_right(shift);
            }
            // println!("after shifts\n15: {:08x?}", block[15]);
        }
    }
    for i in 0..output_size {
        state[i] = state[i] ^ block[MASK - i];
    }
}

pub struct Snefru {
    state: [u32; 16],
    buffer: Vec<u8>,
    bits_taken: u64,
    security_level: u32, // maximum of 16
    variant: SnefruOutputSize,
}

impl Default for Snefru {
    fn default() -> Self {
        Self {
            state: [0; 16],
            buffer: Vec::with_capacity(48),
            bits_taken: 0,
            security_level: 8,
            variant: SnefruOutputSize::W128,
        }
    }
}

impl Snefru {
    pub fn init(security_level: u32, variant: SnefruOutputSize) -> Self {
        Self {
            state: [0; 16],
            buffer: Vec::with_capacity(variant.chunk_size_bytes()),
            bits_taken: 0,
            security_level,
            variant,
        }
    }
}

impl StatefulHasher for Snefru {
    fn update(&mut self, mut bytes: &[u8]) {
        if self.security_level > MAX_SECURITY || self.security_level < MIN_SECURITY {
            panic!("invalid security level")
        }
        while !bytes.is_empty() {
            if self.buffer.len() == self.variant.chunk_size_bytes() {
                self.bits_taken += (self.variant.chunk_size_bytes() * 8) as u64;
                fill_u32s_be(
                    &mut self.state[self.variant.output_block_size()..],
                    &self.buffer,
                );
                compress(
                    &mut self.state,
                    self.security_level,
                    self.variant.output_block_size(),
                );
                self.buffer.clear();
            }
            crate::take_bytes!(self.buffer, bytes, self.variant.chunk_size_bytes());
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        if self.security_level > MAX_SECURITY || self.security_level < MIN_SECURITY {
            panic!("invalid security level")
        }

        if !self.buffer.is_empty() {
            self.bits_taken += (self.buffer.len() * 8) as u64;
            while (self.buffer.len() % self.variant.chunk_size_bytes()) != 0 {
                self.buffer.push(0x00)
            }
            fill_u32s_be(
                &mut self.state[self.variant.output_block_size()..],
                &self.buffer,
            );
            compress(
                &mut self.state,
                self.security_level,
                self.variant.output_block_size(),
            );
        }

        for i in &mut self.state[self.variant.output_block_size()..] {
            *i = 0
        }
        self.state[14] = (self.bits_taken >> 32) as u32;
        self.state[15] = self.bits_taken as u32;

        compress(
            &mut self.state,
            self.security_level,
            self.variant.output_block_size(),
        );

        let mut out = vec![0; self.variant.output_block_size_bytes()];
        u32s_to_bytes_be(&mut out, &self.state[0..self.variant.output_block_size()]);

        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1,
    Snefru::init(8, SnefruOutputSize::W128),
    b"password1234",
    "4da8e8b5cb8585d336301dc6130d294b";

    test_alphabet,
    Snefru::init(8, SnefruOutputSize::W128),
    b"abcdefghijklmnopqrstuvabcdefghijklmnopqrstuvabcdefghijklmnopqrstuv",
    "f8ad3c6cc9d4243e6b15139c42ed9c2c";

    test2_128,
    Snefru::init(8, SnefruOutputSize::W128),
    b"e3f6aa21aa38a6fc369994726031ee8ecc02b9fbc8d6630065fe96c3a0fae599",
    "6b8c3f9937b6fcdfa641ea1c133d6eab";

);
