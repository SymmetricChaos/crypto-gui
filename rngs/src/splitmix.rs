use crate::traits::ClassicRng;

pub struct Splitmix {
    pub state: u64,
}

impl Default for Splitmix {
    fn default() -> Self {
        Self { state: 0 }
    }
}

impl Splitmix {
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut t = self.state;
        t ^= t >> 30;
        t = t.wrapping_mul(0xbf58476d1ce4e5b9);
        t ^= t >> 27;
        t = t.wrapping_mul(0x94d049bb133111eb);
        t ^ (t >> 31)
    }
}

impl ClassicRng for Splitmix {
    // Only the lower 32 bits are used
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

// #[cfg(test)]
// mod splitmix_tests {
//     use super::*;

//     // #[test]
//     // fn first_five() {
//     //     let mut rng = Splitmix::default();
//     //     rng.state = 1234567;
//     //     for _ in 0..5 {
//     //         println!("{}", rng.next_u64())
//     //     }
//     // }
// }
