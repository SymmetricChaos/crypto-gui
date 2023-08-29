use crate::bits::{bits_from_string, Bit, CharToBitError, IntToBitError};
use itertools::Itertools;
use num::{One, Zero};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

// Polynomial of GF(2) with coefficients in ascending order so that the coefficient at index n is with power n
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitPolynomial {
    pub coef: Vec<Bit>,
}

impl Display for BitPolynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.coef.iter().map(|b| b.to_char()).collect();
        write!(f, "{}", s)
    }
}

impl BitPolynomial {
    pub fn trim(&mut self) {
        loop {
            match self.coef.pop() {
                Some(n) => {
                    if n.is_zero() {
                        continue;
                    } else {
                        self.coef.push(n);
                        break;
                    }
                }
                None => break,
            }
        }
    }

    // Get irrefutable, returns a clone of the coefficient or zero if the value is too high
    pub fn get_irref(&self, n: usize) -> Bit {
        match self.get(n) {
            Some(n) => n.clone(),
            None => Bit::zero(),
        }
    }

    pub fn increase_degree(&mut self, n: usize) {
        for _ in 0..n {
            self.coef.insert(0, Bit::zero())
        }
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn degree(&self) -> usize {
        if self.len() > 0 {
            self.len() - 1
        } else {
            0
        }
    }

    pub fn get(&self, n: usize) -> Option<&Bit> {
        self.coef.get(n)
    }

    pub fn get_mut(&mut self, n: usize) -> Option<&mut Bit> {
        self.coef.get_mut(n)
    }

    // Reverse order of coefficients
    pub fn reverse(&mut self) {
        self.coef.reverse()
    }

    // Clone of the polynomial with coefficients in reversed order
    pub fn reversed(&self) -> BitPolynomial {
        let mut out = self.clone();
        out.reverse();
        out
    }

    fn polynomial_term(n: usize) -> String {
        if n == 0 {
            String::from("1")
        } else if n == 1 {
            String::from("x")
        } else {
            format!("x^{n}")
        }
    }

    pub fn polynomial_string(&self) -> String {
        if self.coef.is_empty() {
            return String::from("0");
        }

        let mut out = String::new();

        let mut coefs = self
            .coef
            .iter()
            .rev()
            .enumerate()
            .skip_while(|(_, c)| c.is_zero());

        let m = self.coef.len() - 1;

        match coefs.next() {
            Some((n, _)) => out.push_str(&Self::polynomial_term(m - n)),
            None => return String::from("0"),
        }

        for (n, c) in coefs {
            if c.is_zero() {
                continue;
            }
            out.push_str(" + ");
            out.push_str(&Self::polynomial_term(m - n))
        }

        out
    }

    pub fn evaluate(&self, x: usize) -> usize {
        let mut out = 0;
        let mut n = 1;
        for c in self.coef.iter().rev() {
            if c.is_one() {
                out += n;
            }
            n += x
        }
        out
    }

    pub fn from_int_array<T: Copy, const N: usize>(
        arr: [T; N],
    ) -> Result<BitPolynomial, IntToBitError>
    where
        Bit: TryFrom<T>,
        IntToBitError: From<<Bit as TryFrom<T>>::Error>,
    {
        let mut v = [Bit::Zero; N];
        for (n, i) in arr.iter().enumerate() {
            v[n] = Bit::try_from(*i)?;
        }
        Ok(BitPolynomial::from(v))
    }

    pub fn from_int_vec<T: Copy>(vec: &Vec<T>) -> Result<BitPolynomial, IntToBitError>
    where
        Bit: TryFrom<T>,
        IntToBitError: From<<Bit as TryFrom<T>>::Error>,
    {
        let mut v = vec![Bit::Zero; vec.len()];
        for (n, i) in vec.iter().enumerate() {
            v[n] = Bit::try_from(*i)?;
        }
        Ok(BitPolynomial::from(v))
    }

    pub fn from_str<S: AsRef<str>>(s: S) -> Result<BitPolynomial, CharToBitError> {
        let bits = bits_from_string(s.as_ref())?;
        Ok(BitPolynomial::from(bits.collect_vec()))
    }

    pub fn from_iter(iter: impl Iterator<Item = Bit>) -> BitPolynomial {
        BitPolynomial::from(iter.collect_vec())
    }

    pub fn div_rem(&self, rhs: &BitPolynomial) -> (BitPolynomial, BitPolynomial) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() {
            return (BitPolynomial::zero(), BitPolynomial::zero());
        }

        if self.degree() < rhs.degree() {
            return (BitPolynomial::zero(), self.clone());
        }

        // General case
        let mut quotient = BitPolynomial::zero();
        let mut remainder = self.clone().reversed();

        while !remainder.is_zero() && remainder.degree() >= rhs.degree() {
            let pow = remainder.degree() - rhs.degree();
            let mut intermediate = BitPolynomial::one();
            intermediate.increase_degree(pow);
            quotient += intermediate.clone();
            remainder += intermediate * rhs.clone();
        }

        (quotient, remainder)
    }
}

impl From<Vec<Bit>> for BitPolynomial {
    fn from(value: Vec<Bit>) -> Self {
        let mut p = Self { coef: value };
        p.trim();
        p
    }
}

impl<const N: usize> From<[Bit; N]> for BitPolynomial {
    fn from(value: [Bit; N]) -> Self {
        let mut p = Self {
            coef: value.to_vec(),
        };
        p.trim();
        p
    }
}

impl Index<usize> for BitPolynomial {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<usize> for BitPolynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Zero for BitPolynomial {
    fn zero() -> Self {
        BitPolynomial { coef: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.coef.is_empty()
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

impl One for BitPolynomial {
    fn one() -> Self {
        BitPolynomial {
            coef: vec![Bit::One],
        }
    }
}

// Addition (also Subtraction)
impl Add for BitPolynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.len().max(rhs.len());
        let mut coef = Vec::with_capacity(len);
        for idx in 0..len {
            let sum = self.get_irref(idx) + rhs.get_irref(idx);
            coef.push(sum);
        }
        BitPolynomial::from(coef)
    }
}

impl Add<&BitPolynomial> for BitPolynomial {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let len = self.len().max(rhs.len());
        let mut coef = Vec::with_capacity(len);
        for idx in 0..len {
            let sum = self.get_irref(idx) + rhs.get_irref(idx);
            coef.push(sum);
        }
        BitPolynomial::from(coef)
    }
}

impl AddAssign for BitPolynomial {
    fn add_assign(&mut self, rhs: Self) {
        while self.len() < rhs.len() {
            self.coef.push(Bit::Zero)
        }
        for (idx, rhs_coef) in rhs.coef.iter().cloned().enumerate() {
            self.coef[idx] += rhs_coef;
        }
        self.trim()
    }
}

impl AddAssign<&BitPolynomial> for BitPolynomial {
    fn add_assign(&mut self, rhs: &Self) {
        while self.len() < rhs.len() {
            self.coef.push(Bit::Zero)
        }
        for (idx, rhs_coef) in rhs.coef.iter().cloned().enumerate() {
            self.coef[idx] += rhs_coef;
        }
        self.trim()
    }
}

// Multiplication
impl Mul for BitPolynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coef = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        BitPolynomial::from(coef)
    }
}

impl Mul<&BitPolynomial> for BitPolynomial {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut coef = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        BitPolynomial::from(coef)
    }
}

impl MulAssign for BitPolynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let mut coef = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        *self = BitPolynomial::from(coef);
        self.trim()
    }
}

impl MulAssign<&BitPolynomial> for BitPolynomial {
    fn mul_assign(&mut self, rhs: &Self) {
        let mut coef = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        *self = BitPolynomial::from(coef);
        self.trim()
    }
}

#[cfg(test)]
mod math_function_tests {

    use super::*;

    #[test]
    fn polynomial_mul() {
        let a = BitPolynomial::from_str("11").unwrap();
        let b = BitPolynomial::from_str("11").unwrap();
        assert_eq!(a * b, BitPolynomial::from_int_array([1, 0, 1]).unwrap());

        let a = BitPolynomial::from_str("111").unwrap();
        let b = BitPolynomial::from_str("111").unwrap();
        assert_eq!(
            a * b,
            BitPolynomial::from_int_array([1, 0, 1, 0, 1]).unwrap()
        );
    }

    #[test]
    fn polynomial_add() {
        let m = BitPolynomial::from_str("101").unwrap();
        let n = BitPolynomial::from_str("11").unwrap();
        assert_eq!(m + n, BitPolynomial::from_str("011").unwrap())
    }

    #[test]
    fn polynomial_div() {
        let m = BitPolynomial::from_str("101").unwrap();
        let n = BitPolynomial::from_str("11").unwrap();
        assert_eq!(m.div_rem(&n).0, n)
    }

    #[test]
    fn example_division_for_crc() {
        let m = BitPolynomial::from_str("11010011101100000")
            .unwrap()
            .reversed();
        let n = BitPolynomial::from_str("1011").unwrap().reversed();
        println!("{}\n{}", m.polynomial_string(), n.polynomial_string());
        let (q, r) = m.div_rem(&n);
        println!("{} {}", q, r)
    }
}
