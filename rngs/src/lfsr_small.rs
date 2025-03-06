#[macro_export]
/// Create a Fibonacci LFSR function that shifts the state to the right (toward the least significant bit).
/// Example: for the LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use lfsr_r(my_lfsr, u32, 16; 14, 13, 11)
macro_rules! lfsr_r {
    ($name: ident, $t: ty, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub const fn $name(state: $t) -> $t {
            assert!($bits < <$t>::BITS);
            let mut new_bit = state;
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($bits - $tap);
            )+
            ((state >> 1) | (new_bit << ($bits - 1)) ) & (!0 >> (<$t>::BITS - $bits))
        }
    };
}

#[macro_export]
/// Create a Fibonacci LFSR function that shifts the state to the left (toward the most significant bit).
/// Example: for the LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use lfsr_l(my_lfsr, u32, 16; 14, 13, 11)
macro_rules! lfsr_l {
    ($name: ident, $t: ty, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub const fn $name(state: $t) -> $t {
            assert!($bits < <$t>::BITS);
            let mut new_bit = state >> ($bits - 1);
            $(
                assert!($bits >= $tap);
                new_bit ^= state >> ($tap - 1);
            )+
            ((state << 1) | (new_bit & 1 )) & (!0 >> (<$t>::BITS - $bits))
        }
    };
}

#[macro_export]
/// Create a Galois LFSR function that shifts the state to the right (toward the least significant bit).
/// Example: for the LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use glfsr_r(my_glfsr, u32, 16; 14, 13, 11)
macro_rules! glfsr_r {
    ($name: ident, $t: ty, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub const fn $name(state: $t) -> $t {
            assert!($bits < <$t>::BITS);
            const TOGGLE: $t = 0 $(| (1 << $tap - 1))+ | (1 << $bits - 1);
            const MASK: $t = (!0 >> (<$t>::BITS - $bits));
            if state & 1 == 1 {
                ((state >> 1) ^ TOGGLE) & MASK
            } else {
                (state >> 1) & MASK
            }
        }
    };

}

#[macro_export]
/// Create a Galois LFSR function that shifts the state to the right (toward the least significant bit). The state is a u64 unless specified.
/// Example: for the LFSR defined by the feedback polynomial x^16 + x^14 + x^13 + x^11 + 1 use glfsr_l(my_glfsr, u32, 16; 14, 13, 11)
macro_rules! glfsr_l {
    ($name: ident, $t: ty, $bits: literal; $($tap: literal),+) => {
        /// Advance the state.
        pub const fn $name(state: $t) -> $t {
            assert!($bits < <$t>::BITS);
            const TOGGLE: $t = 1 $(| (1 << $bits - $tap))+;
            const MASK: $t = (!0 >> (<$t>::BITS - $bits));
            if state >> ($bits - 1) == 1 {
                ((state << 1) ^ TOGGLE) & MASK
            } else {
                (state << 1) & MASK
            }
        }
    };
}

/// Get the nth bit from the right. Setting idx = 0 gives the LSB.
pub const fn get_bit64(n: u64, idx: usize) -> u64 {
    assert!(idx < 64);
    (n >> idx) & 1
}

/// Get the nth bit from the right. Setting idx = 0 gives the LSB.
pub const fn get_bit32(n: u32, idx: usize) -> u32 {
    assert!(idx < 32);
    (n >> idx) & 1
}

#[cfg(test)]
mod test {

    macro_rules! cycle_length {
        ($f:ident, $bits: literal) => {
            let mut s = 1;
            let mut period = 0;
            loop {
                s = $f(s);
                period += 1;
                if s == 1 {
                    break;
                }
            }
            if period != 2_u64.pow($bits) - 1 {
                panic!(
                    "{} bits should give {} states, found {}",
                    $bits,
                    2_u64.pow($bits) - 1,
                    period
                )
            }
        };
    }

    #[test]
    fn one_step_fib() {
        lfsr_r!(my_lfsr64_r, u64, 16; 11, 13, 14);
        assert_eq!(my_lfsr64_r(0xACE1), 0x5670);
    }

    #[test]
    fn outputs() {
        let sequence = [
            0b00001, 0b10000, 0b01000, 0b00100, 0b10010, 0b01001, 0b10100, 0b11010, 0b01101,
            0b00110, 0b10011, 0b11001, 0b11100, 0b11110, 0b11111, 0b01111, 0b00111, 0b00011,
        ];
        let mut s = 0b00001;
        lfsr_r!(my_lfsr64, u64, 5; 3);
        for (i, test) in sequence.into_iter().enumerate() {
            assert_eq!(test, s, "{}", i);
            s = my_lfsr64(s);
        }
    }

    #[test]
    fn cycle_length_short() {
        lfsr_r!(my_lfsr64_r, u64, 5; 2,3,4);
        cycle_length!(my_lfsr64_r, 5);

        lfsr_l!(my_lfsr64_l, u64, 5; 2,3,4);
        cycle_length!(my_lfsr64_l, 5);

        glfsr_r!(my_glfsr64_r, u64, 5; 2,3,4);
        cycle_length!(my_glfsr64_r, 5);

        glfsr_l!(my_glfsr64_l, u64, 5; 2,3,4);
        cycle_length!(my_glfsr64_l, 5);
    }

    #[ignore = "long run time for cycle detection"]
    #[test]
    fn cycle_length_long() {
        lfsr_r!(my_lfsr64_r, u64, 16; 11, 13, 14);
        cycle_length!(my_lfsr64_r, 16);

        lfsr_l!(my_lfsr64_l, u64, 16; 11, 13, 14);
        cycle_length!(my_lfsr64_l, 16);

        glfsr_r!(my_glfsr64_r,u64,  16; 11, 13, 14);
        cycle_length!(my_glfsr64_r, 16);

        glfsr_l!(my_glfsr64_l, u64, 16; 11, 13, 14);
        cycle_length!(my_glfsr64_l, 16);
    }
}
