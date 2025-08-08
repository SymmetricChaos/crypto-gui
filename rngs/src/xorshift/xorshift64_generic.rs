use crate::SimpleRng;

// values for the the Star scrambler From Vigna: An experimental exploration of Marsaglia’s xorshift generators, scrambled
// in principle any odd constant other than 1 is valid, values were chosen empirically
const M32: u64 = 0x2545F4914F6CDD1D;
const M8: u64 = 0x106689D45497FDB5;
const M2: u64 = 0x74321163EEC4A005;

// This file creates an interactive (but inefficient) xorshift PRNG.
// Practical xorshift uses just a single choice of triple and matrix and hardcode it.
// See xorshift_transitions for macros that can automatically create hardcoded values.

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter, strum::Display)]
pub enum XorshiftScrambler {
    None,
    Plus,
    Star32,
    Star8,
    Star2,
    WowPlus,
    WowXor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumIter, strum::Display)]
pub enum XorshiftRule {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

impl XorshiftRule {
    pub fn rule(&self) -> &'static str {
        match self {
            XorshiftRule::A0 => "x ^= x << a;\nx ^= x >> b;\nx ^= x << c;\n",
            XorshiftRule::A1 => "x ^= x >> a;\nx ^= x << b;\nx ^= x >> c;\n",
            XorshiftRule::A2 => "x ^= x << c;\nx ^= x >> b;\nx ^= x << a;\n",
            XorshiftRule::A3 => "x ^= x >> c;\nx ^= x << b;\nx ^= x >> a;\n",
            XorshiftRule::A4 => "x ^= x << a;\nx ^= x << c;\nx ^= x >> b;\n",
            XorshiftRule::A5 => "x ^= x >> a;\nx ^= x >> c;\nx ^= x << b;\n",
            XorshiftRule::A6 => "x ^= x >> b;\nx ^= x << a;\nx ^= x << c;\n",
            XorshiftRule::A7 => "x ^= x << b;\nx ^= x >> c;\nx ^= x >> a;\n",
        }
    }
}

/// Given a valid triple from super::TRIPLES_64 perform a maximum length transition on the state.
/// Triples retain the maximum length property when reversed.
pub fn xorshift_transition(mut state: u64, triple: (u64, u64, u64), matrix: XorshiftRule) -> u64 {
    let (a, b, c) = triple;
    match matrix {
        XorshiftRule::A0 => {
            crate::xorshift_a0!(state, a, b, c);
        }
        XorshiftRule::A1 => {
            crate::xorshift_a1!(state, a, b, c);
        }
        XorshiftRule::A2 => {
            crate::xorshift_a2!(state, a, b, c);
        }
        XorshiftRule::A3 => {
            crate::xorshift_a3!(state, a, b, c);
        }
        XorshiftRule::A4 => {
            crate::xorshift_a4!(state, a, b, c);
        }
        XorshiftRule::A5 => {
            crate::xorshift_a5!(state, a, b, c);
        }
        XorshiftRule::A6 => {
            crate::xorshift_a6!(state, a, b, c);
        }
        XorshiftRule::A7 => {
            crate::xorshift_a7!(state, a, b, c);
        }
    }
    state
}

pub struct Xorshift64 {
    pub state: u64,
    pub triple: (u64, u64, u64),
    pub rule: XorshiftRule,
    pub ctr: u32,
    pub weyl: u32,
    pub scrambler: XorshiftScrambler,
    pub reverse_bits: bool,
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self {
            state: 0x139408DCBBF7A44,
            triple: (13, 7, 17),
            rule: XorshiftRule::A0,
            ctr: 0,
            weyl: 362437,
            scrambler: XorshiftScrambler::None,
            reverse_bits: false,
        }
    }
}

impl Xorshift64 {
    pub fn max_length_triple(&self) -> bool {
        let t = self.triple;
        if t.0 > 255 || t.1 > 255 || t.2 > 255 {
            return false;
        }
        super::TRIPLES_64.contains(&(t.0 as u8, t.1 as u8, t.2 as u8))
            || super::TRIPLES_64.contains(&(t.2 as u8, t.1 as u8, t.0 as u8))
    }

    pub fn step(&mut self) {
        self.state = xorshift_transition(self.state, self.triple, self.rule)
    }
}

impl SimpleRng for Xorshift64 {
    fn next_u32(&mut self) -> u32 {
        let out = match self.scrambler {
            XorshiftScrambler::None => (self.state >> 32) as u32,
            XorshiftScrambler::Plus => (self.state >> 32).wrapping_add(self.state << 32) as u32,
            XorshiftScrambler::Star32 => (self.state >> 32).wrapping_mul(M32) as u32,
            XorshiftScrambler::Star8 => (self.state >> 32).wrapping_mul(M8) as u32,
            XorshiftScrambler::Star2 => (self.state >> 32).wrapping_mul(M2) as u32,
            XorshiftScrambler::WowPlus => {
                self.ctr = self.ctr.wrapping_add(self.weyl);
                ((self.state >> 32) as u32).wrapping_add(self.ctr)
            }
            XorshiftScrambler::WowXor => {
                self.ctr = self.ctr.wrapping_add(self.weyl);
                ((self.state >> 32) as u32) ^ self.ctr
            }
        };
        self.step();
        if self.reverse_bits {
            out.reverse_bits()
        } else {
            out
        }
    }
}
