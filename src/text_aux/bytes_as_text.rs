use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteRep {
    Binary,
    Octal,
    Decimal,
    HexLower,
    HexUpper,
}

impl ByteRep {
    pub fn radix(&self) -> u32 {
        match self {
            ByteRep::Binary => 2,
            ByteRep::Octal => 8,
            ByteRep::Decimal => 10,
            ByteRep::HexLower => 16,
            ByteRep::HexUpper => 16,
        }
    }
}

pub fn u8_to_string(n: u8, rep: ByteRep) -> String {
    match rep {
        ByteRep::Binary => format!("{n:08b}"),
        ByteRep::Octal => format!("{n:03o}"),
        ByteRep::Decimal => format!("{n}"),
        ByteRep::HexLower => format!("{n:02x}"),
        ByteRep::HexUpper => format!("{n:02X}"),
    }
}

pub fn u16_to_string(n: u16, rep: ByteRep) -> String {
    match rep {
        ByteRep::Binary => format!("{n:016b}"),
        ByteRep::Octal => format!("{n:06o}"),
        ByteRep::Decimal => format!("{n}"),
        ByteRep::HexLower => format!("{n:04x}"),
        ByteRep::HexUpper => format!("{n:04X}"),
    }
}

pub fn u32_to_string(n: u32, rep: ByteRep) -> String {
    match rep {
        ByteRep::Binary => format!("{n:032b}"),
        ByteRep::Octal => format!("{n:011o}"),
        ByteRep::Decimal => format!("{n}"),
        ByteRep::HexLower => format!("{n:08x}"),
        ByteRep::HexUpper => format!("{n:08X}"),
    }
}

pub fn num_to_string_width<N>(n: &N, rep: ByteRep, width: usize) -> String
where
    N: Display + Binary + Octal + LowerHex + UpperHex,
{
    match rep {
        ByteRep::Binary => format!("{n:0width$b}"),
        ByteRep::Octal => format!("{n:0width$o}"),
        ByteRep::Decimal => format!("{n:0width$}"),
        ByteRep::HexLower => format!("{n:0width$x}"),
        ByteRep::HexUpper => format!("{n:0width$X}"),
    }
}

pub fn u32_from_string(s: &str, rep: ByteRep) -> Result<u32, std::num::ParseIntError> {
    match rep {
        ByteRep::Binary => u32::from_str_radix(s, 2),
        ByteRep::Octal => u32::from_str_radix(s, 8),
        ByteRep::Decimal => u32::from_str_radix(s, 10),
        ByteRep::HexLower => u32::from_str_radix(s, 16),
        ByteRep::HexUpper => u32::from_str_radix(s, 16),
    }
}

#[test]
fn byte_to_string_tests() {
    for rep in [
        ByteRep::Binary,
        ByteRep::Octal,
        ByteRep::Decimal,
        ByteRep::HexLower,
    ] {
        for n in [0, 1, 37, 38, 100, 101, 127, 128, 254, 255] {
            println!("{}", num_to_string_width(&n, rep, 10))
        }
    }
}
