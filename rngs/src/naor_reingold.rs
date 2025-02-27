use crate::ClassicRng;
use num_prime::nt_funcs::is_prime64;
use utils::{
    bits::u64_to_bit_vec,
    math_functions::{mod_mul_64, mod_pow_64},
};

pub fn valid_nr_constants(p: u64, q: u64, g: u64, arr: &Vec<u64>) -> bool {
    p > q
        && is_prime64(p)
        && is_prime64(q)
        && (p - 1) % q == 0
        && mod_pow_64(g, q, p) == 1
        && arr.len() < 64
}

pub fn valid_nr_constants_verbose(
    p: u64,
    q: u64,
    g: u64,
    arr: &Vec<u64>,
) -> Result<(), Vec<&'static str>> {
    let mut err = Vec::new();
    if p <= q {
        err.push("p must be greater than q");
    }
    if !is_prime64(p) {
        err.push("p must be prime");
    }
    if !is_prime64(q) {
        err.push("q must be prime");
    }
    if (p - 1) % q != 0 {
        err.push("q must be a factor of p-1");
    }
    if mod_pow_64(g, q, p) != 1 {
        err.push("g must be of multiplicative order q in the ring F_p");
    }
    if arr.len() >= 64 {
        err.push("arr must have a length less than 64");
    }

    if err.is_empty() {
        Ok(())
    } else {
        Err(err)
    }
}

pub struct NaorReingold {
    p: u64,
    generator: u64,
    arr: Vec<u64>,
    pub ctr: u64,
}

impl Default for NaorReingold {
    fn default() -> Self {
        Self {
            p: 1223,
            generator: 27,
            arr: vec![7, 6, 5, 4, 3, 2],
            ctr: 1,
        }
    }
}

impl NaorReingold {
    // Initialize a Naor-Reingold PRNG, returns None if invalid constants are given
    pub fn init(p: u64, q: u64, generator: u64, arr: Vec<u64>, ctr: u64) -> Option<Self> {
        match valid_nr_constants(p, q, generator, &arr) {
            true => Some(Self {
                p,
                generator,
                arr: arr.iter().map(|a| a % p).collect(),
                ctr,
            }),
            false => None,
        }
    }

    // Initialize a Naor-Reingold PRNG, returns a list of failures if invalid constants are given
    pub fn init_verbose(
        p: u64,
        q: u64,
        generator: u64,
        arr: Vec<u64>,
        ctr: u64,
    ) -> Result<Self, Vec<&'static str>> {
        match valid_nr_constants_verbose(p, q, generator, &arr) {
            Ok(_) => Ok(Self {
                p,
                generator,
                arr: arr.iter().map(|a| a % p).collect(),
                ctr,
            }),
            Err(e) => Err(e),
        }
    }
}

impl ClassicRng for NaorReingold {
    fn next_u32(&mut self) -> u32 {
        // Get the bits of the counter
        let bits = u64_to_bit_vec(self.ctr);

        // Multiply together the elements of the array selected by the bits
        let e = self
            .arr
            .iter()
            .zip(bits.into_iter())
            .filter(|(_, bit)| bool::from(*bit))
            .fold(1, |acc, (a, _)| mod_mul_64(acc, *a, self.p));

        // Raise the generating point to the power of e
        let out = mod_pow_64(self.generator, e, self.p);

        // Increment
        // Any function that visits every possible value in 0..(2^(self.arr.len())) can be used
        // but an increment is the simplest and it is not clear that others offer additional
        // security
        self.ctr += 1;
        // Reduce
        self.ctr %= 1 << self.arr.len() as u64;

        out as u32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore = "visual test"]
    #[test]
    fn runtime() {
        let mut rng = NaorReingold::default();
        for _ in 0..6 {
            println!("{}", rng.next_u32())
        }
    }
}
