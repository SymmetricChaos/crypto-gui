use crate::ClassicRng;

pub struct Ars {
    pub ctr: [u32; 4],
    pub key: [u32; 4],
    pub rounds: usize,
    saved: [u32; 4],
    idx: usize,
}

impl Default for Ars {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            rounds: 7,
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Ars {}

impl ClassicRng for Ars {
    fn next_u32(&mut self) -> u32 {
        if self.rounds >= 1 {}
        if self.rounds >= 2 {}
        if self.rounds >= 3 {}
        if self.rounds >= 4 {}
        if self.rounds >= 5 {}
        if self.rounds >= 6 {}
        if self.rounds >= 7 {}
        if self.rounds >= 8 {}
        if self.rounds >= 9 {}
        if self.rounds >= 10 {}
        todo!()
    }
}
