use crate::ClassicRng;

pub struct Kiss {
    z: u32,
    w: u32,
    jsr: u32,
    jcong: u32,
}

impl Default for Kiss {
    fn default() -> Self {
        Self {
            z: 1,
            w: 1,
            jsr: 1,
            jcong: 1,
        }
    }
}

impl Kiss {
    fn wnew(&mut self) {
        self.w = 36969_u32
            .wrapping_mul(self.w & 0xffff)
            .wrapping_add(self.w >> 16)
    }

    fn znew(&mut self) {
        self.z = 18000_u32
            .wrapping_mul(self.z & 0xffff)
            .wrapping_add(self.z >> 16)
    }

    fn mwc(&mut self) -> u32 {
        self.wnew();
        self.znew();
        (self.z << 16).wrapping_add(self.w)
    }

    fn shr3(&mut self) -> u32 {
        self.jsr ^= self.jsr << 17;
        self.jsr ^= self.jsr >> 13;
        self.jsr ^= self.jsr << 5;
        self.jsr
    }

    fn cong(&mut self) -> u32 {
        self.jcong = 69069_u32.wrapping_mul(self.jcong).wrapping_add(1234567);
        self.jcong
    }
}

impl ClassicRng for Kiss {
    fn next_u32(&mut self) -> u32 {
        (self.mwc() ^ self.cong()).wrapping_add(self.shr3())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs() {
        let mut rng = Kiss::default();
        for _ in 0..10 {
            println!("{:08x?}", rng.next_u32());
        }
    }
}
