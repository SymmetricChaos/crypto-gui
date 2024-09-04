// pub mod simon128;
pub mod simon32;
// pub mod simon64;

// These are the five Z sequences as u64s
// Only the lower 62 bits are used
// The order of the bits is reversed from the paper so that the "first" bit
// is the lowest bit in order to simplify implementation.
pub const Z: [u64; 5] = [
    0b0001100111000011010100100010111110110011100001101010010001011111,
    0b0001011010000110010011111011100010101101000011001001111101110001,
    0b0011001101101001111110001000010100011001001011000000111011110101,
    0b0011110000101100111001010001001000000111101001100011010111011011,
    0b0011110111001001010011000011101000000100011011010110011110001011,
];

pub fn select_z_bit(seq: usize, bit: usize) -> u64 {
    (Z[seq] >> bit) & 1
}

// A constant used in the specification
// implementation does't use this. The
// effect of XORing this into a word is
// more flexibly produced by using a
// bitwise NOT and then XORing 3.
// pub const C: u64 = 0xFFFFFFFFFFFFFFFC;

// The Simon round function
macro_rules! round {
    ($a:ident, $k:ident) => {
        ($a.rotate_left(1) & $a.rotate_left(8)) ^ $a.rotate_left(2) ^ $k
    };
}
pub(self) use round;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
pub enum SimonVariant {
    Simon32_64,
    Simon64_96,
    Simon64_128,
    Simon128_128,
    Simon128_192,
    Simon128_256,
}

impl SimonVariant {
    pub fn block_size(&self) -> u32 {
        match self {
            Self::Simon32_64 => 32,
            Self::Simon64_96 => 64,
            Self::Simon64_128 => 64,
            Self::Simon128_128 => 128,
            Self::Simon128_192 => 128,
            Self::Simon128_256 => 128,
        }
    }

    pub fn key_size(&self) -> u32 {
        match self {
            Self::Simon32_64 => 64,
            Self::Simon64_96 => 96,
            Self::Simon64_128 => 128,
            Self::Simon128_128 => 128,
            Self::Simon128_192 => 192,
            Self::Simon128_256 => 256,
        }
    }
}

impl std::fmt::Display for SimonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simon32_64 => write!(f, "Simon32/64"),
            Self::Simon64_96 => write!(f, "Simon64/96"),
            Self::Simon64_128 => write!(f, "Simon64/128"),
            Self::Simon128_128 => write!(f, "Simon128/128"),
            Self::Simon128_192 => write!(f, "Simon128/192"),
            Self::Simon128_256 => write!(f, "Simon128/256"),
        }
    }
}
