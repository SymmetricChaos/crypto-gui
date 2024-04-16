use crate::{errors::RngError, ClassicRng};
use num::BigUint;
use num_prime::nt_funcs::is_safe_prime;

pub fn acceptable_num(n: u64) -> bool {
    is_safe_prime(&n).probably() && n % 4 == 3
}

// https://shub.ccny.cuny.edu/articles/1986-A_simple_unpredictable_pseudo-random_number_generator.pdf
pub struct BlumBlumShub {
    pub m: BigUint,
    pub state: BigUint,
}

impl Default for BlumBlumShub {
    fn default() -> Self {
        Self {
            m: Default::default(),
            state: Default::default(),
        }
    }
}

impl BlumBlumShub {
    pub fn set_m(&mut self, p: u64, q: u64) -> Result<(), RngError> {
        if acceptable_num(p) && acceptable_num(q) {
            self.m = BigUint::from(p) * BigUint::from(q);
            Ok(())
        } else {
            Err(RngError::general("either p or q is not acceptable"))
        }
    }

    pub fn step(&mut self) {
        self.state = (&self.state * &self.state) % &self.m;
    }

    pub fn next_u8(&mut self) -> u8 {
        let mut out = 0;
        // Extract 8 bits using the parity of 8 consecutive states
        for i in 0..8 {
            self.state = (&self.state * &self.state) % &self.m;
            out |= ((self.state.count_ones() % 2) as u8) << i;
        }
        out
    }
}

impl ClassicRng for BlumBlumShub {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        // Extract 32 bits using the parity of 32 consecutive states
        for i in 0..32 {
            self.state = (&self.state * &self.state) % &self.m;
            out |= ((self.state.count_ones() % 2) as u32) << i;
        }
        out
    }
}
