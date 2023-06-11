use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign},
};

use num::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn flip(&mut self) {
        match self {
            Bit::Zero => *self = Bit::One,
            Bit::One => *self = Bit::Zero,
        }
    }

    pub const fn flipped(&self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }

    pub const fn as_char(&self) -> char {
        match self {
            Bit::Zero => '0',
            Bit::One => '1',
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

impl Sum for Bit {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Zero, |a, b| a + b)
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

impl MulAssign for Bit {
    fn mul_assign(&mut self, rhs: Bit) {
        *self = *self * rhs;
    }
}

impl Add<Bit> for usize {
    type Output = usize;

    fn add(self, rhs: Bit) -> Self::Output {
        match rhs {
            Bit::Zero => self,
            Bit::One => self + 1,
        }
    }
}

impl AddAssign<Bit> for usize {
    fn add_assign(&mut self, rhs: Bit) {
        *self = *self + rhs;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntToBitError;

macro_rules! from_into {
    ($t:ty) => {
        impl From<Bit> for $t {
            fn from(value: Bit) -> Self {
                match value {
                    Bit::Zero => 0,
                    Bit::One => 1,
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

from_into!(u8);
from_into!(u16);
from_into!(u32);
from_into!(u64);
from_into!(u128);

from_into!(i8);
from_into!(i16);
from_into!(i32);
from_into!(i64);
from_into!(i128);

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
