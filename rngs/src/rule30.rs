use crate::SimpleRng;
use num::Integer;

const W: usize = 128;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::EnumIter, strum::Display)]
pub enum WolframCode {
    #[strum(to_string = "Rule 30")]
    R30,
    #[strum(to_string = "Rule 86")]
    R86,
    #[strum(to_string = "Rule 135")]
    R135,
    #[strum(to_string = "Rule 149")]
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

fn rule86(triple: &[bool]) -> bool {
    match triple {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => false,
        [true, false, false] => true,
        [false, true, true] => false,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
        _ => unreachable!("slice did not have exactly three elements"),
    }
}

fn rule135(triple: &[bool]) -> bool {
    match triple {
        [true, true, true] => true,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => false,
        [false, true, false] => false,
        [false, false, true] => false,
        [false, false, false] => true,
        _ => unreachable!("slice did not have exactly three elements"),
    }
}

fn rule149(triple: &[bool]) -> bool {
    match triple {
        [true, true, true] => true,
        [true, true, false] => false,
        [true, false, true] => false,
        [true, false, false] => true,
        [false, true, true] => false,
        [false, true, false] => true,
        [false, false, true] => false,
        [false, false, false] => true,
        _ => unreachable!("slice did not have exactly three elements"),
    }
}

pub struct Rule30 {
    pub rule: WolframCode,
    pub state: [bool; W],
    pub tap: usize,
}

impl Default for Rule30 {
    fn default() -> Self {
        Self::init_30(12345, W - 1)
    }
}

impl Rule30 {
    pub fn init(seed: u64, rule: WolframCode, tap: usize) -> Self {
        let mut state = [false; W];
        for i in 0..64 {
            if (seed >> i).is_odd() {
                state[W - 1 - i] = true
            }
        }
        Self { rule, state, tap }
    }

    pub fn init_30(seed: u64, tap: usize) -> Self {
        Self::init(seed, WolframCode::R30, tap)
    }

    pub fn init_86(seed: u64, tap: usize) -> Self {
        Self::init(seed, WolframCode::R86, tap)
    }

    pub fn init_135(seed: u64, tap: usize) -> Self {
        Self::init(seed, WolframCode::R135, tap)
    }

    pub fn init_149(seed: u64, tap: usize) -> Self {
        Self::init(seed, WolframCode::R149, tap)
    }

    pub fn step(&mut self) {
        let rule = match self.rule {
            WolframCode::R30 => rule30,
            WolframCode::R86 => rule86,
            WolframCode::R135 => rule135,
            WolframCode::R149 => rule149,
        };
        let mut new_state = [false; W];
        for (i, triple) in self.state.windows(3).enumerate() {
            new_state[i + 1] = rule(triple)
        }
        new_state[0] = rule(&[self.state[W - 1], self.state[0], self.state[1]]);
        new_state[W - 1] = rule(&[self.state[W - 2], self.state[W - 1], self.state[0]]);
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

impl SimpleRng for Rule30 {
    fn next_u32(&mut self) -> u32 {
        let mut n = 0;
        for _ in 0..32 {
            n <<= 1;
            n |= self.state[self.tap] as u32;
            self.step();
        }
        n
    }

    fn next_u64(&mut self) -> u64 {
        let mut n = 0;
        for _ in 0..64 {
            n <<= 1;
            n |= self.state[self.tap] as u64;
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
        let mut rng = Rule30::init_86(1 << 40, 127);
        for _ in 0..30 {
            println!("{}", rng.print_state(' ', '#'));
            rng.step();
        }
    }
}
