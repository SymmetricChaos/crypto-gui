use crate::ClassicRng;

pub struct Squares {
    pub key: u64,
    pub ctr: u64,
}

impl Default for Squares {
    fn default() -> Self {
        Self {
            key: Default::default(),
            ctr: Default::default(),
        }
    }
}

impl Squares {}

impl ClassicRng for Squares {
    fn next_u32(&mut self) -> u32 {
        self.ctr = self.ctr.wrapping_add(1);
        let mut x = self.ctr.wrapping_mul(self.key);
        let y = x;
        let z = y.wrapping_add(self.key);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        (x.wrapping_mul(x).wrapping_add(z) >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.ctr = self.ctr.wrapping_add(1);
        let mut x = self.ctr.wrapping_mul(self.key);
        let y = x;
        let z = y.wrapping_add(self.key);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        let t = x;
        x = x.rotate_right(32);
        t ^ (x.wrapping_mul(x).wrapping_add(y) >> 32)
    }
}
