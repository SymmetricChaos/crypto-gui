use crate::traits::ClassicRng;

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
    pub fn from_taps(taps: u32) -> Self {
        // Only 31 bits are usable
        assert!(taps < 0x80000000);
        let n = 32 - taps.leading_zeros();
        let mask = 2_u32.pow(n) - 1;
        Self {
            register: 0_u32,
            taps,
            mask,
        }
    }

    pub fn from_taps_and_register(taps: u32, register: u32) -> Self {
        // Only 31 bits are usable
        assert!(taps < 0x80000000);
        let n = 32 - taps.leading_zeros();
        let mask = n.pow(2) - 1;
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

    pub fn next_bit(&mut self) -> u32 {
        // Mask off everything except the taps, count the one bits, take the parity
        let bit = (self.register & self.taps).count_ones() & 1;
        // Shift the register, mask off the high bits, OR the bit into the register
        self.register <<= 1;
        self.register &= self.mask;
        self.register |= bit;
        bit
    }

    // Fill a byte MSB first
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
        assert_eq!(0b00000000000000000000000000000001, rng.register);
    }

    #[test]
    fn test_get_bit() {
        let rng = Lfsr32::default();
        assert_eq!(rng.get_bit(0), 1);
        assert_eq!(rng.get_bit(1), 0);
    }
}
