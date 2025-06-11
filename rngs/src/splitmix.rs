use crate::traits::ClassicRng;

fn mix64(mut t: u64) -> u64 {
    t ^= t >> 33;
    t = t.wrapping_mul(0xbf58476d1ce4e5b9);
    t ^= t >> 33;
    t = t.wrapping_mul(0x94d049bb133111eb);
    t ^ (t >> 33)
}

// This seem to be the most widely used mixing function
fn mix64_variant_13(mut t: u64) -> u64 {
    t ^= t >> 30;
    t = t.wrapping_mul(0xbf58476d1ce4e5b9);
    t ^= t >> 27;
    t = t.wrapping_mul(0x94d049bb133111eb);
    t ^ (t >> 31)
}

fn mix_gamma(mut t: u64) -> u64 {
    t = mix64_variant_13(t) | 1;
    let a = (t ^ (t >> 1)).count_ones();
    if a >= 24 {
        t
    } else {
        !t
    }
}

pub struct Splitmix {
    pub gamma: u64,
    pub state: u64,
}

impl Default for Splitmix {
    fn default() -> Self {
        Self {
            gamma: 0x9e3779b97f4a7c15,
            state: 0,
        }
    }
}

impl Splitmix {
    fn next_state(&mut self) -> u64 {
        self.state = self.state.wrapping_add(self.gamma);
        self.state
    }

    pub fn split(&mut self) -> Splitmix {
        let new_state = mix64(self.next_state());
        let new_gamma = mix_gamma(self.next_state());
        Splitmix {
            gamma: new_gamma,
            state: new_state,
        }
    }
}

impl ClassicRng for Splitmix {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next_state();
        mix64_variant_13(self.state)
    }
}

#[cfg(test)]
mod tests {
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
