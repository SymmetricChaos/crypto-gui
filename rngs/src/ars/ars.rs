use crate::{ars::block_function::encrypt, ClassicRng};

pub fn make_bytes(key: [u32; 4]) -> [u8; 16] {
    let mut out = [0; 16];
    out[0..4].copy_from_slice(&key[0].to_be_bytes());
    out[4..8].copy_from_slice(&key[1].to_be_bytes());
    out[8..12].copy_from_slice(&key[2].to_be_bytes());
    out[12..16].copy_from_slice(&key[3].to_be_bytes());
    out
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
        let mut ctr = make_bytes(self.ctr);
        let mut key = make_bytes(self.key);
        encrypt(&mut ctr, &mut key, self.rounds);
        todo!()
    }
}
