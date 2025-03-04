#[macro_export]
/// Create a Fibonacci LFSR function that shifts the state to the right (toward the least significant bit). The state is a u64;
/// Example: for the 16-bit LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use lfsr64_r(my_lfsr, 16; 14, 13, 11)
macro_rules! lfsr64_r {

    ($name: ident, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u64) -> u64 {
            assert!($bits < 64);
            let mut new_bit = state;
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($bits - $tap);
            )+
            ((state >> 1) | (new_bit << ($bits - 1)) ) & (!0_u64 >> (64 - $bits))
        }
    };
}

#[macro_export]
/// Create a Fibonacci LFSR function that shifts the state to the right (toward the least significant bit). The state is a u64;
/// Example: for the 16-bit LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use lfsr32_r(my_lfsr, 16; 14, 13, 11)
macro_rules! lfsr32_r {
    ($name: ident, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub fn $name(state: u32) -> u32 {
            assert!($bits <= 32);
            let mut new_bit = state;
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($bits - $tap);
            )+
            ((state >> 1) | (new_bit << ($bits - 1)) ) & (!0_u32 >> (32 - $bits))
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn one_step_fib() {
        lfsr32_r!(my_lfsr32_r, 16; 11, 13, 14);
        assert_eq!(my_lfsr32_r(0xACE1), 0x5670);
        lfsr64_r!(my_lfsr64_r, 16; 11, 13, 14);
        assert_eq!(my_lfsr64_r(0xACE1), 0x5670);
    }

    #[test]
    fn outputs() {
        let sequence = [
            0b00001, 0b10000, 0b01000, 0b00100, 0b10010, 0b01001, 0b10100, 0b11010, 0b01101,
            0b00110, 0b10011, 0b11001, 0b11100, 0b11110, 0b11111, 0b01111, 0b00111, 0b00011,
        ];
        let mut s = 0b00001;
        lfsr32_r!(my_lfsr32, 5; 3);
        for (i, test) in sequence.into_iter().enumerate() {
            assert_eq!(test, s, "{}", i);
            s = my_lfsr32(s);
        }

        let mut s = 0b00001;
        lfsr64_r!(my_lfsr64, 5; 3);
        for (i, test) in sequence.into_iter().enumerate() {
            assert_eq!(test, s as u32, "{}", i);
            s = my_lfsr64(s);
        }
    }

    #[test]
    fn cycle_length_short() {
        lfsr32_r!(my_lfsr32, 5; 2,3,4);
        let mut states = Vec::new();
        let mut s = 1;
        while !states.contains(&s) {
            states.push(s);
            s = my_lfsr32(s);
        }
        // Five bits should give (2^5)-1 = 31 states
        assert_eq!(31, states.len());
    }

    #[test]
    fn cycle_length_long() {
        lfsr32_r!(my_lfsr32_r, 16; 11, 13, 14);
        let mut states = Vec::new();
        let mut s = 1;
        while !states.contains(&s) {
            states.push(s);
            s = my_lfsr32_r(s);
        }
        // 16 bits should give (2^16)-1 = 65535 states
        assert_eq!(65535, states.len());
    }
}
