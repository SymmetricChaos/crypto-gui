use mod_exp::mod_exp;
use num::{
    bigint::ToBigInt, integer::Roots, BigInt, FromPrimitive, Integer, One, Signed, ToPrimitive,
    Unsigned, Zero,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
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

pub fn modular_division<N: Integer + Copy + ToPrimitive + FromPrimitive>(
    n: N,
    d: N,
    m: N,
) -> Option<N> {
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

// Evaluate a polynomial (with aescending degrees) at the point x by converting to BigInt to avoid overflow
pub fn eval_poly<N: Integer + Copy + ToBigInt>(x: N, polynomial: &[N], modulus: N) -> BigInt {
    if polynomial.len() == 0 {
        return BigInt::zero();
    }
    let x = x.to_bigint().unwrap();
    let modulus = modulus.to_bigint().unwrap();
    let mut acc = BigInt::zero();
    for coef in polynomial.iter().map(|n| n.to_bigint().unwrap()).rev() {
        acc *= &x;
        acc += coef;
        acc %= &modulus;
    }
    acc
}

pub fn polynomial_string_unsigned<N: Display + Zero + One + PartialEq + Unsigned>(
    polynomial: &[N],
    ascending: bool,
) -> String {
    if polynomial.is_empty() {
        return String::from("0");
    }

    let mut out = String::new();
    let mut coefs = polynomial.iter().skip_while(|c| c.is_zero()).enumerate();

    if ascending {
        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_unsigned(c, n)),
            None => return String::from("0"),
        };
        for (n, c) in coefs {
            if c.is_zero() {
                continue;
            }
            out.push_str(" + ");
            out.push_str(&term_str_unsigned(c, n))
        }
    } else {
        let m = polynomial.len() - 1;

        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_unsigned(c, m - n)),
            None => return String::from("0"),
        };
        for (n, c) in coefs {
            if c.is_zero() {
                continue;
            }
            out.push_str(" + ");
            out.push_str(&term_str_unsigned(c, m - n))
        }
    }

    out
}

fn first_term_str_unsigned<N: Display + Zero + One + PartialEq + Unsigned>(
    c: &N,
    n: usize,
) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.is_one() {
            format!("x")
        } else {
            format!("{c}x")
        }
    } else {
        if c.is_one() {
            format!("x^{n}")
        } else {
            format!("{c}x^{n}")
        }
    }
}

fn term_str_unsigned<N: Display + Zero + One + PartialEq + Unsigned>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.is_one() {
            format!("x")
        } else {
            format!("{}x", c)
        }
    } else {
        if c.is_one() {
            format!("x^{n}")
        } else {
            format!("{}x^{n}", c)
        }
    }
}

pub fn polynomial_string_signed<N: Display + Zero + One + PartialEq + Signed>(
    polynomial: &[N],
    ascending: bool,
) -> String {
    if polynomial.is_empty() {
        return String::from("0");
    }

    let mut out = String::new();
    let mut coefs = polynomial.iter().skip_while(|c| c.is_zero()).enumerate();

    if ascending {
        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_signed(c, n)),
            None => return String::from("0"),
        };
        for (n, c) in coefs {
            if c.is_zero() {
                continue;
            }
            if c.is_negative() {
                out.push_str(" - ");
            } else {
                out.push_str(" + ");
            }
            out.push_str(&term_str_signed(c, n))
        }
    } else {
        let m = polynomial.len() - 1;

        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_signed(c, m - n)),
            None => return String::from("0"),
        };
        for (n, c) in coefs {
            if c.is_zero() {
                continue;
            }
            if c.is_negative() {
                out.push_str(" - ");
            } else {
                out.push_str(" + ");
            }
            out.push_str(&term_str_signed(c, m - n))
        }
    }
    out
}

fn first_term_str_signed<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{c}x")
        }
    } else {
        if c.abs().is_one() {
            format!("x^{n}")
        } else {
            format!("{c}x^{n}")
        }
    }
}

fn term_str_signed<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{}x", c.abs())
        }
    } else {
        if c.abs().is_one() {
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
    fn polynomial_eval_big() {
        assert_eq!(
            i64::try_from(eval_poly(2, &[1234, 166, 94], 1613)).unwrap(),
            329_i64
        )
    }
    #[test]
    fn polynomial_display() {
        assert_eq!(
            polynomial_string_unsigned(&[1234_u32, 0, 166, 1, 94], true),
            "1234 + 166x^2 + x^3 + 94x^4"
        );
        assert_eq!(
            polynomial_string_signed(&[1234_i64, 0, -166, 1, 94], false),
            "1234x^4 - 166x^2 + x + 94"
        );
    }
}
