pub mod caesar;
pub use caesar::Caesar;
pub mod affine;
pub use affine::Affine;

pub mod cipher_trait;
pub use cipher_trait::{LATIN,Cipher};