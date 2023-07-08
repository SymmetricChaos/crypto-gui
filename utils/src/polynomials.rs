use std::fmt::Display;

use itertools::Itertools;
use num::{bigint::ToBigInt, BigInt, One, Signed, Unsigned, Zero};

use crate::math_functions::modular_division;

pub fn lagrange_interpolation<N: ToBigInt>(x: N, pairs: &[(N, N)], modulus: N) -> Option<BigInt> {
    let mut nums: Vec<BigInt> = Vec::new();
    let mut dens: Vec<BigInt> = Vec::new();
    let x = x.to_bigint().expect("unable to convert x to BigInt");
    let m = modulus
        .to_bigint()
        .expect("unable to convert modulus to BigInt");
    let pairs = pairs
        .iter()
        .map(|(a, b)| {
            (
                a.to_bigint()
                    .expect("unable to convert left side of a pair to BigInt"),
                b.to_bigint()
                    .expect("unable to convert right side of a pair to BigInt"),
            )
        })
        .collect_vec();

    for i in 0..pairs.len() {
        let mut others = pairs.clone();
        let cur = others.remove(i).0;
        nums.push(others.iter().map(|(e, _)| &x - e).product());
        dens.push(others.iter().map(|(e, _)| &cur - e).product());
    }

    let denominator = dens.iter().product::<BigInt>() % &m;

    let numerator = {
        let mut n = BigInt::zero();
        for i in 0..pairs.len() {
            let nm = (&nums[i] * &denominator * &pairs[i].1) % &m;
            n += modular_division(&nm, &dens[i], &m)?;
        }
        n %= &m;
        n
    };

    Some((modular_division(&numerator, &denominator, &m)? % &m + &m) % &m)
}

// Evaluate a polynomial (with aescending terms) at the point x by converting to BigInt to avoid overflow
pub fn eval_poly<N: ToBigInt>(x: N, polynomial: &[N], modulus: N, ascending: bool) -> BigInt {
    if polynomial.len() == 0 {
        return BigInt::zero();
    }
    let x = x.to_bigint().unwrap();
    let modulus = modulus.to_bigint().unwrap();
    let mut acc = BigInt::zero();
    if ascending {
        for coef in polynomial.iter().map(|n| n.to_bigint().unwrap()).rev() {
            acc *= &x;
            acc += coef;
            acc %= &modulus;
        }
    } else {
        for coef in polynomial.iter().map(|n| n.to_bigint().unwrap()) {
            acc *= &x;
            acc += coef;
            acc %= &modulus;
        }
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
    let mut coefs = polynomial
        .iter()
        .enumerate()
        .skip_while(|(_, c)| c.is_zero());

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
    let mut coefs = polynomial
        .iter()
        .enumerate()
        .skip_while(|(_, c)| c.is_zero());

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
    fn polynomial_eval() {
        assert_eq!(
            i64::try_from(eval_poly(2, &[1234, 166, 94], 1613, true)).unwrap(),
            329_i64
        )
    }
    #[test]
    fn polynomial_display() {
        assert_eq!(
            polynomial_string_unsigned(&[1234_u32, 0, 166, 1, 94, 0], true),
            "1234 + 166x^2 + x^3 + 94x^4"
        );
        assert_eq!(
            polynomial_string_unsigned(&[1234_u32, 0, 166, 1, 94, 0], false),
            "1234x^5 + 166x^3 + x^2 + 94x"
        );
        assert_eq!(
            polynomial_string_signed(&[1234_i64, 0, -166, 1, 94, 0], true),
            "1234 - 166x^2 + x^3 + 94x^4"
        );
        assert_eq!(
            polynomial_string_signed(&[1234_i64, 0, -166, 1, 94, 0], false),
            "1234x^5 - 166x^3 + x^2 + 94x"
        );
    }
}
