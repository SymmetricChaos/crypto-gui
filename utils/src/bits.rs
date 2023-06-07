use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign};

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

    pub fn flipped(&self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Bit::Zero => '0',
            Bit::One => '1',
        }
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

impl From<Bit> for u8 {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl TryFrom<u8> for Bit {
    type Error = IntToBitError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::Zero),
            1 => Ok(Bit::One),
            _ => Err(IntToBitError),
        }
    }
}

impl From<Bit> for usize {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl TryFrom<usize> for Bit {
    type Error = IntToBitError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::Zero),
            1 => Ok(Bit::One),
            _ => Err(IntToBitError),
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
