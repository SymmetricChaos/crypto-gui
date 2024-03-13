use crate::bits::{bit_vec_from_bytes, bits_from_str, Bit, CharToBitError, IntToBitError};
use itertools::Itertools;
use num::{One, Zero};
use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

// Polynomial of GF(2) with coefficients in ascending order so that the coefficient at index n is paired with the indeterminate with power n
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

    pub fn decrease_degree(&mut self, n: usize) {
        for _ in 0..n {
            self.coef.remove(0);
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
        let bits = bits_from_str(s.as_ref())?;
        Ok(BitPolynomial::from(bits.collect_vec()))
    }

    pub fn from_bytes(bytes: &[u8]) -> BitPolynomial {
        BitPolynomial::from(bit_vec_from_bytes(bytes))
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
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        // Dividend is now the remainder
        (quotient, dividend)
    }

    pub fn bitwise_xor(&self, rhs: &BitPolynomial) -> BitPolynomial {
        let n = self.len().max(rhs.len());
        let mut vec = vec![Bit::Zero; n];
        for i in 0..n {
            vec[i] = self.get_irref(i) + rhs.get_irref(i)
        }
        BitPolynomial::from(vec)
    }

    pub fn bitwise_and(&self, rhs: &BitPolynomial) -> BitPolynomial {
        let n = self.len().max(rhs.len());
        let mut vec = vec![Bit::Zero; n];
        for i in 0..n {
            vec[i] = self.get_irref(i) & rhs.get_irref(i)
        }
        BitPolynomial::from(vec)
    }

    pub fn bitwise_or(&self, rhs: &BitPolynomial) -> BitPolynomial {
        let n = self.len().max(rhs.len());
        let mut vec = vec![Bit::Zero; n];
        for i in 0..n {
            vec[i] = self.get_irref(i) | rhs.get_irref(i)
        }
        BitPolynomial::from(vec)
    }

    pub fn bitwise_nor(&self, rhs: &BitPolynomial) -> BitPolynomial {
        let n = self.len().max(rhs.len());
        let mut vec = vec![Bit::Zero; n];
        for i in 0..n {
            vec[i] = -(self.get_irref(i) | rhs.get_irref(i))
        }
        BitPolynomial::from(vec)
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

impl From<&[Bit]> for BitPolynomial {
    fn from(value: &[Bit]) -> Self {
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

impl Sub for BitPolynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.len().max(rhs.len());
        let mut coef = Vec::with_capacity(len);
        for idx in 0..len {
            let sum = self.get_irref(idx) + rhs.get_irref(idx);
            coef.push(sum);
        }
        BitPolynomial::from(coef)
    }
}

impl Sub<&BitPolynomial> for BitPolynomial {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let len = self.len().max(rhs.len());
        let mut coef = Vec::with_capacity(len);
        for idx in 0..len {
            let sum = self.get_irref(idx) + rhs.get_irref(idx);
            coef.push(sum);
        }
        BitPolynomial::from(coef)
    }
}

impl SubAssign for BitPolynomial {
    fn sub_assign(&mut self, rhs: Self) {
        while self.len() < rhs.len() {
            self.coef.push(Bit::Zero)
        }
        for (idx, rhs_coef) in rhs.coef.iter().cloned().enumerate() {
            self.coef[idx] += rhs_coef;
        }
        self.trim()
    }
}

impl SubAssign<&BitPolynomial> for BitPolynomial {
    fn sub_assign(&mut self, rhs: &Self) {
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

impl Div for BitPolynomial {
    type Output = BitPolynomial;

    fn div(self, rhs: Self) -> Self::Output {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            return BitPolynomial::zero();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        quotient
    }
}

impl Div<&BitPolynomial> for BitPolynomial {
    type Output = BitPolynomial;

    fn div(self, rhs: &Self) -> Self::Output {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            return BitPolynomial::zero();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        quotient
    }
}

impl DivAssign for BitPolynomial {
    fn div_assign(&mut self, rhs: Self) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            *self = BitPolynomial::zero()
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        *self = quotient
    }
}

impl DivAssign<&BitPolynomial> for BitPolynomial {
    fn div_assign(&mut self, rhs: &Self) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            *self = BitPolynomial::zero()
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        *self = quotient
    }
}

impl Rem for BitPolynomial {
    type Output = BitPolynomial;

    fn rem(self, rhs: Self) -> Self::Output {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            return BitPolynomial::zero();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        dividend
    }
}

impl Rem<&BitPolynomial> for BitPolynomial {
    type Output = BitPolynomial;

    fn rem(self, rhs: &Self) -> Self::Output {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() || self.degree() < rhs.degree() {
            return BitPolynomial::zero();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        dividend
    }
}

impl RemAssign for BitPolynomial {
    fn rem_assign(&mut self, rhs: Self) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() {
            *self = BitPolynomial::zero();
        }

        if self.degree() < rhs.degree() {
            return ();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        *self = dividend
    }
}

impl RemAssign<&BitPolynomial> for BitPolynomial {
    fn rem_assign(&mut self, rhs: &Self) {
        // Handle special cases
        if rhs.is_zero() {
            panic!("division by zero")
        }

        if self.is_zero() {
            *self = BitPolynomial::zero();
        }

        if self.degree() < rhs.degree() {
            return ();
        }

        // General case
        let mut dividend = self.clone();
        let mut quotient = BitPolynomial::zero();
        while !dividend.is_zero() && dividend.degree() >= rhs.degree() {
            let mut intermediate = rhs.clone();
            let alignment = dividend.degree() - intermediate.degree();
            intermediate.increase_degree(alignment);
            let mut bit = BitPolynomial::one();
            bit.increase_degree(alignment);

            quotient += bit;
            dividend += intermediate;
        }

        *self = dividend
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
    fn example_division_for_crc() {
        let a = BitPolynomial::from_str("00000110111001011").unwrap();
        let b = BitPolynomial::from_str("1101").unwrap();
        assert_eq!(
            "x^16 + x^15 + x^13 + x^10 + x^9 + x^8 + x^6 + x^5\nx^3 + x + 1",
            format!("{}\n{}", a.polynomial_string(), b.polynomial_string())
        );
        let (q, r) = a.div_rem(&b);
        assert_eq!("00111110001111 001", format!("{} {}", q, r));
        assert_eq!(q * b + r, a);
    }
}
