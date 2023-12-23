use crate::traits::ClassicRng;

pub struct MersenneTwister {
    n: usize,
}

impl Default for MersenneTwister {
    fn default() -> Self {
        let n = 624;
        let mut arr = [0u32; 624];

        Self { n }
    }
}

impl MersenneTwister {
    pub fn ksa(&mut self, key: &[u8]) {}
}

impl ClassicRng for MersenneTwister {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}
