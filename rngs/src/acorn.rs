use crate::SimpleRng;

pub struct Acorn60 {
    state: u64,
    adders: Vec<u64>,
}

impl Default for Acorn60 {
    fn default() -> Self {
        Self {
            state: Default::default(),
            adders: Default::default(),
        }
    }
}

impl SimpleRng for Acorn60 {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}
