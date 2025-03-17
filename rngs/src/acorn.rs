use crate::ClassicRng;

pub struct Acorn60 {
    seed: u64,
    adders: Vec<u64>,
}

impl ClassicRng for Acorn60 {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}
