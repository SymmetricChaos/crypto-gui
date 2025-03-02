#[macro_export]
/// Create an LFSR that keeps just a single u64 as state and inlines the taps
macro_rules! small_lfsr64 {
    ($name: ident, $($tap: literal),+ $(,)?) => {
        pub struct $name {
            state: u64
        }

        impl $name {
            pub fn shift(&mut self) {
                let mut new_bit = 0;
                $(
                    new_bit ^= (self.state >> $tap) & 1;
                )+
                self.state = (self.state << 1) | new_bit;
            }

            pub fn next_bit(&mut self) -> u64 {
                let mut new_bit = 0;
                $(
                    new_bit ^= (self.state >> $tap) & 1;
                )+
                self.state = (self.state << 1) | new_bit;
                new_bit
            }
        }

        impl ClassicRng for $name {
            fn next_u32(&mut self) -> u32 {
                let mut out = 0;
                for _ in 0..32 {
                    out <<= 1;
                    out |= next_bit;
                }
                out as u32
            }
        }
    };
}

#[macro_export]
/// Create an LFSR that keeps just a single u32 as state and inlines the taps
macro_rules! small_lfsr32 {
    ($name: ident, $($tap: literal),+ $(,)?) => {
        pub struct $name {
            state: u32
        }

        impl $name {
            pub fn shift(&mut self) {
                let mut new_bit = 0;
                $(
                    new_bit ^= (self.state >> $tap) & 1;
                )+
                self.state = (self.state << 1) | new_bit;
            }

            pub fn next_bit(&mut self) -> u64 {
                let mut new_bit = 0;
                $(
                    new_bit ^= (self.state >> $tap) & 1;
                )+
                self.state = (self.state << 1) | new_bit;
                new_bit
            }
        }

        impl ClassicRng for $name {
            fn next_u32(&mut self) -> u32 {
                let mut out = 0;
                for _ in 0..32 {
                    out <<= 1;
                    out |= next_bit;
                }
                out as u32
            }
        }
    };
}
