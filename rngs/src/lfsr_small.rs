#[macro_export]
/// Create a 64-bit Fibonacci LFSR function. The taps should be the powers of the feedback polynomial, excluding 0.
macro_rules! lfsr64 {
    ($name: ident, $($tap: literal),+) => {
        /// Advance the state. Output is (new_state, next_bit).
        pub fn $name(mut state: u64) -> (u64, u64) {
            let mut new_bit = state;
            $(
                new_bit ^= (state >> (64-$tap));
            )+
            new_bit &= 1;
            state = (state >> 1) | (new_bit << 31);
            (state,new_bit)
        }
    };
    ($($tap: literal),+) => {
        paste::paste!{
            /// Advance the state. Output is (new_state, next_bit).
            pub fn [< lfsr $(_$tap)+ >](mut state: u64) -> (u64, u64) {
                let mut new_bit = state;
                $(
                    new_bit ^= (state >> (64-$tap));
                )+
                new_bit &= 1;
                state = (state >> 1) | (new_bit << 31);
                (state,new_bit)
            }
        }
    };
}

#[macro_export]
/// Create a 32-bit Fibonacci LFSR function. The taps should be the powers of the feedback polynomial, excluding 0.
macro_rules! lfsr32 {
    ($name: ident, $($tap: literal),+) => {
        /// Advance the state. Output is (new_state, next_bit).
        pub fn $name(mut state: u32) -> (u32, u32) {
            let mut new_bit = state;
            $(
                new_bit ^= (state >> (33-$tap));
            )+
            new_bit &= 1;
            state = (state >> 1) | (new_bit << 31);
            (state,new_bit)
        }
    };
    ($($tap: literal),+) => {
        paste::paste!{
            /// Advance the state. Output is (new_state, next_bit).
            pub fn [< lfsr $(_$tap)+ >](mut state: u32) -> (u32, u32) {
                let mut new_bit = state;
                $(
                    new_bit ^= (state >> (33-$tap));
                )+
                new_bit &= 1;
                state = (state >> 1) | (new_bit << 31);
                (state,new_bit)
            }
        }
    };
}

#[macro_export]
/// Create a 16-bit Fibonacci LFSR function. The taps should be the powers of the feedback polynomial, excluding 0.
macro_rules! lfsr16 {
    ($name: ident, $($tap: literal),+) => {
        /// Advance the state. Output is (new_state, next_bit).
        pub fn $name(mut state: u16) -> (u16, u16) {
            let mut new_bit = state;
            $(
                new_bit ^= (state >> (17-$tap));
            )+
            new_bit &= 1;
            state = (state >> 1) | (new_bit << 15);
            (state,new_bit)
        }
    };
    ($($tap: literal),+) => {
        paste::paste!{
            /// Advance the state. Output is (new_state, next_bit).
            pub fn [< lfsr $(_$tap)+ >](mut state: u16) -> (u16, u16) {
                let mut new_bit = state & 1;
                $(
                    new_bit ^= (state >> $tap);
                )+
                new_bit &= 1;
                state = (state >> 1) | (new_bit << 15);
                (state,new_bit)
            }
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
    fn example() {
        lfsr64!(my_lfsr, 6, 9, 13, 19, 63);
        lfsr64!(6, 9, 13, 19, 63);
        let (mut state1, mut bit1) = (12345678987654321, 0);
        let (mut state2, mut bit2) = (12345678987654321, 0);
        for _ in 0..20 {
            assert!(state1 == state2);
            assert!(bit1 == bit2);
            (state1, bit1) = my_lfsr(state1);
            (state2, bit2) = lfsr_6_9_13_19_63(state2);
        }
    }

    #[test]
    fn test_one_step() {
        lfsr16!(my_lfsr, 11, 13, 14, 16);
        let (mut state, _) = (0xACE1, 0);
        assert_eq!(my_lfsr(state).0, 0x5670);
    }
}
