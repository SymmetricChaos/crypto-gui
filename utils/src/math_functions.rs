use mod_exp::mod_exp;
use num::{bigint::ToBigInt, integer::Roots, BigInt, Integer, One, ToPrimitive, Unsigned, Zero};
use std::collections::{BTreeMap, HashMap};

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

pub fn mul_inv<N: ToBigInt>(num: &N, modulus: &N) -> Option<BigInt> {
    let num = num.to_bigint().expect("unable to convert num to BigInt");
    let modulus = modulus
        .to_bigint()
        .expect("unable to convert modulus to BigInt");
    if num.is_zero() {
        return None;
    }

    let egcd = num.extended_gcd(&modulus);
    if !egcd.gcd.is_one() {
        None
    } else {
        Some(egcd.x.mod_floor(&modulus))
    }
}

pub fn modular_division<N: ToBigInt>(n: &N, d: &N, m: &N) -> Option<BigInt> {
    Some(
        n.to_bigint()
            .expect("unable to convert numerator to BigInt")
            * mul_inv(d, m)?,
    )
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

// 32-bit primality test
// First checks small prime factors then switches to deterministic Miller-Rabin
pub fn is_prime32<N: Into<u32>>(n: N) -> bool {
    let n = n.into();
    if n <= 1 {
        return false;
    }

    // Check all primes up to 61 (the largest witness used)
    let small_factors = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];

    for p in small_factors.iter() {
        if n == *p {
            return true;
        }
        if n % *p == 0 {
            return false;
        }
    }

    let mut d = (n - 1) / 2;
    let mut r = 1;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    let witnesses = [2, 7, 61];

    'outer: for w in witnesses.iter() {
        let mut x = mod_exp(*w as u64, d as u64, n as u64) as u32;

        if x == 1 || x == n - 1 {
            continue 'outer;
        }
        for _ in 0..r - 1 {
            x = mod_exp(x as u64, 2u64, n as u64) as u32;

            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// 64-bit primality test
// First checks small prime factors then switches to deterministic Miller-Rabin
pub fn is_prime64<N: Into<u64>>(n: N) -> bool {
    let n = n.into();
    if n <= 1 {
        return false;
    }

    // The first 12 primes are sufficient witnesses
    let witnesses = [2_u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    // Quickly check each witness and short circuit if needed
    for p in witnesses.iter() {
        if n == *p {
            return true;
        }
        if n % *p == 0 {
            return false;
        }
    }

    let mut d = (n - 1) / 2;
    let mut r = 1;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'outer: for w in witnesses.iter() {
        let mut x = mod_exp(*w as u128, d as u128, n as u128) as u64;

        if x == 1 || x == n - 1 {
            continue 'outer;
        }
        for _ in 0..r - 1 {
            x = mod_exp(x as u128, 2u128, n as u128) as u64;

            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

pub fn modular_pow(base: u32, pow: u32, modulus: u32) -> u32 {
    let mut out = 1;

    for _ in 0..pow {
        out *= u64::from(base);
        out %= u64::from(modulus);
    }

    // This truncation is always valid because it has been reduced by the modulus which starts as u32
    out as u32
}

pub fn incr_array_ctr_be(ctr: &mut [u8]) {
    for byte in ctr.iter_mut().rev() {
        match byte.checked_add(1) {
            Some(n) => {
                *byte = n;
                return ();
            }

            None => {
                *byte = 0;
            }
        }
    }
}

pub fn incr_array_ctr_le(ctr: &mut [u8]) {
    for byte in ctr.iter_mut() {
        match byte.checked_add(1) {
            Some(n) => {
                *byte = n;
                return ();
            }

            None => {
                *byte = 0;
            }
        }
    }
}

#[cfg(test)]
mod math_tests {

    use super::*;

    #[test]
    fn test_mod_pow() {
        let x = modular_pow(4, 5, 23);
        assert_eq!(4, x);
        let x = modular_pow(3, 5, 23);
        assert_eq!(10, x);
    }

    #[test]
    fn test_incr_array_ctr() {
        let mut ctr = [0x00, 0x00, 0xfe, 0xff, 0xff];
        incr_array_ctr_be(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x00], ctr);
        incr_array_ctr_be(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x01], ctr);

        let mut ctr = [0xff, 0xff, 0xfe, 0x00, 0x00];
        incr_array_ctr_le(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x00], ctr);
        incr_array_ctr_le(&mut ctr);
        assert_eq!([0x01, 0x00, 0xff, 0x00, 0x00], ctr);
    }
}
