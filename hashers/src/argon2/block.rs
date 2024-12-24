use num::traits::ToBytes;
use std::{
    num::Wrapping,
    ops::{BitXor, BitXorAssign, Index, IndexMut},
};
use utils::math_functions::incr_array_ctr_be;

use crate::errors::HasherError;

use super::consts::BLOCK_WORDS;

#[derive(Clone, Copy, Debug)]
pub struct Block([u64; BLOCK_WORDS]);

impl Default for Block {
    fn default() -> Self {
        Self([0u64; BLOCK_WORDS])
    }
}

impl ToBytes for Block {
    type Bytes = Vec<u8>;

    fn to_be_bytes(&self) -> Self::Bytes {
        self.0.iter().flat_map(|x| x.to_be_bytes()).collect()
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.iter().flat_map(|x| x.to_le_bytes()).collect()
    }
}

impl Index<usize> for Block {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Block {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl From<[u64; BLOCK_WORDS]> for Block {
    fn from(value: [u64; BLOCK_WORDS]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Block {
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

impl TryFrom<Vec<u8>> for Block {
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

impl BitXor<&Block> for Block {
    type Output = Block;

    fn bitxor(mut self, rhs: &Block) -> Self::Output {
        for (s, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *s ^= r;
        }
        self
    }
}

impl BitXorAssign<&Block> for Block {
    fn bitxor_assign(&mut self, rhs: &Block) {
        *self = *self ^ rhs
    }
}

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

pub fn compress(rhs: &Block, lhs: &Block) -> Block {
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

// Calculate an address block using the Argon2i method
pub fn argon2i_addr(
    pass: u64,
    lane: u64,
    slice: u64,
    memory_blocks: u64,
    total_passes: u64,
    mode: u64,
    ctr: &mut [u8; 976],
) -> Block {
    // The counter is incrementated before every call
    incr_array_ctr_be(&mut ctr[0..976]);

    let mut input = Vec::new();
    input.extend(pass.to_le_bytes());
    input.extend(lane.to_le_bytes());
    input.extend(slice.to_le_bytes());
    input.extend(memory_blocks.to_le_bytes());
    input.extend(total_passes.to_le_bytes());
    input.extend(mode.to_le_bytes());
    input.extend_from_slice(ctr);
    // println!("input len {}", input.len());
    // println!("{:02x?}", input);
    let zero = Block::default();
    let input_block = Block::try_from(input).expect("input block not constructed correctly");

    // Compress the input block with the zero block twice
    compress(&zero, &compress(&zero, &input_block))
}
