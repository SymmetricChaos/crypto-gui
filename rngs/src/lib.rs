pub mod lcg;
pub mod lfsr;

pub mod errors;
pub mod halton;
pub mod ids;
pub mod lfg;
pub mod mersenne_twister;
pub mod middle_square;
pub mod pcg;
pub mod rc4;
pub mod traits;
pub mod weyl;
pub mod xorshift;
pub use traits::ClassicRng;
