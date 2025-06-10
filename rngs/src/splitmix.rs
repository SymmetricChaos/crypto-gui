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
    fn split(self) -> (Splitmix, Splitmix) {
        todo!()
    }
}

impl ClassicRng for Splitmix {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut t = self.state;
        t ^= t >> 30;
        t = t.wrapping_mul(0xbf58476d1ce4e5b9);
        t ^= t >> 27;
        t = t.wrapping_mul(0x94d049bb133111eb);
        t ^ (t >> 31)
    }
}

#[cfg(test)]
mod splitmix_tests {
    use super::*;

    #[test]
    fn first_five() {
        let mut rng = Splitmix::default();
        let ints: [u64; 5] = [
            6457827717110365317,
            3203168211198807973,
            9817491932198370423,
            4593380528125082431,
            16408922859458223821,
        ];
        rng.state = 1234567;
        for int in ints {
            assert_eq!(int, rng.next_u64());
        }
    }
}
