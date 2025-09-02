use mod_exp::mod_exp;
use num::{Integer, Unsigned};
use std::collections::{BTreeMap, HashMap};

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
    sieve: HashMap<u64, Vec<u64>>,
    n: u64,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve {
            sieve: HashMap::<u64, Vec<u64>>::new(),
            n: 1,
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
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
