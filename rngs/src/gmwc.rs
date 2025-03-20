use crate::ClassicRng;

const MINUSA0: u128 = 0x7d084a4d80885f;
const A0INV: u128 = 0x9b1eea3792a42c61;
const A1: u128 = 0xff002aae7d81a646;

// Effectively an LCG with modulus 0xff00_2aae_7d81_a646_007d_084a_4d80_885f
pub struct GeneralizedMultiplyWithCarry128 {
    pub x: u128,
    pub c: u128,
}

impl Default for GeneralizedMultiplyWithCarry128 {
    fn default() -> Self {
        Self::from_u64(0x0BAD_5EED_0BAD_5EED)
    }
}

impl GeneralizedMultiplyWithCarry128 {
    pub fn step(&mut self) {
        let t = A1.wrapping_mul(self.x).wrapping_add(self.c);
        self.x = (A0INV.wrapping_mul(t)) & 0xffff_ffff_ffff_ffff;
        self.c = (t.wrapping_add(MINUSA0.wrapping_mul(self.x))) >> 64;
    }
}

impl GeneralizedMultiplyWithCarry128 {
    pub fn from_u64(seed: u64) -> Self {
        Self {
            x: seed as u128,
            c: 1,
        }
    }
}

impl ClassicRng for GeneralizedMultiplyWithCarry128 {
    fn next_u32(&mut self) -> u32 {
        self.step();
        self.x as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.step();
        self.x as u64
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outputs() {
        let mut rng = GeneralizedMultiplyWithCarry128::from_u64(0x0BAD_5EED_0BAD_5EED);
        let mut v = Vec::new();
        for _ in 0..10 {
            v.push(rng.next_u64());
        }
        assert_eq!(
            vec![
                0x762A5138BEF7446F,
                0x78154714529F4A41,
                0x2B0C9D1AEE630DEB,
                0x1169B588A54A7F9C,
                0x50FFE8860EABE19D,
                0x071C0F4C65F277AE,
                0xD37749FCD8FF1184,
                0x8826072206DEA566,
                0xCD4015E551A9DFB5,
                0x36A45BCB77A7C5D4,
            ],
            v
        );
    }
}
