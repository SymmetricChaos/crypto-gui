// https://eprint.iacr.org/2016/027.pdf

use std::{mem, ops::Range};

use num::{BigUint, FromPrimitive};

use crate::{
    sha::{Sha256, Sha512},
    traits::{ResettableHasher, StatefulHasher},
};

// pub enum BalloonVariant {
//     Sha256,
//     Sha512,
// }

// impl BalloonVariant {
//     pub fn blocksize(&self) -> usize {
//         match self {
//             BalloonVariant::Sha256 => 32,
//             BalloonVariant::Sha512 => 64,
//         }
//     }

//     pub fn init(&self) -> Box<dyn StatefulHasher> {
//         match self {
//             BalloonVariant::Sha256 => Box::new(Sha2_256::init()),
//             BalloonVariant::Sha512 => Box::new(Sha2_512::init()),
//         }
//     }
// }

pub struct Balloon {
    // variant: BalloonVariant,
    m_cost: u64,
    t_cost: u64,
    salt: Vec<u8>,
    buffer: Vec<u8>,
}

impl Balloon {
    const BLOCKSIZE: usize = 32; // for SHA256
    const DELTA: u64 = 3;

    fn extract_range(n: usize) -> Range<usize> {
        (n * Self::BLOCKSIZE)..(n * Self::BLOCKSIZE + Self::BLOCKSIZE)
    }

    pub fn init(t_cost: u64, m_cost: u64, salt: &[u8]) -> Self {
        Self {
            // variant,
            m_cost,
            t_cost,
            salt: salt.to_vec(),
            buffer: Vec::new(),
        }
    }

    fn total_memory(&self) -> usize {
        self.m_cost as usize * Self::BLOCKSIZE
    }
}

impl StatefulHasher for Balloon {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        let m_cost_big_int = BigUint::from_u64(self.m_cost).unwrap();
        let mut ctr: u64 = 0;
        let mut blocks: Vec<u8> = Vec::with_capacity(self.total_memory());
        // Step 1. Expand input into buffer
        let mut h = Sha256::init();
        h.update(&ctr.to_le_bytes());
        ctr += 1;
        h.update(&self.buffer);
        h.update(&self.salt);
        blocks.extend(h.finalize_and_reset());

        for i in 1..self.m_cost as usize {
            h.update(&ctr.to_le_bytes());
            ctr += 1;
            h.update(&blocks[Self::extract_range(i)]);
            blocks.extend(h.finalize_and_reset());
        }

        // Step 2. Mix buffer contents
        for _t in 0..self.t_cost as usize {
            for m in 0..self.m_cost as usize {
                // Step 2a. Hash last and current blocks
                let p = if m == 0 {
                    (self.m_cost - 1) as usize
                } else {
                    m - 1 as usize
                };
                h.update(&ctr.to_le_bytes());
                ctr += 1;
                h.update(&blocks[Self::extract_range(p)]);
                h.update(&blocks[Self::extract_range(m)]);
                blocks[Self::extract_range(m)].copy_from_slice(&h.finalize_and_reset());

                // Step 2b. Hash in pseudorandom blocks
                for i in 0..Self::DELTA {
                    h.update(&self.t_cost.to_le_bytes());
                    h.update(&(m as u64).to_le_bytes());
                    h.update(&i.to_le_bytes());
                    let idx_block = h.finalize_and_reset();

                    h.update(&ctr.to_le_bytes());
                    ctr += 1;
                    h.update(&self.salt);
                    h.update(&idx_block);
                    let other = BigUint::from_bytes_le(&h.finalize_and_reset()) % &m_cost_big_int; // convert to an integer
                    let other = usize::from_le_bytes(
                        other.to_bytes_le()[..mem::size_of::<usize>()]
                            .try_into()
                            .unwrap(),
                    );

                    h.update(&ctr.to_le_bytes());
                    ctr += 1;
                    h.update(&blocks[Self::extract_range(m)]);
                    h.update(&blocks[Self::extract_range(other)]);
                    blocks[Self::extract_range(m)].copy_from_slice(&h.finalize_and_reset());
                }
            }
        }

        // Step 3. Extract output
        blocks[Self::extract_range((self.m_cost - 1) as usize)].to_vec()
    }

    
}
