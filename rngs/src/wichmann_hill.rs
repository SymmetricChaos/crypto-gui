use crate::SimpleRng;

pub struct WichmannHill {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

impl Default for WichmannHill {
    fn default() -> Self {
        Self {
            s1: 1,
            s2: 2,
            s3: 3,
        }
    }
}

impl WichmannHill {
    /// In the range 0..1
    pub fn next_f32(&mut self) -> f32 {
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        ((self.s1 as f32) / 30269.0 + (self.s2 as f32) / 30307.0 + (self.s3 as f32) / 30323.0)
            .fract()
    }

    /// Actually closer to 15 bits
    pub fn next_u16(&mut self) -> u16 {
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        (self.s1 + self.s2 + self.s3) as u16
    }
}

impl SimpleRng for WichmannHill {
    /// Closer to 31 bits of entropy
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        out += (self.s1 + self.s2 + self.s3) << 16;
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        out = out.wrapping_add(self.s1 + self.s2 + self.s3);
        out
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vector() {}
// }
