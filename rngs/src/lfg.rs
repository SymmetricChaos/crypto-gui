use std::collections::VecDeque;

use crate::{errors::RngError, traits::ClassicRng};

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

impl Lfg {
    pub fn set_tap(&mut self, position: u32) -> Result<(), RngError> {
        if position == 0 {
            return Err(RngError::general("LFG tap cannot be at position zero"));
        } else if position as usize >= self.state.len() {
            return Err(RngError::general("LFG tap must be within the state vector"));
        } else {
            self.tap = position as usize;
        }

        Ok(())
    }
}

impl ClassicRng for Lfg {
    // Will panic if tap is invalid
    // No overflows can happen here for addition, multiplication, or XOR because the inputs are are u32 initially
    fn next_u32(&mut self) -> u32 {
        let m = (self
            .op
            .op(self.state[0] as u64, self.state[self.tap] as u64)
            % self.modulus as u64) as u32;
        self.state.pop_front();
        self.state.push_back(m);
        m
    }
}
