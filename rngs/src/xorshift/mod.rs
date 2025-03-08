use strum::EnumIter;

pub mod xoroshiro;
pub mod xorshift;
pub mod xoshiro128;
pub mod xoshiro256;
pub mod xoshiro512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, strum::Display)]
pub enum Scrambler {
    Plus,
    PlusPlus,
    StarStar,
}
