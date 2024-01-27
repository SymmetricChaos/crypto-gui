pub mod aes;
pub mod des;
pub mod rc4;
pub mod rc5;
pub use rc4::Rc4;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteFormat {
    Hex,
    Utf8,
}
