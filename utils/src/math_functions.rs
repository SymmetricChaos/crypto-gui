use std::collections::{BTreeMap, HashMap};

use num::{
    integer::Roots, traits::MulAddAssign, FromPrimitive, Integer, One, ToPrimitive, Unsigned,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Odd,
    Even,
}

impl Parity {
    pub fn cycle(&self) -> std::iter::Cycle<std::array::IntoIter<Parity, 2>> {
        match self {
            Parity::Odd => [Self::Odd, Self::Even].into_iter().cycle(),
            Parity::Even => [Self::Even, Self::Odd].into_iter().cycle(),
        }
    }
}

pub fn is_square(n: usize) -> bool {
    n.sqrt().pow(2) == n
}

pub fn mul_inv<N: Integer + Copy + ToPrimitive + FromPrimitive>(num: N, modulus: N) -> Option<N> {
    if num < N::one() {
        return None;
    }
    let num = num.to_isize()?;
    let modulus = modulus.to_isize()?;
    let egcd = num.extended_gcd(&modulus);
    if !egcd.gcd.is_one() {
        None
    } else {
        Some(N::from_isize(egcd.x.mod_floor(&modulus))?)
    }
}

// We're not going to deal with big numbers so this thse crude factorizations are plenty
pub fn factors<N: Integer + Copy + ToPrimitive>(n: N) -> Vec<N> {
    let mut out = Vec::new();
    for f in num::range(N::one(), n) {
        if n.is_multiple_of(&f) {
            out.push(n);
            out.push(n.div(f));
        }
    }
    out.sort();
    out
}

pub fn prime_factorization<N: Integer + Copy + Unsigned>(n: N) -> Vec<N> {
    if n.is_zero() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut n = n;
    let mut f = N::one();
    while !n.is_one() {
        f = f + N::one();
        while n.is_multiple_of(&f) {
            n = n / f;
            out.push(f);
        }
    }
    out.sort();
    out
}

pub fn prime_factorization_map<N: Integer + Copy + Unsigned>(n: N) -> BTreeMap<N, usize> {
    if n.is_zero() {
        return BTreeMap::new();
    }
    let mut out = BTreeMap::new();
    let mut n = n;
    let mut f = N::one();
    while !n.is_one() {
        f = f + N::one();
        let mut ctr = 0;
        while n.is_multiple_of(&f) {
            ctr += 1;
            n = n / f;
        }
        if ctr != 0 {
            out.insert(n, ctr);
        }
    }
    out
}

pub fn prime_factors<N: Integer + Copy + Unsigned>(n: N) -> Vec<N> {
    let mut out = prime_factorization(n);
    out.dedup();
    out
}

pub struct PrimeSieve {
    sieve: HashMap<usize, Vec<usize>>,
    n: usize,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve {
            sieve: HashMap::<usize, Vec<usize>>::new(),
            n: 1usize,
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(self.n + self.n, vec![self.n]);
                return Some(self.n);
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    if self.sieve.contains_key(&(factor + self.n)) {
                        self.sieve
                            .get_mut(&(factor + self.n))
                            .unwrap()
                            .push(*factor);
                    } else {
                        self.sieve.insert(factor + self.n, vec![*factor]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}

pub fn eval_poly(x: u32, polynomial: &[u32], modulus: u32) -> u32 {
    if polynomial.len() == 0 {
        return 0;
    }
    let mut acc = 0;
    for &coef in polynomial.iter().rev() {
        acc.mul_add_assign(x, coef);
        acc %= modulus;
    }
    acc
}

#[cfg(test)]
mod math_function_tests {
    use super::*;
    #[test]
    fn polynomial() {
        assert_eq!(eval_poly(2, &[1234, 166, 94], 1613), 329)
    }
}
