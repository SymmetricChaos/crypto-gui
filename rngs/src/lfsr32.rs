use crate::traits::ClassicRng;

// macro_rules! lfsr {
//     ($name: ident, $integer: ty, $width: literal) => {

//     };

#[derive(Debug, Clone)]
pub struct Lfsr32 {
    pub register: u32,
    pub taps: u32,
    pub mask: u32,
}

impl Default for Lfsr32 {
    fn default() -> Self {
        Self::from_taps(0b1011010000000000)
    }
}

impl Lfsr32 {
    /// Create an LFSR with the provided taps and the register set to zero
    pub fn from_taps(taps: u32) -> Self {
        // Only 31 bits are usable
        assert!(taps < 0x80000000);
        let n = 32 - taps.leading_zeros();
        let mask = (2 << n - 1) - 1;
        Self {
            register: 0,
            taps,
            mask,
        }
    }

    /// Create an LFSR with the provided taps and register
    pub fn from_taps_and_register(taps: u32, register: u32) -> Self {
        // Only 31 bits are usable
        assert!(taps < 0x80000000);
        let n = 32 - taps.leading_zeros();
        let mask = (2 << n - 1) - 1;
        Self {
            register: register & mask,
            taps,
            mask,
        }
    }

    pub fn get_bit(&self, idx: u32) -> u32 {
        assert!(idx < 32);
        (self.register & (1 << idx)).count_ones()
    }

    pub fn bit_from_taps(&self) -> u32 {
        // Mask off everything except the taps, count the one bits, take the parity
        (self.register & self.taps).count_ones() & 1
    }

    /// Step the LFSR without outputting
    pub fn step(&mut self) {
        let bit = self.bit_from_taps();
        // Shift the register, mask off the high bits, OR the bit into the register
        self.register <<= 1;
        self.register &= self.mask;
        self.register |= bit;
    }

    /// Step the LFSR and output the bit
    pub fn next_bit(&mut self) -> u32 {
        let bit = self.bit_from_taps();
        self.register <<= 1;
        self.register &= self.mask;
        self.register |= bit;
        bit
    }

    /// Fill a byte MSB first
    pub fn next_byte(&mut self) -> u8 {
        let mut out = 0;
        for _ in 0..8 {
            out <<= 1;
            out |= self.next_bit() as u8;
        }
        out
    }
}

impl ClassicRng for Lfsr32 {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        for _ in 0..32 {
            out <<= 1;
            out |= self.next_bit();
        }
        out
    }
}

#[cfg(test)]
mod lfsr32_tests {
    use super::Lfsr32;

    #[test]
    fn test_init() {
        let rng = Lfsr32::default();
        assert_eq!(0b00000000000000001111111111111111, rng.mask);
        assert_eq!(0b00000000000000001011010000000000, rng.taps);
        assert_eq!(0b00000000000000000000000000000000, rng.register);
    }
}
