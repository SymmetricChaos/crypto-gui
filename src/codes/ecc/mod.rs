pub mod hamming;
pub mod m_of_n;
pub mod parity_check;
pub mod repetition;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::Error;

lazy_static! {
    pub static ref IS_BITS: Regex = Regex::new(r"^[01\s]*$").unwrap();
}

pub fn check_bitstring(text: &str) -> Result<(), Error> {
    if !IS_BITS.is_match(text) {
        return Err(Error::Input(format!(
            "bitstrings can only contain 0, 1, and whitespace",
        )));
    } else {
        Ok(())
    }
}

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

fn char_to_bit(c: char) -> Bit {
    match c {
        '0' => Bit::Zero,
        '1' => Bit::One,
        _ => unreachable!("chars other than 0 and 1 should be filtered out by input"),
    }
}

pub fn bits_from_bitstring(text: &str) -> impl Iterator<Item = Bit> + '_ {
    text.chars()
        .filter(|b| !b.is_whitespace())
        .map(|b| char_to_bit(b))
}
