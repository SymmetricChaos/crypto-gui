use crate::traits::ClassicRng;

macro_rules! middle_square_binary {
    ($name: ident, $state_type: ty, $square_type: ty, $shift: literal) => {
        pub struct $name {
            pub state: $state_type,
        }

        impl Default for $name {
            fn default() -> Self {
                Self { state: 255 }
            }
        }

        impl $name {
            /// Step the RNG forward
            pub fn next(&mut self) -> $state_type {
                let sq = self.state as $square_type * self.state as $square_type;
                let mid = (sq >> $shift) as $state_type;
                self.state = mid;
                mid
            }

            /// Nonadvancing version of next
            pub fn peek_next(&self) -> $state_type {
                let sq = self.state as $square_type * self.state as $square_type;
                (sq >> $shift) as $state_type
            }
        }
    };
}

middle_square_binary!(MiddleSquareBinary64, u64, u128, 32);
middle_square_binary!(MiddleSquareBinary32, u32, u64, 16);
middle_square_binary!(MiddleSquareBinary16, u16, u32, 8);
middle_square_binary!(MiddleSquareBinary8, u8, u16, 4);

impl ClassicRng for MiddleSquareBinary32 {
    fn next_u32(&mut self) -> u32 {
        self.next()
    }
}
