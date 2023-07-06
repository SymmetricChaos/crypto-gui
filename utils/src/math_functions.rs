use mod_exp::mod_exp;
use num::{
    integer::Roots, traits::MulAddAssign, BigInt, FromPrimitive, Integer, One, ToPrimitive,
    Unsigned, Zero,
};
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

pub fn mul_inv<N: Integer + Copy + ToPrimitive + FromPrimitive>(num: N, modulus: N) -> Option<N> {
    if num.is_zero() {
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

pub fn modular_division(n: i32, d: i32, m: i32) -> Option<i32> {
    Some(n * mul_inv(d, m)?)
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
// First checks small possible factors then switches to deterministic Miller-Rabin
pub fn is_prime32(n: u32) -> bool {
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

pub fn eval_poly(x: i64, polynomial: &[i64], modulus: i64) -> i64 {
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

pub fn eval_poly_big(x: i32, polynomial: &[i32], modulus: i32) -> i32 {
    if polynomial.len() == 0 {
        return 0;
    }
    let x = BigInt::from(x);
    let mut acc = BigInt::zero();
    for coef in polynomial.iter().map(|n| BigInt::from(*n)).rev() {
        acc *= &x;
        acc += coef;
        acc %= modulus;
    }
    i32::try_from(acc).expect("accumulation should be i32 range due to modulo operation")
}

pub fn polynomial_string(polynomial: &[i32], ascending: bool) -> String {
    if polynomial.is_empty() {
        return String::from("0");
    }

    let mut out = String::new();
    let mut first_term = true;
    if ascending {
        for (n, c) in polynomial.iter().enumerate() {
            if c == &0 {
                continue;
            }
            if first_term {
                first_term = false;
                out.push_str(&first_term_str(c, n))
            } else {
                if c < &0 {
                    out.push_str(" - ");
                } else {
                    out.push_str(" + ");
                }
                out.push_str(&term_str(c, n))
            }
        }
    } else {
        let m = polynomial.len() - 1;
        for (n, c) in polynomial.iter().enumerate() {
            if c == &0 {
                continue;
            }
            if first_term {
                first_term = false;
                out.push_str(&first_term_str(c, m - n))
            } else {
                if c < &0 {
                    out.push_str(" - ");
                } else {
                    out.push_str(" + ");
                }
                out.push_str(&term_str(c, m - n))
            }
        }
    }

    out
}

fn first_term_str(c: &i32, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.is_one() {
            format!("x")
        } else if *c == -1 {
            format!("-x")
        } else {
            format!("{c}x")
        }
    } else {
        if c.is_one() {
            format!("x^{n}")
        } else if *c == -1 {
            format!("-x^{n}")
        } else {
            format!("{c}x^{n}")
        }
    }
}

fn term_str(c: &i32, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.abs() == 1 {
            format!("x")
        } else {
            format!("{}x", c.abs())
        }
    } else {
        if c.abs() == 1 {
            format!("x^{n}")
        } else {
            format!("{}x^{n}", c.abs())
        }
    }
}

#[cfg(test)]
mod math_function_tests {
    use super::*;
    #[test]
    fn polynomial_eval() {
        assert_eq!(eval_poly(2, &[1234, 166, 94], 1613), 329)
    }
    #[test]
    fn polynomial_eval_big() {
        assert_eq!(eval_poly_big(2, &[1234, 166, 94], 1613), 329)
    }
    #[test]
    fn polynomial_display() {
        assert_eq!(
            polynomial_string(&[1234, 166, 94], true),
            "1234 + 166x + 94x^2"
        );
        assert_eq!(
            polynomial_string(&[1234, 166, 94], false),
            "1234x^2 + 166x + 94"
        );
    }
}
