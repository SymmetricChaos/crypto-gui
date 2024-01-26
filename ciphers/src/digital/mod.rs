pub mod aes;
pub mod des;
pub mod rc4;
pub mod rc5;
pub use rc4::Rc4;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Hex,
    Utf8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputFormat {
    Hex,
    Utf8,
}
