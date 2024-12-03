pub mod speck128;
pub mod speck32;
pub mod speck48;
pub mod speck64;
pub mod speck96;

// These macros make it straightforward to implement speck for the various word sizes
// The name are short and generic so `pub(self) use foo;` is used to make them only accessible in this module
macro_rules! enc {
    ($x:ident, $y:ident, $k:ident, $alpha:literal, $beta:literal) => {
        $x = $x.rotate_right($alpha);
        $x = $x.wrapping_add($y);
        $x ^= $k;
        $y = $y.rotate_left($beta);
        $y ^= $x;
    };
}
pub(self) use enc;

macro_rules! dec {
    ($x:ident, $y:ident, $k:ident, $alpha:literal, $beta:literal) => {
        $y ^= $x;
        $y = $y.rotate_right($beta);
        $x ^= $k;
        $x = $x.wrapping_sub($y);
        $x = $x.rotate_left($alpha);
    };
}
pub(self) use dec;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
pub enum SpeckVariant {
    Speck32_64,
    Speck48_72,
    Speck48_96,
    Speck64_96,
    Speck64_128,
    Speck96_96,
    Speck96_144,
    #[default]
    Speck128_128,
    Speck128_192,
    Speck128_256,
}

impl SpeckVariant {
    /// Block size in bytes
    pub fn block_size(&self) -> u32 {
        match self {
            Self::Speck32_64 => 4,
            Self::Speck48_72 => 6,
            Self::Speck48_96 => 6,
            Self::Speck64_96 => 8,
            Self::Speck64_128 => 8,
            Self::Speck96_96 => 12,
            Self::Speck96_144 => 12,
            Self::Speck128_128 => 16,
            Self::Speck128_192 => 16,
            Self::Speck128_256 => 16,
        }
    }

    // Key size in bytes
    pub fn key_size(&self) -> u32 {
        match self {
            Self::Speck32_64 => 8,
            Self::Speck48_72 => 9,
            Self::Speck48_96 => 12,
            Self::Speck64_96 => 12,
            Self::Speck64_128 => 16,
            Self::Speck96_96 => 12,
            Self::Speck96_144 => 18,
            Self::Speck128_128 => 16,
            Self::Speck128_192 => 24,
            Self::Speck128_256 => 32,
        }
    }

    /// Number of rounds
    pub fn rounds(&self) -> u32 {
        match self {
            Self::Speck32_64 => 22,
            Self::Speck48_72 => 22,
            Self::Speck48_96 => 23,
            Self::Speck64_96 => 26,
            Self::Speck64_128 => 27,
            Self::Speck96_96 => 28,
            Self::Speck96_144 => 29,
            Self::Speck128_128 => 32,
            Self::Speck128_192 => 33,
            Self::Speck128_256 => 34,
        }
    }
}

impl std::fmt::Display for SpeckVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Speck32_64 => write!(f, "Speck32/64"),
            Self::Speck48_72 => write!(f, "Speck48/72"),
            Self::Speck48_96 => write!(f, "Speck48/96"),
            Self::Speck64_96 => write!(f, "Speck64/96"),
            Self::Speck64_128 => write!(f, "Speck64/128"),
            Self::Speck96_96 => write!(f, "Speck96/96"),
            Self::Speck96_144 => write!(f, "Speck96/144"),
            Self::Speck128_128 => write!(f, "Speck128/128"),
            Self::Speck128_192 => write!(f, "Speck128/192"),
            Self::Speck128_256 => write!(f, "Speck128/256"),
        }
    }
}
