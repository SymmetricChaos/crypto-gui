use bimap::BiMap;
use itertools::Itertools;
use num::Integer;
use std::hash::Hash;

pub fn bimap_from_iter<I, S, T>(iter: I) -> BiMap<S, T>
where
    I: Iterator<Item = (S, T)>,
    S: Hash + Eq,
    T: Hash + Eq,
{
    let mut map = BiMap::new();
    for (l, r) in iter {
        map.insert(l, r);
    }
    map
}

pub fn chunk_and_join(text: &str, width: usize, sep: char) -> String {
    text.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % width == 0 {
                Some(sep)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

pub fn string_chunks(text: &str, width: usize) -> Vec<String> {
    text.chars()
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect_vec()
}

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

pub fn u8_to_string_with_radix(byte: &u8, radix: u8) -> String {
    if byte == &0 {
        return String::from("0");
    }
    let mut b = *byte;
    let mut s = Vec::new();
    while b != 0 {
        let (q, r) = b.div_rem(&radix);
        if r < 10 {
            s.push(r + 48) // shift to start of ASCII numbers
        } else {
            s.push(r + 55) // shift to start of ASCII uppercase letters
        }
        b = q;
    }
    String::from_utf8(s.into_iter().rev().collect()).unwrap()
}

pub fn u8_to_string_with_radix_and_width(byte: &u8, radix: u8, width: usize) -> String {
    assert!(radix > 1);
    assert!(radix < 37);
    if byte == &0 {
        return "0".repeat(width);
    }
    let mut b = *byte;
    let mut s = Vec::with_capacity(8); // Largest size needed for a valid radix
    while b != 0 {
        let (q, r) = b.div_rem(&radix);
        if r < 10 {
            s.push(r + 48) // shift to start of ASCII numbers
        } else {
            s.push(r + 55) // shift to start of ASCII uppercase letters
        }
        b = q;
    }
    let zeroes = std::iter::repeat('0' as u8).take(width - s.len());
    String::from_utf8(zeroes.chain(s.into_iter().rev()).collect()).unwrap()
}
