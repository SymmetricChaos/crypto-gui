use crate::{errors::RngError, SimpleRng};

pub struct MRG32k3a {
    state: [u32; 6],
}

impl MRG32k3a {
    const M1: u32 = 4294967087; // 2^32 - 209
    const M2: u32 = 4294944443; // 2^32 - 22853
    const A12: u32 = 1403580;
    const A13N: u32 = 810728;
    const A21: u32 = 527612;
    const A23N: u32 = 1370589;

    pub fn new(seed: [u32; 6]) -> Result<Self, RngError> {
        if seed[0] >= Self::M1 || seed[1] >= Self::M1 || seed[2] >= Self::M1 {
            return Err(RngError::general("invalid seed value"));
        };
        if seed[3] >= Self::M2 || seed[4] >= Self::M2 || seed[5] >= Self::M2 {
            return Err(RngError::general("invalid seed value"));
        };
        Ok(Self { state: seed })
    }
}

impl SimpleRng for MRG32k3a {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}

pub struct MRG63k3a {
    state: [i64; 6],
}

impl MRG63k3a {
    const M1: i64 = 9223372036854769163; // 2^63 - 6645
    const M2: i64 = 9223372036854754679; // 2^63 - 21129
    const A12: i64 = 1754669720;
    const Q12: i64 = 5256471877;
    const R12: i64 = 251304723;
    const A13N: i64 = 3182104042;
    const Q13: i64 = 2898513661;
    const R13: i64 = 394451401;
    const A21: i64 = 31387477935;
    const Q21: i64 = 293855150;
    const R21: i64 = 143639429;
    const A23N: i64 = 6199136374;
    const Q23: i64 = 1487847900;
    const R23: i64 = 985240079;

    pub fn new(seed: [i64; 6]) -> Result<Self, RngError> {
        if seed[0] >= Self::M1 || seed[1] >= Self::M1 || seed[2] >= Self::M1 {
            return Err(RngError::general(
                "seed values 0..3 must be less than 9223372036854769163 (2^63 - 6645)",
            ));
        };
        if seed[3] >= Self::M2 || seed[4] >= Self::M2 || seed[5] >= Self::M2 {
            return Err(RngError::general(
                "seed values 0..3 must be less than 9223372036854754679 (2^63 - 21129)",
            ));
        };
        Ok(Self { state: seed })
    }
}

impl SimpleRng for MRG63k3a {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let h = self.state[0] / Self::Q13;
        let mut p13 = Self::A13N * (self.state[0] - h * Self::Q13) - h * Self::R13;
        let h = self.state[1] / Self::Q12;
        let mut p12 = Self::A12 * (self.state[1] - h * Self::Q12) - h * Self::R12;

        if p13 < 0 {
            p13 += Self::M1;
        }
        if p12 < 0 {
            p12 += Self::M1 - p13;
        } else {
            p12 -= p13;
        }
        if p12 < 0 {
            p12 += Self::M1
        }

        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = p12;

        let h = self.state[3] / Self::Q23;
        let mut p23 = Self::A23N * (self.state[3] - h * Self::Q23) - h * Self::R23;
        let h = self.state[4] / Self::Q21;
        let mut p21 = Self::A21 * (self.state[4] - h * Self::Q21) - h * Self::R21;

        if p23 < 0 {
            p23 += Self::M2;
        }
        if p21 < 0 {
            p21 += Self::M2 - p23;
        } else {
            p21 -= p23;
        }
        if p21 < 0 {
            p21 += Self::M2
        }

        self.state[3] = self.state[4];
        self.state[4] = self.state[5];
        self.state[5] = p21;

        // converting i64 to u64 keeps identical bits
        if p12 > p21 {
            (p12 - p21) as u64
        } else {
            (p12 - p21 + Self::M1) as u64
        }
    }
}
