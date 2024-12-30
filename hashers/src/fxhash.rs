use crate::traits::StatefulHasher;
use std::ops::BitXor;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FxHashVariant {
    W32,
    W64,
}

pub struct FxHash {
    variant: FxHashVariant,
    buffer: Vec<u8>,
    state64: u64,
    state32: u32,
}

impl Default for FxHash {
    fn default() -> Self {
        Self {
            variant: FxHashVariant::W64,
            buffer: Vec::new(),
            state64: 0,
            state32: 0,
        }
    }
}

impl FxHash {
    pub fn init(variant: FxHashVariant) -> Self {
        Self {
            variant,
            buffer: Vec::new(),
            state64: 0,
            state32: 0,
        }
    }

    pub fn init_32() -> Self {
        Self::init(FxHashVariant::W32)
    }

    pub fn init_64() -> Self {
        Self::init(FxHashVariant::W64)
    }

    pub fn hash_word_64(state: &mut u64, word: u64) {
        *state = state
            .rotate_left(5)
            .bitxor(word)
            .wrapping_mul(0x517cc1b727220a95);
    }

    pub fn hash_word_32(state: &mut u32, word: u32) {
        *state = state.rotate_left(5).bitxor(word).wrapping_mul(0x27220a95);
    }
}

impl StatefulHasher for FxHash {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        match self.variant {
            FxHashVariant::W32 => {
                let chunks = self.buffer.chunks_exact(4);
                let rem = chunks.remainder().to_vec();
                for chunk in chunks {
                    let n = u32::from_be_bytes(chunk.try_into().unwrap());
                    Self::hash_word_32(&mut self.state32, n);
                }
                self.buffer = rem;
            }
            FxHashVariant::W64 => {
                let chunks = self.buffer.chunks_exact(8);
                let rem = chunks.remainder().to_vec();
                for chunk in chunks {
                    let n = u64::from_be_bytes(chunk.try_into().unwrap());
                    Self::hash_word_64(&mut self.state64, n);
                }
                self.buffer = rem;
            }
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        match self.variant {
            FxHashVariant::W32 => {
                for byte in self.buffer {
                    Self::hash_word_32(&mut self.state32, byte as u32);
                }
                self.state32.to_be_bytes().to_vec()
            }
            FxHashVariant::W64 => {
                let chunks = self.buffer.chunks_exact(4);
                let rem = chunks.remainder();
                for chunk in chunks {
                    let n = u64::from_be_bytes(chunk.try_into().unwrap());
                    Self::hash_word_64(&mut self.state64, n);
                }
                for byte in rem {
                    Self::hash_word_64(&mut self.state64, *byte as u64);
                }
                self.state64.to_be_bytes().to_vec()
            }
        }
    }

    crate::stateful_hash_helpers!();
}
