use crate::traits::ClassicRng;

#[derive(Debug, PartialEq, Eq)]
pub enum PcgTransform {
    Rs,
    Rr,
    XshRr,
    XshRs,
}

impl PcgTransform {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rs => "Random Shift",
            Self::Rr => "Random Rotation",
            Self::XshRr => "Xorshift w/ Random Rotation",
            Self::XshRs => "Xorshift w/ Random Shift",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Rs => {
                "Take the top three bits as a binary number 'n'. Shift the bits of the state to the right 29 - n places."
            }
            Self::Rr => {
                "Take the top three bits as a binary number 'n'. Rotate the bits of the state to the right 29 - n places."
            }
            Self::XshRr => "u64::rotate_right((n ^ (n >> 18)) >> 27, (n >> 59) as u32)",
            Self::XshRs => " (the bits XORed with (the bits shifted right 22 places)) shifted right (22 places plus (the bits shifted right 61 places))",
        }
    }

    pub fn transform(&self, n: u64) -> u32 {
        (match self {
            Self::Rs => n >> (29 - (n >> 61)),
            Self::Rr => n.rotate_right(29 - (n >> 61) as u32),
            Self::XshRr => u64::rotate_right((n ^ (n >> 18)) >> 27, (n >> 59) as u32),
            Self::XshRs => (n ^ (n >> 22)) >> (22 + (n >> 61)),
        }) as u32 // truncate the transformed u64 to a u32
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

impl Pcg {
    pub fn transform(&self, n: u64) -> u32 {
        self.transform.transform(n)
    }
}

impl ClassicRng for Pcg {
    fn step(&mut self) -> u32 {
        self.state = (self.state)
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
        self.transform(self.state)
    }
}
