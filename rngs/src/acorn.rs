use crate::SimpleRng;

pub struct Acorn {
    state: Vec<u64>,
}

impl Default for Acorn {
    fn default() -> Self {
        Self {
            state: vec![
                0xE907505CB59C77D1,
                0x8D6CA05F581DA875,
                0x66F0C7ABEDA8D656,
                0xEA0FF0093C2D3AB4,
                0x48DE7C3E7FA5F645,
                0xDF816A1E4941B30E,
                0x28FF5C18C591C8B2,
                0xD86AB519D00D93E1,
                0x7D1A153C980DEA8A,
                0xAFF20955CB3FFC46,
                0xA63707C164CD19F4,
                0x953458E388B9020C,
                0xCFCA87EBA0626046,
                0x1E9AF6B90B7EBF91,
                0xE86BDCAAE59D8EDA,
                0xD0B78E2AD9B6C63D,
            ],
        }
    }
}

impl Acorn {
    pub fn new(state: Vec<u64>) -> Self {
        assert!(state[0] % 2 == 1, "the first state value must be odd");
        Self { state }
    }
}

impl SimpleRng for Acorn {
    fn next_u32(&mut self) -> u32 {
        for i in 1..self.state.len() {
            self.state[i] = self.state[i].wrapping_add(self.state[i - 1]);
        }
        (self.state.last().unwrap() >> 32) as u32
    }

    // This may be poor quality of the lower bits
    // fn next_u64(&mut self) -> u64 {
    //     for i in 1..self.state.len() {
    //         self.state[i] = self.state[i].wrapping_add(self.state[i - 1]);
    //     }
    //     *self.state.last().unwrap()
    // }
}
