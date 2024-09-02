pub mod speck128;
pub mod speck32;
pub mod speck64;

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

pub enum Speck {
    Speck32_64,
    Speck64_96,
    Speck64_128,
    Speck128_128,
    Speck128_192,
    Speck128_256,
}

use std::fmt::Display;
impl Display for Speck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speck::Speck32_64 => write!(f, "Speck32/64"),
            Speck::Speck64_96 => write!(f, "Speck64/96"),
            Speck::Speck64_128 => write!(f, "Speck64/128"),
            Speck::Speck128_128 => write!(f, "Speck128/128"),
            Speck::Speck128_192 => write!(f, "Speck128/192"),
            Speck::Speck128_256 => write!(f, "Speck128/256"),
        }
    }
}
