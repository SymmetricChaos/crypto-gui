use crate::traits::ClassicRng;

pub struct WeylSequence {
    pub state: u64,
    pub increment: u64,
    pub modulus: u64,
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
    fn step(&mut self) {
        self.state = (self.state + self.increment) % self.modulus;
    }
}
