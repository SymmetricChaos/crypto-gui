use crate::traits::StatefulHasher;

pub struct Pjw {
    state: u32,
}

impl StatefulHasher for Pjw {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = (self.state << 4).wrapping_add(*byte as u32);
            let g = self.state & 0xf0000000;
            if g != 0 {
                self.state &= g >> 24;
                self.state ^= g;
            }
        }
    }

    fn finalize(self) -> Vec<u8> {
        self.state.to_be_bytes()[1..].to_vec()
    }
}
