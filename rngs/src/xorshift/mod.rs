use strum::EnumIter;

pub mod xoroshiro128;
pub mod xorshift128;
pub mod xorshift32;
pub mod xorshift64;
pub mod xorshift_transitions;
pub mod xorwow;
pub mod xoshiro128;
pub mod xoshiro256;
pub mod xoshiro512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, strum::Display)]
pub enum XoshiroScrambler {
    Plus,
    PlusPlus,
    StarStar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, strum::Display)]
pub enum XorshiftScrambler {
    None,
    Plus,
    Star,
}
