use std::collections::VecDeque;

use crate::traits::ClassicRng;

pub struct Lfg {
    pub state: VecDeque<u32>,
    pub modulus: u32,
    pub tap: usize,
}

impl Default for Lfg {
    fn default() -> Self {
        Self {
            state: VecDeque::from([2, 5, 13, 29, 47]),
            modulus: 1000,
            tap: 3,
        }
    }
}

impl ClassicRng for Lfg {
    fn step(&mut self) -> u32 {
        // Will panic if tap is invalid
        // No overflows can happen here because the inputs are are u32 initially
        let m =
            ((self.state[0] as u64) + (self.state[self.tap] as u64) % self.modulus as u64) as u32;
        self.state.pop_front();
        self.state.push_back(m);
        m
    }
}
