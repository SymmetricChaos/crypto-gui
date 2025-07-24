use crate::ClassicRng;

// pub struct Threefry32_2 {}

// impl Threefry32_2 {}

// pub struct Threefry32_4 {}

// impl Threefry32_4 {}

// pub struct Threefry64_2 {}

// impl Threefry64_2 {}

pub struct Threefry64_4 {
    rounds: usize,
    ctr: [u64; 4],
    key: [u64; 4],
}

impl Default for Threefry64_4 {
    fn default() -> Self {
        Self {
            rounds: 20,
            ctr: [0; 4],
            key: [0; 4],
        }
    }
}

impl Threefry64_4 {
    pub fn array(&self) -> [u64; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key = [0; 4 + 1];
        ex_key[4] = super::C240;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        for i in 0..self.rounds {}
        todo!()
    }
}

impl ClassicRng for Threefry64_4 {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}
