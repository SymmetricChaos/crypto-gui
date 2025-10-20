use num::integer::mod_floor;
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One, Zero};
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// FiniteInt uses an i32 internally so N should not be more than 46340 to avoid issues with multiplication
/// If N is not prime, division will fail for some inputs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FiniteInt<const N: i32>(i32);

impl<const N: i32> FiniteInt<N> {
    /// Create a new FiniteInt by reducing the input to ensure it is valid
    pub fn new(n: i32) -> Self {
        Self(mod_floor(n, N))
    }

    /// Create a new FiniteInt without checking the input
    pub fn new_raw(n: i32) -> Self {
        Self(n)
    }

    /// The multiplicative inverse if it exists
    pub fn recip(&self) -> Option<Self> {
        let egcd = self.0.extended_gcd(&N);
        if !egcd.gcd.is_one() {
            None
        } else {
            Some(Self::new(egcd.x))
        }
    }
}

impl<const N: i32> Zero for FiniteInt<N> {
    fn zero() -> Self {
        FiniteInt(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const N: i32> One for FiniteInt<N> {
    fn one() -> Self {
        FiniteInt(1)
    }

    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

impl<const N: i32> Display for FiniteInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const N: i32> Debug for FiniteInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.0, N)
    }
}

impl<const N: i32> Add for FiniteInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % N)
    }
}

impl<const N: i32> AddAssign for FiniteInt<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self((self.0 + rhs.0) % N)
    }
}

impl<const N: i32> CheckedAdd for FiniteInt<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_add(v.0)? % N))
    }
}

impl<const N: i32> Sub for FiniteInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: i32> SubAssign for FiniteInt<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + -rhs;
    }
}

impl<const N: i32> CheckedSub for FiniteInt<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Self((self.0 + N).checked_sub(v.0)? % N))
    }
}

impl<const N: i32> Mul for FiniteInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % N)
    }
}

impl<const N: i32> MulAssign for FiniteInt<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self((self.0 * rhs.0) % N)
    }
}

impl<const N: i32> CheckedMul for FiniteInt<N> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_mul(v.0)? % N))
    }
}

impl<const N: i32> Div for FiniteInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip().unwrap()
    }
}

impl<const N: i32> DivAssign for FiniteInt<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs.recip().unwrap()
    }
}

impl<const N: i32> CheckedDiv for FiniteInt<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.checked_mul(&v.recip()?)
    }
}

impl<const N: i32> Neg for FiniteInt<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.0)
    }
}

#[cfg(test)]
mod math_tests {

    use super::*;

    #[test]
    fn mul() {
        type FI26 = FiniteInt<26>;
        let a = FI26::new(5);
        let b = FI26::new(7);
        println!("{} * {} = {}", a, b, a * b);
        let a = FI26::new(5);
        let b = FI26::new(21);
        println!("{} * {} = {}", a, b, a * b);
    }

    #[test]
    fn add() {
        let a = FiniteInt::<26>(5);
        let b = FiniteInt::<26>(7);
        println!("{} + {} = {}", a, b, a + b);
        let a = FiniteInt::<26>(20);
        let b = FiniteInt::<26>(10);
        println!("{} + {} = {}", a, b, a + b)
    }

    #[test]
    fn sub() {
        let a = FiniteInt::<26>(5);
        let b = FiniteInt::<26>(7);
        println!("{} - {} = {}", a, b, a - b)
    }

    #[test]
    fn div() {
        let a = FiniteInt::<26>(5);
        let b = FiniteInt::<26>(7);
        println!("{} / {} = {}", a, b, a / b)
    }

    #[test]
    fn recip() {
        let a = FiniteInt::<26>(5);
        println!("1 / {} = {}", a, a.recip().unwrap())
    }

    #[test]
    fn neg() {
        let a = FiniteInt::<26>(11);
        println!("{} + {} = {}", a, -a, a + -a)
    }
}
