use crate::traits::SimpleRng;

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

impl SimpleRng for MiddleSquare {
    fn next_u32(&mut self) -> u32 {
        let sq = self.state * self.state;
        let digits = format!("{:0w$}", sq, w = self.width * 2);
        let mid =
            u64::from_str_radix(&digits[self.width / 2..self.width + self.width / 2], 10).unwrap();
        self.state = mid;
        mid as u32
    }
}
