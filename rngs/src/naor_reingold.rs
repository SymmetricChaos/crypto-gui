use crate::ClassicRng;
use num_prime::nt_funcs::is_prime64;
use utils::{
    bits::u64_to_bit_vec,
    math_functions::{mod_mul_64, mod_pow_64},
};

pub fn valid_nr_constants(p: u64, q: u64, g: u64) -> bool {
    p > q && is_prime64(p) && is_prime64(q) && (p - 1) % q == 0 && mod_pow_64(g, q, p) == 1
}

pub fn valid_nr_constants_verbose(p: u64, q: u64, g: u64) -> Result<(), String> {
    let mut err = String::new();
    if p <= q {
        err.push_str("p must be greater than q;");
    }
    if !is_prime64(p) {
        err.push_str("p must be prime;");
    }
    if !is_prime64(q) {
        err.push_str("q must be prime;");
    }
    if (p - 1) % q != 0 {
        err.push_str("p-1 must be a multiple of q;");
    }
    if mod_pow_64(g, q, p) != 1 {
        err.push_str("g must be of multiplicative order q in the finite field F_p;");
    }

    if err.is_empty() {
        Ok(())
    } else {
        Err(err)
    }
}

pub struct NaorReingold {
    p: u64,
    q: u64, // not actually used in the generating process
    generator: u64,
    arr: Vec<u64>,
    ctr: u64,
}

impl Default for NaorReingold {
    fn default() -> Self {
        Self {
            p: 1223,
            q: 47,
            generator: 27,
            arr: vec![7, 6, 5, 4, 3, 2],
            ctr: 1,
        }
    }
}

impl NaorReingold {
    pub fn init(p: u64, q: u64, g: u64, a: Vec<u64>, x: u64) -> Result<Self, String> {
        match valid_nr_constants_verbose(p, q, g) {
            Ok(_) => Ok(Self {
                p,
                q,
                generator: g,
                arr: a,
                ctr: x,
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
        self.ctr += 1;

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
