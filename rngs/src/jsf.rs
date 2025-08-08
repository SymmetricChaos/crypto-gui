use crate::traits::SimpleRng;

pub struct Jsf32 {
    pub state: [u32; 4],
}

impl Default for Jsf32 {
    fn default() -> Self {
        Self {
            state: [0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f],
        }
    }
}

impl Jsf32 {
    pub fn step(&mut self) {
        let s = &mut self.state;
        let e = s[0].wrapping_sub(s[1].rotate_left(27));
        s[0] = s[1] ^ s[2].rotate_left(17);
        s[1] = s[2].wrapping_add(s[3]);
        s[2] = s[3].wrapping_add(e);
        s[3] = e.wrapping_add(s[0])
    }
}

impl SimpleRng for Jsf32 {
    fn next_u32(&mut self) -> u32 {
        self.step();
        self.state[3]
    }
}

pub struct Jsf64 {
    pub state: [u64; 4],
}

impl Default for Jsf64 {
    fn default() -> Self {
        Self {
            state: [
                0x1011121300010203,
                0x1415161704050607,
                0x18191a1b08090a0b,
                0x1c1d1e1f0c0d0e0f,
            ],
        }
    }
}

impl Jsf64 {
    pub fn step(&mut self) {
        let s = &mut self.state;
        let e = s[0].wrapping_sub(s[1].rotate_left(7));
        s[0] = s[1] ^ s[2].rotate_left(13);
        s[1] = s[2].wrapping_add(s[3].rotate_left(37));
        s[2] = s[3].wrapping_add(e);
        s[3] = e.wrapping_add(s[0])
    }
}

impl SimpleRng for Jsf64 {
    fn next_u32(&mut self) -> u32 {
        self.step();
        self.state[3] as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.step();
        self.state[3]
    }
}

#[cfg(test)]
mod jsf_tests {
    use super::*;

    #[test]
    fn fixed_point_test() {
        let mut rng = Jsf32::default();
        // Three of the known fixed point seeds for JSF32
        for bad_seed in [
            [0x77777777, 0x55555555, 0x11111111, 0x44444444],
            [0x5591F2E3, 0x69EBA6CD, 0x2A171E3D, 0x3FD48890],
            [0x47CB8D56, 0xAE9B35A7, 0x5C78F4A8, 0x522240FF],
        ] {
            rng.state = bad_seed;
            rng.step();
            assert_eq!(rng.state, bad_seed);
        }
    }
}
