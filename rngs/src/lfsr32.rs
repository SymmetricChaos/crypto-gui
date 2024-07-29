use crate::traits::ClassicRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfsrMode {
    Fibonncci,
    Galois,
}

pub struct Lfsr32 {
    pub register: u32,
    pub taps: u32,
    pub mask: u32,
    pub mode: LfsrMode,
}

impl Default for Lfsr32 {
    fn default() -> Self {
        Self::from_taps(0b1011010000000000)
    }
}

impl Lfsr32 {
    pub fn from_taps(taps: u32) -> Self {
        let n = 32 - taps.leading_zeros();
        let mask = n.pow(2) - 1;
        Self {
            register: 1_u32,
            taps,
            mask,
            mode: LfsrMode::Fibonncci,
        }
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

    pub fn peek_next_bit(&self) -> u32 {
        (self.register & self.taps).count_ones() & 1
    }
}

impl ClassicRng for Lfsr32 {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod lfsr32_tests {}
