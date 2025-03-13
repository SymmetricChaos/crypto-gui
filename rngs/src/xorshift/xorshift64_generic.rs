use crate::ClassicRng;

// From Vigna: An experimental exploration of Marsaglia’s xorshift generators, scrambled

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
    Star,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumIter, strum::Display)]
pub enum XorshiftMatrix {
    LRL,
    RLR,
    LLR,
    RRL,
}

/// Given a valid triple from super::TRIPLES_64 perform a maximum length transition on the state.
/// Triples retain the maximum length property when reversed.
pub fn xorshift_transition(mut state: u64, triple: (u64, u64, u64), matrix: XorshiftMatrix) -> u64 {
    let (a, b, c) = triple;
    match matrix {
        XorshiftMatrix::LRL => {
            crate::xorshift_lrl!(state, a, b, c);
        }
        XorshiftMatrix::RLR => {
            crate::xorshift_rlr!(state, a, b, c);
        }
        XorshiftMatrix::LLR => {
            crate::xorshift_llr!(state, a, b, c);
        }
        XorshiftMatrix::RRL => {
            crate::xorshift_rrl!(state, a, b, c);
        }
    }
    state
}

pub struct Xorshift64 {
    pub state: u64,
    pub triple: (u64, u64, u64),
    pub matrix: XorshiftMatrix,
    pub scrambler: XorshiftScrambler,
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self {
            state: 0x139408DCBBF7A44,
            triple: (13, 7, 17),
            matrix: XorshiftMatrix::LRL,
            scrambler: XorshiftScrambler::None,
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
        self.state = xorshift_transition(self.state, self.triple, self.matrix)
    }
}

impl ClassicRng for Xorshift64 {
    fn next_u32(&mut self) -> u32 {
        let out = match self.scrambler {
            XorshiftScrambler::None => (self.state >> 32) as u32,
            XorshiftScrambler::Plus => (self.state >> 32).wrapping_add(self.state << 32) as u32,
            XorshiftScrambler::Star => (self.state >> 32).wrapping_mul(M32) as u32, // in principle any odd constant other than 1 is valid here, value chosen empirically by Vigna
        };
        self.step();
        out
    }
}
