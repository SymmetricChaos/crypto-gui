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
    fn next_u32(&mut self) -> u32 {
        self.state = (self.state + self.increment) % self.modulus;
        self.state as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = (self.state + self.increment) % self.modulus;
        self.state
    }
}

pub struct WeylSequence32 {
    pub state: u32,
    pub increment: u32,
}

impl Default for WeylSequence32 {
    fn default() -> Self {
        Self {
            state: 0,
            increment: 3211,
        }
    }
}

impl ClassicRng for WeylSequence32 {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(self.increment);
        self.state
    }
}

pub struct WeylSequence64 {
    pub state: u64,
    pub increment: u64,
}

impl Default for WeylSequence64 {
    fn default() -> Self {
        Self {
            state: 0,
            increment: 3211,
        }
    }
}

impl ClassicRng for WeylSequence64 {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(self.increment);
        self.state as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(self.increment);
        self.state
    }
}
