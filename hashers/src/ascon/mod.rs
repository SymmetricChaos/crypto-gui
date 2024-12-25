use strum::EnumIter;

pub mod hash;
pub mod state;
pub mod tests;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    Hash,
    Hasha,
    Xof,
    Xofa,
    Mac,
    Maca,
    Prf,
    Prfa,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hash => write!(f, "Ascon-Hash"),
            Self::Hasha => write!(f, "Ascon-Hasha"),
            Self::Xof => write!(f, "Ascon-XOF"),
            Self::Xofa => write!(f, "Ascon-XOFa"),
            Self::Mac => write!(f, "Ascon-MAC"),
            Self::Maca => write!(f, "Ascon-MACa"),
            Self::Prf => write!(f, "Ascon-XOF"),
            Self::Prfa => write!(f, "Ascon-XOFa"),
        }
    }
}

impl Variant {
    pub fn a(&self) -> usize {
        match self {
            Variant::Hash => 12,
            Variant::Hasha => 8,
            Variant::Xof => 12,
            Variant::Xofa => 8,
            Variant::Mac => 12,
            Variant::Maca => 8,
            Variant::Prf => 12,
            Variant::Prfa => 8,
        }
    }

    pub fn rate(&self) -> usize {
        match self {
            Variant::Hash => 8,
            Variant::Hasha => 8,
            Variant::Xof => 8,
            Variant::Xofa => 8,
            Variant::Mac => 32,
            Variant::Maca => 40,
            Variant::Prf => 32,
            Variant::Prfa => 40,
        }
    }
}
