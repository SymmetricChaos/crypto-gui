use crate::{lcg::LcgM, traits::SimpleRng};

pub struct ClcgM {
    pub lcgs: Vec<LcgM>,
}

impl Default for ClcgM {
    fn default() -> Self {
        Self {
            lcgs: vec![LcgM::default()],
        }
    }
}

impl SimpleRng for ClcgM {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        let modulus = (self.lcgs[0].modulus - 1) as u64;
        for lcg in self.lcgs.iter_mut() {
            out = (out + lcg.next_u32() as u64) % modulus;
        }
        out as u32
    }
}
