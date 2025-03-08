use super::XorshiftScrambler;
use crate::traits::ClassicRng;

pub struct Xorshift32 {
    pub state: u32,
    pub scrambler: XorshiftScrambler,
}

impl Default for Xorshift32 {
    fn default() -> Self {
        Self {
            state: 0x0BAD_5EED0,
            scrambler: XorshiftScrambler::None,
        }
    }
}

impl ClassicRng for Xorshift32 {
    fn next_u32(&mut self) -> u32 {
        crate::xorshift_lrl_inv!(self.state, 5, 17, 13);
        match self.scrambler {
            XorshiftScrambler::None => self.state,
            _ => todo!(),
        }
    }
}
