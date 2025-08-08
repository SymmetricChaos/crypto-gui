use crate::traits::SimpleRng;

pub struct Lcg32 {
    pub state: u32,
    pub multiplier: u32,
    pub increment: u32,
}

impl Default for Lcg32 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl Lcg32 {
    pub fn new(state: u32, multiplier: u32, increment: u32) -> Self {
        Self {
            state,
            multiplier,
            increment,
        }
    }
}

impl SimpleRng for Lcg32 {
    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
        self.state
    }
}

pub struct Lcg64 {
    pub state: u64,
    pub multiplier: u64,
    pub increment: u64,
}

impl Default for Lcg64 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl Lcg64 {
    pub fn new(state: u64, multiplier: u64, increment: u64) -> Self {
        Self {
            state,
            multiplier,
            increment,
        }
    }
}

impl SimpleRng for Lcg64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
        self.state
    }
}

// LCG with a selectable modulus
pub struct LcgM {
    pub state: u64,
    pub modulus: u64,
    pub multiplier: u64,
    pub increment: u64,
}

impl Default for LcgM {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            multiplier: 1664525,
            increment: 1013904223,
        }
    }
}

impl LcgM {
    pub fn new(state: u64, modulus: u64, multiplier: u64, increment: u64) -> Self {
        Self {
            state,
            modulus,
            multiplier,
            increment,
        }
    }
}

impl SimpleRng for LcgM {
    fn next_u64(&mut self) -> u64 {
        let m = (self.state as u128 * self.multiplier as u128) % self.modulus as u128;
        self.state = ((m + self.increment as u128) % self.modulus as u128) as u64;
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs() {
        let mut rng = LcgM::new(0, 2147483648, 1103515245, 12345);
        assert_eq!(12345, rng.next_u32());
        assert_eq!(1406932606, rng.next_u32());
        assert_eq!(654583775, rng.next_u32());
        assert_eq!(1449466924, rng.next_u32());
        assert_eq!(229283573, rng.next_u32());
    }
}
