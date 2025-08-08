use std::collections::VecDeque;

use crate::{errors::RngError, traits::SimpleRng};

#[derive(Debug, PartialEq, Eq)]
pub enum FibOp32 {
    Add,
    Mul,
    Xor,
}

impl FibOp32 {
    pub fn op(&self, a: u64, b: u64) -> u64 {
        match self {
            FibOp32::Add => a + b,
            FibOp32::Mul => a * b,
            FibOp32::Xor => a ^ b,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FibOp64 {
    Add,
    Mul,
    Xor,
}

impl FibOp64 {
    pub fn op(&self, a: u128, b: u128) -> u128 {
        match self {
            FibOp64::Add => a + b,
            FibOp64::Mul => a * b,
            FibOp64::Xor => a ^ b,
        }
    }
}

pub struct Lfg32 {
    pub state: VecDeque<u32>,
    pub modulus: u32,
    pub tap: usize,
    pub op: FibOp32,
}

impl Default for Lfg32 {
    fn default() -> Self {
        Self {
            state: VecDeque::from([2, 5, 13, 29, 47]),
            modulus: 1000,
            tap: 3,
            op: FibOp32::Add,
        }
    }
}

impl Lfg32 {
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

impl SimpleRng for Lfg32 {
    // Will panic if tap is invalid
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

pub struct Lfg64 {
    pub state: VecDeque<u64>,
    pub modulus: u64,
    pub tap: usize,
    pub op: FibOp64,
}

impl Default for Lfg64 {
    fn default() -> Self {
        Self {
            state: VecDeque::from([2, 5, 13, 29, 47]),
            modulus: 1000,
            tap: 3,
            op: FibOp64::Add,
        }
    }
}

impl Lfg64 {
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

impl SimpleRng for Lfg64 {
    // Will panic if tap is invalid
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    // Will panic if tap is invalid
    fn next_u64(&mut self) -> u64 {
        let m = (self
            .op
            .op(self.state[0] as u128, self.state[self.tap] as u128)
            % self.modulus as u128) as u64;
        self.state.pop_front();
        self.state.push_back(m);
        m
    }
}
