use crate::{errors::RngError, ClassicRng};
use num::BigUint;
use num_prime::nt_funcs::is_safe_prime;

pub fn acceptable_num(n: u64) -> bool {
    is_safe_prime(&n).probably() && n % 4 == 3
}

pub struct BlumBlumShub {
    m: BigUint,
    state: BigUint,
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
}

impl ClassicRng for BlumBlumShub {
    fn next_u32(&mut self) -> u32 {
        self.state = (&self.state * &self.state) % &self.m;
        // Lower 32 bits
        self.state.to_u32_digits()[0]
    }
}
