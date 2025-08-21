use crate::traits::StatefulHasher;

// Algorithms in C, Robert Sedgwicks

const B: u32 = 378551;

pub struct Rs {
    state: u32,
    a: u32,
}

impl StatefulHasher for Rs {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = self.state.wrapping_mul(self.a).wrapping_add(*byte as u32);
            self.a = self.a.wrapping_mul(B);
        }
    }

    fn finalize(self) -> Vec<u8> {
        self.state.to_be_bytes().to_vec()
    }
}
