// reference implementation
// https://docs.rs/argon2/latest/src/argon2/block.rs.html
// specs
// https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf

use super::consts::{
    Mode, Version, BLOCK_BYTES, MAX_KEY, MAX_PAR, MAX_PASS, MAX_SALT, MIN_SALT, SYNC_POINTS,
};
use crate::{
    argon2::block::{argon2i_addr, compress, Block},
    blake::{Blake2b, Blake2bLong},
    errors::HasherError,
    traits::{ClassicHasher, StatefulHasher},
};
use num::traits::ToBytes;
use utils::byte_formatting::ByteFormat;

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
        let mut h = Blake2b::init_hash_512();

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

        // Initialize the first two block of each lane
        for lane in (0..num_blocks).step_by(lane_length) {
            // G(h|0|lane)
            let mut h = h0.clone();
            h.extend(0_u32.to_le_bytes());
            h.extend((lane as u32).to_le_bytes());
            let block = Blake2bLong::init_hash(BLOCK_BYTES as u64)
                .hash(&h)
                .try_into()
                .expect("blocks should be 1024-bytes");
            mem_blocks[lane] = block;

            // G(h|1|lane)
            let mut h = h0.clone();
            h.extend(1_u32.to_le_bytes());
            h.extend((lane as u32).to_le_bytes());
            let block = Blake2bLong::init_hash(BLOCK_BYTES as u64)
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
        Blake2bLong::init_hash(self.tag_len as u64).hash(&c.to_be_bytes())
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
