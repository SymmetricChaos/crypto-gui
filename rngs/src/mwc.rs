use crate::SimpleRng;

const A1: u128 = 0xffeb_b71d_94fc_daf9;
// const MOD: u128 = (A1 - 1) << 64 | 0xffff_ffff_ffff_ffff;
// const JUMP: u128 = 0xa72f_9a35_4720_8003_2f65_fed2_e840_0983;
// const LONG_JUMP: u128 = 0xe6f7_8144_67f3_fcdd_3946_49cf_d676_9c91;

pub struct MultiplyWithCarry128 {
    x: u128,
    c: u128,
}

impl Default for MultiplyWithCarry128 {
    fn default() -> Self {
        Self::from_u64(0x0BAD_5EED_0BAD_5EED)
    }
}

impl MultiplyWithCarry128 {
    // pub fn jump(&mut self) {
    //     let s = self.x << 64 + self.c;
    //     let t = s.wrapping_mul(JUMP) % MOD;
    //     self.x = t >> 64;
    //     self.c = t & 0xffff_ffff_ffff_ffff;
    // }

    // pub fn long_jump(&mut self) {
    //     let s = self.x << 64 + self.c;
    //     let t = s.wrapping_mul(LONG_JUMP) % MOD;
    //     self.x = t >> 64;
    //     self.c = t & 0xffff_ffff_ffff_ffff;
    // }

    pub fn step(&mut self) {
        let t = A1.wrapping_mul(self.x).wrapping_add(self.c);
        self.x = t & 0xffff_ffff_ffff_ffff;
        self.c = t >> 64;
    }
}

impl MultiplyWithCarry128 {
    pub fn from_u64(seed: u64) -> Self {
        Self {
            x: seed as u128,
            c: 1,
        }
    }
}

impl SimpleRng for MultiplyWithCarry128 {
    fn next_u64(&mut self) -> u64 {
        let out = self.x;
        self.step();
        out as u64
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outputs() {
        let mut rng = MultiplyWithCarry128::from_u64(0x0BAD5EED0BAD5EED);
        let mut v = Vec::new();
        for _ in 0..10 {
            v.push(rng.next_u64());
        }
        assert_eq!(
            vec![
                0x0BAD5EED0BAD5EED,
                0xF29D15E573C32686,
                0x748E105DB60CF5C4,
                0x4E393AC554698EB6,
                0x85526AEE1557EA7C,
                0x1887D1ED977F5D94,
                0x93CC86E1206FDA71,
                0xA83AE51E73C8014C,
                0x7C68AD467E069B9B,
                0x5C106F873028C933,
            ],
            v
        );
    }

    // #[test]
    // fn test_jump() {
    //     let mut rng = MultiplyWithCarry128::from_u64(0x0BAD5EED0BAD5EED);
    //     rng.jump();
    //     assert!(rng.x == 6805729132081258701);
    //     assert!(rng.c == 18339831245484978469);
    // }
}
