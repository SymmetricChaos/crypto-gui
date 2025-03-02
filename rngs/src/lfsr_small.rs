#[macro_export]
/// Create a Fibonacci LFSR function with inlined taps, will not compile with invalid taps
macro_rules! lfsr64 {
    ($name: ident, $($tap: literal),+) => {
        pub fn $name(mut state: u64) -> u64 {
            let mut new_bit = 0;
            $(
                new_bit ^= (state >> $tap) & 1;
            )+
            state = (state << 1) | new_bit;
            state
        }
    };
}

#[macro_export]
/// Create a Galois LFSR function with inlined taps, will not compile with invalid taps
macro_rules! glfsr64 {
    ($name: ident, $taps: literal) => {
        pub fn $name(mut state: u64) -> u64 {
            let new_bit = state & 1;
            state >>= 1;
            state ^= (taps * new_bit);
            state
        }
    };
}

#[macro_export]
/// Create a Fibonacci LFSR function with inlined taps, will not compile with invalid taps
macro_rules! lfsr32 {
    ($name: ident, $($tap: literal),+) => {
        pub fn $name(mut state: u32) -> u32 {
            let mut new_bit = 0;
            $(
                new_bit ^= (state >> $tap) & 1;
            )+
            state = (state << 1) | new_bit;
            state
        }
    };
}

#[macro_export]
/// Create a Galois LFSR function with inlined taps, will not compile with invalid taps
macro_rules! glfsr32 {
    ($name: ident, $taps: literal) => {
        pub fn $name(mut state: u32) -> u32 {
            let new_bit = state & 1;
            state >>= 1;
            state ^= (taps * new_bit);
            state
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
        let mut x = 123456789;
        for _ in 0..40 {
            x = my_lfsr(x);
            println!("{x}")
        }
    }
}
