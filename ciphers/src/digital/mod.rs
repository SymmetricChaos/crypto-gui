pub mod aes;
pub mod chacha;
pub mod des;
pub mod elgamal;
pub mod rc4;
pub mod rc5;
pub mod rsa;
pub mod salsa20;
pub mod tea;
pub mod xtea;

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherMode {
    ECB,
    CTR,
}
