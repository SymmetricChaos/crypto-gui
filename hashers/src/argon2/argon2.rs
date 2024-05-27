use std::ops::{Index, IndexMut};

use itertools::Itertools;
use num::Integer;
use utils::byte_formatting::ByteFormat;

use crate::{blake::Blake2b, errors::HasherError, traits::ClassicHasher};

#[derive(Clone, Debug)]
pub struct Argon2Block(Vec<Vec<u8>>);

impl Default for Argon2Block {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Index<usize> for Argon2Block {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Argon2Block {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

pub struct Argon2 {
    input_format: ByteFormat,
    output_format: ByteFormat,
    salt: Vec<u8>,
    key: Vec<u8>,
    associated_data: Vec<u8>,
    parallelism: u32, // this will always be 1 unless I figure out something clever
    tag_len: u32,     // tag length in bytes
    memory: u32,      // memory requirement in kibibytes (1024 bytes)
    iterations: u32,  // number of iterations run
    version: u8,      // currently 0x13
    mode: u32,        // 0 for Argon2d, 1 for Argon2i, 2 for Argon2id
}

impl Default for Argon2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            salt: Default::default(),
            key: Default::default(),
            associated_data: Default::default(),
            parallelism: Default::default(),
            tag_len: Default::default(),
            memory: Default::default(),
            iterations: Default::default(),
            version: 0x13,
            mode: Default::default(),
        }
    }
}

impl Argon2 {
    pub fn buffer(&self, password: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.parallelism.to_le_bytes());
        buffer.extend(self.tag_len.to_le_bytes());
        buffer.extend(self.memory.to_le_bytes());
        buffer.extend(self.iterations.to_le_bytes());
        buffer.extend(self.version.to_le_bytes());
        buffer.extend(self.mode.to_le_bytes());
        buffer.extend((password.len() as u32).to_le_bytes());
        buffer.extend_from_slice(password);
        buffer.extend((self.salt.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.salt);
        buffer.extend((self.key.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.key);
        buffer.extend((self.associated_data.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.associated_data);

        buffer
    }
}

impl ClassicHasher for Argon2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.parallelism > 0, "parallelism cannot be 0");
        assert!(
            self.parallelism < 0x1000000,
            "parallelism must be less than 2^24"
        );
        assert!(self.iterations > 0, "iterations cannot be 0");
        assert!(self.tag_len >= 4, "tag_len must be at least 4");
        assert!(
            self.memory >= 8 * self.parallelism,
            "memory must be at least 8 times parallelism"
        );
        assert!(self.salt.len() >= 8, "salt length must be at least 8");

        let mut hasher = Blake2b::default();
        hasher.hash_len = 64;
        let buffer = self.buffer(bytes);

        // Initialization block
        let h0 = hasher.hash(&buffer);

        let block_count = self.memory.prev_multiple_of(&(4 * self.parallelism));
        let column_count = block_count / self.parallelism;

        let mut blocks = vec![Argon2Block::default(); block_count as usize];

        // Initialize the blocks
        hasher.hash_len = 1024;
        for i in 0..self.parallelism {
            let mut h = h0.clone();
            h.extend(0_u32.to_le_bytes());
            h.extend((i).to_le_bytes());
            blocks[i as usize].0.push(hasher.hash(&h));

            let mut h = h0.clone();
            h.extend(1_u32.to_le_bytes());
            h.extend((i).to_le_bytes());
            blocks[i as usize].0.push(hasher.hash(&h))
        }
        todo!()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if self.parallelism == 0 {
            return Err(HasherError::general("parallelism cannot be 0"));
        }
        if self.salt.len() == 0 {
            return Err(HasherError::general("salt length must be at least 8"));
        }
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}
