// reference implementation
// https://docs.rs/argon2/latest/src/argon2/block.rs.html
// specs
// https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf

use std::{
    num::Wrapping,
    ops::{BitXor, BitXorAssign, Index, IndexMut},
};

use super::consts::{
    Mode, Version, BLOCK_BYTES, BLOCK_WORDS, MAX_KEY, MAX_PAR, MAX_PASS, MAX_SALT, MIN_SALT,
    SYNC_POINTS,
};
use crate::{
    errors::HasherError,
    traits::{ClassicHasher, StatefulHasher},
};
use num::traits::ToBytes;
use utils::{byte_formatting::ByteFormat, math_functions::incr_array_ctr_be};

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
fn argon2i_addr(
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

#[derive(Debug, Clone)]
pub struct Argon2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    salt: Vec<u8>,
    key: Vec<u8>,
    associated_data: Vec<u8>,
    tag_len: u32,         // tag length in bytes
    par_cost: u32,        // this will always be 1 unless I figure out something clever
    mem_cost: u32,        // memory requirement in kibibytes (1024 bytes)
    iterations: u32,      // number of iterations run
    pub version: Version, // currently 0x13
    pub mode: Mode,       // 0 for Argon2d, 1 for Argon2i, 2 for Argon2id
}

impl Default for Argon2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: vec![0, 0, 0, 0],
            key: Default::default(),
            associated_data: Default::default(),
            tag_len: 32,           // minimum
            par_cost: 1,           // minimum
            mem_cost: 8,           // minimum
            iterations: 1,         // minimum
            version: Version::V13, // current corrected version
            mode: Mode::ID,        // default to Argon2id (recommended)
        }
    }
}

impl Argon2 {
    pub fn argon2i() -> Self {
        Argon2::default().with_mode(Mode::I)
    }

    pub fn argon2d() -> Self {
        Argon2::default().with_mode(Mode::D)
    }

    pub fn argon2id() -> Self {
        Argon2::default().with_mode(Mode::ID)
    }

    pub fn with_tag_len(mut self, tag_len: u32) -> Self {
        assert!(tag_len > 1);
        self.tag_len = tag_len;
        self
    }

    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_salt<T: AsRef<[u8]>>(mut self, salt: T) -> Self {
        assert!(
            salt.as_ref().len() >= MIN_SALT,
            "salt length must be at least 8 bytes"
        );
        assert!(
            salt.as_ref().len() <= MAX_SALT,
            "salt length cannot be more than 2^32 bytes"
        );
        self.salt = salt.as_ref().to_vec();
        self
    }

    pub fn with_key<T: AsRef<[u8]>>(mut self, key: T) -> Self {
        assert!(
            key.as_ref().len() <= MAX_KEY,
            "key length cannot be more than 2^32 bytes"
        );
        self.key = key.as_ref().to_vec();
        self
    }

    pub fn with_ad<T: AsRef<[u8]>>(mut self, ad: T) -> Self {
        assert!(
            ad.as_ref().len() <= MAX_KEY,
            "associated data length cannot be more than 2^32 bytes"
        );
        self.associated_data = ad.as_ref().to_vec();
        self
    }

    pub fn with_iterations(mut self, iterations: u32) -> Self {
        assert!(iterations != 0, "iterations cannot be 0");
        self.iterations = iterations;
        self
    }

    pub fn with_par_cost(mut self, par_cost: u32) -> Self {
        assert!(self.par_cost > 0, "parallelism cannot be 0");
        assert!(
            self.par_cost < MAX_PAR,
            "parallelism must be less than 2^24"
        );
        self.par_cost = par_cost;
        self.mem_cost = std::cmp::max(self.mem_cost, 8 * self.par_cost); // increase mem_cost to the minimum allowed value if required
        self
    }

    pub fn with_mem_cost(mut self, mem_cost: u32) -> Self {
        self.mem_cost = std::cmp::max(mem_cost, 8 * self.par_cost);
        self
    }

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn initial_hash(&self, password: &[u8]) -> Vec<u8> {
        let mut h = Blake2bStateful::init(&[], 64);

        h.update(&self.par_cost.to_le_bytes());
        h.update(&self.tag_len.to_le_bytes());
        h.update(&self.mem_cost.to_le_bytes());
        h.update(&self.iterations.to_le_bytes());
        h.update(&self.version.to_u32().to_le_bytes());
        h.update(&self.mode.to_u32().to_le_bytes());
        h.update(&(password.len() as u32).to_le_bytes());
        h.update(password);
        h.update(&(self.salt.len() as u32).to_le_bytes());
        h.update(&self.salt);
        h.update(&(self.key.len() as u32).to_le_bytes());
        h.update(&self.key);
        h.update(&(self.associated_data.len() as u32).to_le_bytes());
        h.update(&self.associated_data);

        h.finalize()
    }

    fn num_lanes(&self) -> usize {
        self.par_cost as usize
    }

    fn lane_length(&self) -> usize {
        self.segment_length() * SYNC_POINTS
    }

    fn segment_length(&self) -> usize {
        let m_cost = self.mem_cost as usize;

        let memory_blocks = if m_cost < 2 * SYNC_POINTS * self.num_lanes() {
            2 * SYNC_POINTS * self.num_lanes()
        } else {
            m_cost
        };

        memory_blocks / (self.num_lanes() * SYNC_POINTS)
    }

    fn num_blocks(&self) -> usize {
        self.segment_length() * self.num_lanes() * SYNC_POINTS
    }
}

impl ClassicHasher for Argon2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.tag_len >= 4, "tag_len must be at least 4 bytes");

        assert!(
            bytes.len() <= MAX_PASS,
            "password length cannot be more than 2^32 bytes"
        );

        // Initialization block
        let h0 = self.initial_hash(bytes);

        println!("{:02x?}", h0);

        let num_blocks = self.num_blocks(); // total number of blocks in the memory grid
        let num_lanes = self.num_lanes(); // number of "rows" in the memory grid (there are always four "columns"), each lane can be computed independently
        let lane_length = self.lane_length(); // the length of each "row" of the memory grid
        let segment_length = self.segment_length(); // number of blocks in each cell of the memory grid
        let iterations = self.iterations as usize; // number of passes over the blocks

        let mut mem_blocks = vec![Block::default(); num_blocks];

        let mut hasher = Blake2bLong::default().with_hash_len(BLOCK_BYTES as u64);
        // Initialize the first two block of each lane
        for lane in (0..num_blocks).step_by(lane_length) {
            // G(h|0|lane)
            let mut h = h0.clone();
            h.extend(0_u32.to_le_bytes());
            h.extend((lane as u32).to_le_bytes());
            let block = hasher
                .hash(&h)
                .try_into()
                .expect("blocks should be 1024-bytes");
            mem_blocks[lane] = block;

            // G(h|1|lane)
            let mut h = h0.clone();
            h.extend(1_u32.to_le_bytes());
            h.extend((lane as u32).to_le_bytes());
            let block = hasher
                .hash(&h)
                .try_into()
                .expect("blocks should be 1024-bytes");
            mem_blocks[lane + 1] = block;
        }

        println!("Block 0 (first four words)");
        println!("{:016x?}", mem_blocks[0][0]);
        println!("{:016x?}", mem_blocks[0][1]);
        println!("{:016x?}", mem_blocks[0][2]);
        println!("{:016x?}", mem_blocks[0][3]);
        println!("");

        println!("Block 31 (last four words)");
        println!("{:016x?}", mem_blocks[31][124]);
        println!("{:016x?}", mem_blocks[31][125]);
        println!("{:016x?}", mem_blocks[31][126]);
        println!("{:016x?}", mem_blocks[31][127]);
        println!("");

        let mut ctr = [0u8; 976];
        // Additional passes over the lanes
        for pass in 2..iterations {
            for slice in 0..SYNC_POINTS {
                // Determine if addressing in data dependent on independent for this slice
                let data_independent_addressing = self.mode == Mode::I
                    || (self.mode == Mode::ID && pass == 0 && slice < SYNC_POINTS / 2);

                // For data dependent addressing this is simply ignored
                let mut addr_block = Block::default();

                for lane in 0..(self.par_cost as usize) {
                    // for the first pass on each slice the address block is reset for Mode::I and the first two blocks are skipped
                    let first_block = if pass == 0 && slice == 0 {
                        if data_independent_addressing {
                            addr_block = argon2i_addr(
                                pass as u64,
                                lane as u64,
                                slice as u64,
                                num_blocks as u64,
                                self.iterations as u64,
                                self.mode.to_u64() as u64,
                                &mut ctr,
                            );
                        }
                        2
                    } else {
                        0
                    };

                    let mut cur_idx = lane * lane_length + slice * segment_length + first_block;
                    let mut prev_idx = if slice == 0 && first_block == 0 {
                        // Last block in current lane
                        cur_idx + lane_length - 1
                    } else {
                        // Previous block
                        cur_idx - 1
                    };

                    for block in first_block..segment_length {
                        let r = if data_independent_addressing {
                            let addr_index = block % 128;

                            // If the address index rolls over create a new block
                            if addr_index == 0 {
                                addr_block = argon2i_addr(
                                    pass as u64,
                                    lane as u64,
                                    slice as u64,
                                    num_blocks as u64,
                                    self.iterations as u64,
                                    self.mode.to_u64() as u64,
                                    &mut ctr,
                                );
                            }

                            addr_block[addr_index]
                        } else {
                            mem_blocks[prev_idx][0]
                        };

                        let ref_lane = if pass == 0 && slice == 0 {
                            // Cannot reference other lanes yet
                            lane
                        } else {
                            (r >> 32) as usize % num_lanes
                        };

                        // Determine the number of blocks that can be used
                        let reference_area_size = if pass == 0 {
                            // First pass
                            if slice == 0 {
                                // First slice
                                block - 1 // all but the previous
                            } else if ref_lane == lane {
                                // The same lane => add current segment
                                slice * segment_length + block - 1
                            } else {
                                slice * segment_length - if block == 0 { 1 } else { 0 }
                            }
                        } else {
                            // Second pass
                            if ref_lane == lane {
                                lane_length - segment_length + block - 1
                            } else {
                                lane_length - segment_length - if block == 0 { 1 } else { 0 }
                            }
                        };

                        // 1.2.4. Mapping r to 0..<reference_area_size-1> and produce
                        // relative position
                        let mut map = r & 0xFFFFFFFF;
                        map = (map * map) >> 32;
                        let relative_position = reference_area_size
                            - 1
                            - ((reference_area_size as u64 * map) >> 32) as usize;

                        // 1.2.5 Computing starting position
                        let start_position = if pass != 0 && slice != SYNC_POINTS - 1 {
                            (slice + 1) * segment_length
                        } else {
                            0
                        };

                        let lane_idx = (start_position + relative_position) % lane_length;
                        let ref_idx = ref_lane * lane_length + lane_idx;

                        let result = compress(&mem_blocks[prev_idx], &mem_blocks[ref_idx]);

                        // The original 1.0 version simply overwrites the block
                        // The 1.3 version XORs the result into the block
                        if self.version == Version::V10 || pass == 0 {
                            mem_blocks[cur_idx] = result;
                        } else {
                            mem_blocks[cur_idx] ^= &result;
                        };

                        prev_idx = cur_idx;
                        cur_idx += 1;
                    }
                }
            }
            println!("Block 0 (first four words)");
            println!("{:016x?}", mem_blocks[0][0]);
            println!("{:016x?}", mem_blocks[0][1]);
            println!("{:016x?}", mem_blocks[0][2]);
            println!("{:016x?}", mem_blocks[0][3]);
            println!("");

            println!("Block 31 (last four words)");
            println!("{:016x?}", mem_blocks[31][124]);
            println!("{:016x?}", mem_blocks[31][125]);
            println!("{:016x?}", mem_blocks[31][126]);
            println!("{:016x?}", mem_blocks[31][127]);
            println!("");
        }

        // XOR together the final block of each lane
        let mut c = Block::default();
        for lane in (0..num_blocks).step_by(lane_length) {
            c ^= &mem_blocks[lane + lane_length - 1];
        }

        // Hash the final value
        hasher.hash_len = self.tag_len as u64;
        hasher.hash(&c.to_be_bytes())
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if self.mem_cost < 8 * self.par_cost {
            return Err(HasherError::general(
                "memory must be at least 8 times parallelism",
            ));
        }

        if self.tag_len < 4 {
            return Err(HasherError::general("tag_len must be at least 4 bytes"));
        }

        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;

        if bytes.len() > MAX_PASS {
            return Err(HasherError::general(
                "password length cannot be more than 2^32 bytes",
            ));
        }

        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

crate::basic_hash_tests!(
    test_argon2d, Argon2::argon2d().input(ByteFormat::Hex).with_iterations(3).with_par_cost(4).with_mem_cost(32).with_tag_len(32).with_salt([0x02; 16]).with_key([0x03; 8]).with_ad([0x04; 12]),
    "0101010101010101010101010101010101010101010101010101010101010101",
    "512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb";
    test_argon2i, Argon2::argon2i().input(ByteFormat::Hex).with_iterations(3).with_par_cost(4).with_mem_cost(32).with_tag_len(32).with_salt([0x02; 16]).with_key([0x03; 8]).with_ad([0x04; 12]),
    "0101010101010101010101010101010101010101010101010101010101010101",
    "c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8";
);
