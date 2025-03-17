use num::Integer;

use crate::ClassicRng;

pub enum WolframCode {
    R30,
    R86,
    R135,
    R149,
}

fn rule30(triple: &[bool]) -> bool {
    match triple {
        [true, true, true] => false,
        [true, true, false] => false,
        [true, false, true] => false,
        [true, false, false] => true,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
        _ => unreachable!("slice did not have exactly three elements"),
    }
}

pub struct Rule30 {
    pub rule: WolframCode,
    pub state: [bool; 64],
    pub tap: usize,
}

impl Rule30 {
    pub fn from_u64(seed: u64) -> Self {
        let mut state = [false; 64];
        for i in 0..64 {
            if (seed >> i).is_odd() {
                state[63 - i] = true
            }
        }
        Self {
            rule: WolframCode::R30,
            state,
            tap: 63,
        }
    }

    pub fn step(&mut self) {
        let mut new_state = [false; 64];
        for (i, triple) in self.state.windows(3).enumerate() {
            new_state[i + 1] = rule30(triple)
        }
        new_state[0] = rule30(&[self.state[63], self.state[0], self.state[1]]);
        new_state[63] = rule30(&[self.state[62], self.state[63], self.state[0]]);
        self.state = new_state;
    }

    pub fn print_state(&self, zero: char, one: char) -> String {
        self.state
            .clone()
            .map(|b| if b == true { one } else { zero })
            .iter()
            .collect::<String>()
    }
}

impl ClassicRng for Rule30 {
    fn next_u32(&mut self) -> u32 {
        let mut n = 0;
        for _ in 0..32 {
            n <<= 1;
            n |= self.state[self.tap] as u32;
            self.step();
        }
        n
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore = "visual test"]
    #[test]
    fn visual_test_of_state() {
        let mut rng = Rule30::from_u64(1 + 2 + 256);
        for _ in 0..20 {
            println!("{}", rng.print_state(' ', '#'));
            rng.step();
        }
    }
}
