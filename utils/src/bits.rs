use lazy_static::lazy_static;
use num::{One, Zero};
use regex::Regex;
use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul,
        MulAssign, Not,
    },
};

lazy_static! {
    pub static ref IS_BITS: Regex = Regex::new(r"^[01\s]*$").unwrap();
}

pub fn bits_from_bitstring(text: &str) -> Option<impl Iterator<Item = Bit> + '_> {
    if !IS_BITS.is_match(text) {
        None
    } else {
        Some(
            text.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| Bit::try_from(c).unwrap()),
        )
    }
}

pub fn bits_to_int_little_endian(bits: &[Bit]) -> u32 {
    let mut out = 0;
    let mut p = 1;
    for b in bits.iter().rev() {
        if b.is_one() {
            out += p;
        }
        p *= 2
    }
    out
}

pub fn bits_to_int_big_endian(bits: &[Bit]) -> u32 {
    let mut out = 0;
    let mut p = 1;
    for b in bits.iter() {
        if b.is_one() {
            out += p;
        }
        p *= 2
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    // Invert the value of the Bit.
    pub fn flip(&mut self) {
        match self {
            Bit::Zero => *self = Bit::One,
            Bit::One => *self = Bit::Zero,
        }
    }

    // Return the inverse of the Bit.
    pub const fn flipped(&self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

///////////////////////////////////
// Fundamental Arithmetic Traits //
///////////////////////////////////
impl Zero for Bit {
    fn zero() -> Self {
        Self::Zero
    }

    fn is_zero(&self) -> bool {
        match self {
            Bit::Zero => true,
            Bit::One => false,
        }
    }
}

impl One for Bit {
    fn one() -> Self {
        Self::One
    }
}

impl Add for Bit {
    type Output = Self;
    fn add(self, rhs: Bit) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::Zero,
        }
    }
}

impl Add<&Bit> for Bit {
    type Output = Self;
    fn add(self, rhs: &Bit) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::Zero,
        }
    }
}

impl AddAssign for Bit {
    fn add_assign(&mut self, rhs: Bit) {
        *self = *self + rhs;
    }
}

impl Mul for Bit {
    type Output = Self;

    fn mul(self, rhs: Bit) -> Self::Output {
        if self == Bit::Zero || rhs == Bit::Zero {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl Mul<&Bit> for Bit {
    type Output = Self;

    fn mul(self, rhs: &Bit) -> Self::Output {
        if self == Bit::Zero || *rhs == Bit::Zero {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl MulAssign for Bit {
    fn mul_assign(&mut self, rhs: Bit) {
        *self = *self * rhs;
    }
}

////////////////////////////////
// Fundamental Logical Traits //
////////////////////////////////
impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            Bit::Zero => self,
            Bit::One => rhs,
        }
    }
}

impl BitAnd<&Bit> for Bit {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        match self {
            Bit::Zero => self,
            Bit::One => *rhs,
        }
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        match self {
            Bit::Zero => *self = *self,
            Bit::One => *self = rhs,
        }
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Bit::Zero => rhs,
            Bit::One => self,
        }
    }
}

impl BitOr<&Bit> for Bit {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        match self {
            Bit::Zero => *rhs,
            Bit::One => self,
        }
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        match self {
            Bit::Zero => *self = rhs,
            Bit::One => *self = *self,
        }
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::Zero,
        }
    }
}

impl BitXor<&Bit> for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::Zero,
        }
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        match (&self, rhs) {
            (Bit::Zero, Bit::Zero) => *self = Bit::Zero,
            (Bit::Zero, Bit::One) => *self = Bit::One,
            (Bit::One, Bit::Zero) => *self = Bit::One,
            (Bit::One, Bit::One) => *self = Bit::Zero,
        };
    }
}

//////////////////////////////////
// Allow Adding Bit to Integers //
//////////////////////////////////
macro_rules! add_to_int {
    ($t:ty) => {
        impl Add<Bit> for $t {
            type Output = $t;

            fn add(self, rhs: Bit) -> Self::Output {
                match rhs {
                    Bit::Zero => self,
                    Bit::One => self + 1,
                }
            }
        }

        impl Add<&Bit> for $t {
            type Output = $t;

            fn add(self, rhs: &Bit) -> Self::Output {
                match rhs {
                    Bit::Zero => self,
                    Bit::One => self + 1,
                }
            }
        }

        impl AddAssign<Bit> for $t {
            fn add_assign(&mut self, rhs: Bit) {
                *self = *self + rhs;
            }
        }
    };
}

add_to_int!(usize);
add_to_int!(u8);
add_to_int!(u16);
add_to_int!(u32);
add_to_int!(u64);
add_to_int!(u128);

add_to_int!(isize);
add_to_int!(i8);
add_to_int!(i16);
add_to_int!(i32);
add_to_int!(i64);
add_to_int!(i128);

////////////////////////
// Conversion Methods //
////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntToBitError;

macro_rules! from_into_int {
    ($t:ty) => {
        impl From<Bit> for $t {
            fn from(value: Bit) -> Self {
                match value {
                    Bit::Zero => 0,
                    Bit::One => 1,
                }
            }
        }

        impl From<&Bit> for $t {
            fn from(value: &Bit) -> Self {
                match value {
                    &Bit::Zero => 0,
                    &Bit::One => 1,
                }
            }
        }

        impl TryFrom<$t> for Bit {
            type Error = IntToBitError;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Bit::Zero),
                    1 => Ok(Bit::One),
                    _ => Err(IntToBitError),
                }
            }
        }
    };
}

from_into_int!(usize);
from_into_int!(u8);
from_into_int!(u16);
from_into_int!(u32);
from_into_int!(u64);
from_into_int!(u128);

from_into_int!(isize);
from_into_int!(i8);
from_into_int!(i16);
from_into_int!(i32);
from_into_int!(i64);
from_into_int!(i128);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CharToBitError;

impl From<Bit> for char {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => '0',
            Bit::One => '1',
        }
    }
}

impl From<&Bit> for char {
    fn from(value: &Bit) -> Self {
        match value {
            Bit::Zero => '0',
            Bit::One => '1',
        }
    }
}

impl TryFrom<char> for Bit {
    type Error = CharToBitError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Bit::Zero),
            '1' => Ok(Bit::One),
            _ => Err(CharToBitError),
        }
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        match value {
            false => Bit::Zero,
            true => Bit::One,
        }
    }
}

#[cfg(test)]
mod text_function_tests {

    use super::*;

    #[test]
    fn bits_to_int() {
        assert_eq!(
            5,
            bits_to_int_little_endian(&[0, 0, 1, 0, 1].map(|n| Bit::try_from(n).unwrap()))
        );
        assert_eq!(
            20,
            bits_to_int_big_endian(&[0, 0, 1, 0, 1].map(|n| Bit::try_from(n).unwrap()))
        );
    }
}
