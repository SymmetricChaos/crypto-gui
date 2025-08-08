use crate::traits::SimpleRng;
use strum::{Display, EnumIter};

#[derive(Debug, PartialEq, Eq, EnumIter, Display, Clone, Copy)]
pub enum PcgTransform {
    #[strum(to_string = "Random Shift")]
    Rs,
    #[strum(to_string = "Random Rotation")]
    Rr,
    #[strum(to_string = "Xorshift w/ Random Rotation")]
    XshRr,
    #[strum(to_string = "Xorshift w/ Random Shift")]
    XshRs,
}

impl PcgTransform {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Rs => "n >> (29 - (n >> 61))",
            Self::Rr => "n >>> (29 - (n >> 61))",
            Self::XshRr => "((n ^ (n >> 18)) >> 27) >>> (n >> 59)",
            Self::XshRs => " (n ^ (n >> 22)) >> (22 + (n >> 61))",
        }
    }

    pub fn transform(&self, n: u64) -> u64 {
        match self {
            Self::Rs => n >> (29 - (n >> 61)),
            Self::Rr => n.rotate_right(29 - (n >> 61) as u32),
            Self::XshRr => u64::rotate_right((n ^ (n >> 18)) >> 27, (n >> 59) as u32),
            Self::XshRs => (n ^ (n >> 22)) >> (22 + (n >> 61)),
        }
    }
}

pub struct Pcg {
    pub state: u64,
    pub multiplier: u64,
    pub increment: u64,
    pub transform: PcgTransform,
}

impl Default for Pcg {
    fn default() -> Self {
        Self {
            state: 1257924810,
            multiplier: 1664525,
            increment: 1013904223,
            transform: PcgTransform::Rr,
        }
    }
}

impl SimpleRng for Pcg {
    fn next_u64(&mut self) -> u64 {
        self.state = (self.state)
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
        self.transform.transform(self.state)
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn sequence() {
//         let mut rng = Pcg::default();
//     }
// }
