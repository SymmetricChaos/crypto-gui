use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use itertools::Itertools;
use num::{One, Zero};

use crate::bits::{Bit, IntToBitError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolynomialUV<T> {
    pub coef: Vec<T>,
}

impl<T: Zero> PolynomialUV<T> {
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
}

impl<T: Zero + Clone> PolynomialUV<T> {
    // Get irrefutable, returns a clone of the coefficient or zero if the value is too high
    pub fn get_irref(&self, n: usize) -> T {
        match self.get(n) {
            Some(n) => n.clone(),
            None => T::zero(),
        }
    }

    pub fn increase_degree(mut self, n: usize) -> PolynomialUV<T> {
        for _ in 0..n {
            self.coef.insert(0, T::zero());
        }
        self
    }
}

impl<T> PolynomialUV<T> {
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

    pub fn get(&self, n: usize) -> Option<&T> {
        self.coef.get(n)
    }

    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.coef.get_mut(n)
    }
}

impl PolynomialUV<Bit> {
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
    ) -> Result<PolynomialUV<Bit>, IntToBitError>
    where
        Bit: TryFrom<T>,
        IntToBitError: From<<Bit as TryFrom<T>>::Error>,
    {
        let mut v = [Bit::Zero; N];
        for (n, i) in arr.iter().enumerate() {
            v[n] = Bit::try_from(*i)?;
        }
        Ok(PolynomialUV::from(v))
    }
}

impl<T: Zero> From<Vec<T>> for PolynomialUV<T> {
    fn from(coef: Vec<T>) -> Self {
        let mut p = PolynomialUV { coef };
        p.trim();
        p
    }
}

impl<T: Clone + Zero, const K: usize> From<[T; K]> for PolynomialUV<T> {
    fn from(coef: [T; K]) -> Self {
        let coef: Vec<T> = coef.iter().cloned().collect_vec();
        let mut p = PolynomialUV { coef };
        p.trim();
        p
    }
}

impl PolynomialUV<Bit> {
    pub fn div_rem(&self, rhs: PolynomialUV<Bit>) -> (PolynomialUV<Bit>, PolynomialUV<Bit>) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() {
            return (PolynomialUV::zero(), PolynomialUV::zero());
        }

        if self.degree() < rhs.degree() {
            return (PolynomialUV::zero(), self.clone());
        }

        // General case
        let mut quotient = PolynomialUV::zero();
        let mut remainder = self.clone();

        while !remainder.is_zero() && remainder.degree() >= rhs.degree() {
            let pow = remainder.degree() - rhs.degree();
            let intermediate = PolynomialUV::one().increase_degree(pow);
            quotient += intermediate.clone();
            remainder += intermediate * rhs.clone();
        }

        (quotient, remainder)
    }
}

impl Zero for PolynomialUV<Bit> {
    fn zero() -> Self {
        PolynomialUV { coef: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.coef.is_empty()
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

impl One for PolynomialUV<Bit> {
    fn one() -> Self {
        PolynomialUV {
            coef: vec![Bit::One],
        }
    }
}

// Addition (also Subtraction for PolynomialUV<Bit>)
impl Add for PolynomialUV<Bit> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.len().max(rhs.len());
        let mut coef: Vec<Bit> = Vec::with_capacity(len);
        for idx in 0..len {
            let sum = self.get_irref(idx) + rhs.get_irref(idx);
            coef.push(sum);
        }
        PolynomialUV::from(coef)
    }
}

impl AddAssign for PolynomialUV<Bit> {
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

// Addition of an scalar
impl Add<Bit> for PolynomialUV<Bit> {
    type Output = Self;

    fn add(self, rhs: Bit) -> Self::Output {
        let mut coef: Vec<Bit> = self.coef.clone();
        coef[0] += rhs;
        PolynomialUV::from(coef)
    }
}

impl AddAssign<Bit> for PolynomialUV<Bit> {
    fn add_assign(&mut self, rhs: Bit) {
        self.coef[0] += rhs;
        self.trim();
    }
}

// Multiplication
impl Mul for PolynomialUV<Bit> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coef: Vec<Bit> = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        PolynomialUV::from(coef)
    }
}

impl MulAssign for PolynomialUV<Bit> {
    fn mul_assign(&mut self, rhs: Self) {
        let mut coef: Vec<Bit> = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        *self = PolynomialUV::from(coef)
    }
}

// Scalar Multiplication
impl Mul<Bit> for PolynomialUV<Bit> {
    type Output = Self;

    fn mul(self, rhs: Bit) -> Self::Output {
        PolynomialUV::from(
            self.coef
                .iter()
                .map(|x| x.clone() * rhs.clone())
                .collect_vec(),
        )
    }
}

impl MulAssign<Bit> for PolynomialUV<Bit> {
    fn mul_assign(&mut self, rhs: Bit) {
        *self = PolynomialUV::from(
            self.coef
                .iter()
                .map(|x| x.clone() * rhs.clone())
                .collect_vec(),
        )
    }
}

// Additive inverse
impl Neg for PolynomialUV<Bit> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        PolynomialUV::from(self.coef.iter().cloned().map(|x| -x).collect_vec())
    }
}

#[cfg(test)]
mod math_function_tests {

    use super::*;

    #[test]
    fn polynomial_mul() {
        let m = PolynomialUV::from_int_array([1, 0, 1]).unwrap();
        let n = PolynomialUV::from_int_array([1, 1]).unwrap();
        assert_eq!(n.clone() * n, m)
    }

    #[test]
    fn polynomial_add() {
        let m = PolynomialUV::from_int_array([1, 0, 1]).unwrap();
        let n = PolynomialUV::from_int_array([1, 1]).unwrap();
        assert_eq!(m + n, PolynomialUV::from_int_array([0, 1, 1]).unwrap())
    }

    #[test]
    fn polynomial_div() {
        let m = PolynomialUV::from_int_array([1, 0, 1]).unwrap();
        let n = PolynomialUV::from_int_array([1, 1]).unwrap();
        assert_eq!(m.div_rem(n.clone()).0, n)
    }
}
