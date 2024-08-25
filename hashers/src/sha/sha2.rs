use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

use super::sha256::Sha2_224;
use super::sha256::Sha2_256;
use super::sha512::{Sha2_384, Sha2_512, Sha2_512_224, Sha2_512_256};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sha2Variant {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
}

#[derive(Debug, Clone)]
pub struct Sha2 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub variant: Sha2Variant,
}

impl Default for Sha2 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Sha2Variant::Sha256,
        }
    }
}

impl Sha2 {
    pub fn variant(mut self, variant: Sha2Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for Sha2 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            Sha2Variant::Sha224 => Sha2_224::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            Sha2Variant::Sha256 => Sha2_256::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            Sha2Variant::Sha384 => Sha2_384::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            Sha2Variant::Sha512 => Sha2_512::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            Sha2Variant::Sha512_224 => Sha2_512_224::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
            Sha2Variant::Sha512_256 => Sha2_512_256::default()
                .input(self.input_format)
                .output(self.output_format)
                .hash(bytes),
        }
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    Sha2::default().variant(Sha2Variant::Sha256), test256, "",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    Sha2::default().variant(Sha2Variant::Sha224), test224, "",
    "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f";
    Sha2::default().variant(Sha2Variant::Sha512), test512, "",
    "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    Sha2::default().variant(Sha2Variant::Sha384), test384, "",
    "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b";
    Sha2::default().variant(Sha2Variant::Sha512_224), test512_224, "",
    "6ed0dd02806fa89e25de060c19d3ac86cabb87d6a0ddd05c333b84f4";
    Sha2::default().variant(Sha2Variant::Sha512_256), test512_256, "",
    "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a";
);
