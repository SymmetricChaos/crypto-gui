#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteRep {
    Binary,
    Octal,
    Decimal,
    Hex,
    HexCap,
}

impl ByteRep {
    pub fn radix(&self) -> u32 {
        match self {
            ByteRep::Binary => 2,
            ByteRep::Octal => 8,
            ByteRep::Decimal => 10,
            ByteRep::Hex => 16,
            ByteRep::HexCap => 16,
        }
    }
}

pub fn byte_to_string(n: u8, rep: ByteRep) -> String {
    match rep {
        ByteRep::Binary => format!("{n:08b}"),
        ByteRep::Octal => format!("{n:03o}"),
        ByteRep::Decimal => format!("{n}"),
        ByteRep::Hex => format!("{n:02x}"),
        ByteRep::HexCap => format!("{n:02X}"),
    }
}

pub fn byte_to_string_with_width(n: u8, rep: ByteRep, width: usize) -> String {
    match rep {
        ByteRep::Binary => format!("{n:0width$b}"),
        ByteRep::Octal => format!("{n:0width$o}"),
        ByteRep::Decimal => format!("{n:0width$}"),
        ByteRep::Hex => format!("{n:0width$x}"),
        ByteRep::HexCap => format!("{n:0width$X}"),
    }
}

#[test]
fn byte_to_string_tests() {
    for rep in [
        ByteRep::Binary,
        ByteRep::Octal,
        ByteRep::Decimal,
        ByteRep::Hex,
    ] {
        for n in [0, 1, 37, 38, 100, 101, 127, 128, 254, 255] {
            println!("{}", byte_to_string_with_width(n, rep, 10))
        }
    }
}
