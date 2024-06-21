use crate::traits::ClassicRng;

// macro_rules! middle_square_binary {
//     ($name: ident, $state_type: ty, $square_type: ty, $shift: literal) => {
//         pub struct $name {
//             pub state: $state_type,
//         }

//         impl Default for $name {
//             fn default() -> Self {
//                 Self { state: 255 }
//             }
//         }

//         impl $name {
//             /// Step the RNG forward
//             pub fn next(&mut self) -> $state_type {
//                 let sq = self.state as $square_type * self.state as $square_type;
//                 let mid = (sq >> $shift) as $state_type;
//                 self.state = mid;
//                 mid
//             }

//             /// Nonadvancing version of next
//             pub fn peek_next(&self) -> $state_type {
//                 let sq = self.state as $square_type * self.state as $square_type;
//                 (sq >> $shift) as $state_type
//             }
//         }

//         impl ClassicRng for $name {
//             fn next_u32(&mut self) -> u32 {
//                 self.next() as u32
//             }
//         }
//     };
// }

// middle_square_binary!(MiddleSquareBinary64, u64, u128, 16);
// middle_square_binary!(MiddleSquareBinary32, u32, u64, 8);
// middle_square_binary!(MiddleSquareBinary16, u16, u32, 4);
// middle_square_binary!(MiddleSquareBinary8, u8, u16, 2);

#[derive(Debug, PartialEq, Eq)]
pub enum MSBSize {
    // B64,
    B32,
    B16,
    B8,
}

impl MSBSize {
    pub fn mask(&self) -> u64 {
        match self {
            // MSBSize::B64 => 0xFFFFFFFFFFFFFFFF,
            MSBSize::B32 => 0xFFFFFFFF,
            MSBSize::B16 => 0xFFFF,
            MSBSize::B8 => 0xFF,
        }
    }

    pub fn quarter_size(&self) -> usize {
        match self {
            // MSBSize::B64 => 16,
            MSBSize::B32 => 8,
            MSBSize::B16 => 4,
            MSBSize::B8 => 2,
        }
    }
}

pub struct MiddleSquareBinary {
    pub width: MSBSize,
    pub state: u64,
}

impl Default for MiddleSquareBinary {
    fn default() -> Self {
        Self {
            width: MSBSize::B32,
            state: 255,
        }
    }
}

impl MiddleSquareBinary {
    /// Step the RNG forward
    pub fn next(&mut self) -> u64 {
        let sq = self.state * self.state;
        let mid = (sq >> self.width.quarter_size()) & self.width.mask();
        self.state = mid;
        mid
    }

    /// Nonadvancing version of next
    pub fn peek_next(&self) -> u64 {
        let sq = self.state * self.state;
        (sq >> self.width.quarter_size()) & self.width.mask()
    }
}

impl ClassicRng for MiddleSquareBinary {
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }
}
