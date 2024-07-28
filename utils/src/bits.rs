use lazy_static::lazy_static;
use num::{Integer, One, Zero};
use regex::Regex;
use std::{
    fmt::Display,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul,
        MulAssign, Neg, Not,
    },
};

lazy_static! {
    pub static ref IS_BITS: Regex = Regex::new(r"^[01\s]*$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CharToBitError;

impl Display for CharToBitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "character could not be converted to a bit")
    }
}

pub fn bits_from_str(text: &str) -> Result<impl Iterator<Item = Bit> + '_, CharToBitError> {
    if !IS_BITS.is_match(text) {
        Err(CharToBitError)
    } else {
        Ok(text
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| Bit::try_from(c).unwrap()))
    }
}

pub fn bools_from_str(text: &str) -> Result<impl Iterator<Item = bool> + '_, CharToBitError> {
    if !IS_BITS.is_match(text) {
        Err(CharToBitError)
    } else {
        Ok(text
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| bool::from(Bit::try_from(c).unwrap())))
    }
}

// Take bytes in sequence, each is turned into an array of bits starting with the MSB and then flattened into a vector
pub fn bit_vec_from_bytes(bytes: &[u8]) -> Vec<Bit> {
    bytes.iter().map(|c| u8_to_bits(*c)).flatten().collect()
}

macro_rules! bit_conversions {
    ($name1: ident, $name2: ident, $name3: ident,$type: ty, $width: literal) => {
        /// Convert bits in to integer type such that the bit at index 0 is the MSB
        /// Panics if the bits argument is too long to fit the type
        /// Inverse of the <int>_to_bits() functions.
        /// bits_to_u8(\[Zero, One, One, Zero\]) == 0b01100000
        pub fn $name1<T: AsRef<[Bit]>>(bits: T) -> $type {
            assert!(bits.as_ref().len() <= $width);
            let mut out = 0 as $type;
            for (i, b) in bits.as_ref().into_iter().enumerate() {
                out ^= (*b as $type) << ($width - i - 1);
            }
            out
        }

        /// Convert an integer to an array of bits of equal width with the MSB at index 0
        /// Inverse of bits_to_<int>() functions.
        /// u8_to_bits(0b0110) == \[Zero, Zero, Zero, Zero, Zero, One, One, Zero\])
        pub fn $name2(n: $type) -> [Bit; $width] {
            let mut bits = [Bit::Zero; $width];
            for i in 0..$width {
                let shifted_num = n >> i;
                // Get the rightmost bit by masking
                let cur_bit = shifted_num & 1;
                if cur_bit == 1 {
                    bits[($width - 1) - i] = Bit::One;
                } else {
                    bits[($width - 1) - i] = Bit::Zero;
                }
            }
            bits
        }

        /// Convert an integer to a vector of bits with LSB at index 0 and high null bits ignored
        /// example: u8_to_bit_vec(0b01101) == vec!\[One, Zero, One, One\]
        pub fn $name3(num: $type) -> Vec<Bit> {
            let mut bits = Vec::new();
            let mut n = num;
            while !n.is_zero() {
                let (q, r) = n.div_rem(&2);
                if r.is_zero() {
                    bits.push(Bit::Zero)
                } else {
                    bits.push(Bit::One)
                }
                n = q;
            }
            bits
        }
    };
}

bit_conversions!(bits_to_u8, u8_to_bits, u8_to_bit_vec, u8, 8);
bit_conversions!(bits_to_u16, u16_to_bits, u16_to_bit_vec, u16, 16);
bit_conversions!(bits_to_u32, u32_to_bits, u32_to_bit_vec, u32, 32);
bit_conversions!(bits_to_u64, u64_to_bits, u64_to_bit_vec, u64, 64);

pub fn to_bit_array<T: Copy, const N: usize>(arr: [T; N]) -> Result<[Bit; N], IntToBitError>
where
    Bit: TryFrom<T>,
    IntToBitError: From<<Bit as TryFrom<T>>::Error>,
{
    let mut v = [Bit::Zero; N];
    for (n, i) in arr.iter().enumerate() {
        v[n] = Bit::try_from(*i)?;
    }
    Ok(v)
}

pub fn to_bit_vec<T: Copy>(arr: Vec<T>) -> Result<Vec<Bit>, IntToBitError>
where
    Bit: TryFrom<T>,
    IntToBitError: From<<Bit as TryFrom<T>>::Error>,
{
    let mut v = Vec::with_capacity(arr.len());
    for (n, i) in arr.iter().enumerate() {
        v[n] = Bit::try_from(*i)?;
    }
    Ok(v)
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

    pub const fn to_char(&self) -> char {
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

pub fn bit_string(bits: &[Bit]) -> String {
    let mut s = String::with_capacity(bits.len());
    for b in bits {
        s.push(b.to_char())
    }
    s
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

impl Add<&Bit> for &Bit {
    type Output = Self;
    fn add(self, rhs: &Bit) -> Self::Output {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => &Bit::Zero,
            (Bit::Zero, Bit::One) => &Bit::One,
            (Bit::One, Bit::Zero) => &Bit::One,
            (Bit::One, Bit::One) => &Bit::Zero,
        }
    }
}

impl AddAssign for Bit {
    fn add_assign(&mut self, rhs: Bit) {
        *self = *self + rhs;
    }
}

impl AddAssign for &Bit {
    fn add_assign(&mut self, rhs: &Bit) {
        *self = *self + rhs;
    }
}

impl Sum for Bit {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Bit::Zero, |acc, x| acc + x)
    }
}

impl Sum for &Bit {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(&Bit::Zero, |acc, x| acc + x)
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
        if self.is_zero() || rhs.is_zero() {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl Mul<&Bit> for &Bit {
    type Output = Self;

    fn mul(self, rhs: &Bit) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            &Bit::Zero
        } else {
            &Bit::One
        }
    }
}

impl MulAssign for Bit {
    fn mul_assign(&mut self, rhs: Bit) {
        *self = *self * rhs;
    }
}

impl MulAssign for &Bit {
    fn mul_assign(&mut self, rhs: &Bit) {
        *self = *self * rhs;
    }
}

impl Product for Bit {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Bit::One, |acc, x| acc * x)
    }
}

impl Product for &Bit {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(&Bit::One, |acc, x| acc * x)
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

impl Neg for Bit {
    type Output = Bit;

    fn neg(self) -> Self::Output {
        self
    }
}

//////////////////////////////////////////////////////
// Addition and Conversion with Primitive Int Types //
//////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntToBitError;

impl Display for IntToBitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "integer could not be converted to a bit")
    }
}

macro_rules! int_methods {
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

        impl TryFrom<&$t> for Bit {
            type Error = IntToBitError;

            fn try_from(value: &$t) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Bit::Zero),
                    1 => Ok(Bit::One),
                    _ => Err(IntToBitError),
                }
            }
        }
    };
}

int_methods!(usize);
int_methods!(u8);
int_methods!(u16);
int_methods!(u32);
int_methods!(u64);
int_methods!(u128);

int_methods!(isize);
int_methods!(i8);
int_methods!(i16);
int_methods!(i32);
int_methods!(i64);
int_methods!(i128);

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
mod bit_function_tests {

    use super::*;

    #[test]
    fn bits_to_u8_test() {
        assert_eq!(
            bits_to_u8(&[Bit::One, Bit::Zero, Bit::One, Bit::One]),
            0b10110000
        );
    }

    #[test]
    fn u8_to_bits_test() {
        assert_eq!(
            u8_to_bits(0b00001011),
            [
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::One,
            ]
        )
    }

    #[test]
    fn bits_to_u8_and_back_test() {
        for n in 0..=255 {
            let bits = u8_to_bits(n);
            assert_eq!(n, bits_to_u8(&bits));
        }
    }

    #[test]
    fn u8_to_bit_vec_test() {
        assert_eq!(
            u8_to_bit_vec(0b01101),
            vec![Bit::One, Bit::Zero, Bit::One, Bit::One]
        )
    }
}
