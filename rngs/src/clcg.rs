use crate::{
    lcg::{Lcg32, Lcg64},
    traits::ClassicRng,
};

pub struct Clcg32 {
    pub lcgs: Vec<Lcg32>,
}

impl Default for Clcg32 {
    fn default() -> Self {
        Self {
            lcgs: vec![Lcg32::default()],
        }
    }
}

impl ClassicRng for Clcg32 {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        let modulus = (self.lcgs[0].modulus - 1) as u64;
        for lcg in self.lcgs.iter_mut() {
            out = (out + lcg.next_u32() as u64) % modulus;
        }
        out as u32
    }
}

pub struct Clcg64 {
    pub lcgs: Vec<Lcg64>,
}

impl Default for Clcg64 {
    fn default() -> Self {
        Self {
            lcgs: vec![Lcg64::default()],
        }
    }
}

impl ClassicRng for Clcg64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = 0;
        let modulus = (self.lcgs[0].modulus - 1) as u128;
        for lcg in self.lcgs.iter_mut() {
            out = (out + lcg.next_u32() as u128) % modulus;
        }
        out as u64
    }
}
