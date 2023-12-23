use std::collections::VecDeque;

use crate::traits::ClassicRng;

#[derive(Debug, PartialEq, Eq)]
pub enum FibOp {
    Add,
    Mul,
    Xor,
}

impl FibOp {
    pub fn op(&self, a: u64, b: u64) -> u64 {
        match self {
            FibOp::Add => a + b,
            FibOp::Mul => a * b,
            FibOp::Xor => a ^ b,
        }
    }
}

pub struct Lfg {
    pub state: VecDeque<u32>,
    pub modulus: u32,
    pub tap: usize,
    pub op: FibOp,
}

impl Default for Lfg {
    fn default() -> Self {
        Self {
            state: VecDeque::from([2, 5, 13, 29, 47]),
            modulus: 1000,
            tap: 3,
            op: FibOp::Add,
        }
    }
}

impl ClassicRng for Lfg {
    fn next_u32(&mut self) -> u32 {
        // Will panic if tap is invalid
        // No overflows can happen here for addition, multiplication, or XOR because the inputs are are u32 initially
        let m = (self
            .op
            .op(self.state[0] as u64, self.state[self.tap] as u64)
            % self.modulus as u64) as u32;
        self.state.pop_front();
        self.state.push_back(m);
        m
    }
}
