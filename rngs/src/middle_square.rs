use crate::traits::ClassicRng;

pub struct MiddleSquare {
    pub state: u64,
    pub width: usize,
}

impl Default for MiddleSquare {
    fn default() -> Self {
        Self {
            state: 123456,
            width: 6,
        }
    }
}

impl ClassicRng for MiddleSquare {
    fn step(&mut self) {
        let sq = self.state * self.state;
        let digits = format!("{:0w$}", sq, w = self.width);
        self.state =
            u64::from_str_radix(&digits[self.width / 2..self.width + self.width / 2], 10).unwrap();
    }
}
