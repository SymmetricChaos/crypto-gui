use crate::ClassicRng;
use num_prime::nt_funcs::is_prime64;
use utils::{
    bits::u64_to_bit_vec,
    math_functions::{mod_mul_64, mod_pow_64},
};

pub struct NaorReingold {
    p: u64,
    q: u64,
    g: u64,
    a: Vec<u64>,
    x: u64,
}

impl NaorReingold {
    pub fn valid_constants(&self) -> bool {
        is_prime64(self.p)
            && is_prime64(self.q)
            && (self.p - 1) % self.q == 0
            && mod_pow_64(self.g, self.q, self.p) == 1
    }
}

impl ClassicRng for NaorReingold {
    fn next_u32(&mut self) -> u32 {
        let v = u64_to_bit_vec(self.x);
        let e = self
            .a
            .iter()
            .zip(v.into_iter())
            .filter(|(_, v)| bool::from(*v))
            .fold(1, |acc, (a, _)| mod_mul_64(acc, *a, self.p));
        let out = mod_pow_64(self.g, e, self.p);
        self.x += 1;
        out as u32
    }
}
