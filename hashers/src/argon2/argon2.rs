use std::{
    num::Wrapping,
    ops::{BitXor, BitXorAssign, Index, IndexMut},
};

use num::Integer;
use utils::byte_formatting::ByteFormat;

use crate::{blake::Blake2b, errors::HasherError, traits::ClassicHasher};

const BLOCK_BYTES: usize = 1024;
const BLOCK_WORDS: usize = BLOCK_BYTES / 8;

#[derive(Clone, Copy, Debug)]
pub struct Argon2Block([u64; BLOCK_WORDS]);

impl Default for Argon2Block {
    fn default() -> Self {
        Self([0u64; BLOCK_WORDS])
    }
}

impl Index<usize> for Argon2Block {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Argon2Block {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl From<[u64; BLOCK_WORDS]> for Argon2Block {
    fn from(value: [u64; BLOCK_WORDS]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Argon2Block {
    type Error = HasherError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1024 {
            return Err(HasherError::general(
                "Argon2Block must be exactly 1024 bytes",
            ));
        };
        let mut block = [0u64; BLOCK_WORDS];
        for (i, chunk) in value.chunks_exact(8).enumerate() {
            block[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        Ok(Self(block))
    }
}

impl TryFrom<Vec<u8>> for Argon2Block {
    type Error = HasherError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 1024 {
            return Err(HasherError::general(
                "Argon2Block must be exactly 1024 bytes",
            ));
        };
        let mut block = [0u64; BLOCK_WORDS];
        for (i, chunk) in value.chunks_exact(8).enumerate() {
            block[i] = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        Ok(Self(block))
    }
}

impl BitXor<&Argon2Block> for Argon2Block {
    type Output = Argon2Block;

    fn bitxor(mut self, rhs: &Argon2Block) -> Self::Output {
        for (s, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *s ^= r;
        }
        self
    }
}

impl BitXorAssign<&Argon2Block> for Argon2Block {
    fn bitxor_assign(&mut self, rhs: &Argon2Block) {
        *self = *self ^ rhs
    }
}

// https://docs.rs/argon2/latest/src/argon2/block.rs.html

const TRUNC: u64 = u32::MAX as u64;

// Notice that this is nearly identical to the BLAKE and BLAKE2 mixing function.
#[rustfmt::skip]
macro_rules! permute_step {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $a = (Wrapping($a) + Wrapping($b) + (Wrapping(2) * Wrapping(($a & TRUNC) * ($b & TRUNC)))).0;
        $d = ($d ^ $a).rotate_right(32);
        $c = (Wrapping($c) + Wrapping($d) + (Wrapping(2) * Wrapping(($c & TRUNC) * ($d & TRUNC)))).0;
        $b = ($b ^ $c).rotate_right(24);

        $a = (Wrapping($a) + Wrapping($b) + (Wrapping(2) * Wrapping(($a & TRUNC) * ($b & TRUNC)))).0;
        $d = ($d ^ $a).rotate_right(16);
        $c = (Wrapping($c) + Wrapping($d) + (Wrapping(2) * Wrapping(($c & TRUNC) * ($d & TRUNC)))).0;
        $b = ($b ^ $c).rotate_right(63);
    };
}

macro_rules! permute {
    (
        $v0:expr, $v1:expr, $v2:expr, $v3:expr,
        $v4:expr, $v5:expr, $v6:expr, $v7:expr,
        $v8:expr, $v9:expr, $v10:expr, $v11:expr,
        $v12:expr, $v13:expr, $v14:expr, $v15:expr,
    ) => {
        permute_step!($v0, $v4, $v8, $v12);
        permute_step!($v1, $v5, $v9, $v13);
        permute_step!($v2, $v6, $v10, $v14);
        permute_step!($v3, $v7, $v11, $v15);
        permute_step!($v0, $v5, $v10, $v15);
        permute_step!($v1, $v6, $v11, $v12);
        permute_step!($v2, $v7, $v8, $v13);
        permute_step!($v3, $v4, $v9, $v14);
    };
}

pub fn compress(rhs: &Argon2Block, lhs: &Argon2Block) -> Argon2Block {
    let r = *rhs ^ lhs;
    let mut q = r;

    // Row by row
    for chunk in q.0.chunks_exact_mut(16) {
        permute!(
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        );
    }

    // Column by column
    for i in (0..16).step_by(2) {
        permute!(
            q[i],
            q[i + 1],
            q[i + 16],
            q[i + 17],
            q[i + 32],
            q[i + 33],
            q[i + 48],
            q[i + 49],
            q[i + 64],
            q[i + 65],
            q[i + 80],
            q[i + 81],
            q[i + 96],
            q[i + 97],
            q[i + 112],
            q[i + 113],
        );
    }

    q ^ &r
}

const MAX_PAR: u32 = 1 << 24;

#[derive(Debug, Clone)]
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
    version: u32,     // currently 0x13
    mode: u32,        // 0 for Argon2d, 1 for Argon2i, 2 for Argon2id
}

impl Default for Argon2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: Default::default(),
            key: Default::default(),
            associated_data: Default::default(),
            parallelism: Default::default(),
            tag_len: Default::default(),
            memory: Default::default(),
            iterations: Default::default(),
            version: 0x13,
            mode: 2, // default to Argon2id (recommended)
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

    // Exact order to indicies depends on mode
    fn block_indicies(&self, a: usize, b: usize) -> (usize, usize) {
        if self.mode == 0 {
            todo!()
        } else if self.mode == 1 {
            todo!()
        } else if self.mode == 2 {
            todo!()
        } else {
            panic!("invalid mode choice")
        }
    }
}

impl ClassicHasher for Argon2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.parallelism > 0, "parallelism cannot be 0");
        assert!(
            self.parallelism < MAX_PAR,
            "parallelism must be less than 2^24"
        );
        assert!(self.iterations > 0, "iterations cannot be 0");
        assert!(self.tag_len >= 4, "tag_len must be at least 4 bytes");
        assert!(
            self.memory >= 8 * self.parallelism,
            "memory must be at least 8 times parallelism"
        );
        assert!(self.salt.len() >= 8, "salt length must be at least 8 bytes");

        let mut hasher = Blake2b::default();
        hasher.hash_len = 64;
        let buffer = self.buffer(bytes);

        // Initialization block
        let h0 = hasher.hash(&buffer);

        let block_count = self.memory.prev_multiple_of(&(4 * self.parallelism));
        let column_count = (block_count / self.parallelism) as usize;

        let mut blocks =
            vec![vec![Argon2Block::default(); block_count as usize]; self.parallelism as usize];

        // Initialize the columns with the first two blocks of each
        hasher.hash_len = BLOCK_BYTES;
        for i in 0..self.parallelism {
            let mut h = h0.clone();
            h.extend(0_u32.to_le_bytes());
            h.extend(i.to_le_bytes());
            let block = hasher
                .hash(&h)
                .try_into()
                .expect("blocks should be 1024-bytes");
            blocks[i as usize][0] = block;

            let mut h = h0.clone();
            h.extend(1_u32.to_le_bytes());
            h.extend(i.to_le_bytes());
            let block = hasher
                .hash(&h)
                .try_into()
                .expect("blocks should be 1024-bytes");
            blocks[i as usize][1] = block;
        }

        // Fill each lane forcing a large amount of memory to be allocated
        for i in 0..self.parallelism {
            for j in 2..column_count {
                let i = i as usize;
                let j = j as usize;
                let (a, b) = self.block_indicies(i, j);
                blocks[i][j] = compress(&blocks[i][j - 1], &blocks[a][b]);
            }
        }

        // Additional passes over the lanes
        for _iterations in 2..self.iterations {
            for i in 0..self.parallelism {
                for j in 0..column_count {
                    let i = i as usize;
                    let j = j as usize;
                    let (a, b) = self.block_indicies(i, j);
                    if j == 0 {
                        let c = compress(&blocks[i][column_count - 1], &blocks[a][b]);
                        blocks[i][0] ^= &c;
                    } else {
                        let c = compress(&blocks[i][j - 1], &blocks[a][b]);
                        blocks[i][j] ^= &c;
                    }
                }
            }
        }

        // XOR together the final block of each lane
        let mut c = Argon2Block::default();
        for i in 0..self.parallelism {
            c ^= &blocks[i as usize][(column_count - 1)];
        }
        let out_bytes: Vec<u8> = c.0.iter().flat_map(|x| x.to_be_bytes()).collect();

        // Hash the final value
        hasher.hash_len = self.tag_len as usize;
        hasher.hash(&out_bytes)
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if self.parallelism == 0 {
            return Err(HasherError::general("parallelism cannot be 0"));
        }
        if self.parallelism > MAX_PAR {
            return Err(HasherError::general("parallelism must be less than 2^24"));
        }
        if self.salt.len() == 0 {
            return Err(HasherError::general("salt length must be at least 8 bytes"));
        }
        if self.iterations == 0 {
            return Err(HasherError::general("iterations cannot be 0"));
        }
        if self.tag_len < 4 {
            return Err(HasherError::general("tag_len must be at least 4 bytes"));
        }
        if self.memory < 8 * self.parallelism {
            return Err(HasherError::general(
                "memory must be at least 8 times parallelism",
            ));
        }
        if self.salt.len() < 8 {
            return Err(HasherError::general("salt length must be at least 8 bytes"));
        }

        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}
