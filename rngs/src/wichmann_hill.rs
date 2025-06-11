use crate::ClassicRng;

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
    pub fn next_f32(&mut self) -> f32 {
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        ((self.s1 as f32) / 30269.0 + (self.s2 as f32) / 30307.0 + (self.s3 as f32) / 30323.0)
            .fract()
    }
}

impl ClassicRng for WichmannHill {
    fn next_u32(&mut self) -> u32 {
        self.s1 = (self.s1 * 171) % 30269;
        self.s2 = (self.s2 * 172) % 30307;
        self.s3 = (self.s3 * 170) % 30323;
        self.s1 + self.s2 + self.s3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_five() {
        let mut rng = WichmannHill::default();
        for i in 0..10 {
            println!("{}", rng.next_f32());
        }
    }
}
