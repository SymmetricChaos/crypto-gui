use crate::ClassicRng;

pub struct WyRand {
    state: u64,
}

impl ClassicRng for WyRand {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0xa0761d6478bd642f);
        let t = (self.state as u128).wrapping_mul((self.state ^ 0xe7037ed1a0b428db) as u128);
        (t.wrapping_shr(64) ^ t) as u64
    }
}
