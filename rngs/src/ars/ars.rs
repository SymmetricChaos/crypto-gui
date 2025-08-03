use itertools::Itertools;

use crate::{ars::block_function::encrypt, ClassicRng};

pub fn make_bytes(key: [u32; 4]) -> [u8; 16] {
    key.into_iter()
        .map(|w| w.to_be_bytes())
        .flatten()
        .collect_vec()
        .try_into()
        .unwrap()
}

pub struct Ars {
    pub ctr: [u32; 4],
    pub key: [u32; 4],
    pub rounds: usize,
    saved: [u32; 4],
    idx: usize,
}

impl Default for Ars {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            rounds: 7,
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Ars {}

impl ClassicRng for Ars {
    fn next_u32(&mut self) -> u32 {
        let ctr = make_bytes(self.ctr);
        encrypt(&mut ctr, round_keys, self.rounds);
        todo!()
    }
}
