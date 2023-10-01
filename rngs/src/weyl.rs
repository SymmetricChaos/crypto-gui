use crate::traits::ClassicRng;

pub struct WeylSequence {
    pub state: u32,
    pub increment: u32,
    pub modulus: u32,
}

impl Default for WeylSequence {
    fn default() -> Self {
        Self {
            state: 0,
            increment: 3212,
            modulus: 7919,
        }
    }
}

impl ClassicRng for WeylSequence {
    fn step(&mut self) -> u32 {
        self.state = (self.state + self.increment) % self.modulus;
        self.state
    }
}
