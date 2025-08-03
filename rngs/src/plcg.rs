use crate::traits::ClassicRng;

pub struct Plcg32 {
    pub state: u32,
    pub modulus: u32,
    pub coefs: Vec<u32>,
}

impl Default for Plcg32 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            coefs: vec![1013904223, 1664525],
        }
    }
}

impl Plcg32 {
    pub fn new(state: u32, modulus: u32, coefs: &[u32]) -> Self {
        Self {
            state,
            modulus,
            coefs: coefs.to_vec(),
        }
    }
}

impl ClassicRng for Plcg32 {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        let s = self.state as u64;
        let m = self.modulus as u64;
        // using Horner's method
        for coef in self.coefs.iter().rev().map(|c| *c as u64) {
            out = ((s * out) % m + coef) % m;
        }
        self.state = out as u32;
        self.state
    }

    // Default method is meaningless here
    fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64
    }
}

pub struct Plcg64 {
    pub state: u64,
    pub modulus: u64,
    pub coefs: Vec<u64>,
}

impl Default for Plcg64 {
    fn default() -> Self {
        Self {
            state: 1257924810,
            modulus: 4294967295,
            coefs: vec![1013904223, 1664525],
        }
    }
}

impl Plcg64 {
    pub fn new(state: u64, modulus: u64, coefs: &[u64]) -> Self {
        Self {
            state,
            modulus,
            coefs: coefs.to_vec(),
        }
    }
}

impl ClassicRng for Plcg64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = 0;
        let s = self.state as u128;
        let m = self.modulus as u128;
        // using Horner's method
        for coef in self.coefs.iter().rev().map(|c| *c as u128) {
            out = ((s * out) % m + coef) % m;
        }
        self.state = out as u64;
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs32() {
        let mut rng = Plcg32::new(0, 2147483648, &[12345, 1103515245]);
        assert_eq!(12345, rng.next_u32());
        assert_eq!(1406932606, rng.next_u32());
        assert_eq!(654583775, rng.next_u32());
        assert_eq!(1449466924, rng.next_u32());
        assert_eq!(229283573, rng.next_u32());
    }

    #[test]
    fn outputs64() {
        let mut rng = Plcg64::new(0, 2147483648, &[12345, 1103515245]);
        assert_eq!(12345, rng.next_u32());
        assert_eq!(1406932606, rng.next_u32());
        assert_eq!(654583775, rng.next_u32());
        assert_eq!(1449466924, rng.next_u32());
        assert_eq!(229283573, rng.next_u32());
    }
}
