use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumRep {
    Binary,
    Octal,
    Decimal,
    HexLower,
    HexUpper,
}

impl NumRep {
    pub fn radix(&self) -> u32 {
        match self {
            NumRep::Binary => 2,
            NumRep::Octal => 8,
            NumRep::Decimal => 10,
            NumRep::HexLower => 16,
            NumRep::HexUpper => 16,
        }
    }
}

pub fn u8_to_string(n: u8, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:08b}"),
        NumRep::Octal => format!("{n:03o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:02x}"),
        NumRep::HexUpper => format!("{n:02X}"),
    }
}

pub fn u16_to_string(n: u16, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:016b}"),
        NumRep::Octal => format!("{n:06o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:04x}"),
        NumRep::HexUpper => format!("{n:04X}"),
    }
}

pub fn u32_to_string(n: u32, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:032b}"),
        NumRep::Octal => format!("{n:011o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:08x}"),
        NumRep::HexUpper => format!("{n:08X}"),
    }
}

pub fn num_to_string_width<N>(n: &N, rep: NumRep, width: usize) -> String
where
    N: Display + Binary + Octal + LowerHex + UpperHex,
{
    match rep {
        NumRep::Binary => format!("{n:0width$b}"),
        NumRep::Octal => format!("{n:0width$o}"),
        NumRep::Decimal => format!("{n:0width$}"),
        NumRep::HexLower => format!("{n:0width$x}"),
        NumRep::HexUpper => format!("{n:0width$X}"),
    }
}

pub fn u32_from_string(s: &str, rep: NumRep) -> Result<u32, std::num::ParseIntError> {
    match rep {
        NumRep::Binary => u32::from_str_radix(s, 2),
        NumRep::Octal => u32::from_str_radix(s, 8),
        NumRep::Decimal => u32::from_str_radix(s, 10),
        NumRep::HexLower => u32::from_str_radix(s, 16),
        NumRep::HexUpper => u32::from_str_radix(s, 16),
    }
}

#[test]
fn byte_to_string_tests() {
    for rep in [
        NumRep::Binary,
        NumRep::Octal,
        NumRep::Decimal,
        NumRep::HexLower,
    ] {
        for n in [0, 1, 37, 38, 100, 101, 127, 128, 254, 255] {
            println!("{}", num_to_string_width(&n, rep, 10))
        }
    }
}
