use crate::ClassicRng;

const A1: u128 = 0xffeb_b71d_94fc_daf9;
const MOD: u128 = (0xffff_ffff_ffff_ffff << 64) | (A1 - 1);
const JUMP: u128 = 0xa72f_9a35_4720_8003_2f65_fed2_e840_0983;
const LONG_JUMP: u128 = 0xe6f7_8144_67f3_fcdd_3946_49cf_d676_9c91;

pub struct MultiplyWithCarry128 {
    x: u128,
    c: u128,
}

impl Default for MultiplyWithCarry128 {
    fn default() -> Self {
        Self {
            x: 0x0BAD_5EED_0BAD_5EED,
            c: 1,
        }
    }
}

impl MultiplyWithCarry128 {
    pub fn jump(&mut self) {
        let s = self.x << 64 + self.c;
        let t = s.wrapping_mul(JUMP) % MOD;
        self.x = t >> 64;
        self.c = t & 0xffff_ffff_ffff_ffff;
    }

    pub fn long(&mut self) {
        let s = self.x << 64 + self.c;
        let t = s.wrapping_mul(LONG_JUMP) % MOD;
        self.x = t >> 64;
        self.c = t & 0xffff_ffff_ffff_ffff;
    }

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

impl ClassicRng for MultiplyWithCarry128 {
    fn next_u32(&mut self) -> u32 {
        let out = self.x;
        self.step();
        out as u32
    }

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
                841433077584322285,
                17482153403755996806,
                8398668349764859332,
                5636601027830058678,
                9606858525927533180,
                1767612197145370004,
                10650035520294345329,
                12122253265975378252,
                8964605576554650523,
                6633924877535988019,
            ],
            v
        );
    }

    #[test]
    fn test_jump() {
        let mut rng = MultiplyWithCarry128::from_u64(0x0BAD5EED0BAD5EED);
        rng.jump();
        println!("{} {}", rng.x, rng.c)
    }
}
