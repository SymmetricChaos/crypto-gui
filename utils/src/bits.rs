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

pub fn byte_to_bits_ltr(byte: u8) -> [Bit; 8] {
    let mut out = [Bit::Zero; 8];
    for idx in 0..8_u8 {
        if (1 << idx) & byte != 0 {
            out[(7 - idx) as usize] = Bit::One
        }
    }
    out
}

pub fn byte_to_bits_rtl(byte: u8) -> [Bit; 8] {
    let mut out = [Bit::Zero; 8];
    for idx in 0..8_u8 {
        if (1 << idx) & byte != 0 {
            out[idx as usize] = Bit::One
        }
    }
    out
}

// Take bytes in sequence and read their bits from left to right
pub fn bit_vec_from_bytes_ltr(bytes: &[u8]) -> Vec<Bit> {
    bytes
        .iter()
        .map(|c| byte_to_bits_ltr(*c))
        .flatten()
        .collect()
}

// Take bytes in sequence and read their bits from right to left
pub fn bit_vec_from_bytes_rtl(bytes: &[u8]) -> Vec<Bit> {
    bytes
        .iter()
        .map(|c| byte_to_bits_rtl(*c))
        .flatten()
        .collect()
}

/// Panics if bits.len() > 32
pub fn bits_to_u32_rtl(bits: &[Bit]) -> u32 {
    let mut it = bits.iter().rev();
    let mut out = *it.next().unwrap() as u32;
    let mut p = 1;
    for b in it {
        p *= 2;
        if b.is_one() {
            out += p;
        }
    }
    out
}

/// Panics if bits.len() > 32
pub fn bits_to_u32_ltr(bits: &[Bit]) -> u32 {
    let mut out = bits[0] as u32;
    let mut p = 1;
    for b in bits.iter().skip(1) {
        p *= 2;
        if b.is_one() {
            out += p;
        }
    }
    out
}

macro_rules! num_to_bit_vec {
    ($name: ident, $type: ty) => {
        /// Convert an integer to a vector of bits with LSB at index 0 and high null bits ignored
        /// example: u8_to_bit_vec(0x2f) == vec![1,1,1,1,0,1]
        /// If the integer == 0 the vector is empty
        pub fn $name(num: $type) -> Vec<Bit> {
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

macro_rules! num_to_bits {
    ($name: ident, $type: ty, $width: literal) => {
        /// Convert an integer to an array of bits of equal width with the MSB at index 0
        pub fn $name(num: $type) -> [Bit; $width] {
            let mut bits = [Bit::Zero; $width];
            for i in 0..$width {
                let shifted_num = num >> i;
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
    };
}

num_to_bits!(u8_to_bits, u8, 8);
num_to_bits!(u16_to_bits, u16, 16);
num_to_bits!(u32_to_bits, u32, 32);
num_to_bits!(u64_to_bits, u64, 64);

num_to_bit_vec!(u8_to_bit_vec, u8);
num_to_bit_vec!(u16_to_bit_vec, u16);
num_to_bit_vec!(u32_to_bit_vec, u32);
num_to_bit_vec!(u64_to_bit_vec, u64);

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
    fn bits_to_int() {
        assert_eq!(5, bits_to_u32_rtl(&to_bit_array([0, 0, 1, 0, 1]).unwrap()));
        assert_eq!(20, bits_to_u32_ltr(&to_bit_array([0, 0, 1, 0, 1]).unwrap()));
    }

    #[test]
    fn byte_to_bits() {
        let byte = 0b11000001_u8;
        println!("{:?}", byte_to_bits_ltr(byte));
        println!("{:?}", byte_to_bits_rtl(byte));
    }
}
