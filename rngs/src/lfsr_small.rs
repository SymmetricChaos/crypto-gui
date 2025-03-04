#[macro_export]
/// Create a 64-bit Fibonacci LFSR function that shifts the state to the right (toward the least significant bit). The taps should be the powers of the feedback polynomial, excluding 0.
macro_rules! lfsr64 {
    ($name: ident, $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u64) -> u64 {
            let mut new_bit = state;
            $(
                new_bit ^= (state >> (65-$tap));
            )+
            new_bit &= 1;
            (state >> 1) | (new_bit << 63)
        }
    };
    ($name: ident, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u64) -> u64 {
            assert!($bits <= 64);
            let mut new_bit = state;
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($bits + 1 - $tap);
            )+
            new_bit &= 1;
            ((state >> 1) | (new_bit << ($bits - 1)) ) & (!0_u64 >> (64 - $bits))
        }
    };
}

#[macro_export]
/// Create a 32-bit Fibonacci LFSR function that shifts the state to the right. The taps should be the powers of the feedback polynomial, excluding 0.
macro_rules! lfsr32 {
    ($name: ident, $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u32) -> u32 {
            let mut new_bit = state;
            $(
                new_bit ^= (state >> (33-$tap));
            )+
            new_bit &= 1;
            (state >> 1) | (new_bit << 31)
        }
    };
    ($name: ident, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u32) -> u32 {
            assert!($bits <= 32);
            let mut new_bit = state;
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($bits + 1 - $tap);
            )+
            new_bit &= 1;
            ((state >> 1) | (new_bit << ($bits - 1)) ) & (!0_u32 >> (32 - $bits))
        }
    };
}

#[inline]
pub fn get_bit_64(state: u64, idx: u64) -> u64 {
    assert!(idx < 64);
    (state >> idx) & 1
}

#[inline]
pub fn get_bit_32(state: u32, idx: u32) -> u32 {
    assert!(idx < 32);
    (state >> idx) & 1
}

#[cfg(test)]
mod test {

    #[test]
    fn test_one_step() {
        lfsr32!(my_lfsr32, 16; 11, 13, 14, 16);
        assert_eq!(my_lfsr32(0xACE1), 0x5670);
        lfsr32!(my_lfsr64, 16; 11, 13, 14, 16);
        assert_eq!(my_lfsr64(0xACE1), 0x5670);
    }
}
