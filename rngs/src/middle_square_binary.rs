use crate::traits::ClassicRng;

#[derive(Debug, PartialEq, Eq)]
pub enum MSBSize {
    B64,
    B32,
    B16,
    B8,
}

impl MSBSize {
    pub fn mask(&self) -> u128 {
        match self {
            MSBSize::B64 => 0xFFFFFFFFFFFFFFFF,
            MSBSize::B32 => 0xFFFFFFFF,
            MSBSize::B16 => 0xFFFF,
            MSBSize::B8 => 0xFF,
        }
    }

    pub fn quarter_size(&self) -> usize {
        self.size() / 4
    }

    pub fn half_size(&self) -> usize {
        self.size() / 2
    }

    pub fn size(&self) -> usize {
        match self {
            MSBSize::B64 => 64,
            MSBSize::B32 => 32,
            MSBSize::B16 => 16,
            MSBSize::B8 => 8,
        }
    }
}

pub struct MiddleSquareBinary {
    pub width: MSBSize,
    pub state: u128,
}

impl Default for MiddleSquareBinary {
    fn default() -> Self {
        Self {
            width: MSBSize::B32,
            state: 255,
        }
    }
}

impl MiddleSquareBinary {
    pub fn step(&mut self) {
        let sq = self.state * self.state;
        self.state = (sq >> self.width.half_size()) & self.width.mask();
    }

    /// Nonadvancing version of next
    pub fn peek_next(&self) -> u128 {
        let sq = self.state * self.state;
        (sq >> self.width.half_size()) & self.width.mask()
    }
}

impl ClassicRng for MiddleSquareBinary {
    fn next_u32(&mut self) -> u32 {
        let mut out = 0;
        match self.width {
            MSBSize::B64 => {
                self.step();
                out = self.state as u32;
            }
            MSBSize::B32 => {
                self.step();
                out = self.state as u32;
            }
            MSBSize::B16 => {
                self.step();
                out <<= 16;
                out |= self.state as u32;
                self.step();
                out <<= 16;
                out |= self.state as u32;
            }
            MSBSize::B8 => {
                self.step();
                out <<= 8;
                out |= self.state as u32;
                self.step();
                out <<= 8;
                out |= self.state as u32;
                self.step();
                out <<= 8;
                out |= self.state as u32;
                self.step();
                out <<= 8;
                out |= self.state as u32;
            }
        }
        out
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = 0;
        match self.width {
            MSBSize::B64 => {
                self.step();
                out = self.state as u64;
            }
            MSBSize::B32 => {
                self.step();
                out |= self.state as u64;
                self.step();
                out <<= 32;
                out |= self.state as u64;
            }
            MSBSize::B16 => {
                self.step();
                out <<= 16;
                out |= self.state as u64;
                self.step();
                out <<= 16;
                out |= self.state as u64;
                self.step();
                out <<= 16;
                out |= self.state as u64;
                self.step();
                out <<= 16;
                out |= self.state as u64;
            }
            MSBSize::B8 => {
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
                self.step();
                out <<= 8;
                out |= self.state as u64;
            }
        }
        out
    }
}
