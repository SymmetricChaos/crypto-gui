// Not useful digital ciphers themselves but used as a component

#[derive(Debug, Clone, Copy)]
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
        // Mask off everything except the taps, take the parity
        let bit = (self.register & self.taps).count_ones() & 1;
        // Shift the register, mask off the high bits, OR the bit into the register
        self.register <<= 1;
        self.register &= self.mask;
        self.register |= bit;
        bit
    }

    pub fn step(&mut self) {
        self.next_bit();
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

#[derive(Debug, Clone, Copy)]
pub struct Lfsr64 {
    pub register: u64,
    pub taps: u64,
    pub mask: u64,
}

impl Default for Lfsr64 {
    fn default() -> Self {
        Self::from_taps(0b1011010000000000)
    }
}

impl Lfsr64 {
    pub fn from_taps(taps: u64) -> Self {
        // Only 63 bits are usable
        assert!(taps < 0x8000000000000000);
        let n = 64 - taps.leading_zeros();
        let mask = 2_u64.pow(n) - 1;
        Self {
            register: 0_u64,
            taps,
            mask,
        }
    }

    pub fn from_taps_and_register(taps: u64, register: u64) -> Self {
        // Only 63 bits are usable
        assert!(taps < 0x8000000000000000);
        let n = (64 - taps.leading_zeros()) as u64;
        let mask = n.pow(2) - 1;
        Self {
            register: register & mask,
            taps,
            mask,
        }
    }

    pub fn get_bit(&self, idx: u64) -> u64 {
        assert!(idx < 64);
        (self.register & (1 << idx)).count_ones() as u64
    }

    pub fn next_bit(&mut self) -> u64 {
        // Mask off everything except the taps, take the parity
        let bit = ((self.register & self.taps).count_ones() & 1) as u64;
        // Shift the register, mask off the high bits, OR the bit into the register
        self.register <<= 1;
        self.register &= self.mask;
        self.register |= bit;
        bit as u64
    }

    pub fn step(&mut self) {
        self.next_bit();
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

#[cfg(test)]
mod lfsr32_tests {
    use super::*;

    #[test]
    fn test_init() {
        let rng = Lfsr32::default();
        assert_eq!(0b00000000000000001111111111111111, rng.mask);
        assert_eq!(0b00000000000000001011010000000000, rng.taps);
        assert_eq!(0b00000000000000000000000000000000, rng.register);
    }

    #[test]
    fn test_get_bit() {
        let rng = Lfsr32::default();
        assert_eq!(rng.get_bit(0), 1);
        assert_eq!(rng.get_bit(1), 0);
    }

    #[test]
    fn test_init_64() {
        let rng = Lfsr64::default();
        assert_eq!(
            0b0000000000000000000000000000000000000000000000001111111111111111,
            rng.mask
        );
        assert_eq!(
            0b0000000000000000000000000000000000000000000000001011010000000000,
            rng.taps
        );
        assert_eq!(
            0b0000000000000000000000000000000000000000000000000000000000000000,
            rng.register
        );
    }

    #[test]
    fn test_get_bit_64() {
        let rng = Lfsr64::default();
        assert_eq!(rng.get_bit(0), 1);
        assert_eq!(rng.get_bit(1), 0);
    }
}
