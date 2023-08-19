use crate::bits::{Bit, IntToBitError};
use num::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitPolynomial {
    pub coef: Vec<Bit>,
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

    pub fn increase_degree(mut self, n: usize) -> BitPolynomial {
        for _ in 0..n {
            self.coef.insert(0, Bit::zero());
        }
        self
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

    pub fn div_rem(&self, rhs: BitPolynomial) -> (BitPolynomial, BitPolynomial) {
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
        let mut remainder = self.clone();

        while !remainder.is_zero() && remainder.degree() >= rhs.degree() {
            let pow = remainder.degree() - rhs.degree();
            let intermediate = BitPolynomial::one().increase_degree(pow);
            quotient += intermediate.clone();
            remainder += intermediate * rhs.clone();
        }

        (quotient, remainder)
    }
}

impl From<Vec<Bit>> for BitPolynomial {
    fn from(value: Vec<Bit>) -> Self {
        Self { coef: value }
    }
}

impl<const N: usize> From<[Bit; N]> for BitPolynomial {
    fn from(value: [Bit; N]) -> Self {
        Self {
            coef: value.to_vec(),
        }
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

// Addition (also Subtraction for PolynomialUV)
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

impl MulAssign for BitPolynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let mut coef = vec![Bit::Zero; self.len() + rhs.len()];
        for (n, lhs_coef) in self.coef.iter().enumerate() {
            for (k, rhs_coef) in rhs.coef.iter().enumerate() {
                coef[n + k] += *lhs_coef * rhs_coef;
            }
        }
        *self = BitPolynomial::from(coef)
    }
}

#[cfg(test)]
mod math_function_tests {

    use super::*;

    #[test]
    fn polynomial_mul() {
        let m = BitPolynomial::from_int_array([1, 0, 1]).unwrap();
        let n = BitPolynomial::from_int_array([1, 1]).unwrap();
        assert_eq!(n.clone() * n, m)
    }

    #[test]
    fn polynomial_add() {
        let m = BitPolynomial::from_int_array([1, 0, 1]).unwrap();
        let n = BitPolynomial::from_int_array([1, 1]).unwrap();
        assert_eq!(m + n, BitPolynomial::from_int_array([0, 1, 1]).unwrap())
    }

    #[test]
    fn polynomial_div() {
        let m = BitPolynomial::from_int_array([1, 0, 1]).unwrap();
        let n = BitPolynomial::from_int_array([1, 1]).unwrap();
        assert_eq!(m.div_rem(n.clone()).0, n)
    }
}
