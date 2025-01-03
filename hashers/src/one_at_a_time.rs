use crate::traits::StatefulHasher;
use std::num::Wrapping;

#[derive(Debug, Clone)]
pub struct OneAtATime {
    state: Wrapping<u32>,
}

impl OneAtATime {
    pub fn init() -> Self {
        Self { state: Wrapping(0) }
    }
}

impl StatefulHasher for OneAtATime {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes.into_iter() {
            self.state += *byte as u32;
            self.state += self.state << 10;
            self.state ^= self.state >> 6;
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        self.state += self.state << 3;
        self.state ^= self.state >> 11;
        self.state += self.state << 15;
        self.state.0.to_be_bytes().to_vec()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1, OneAtATime::init(), b"a", "ca2e9442";
    test2, OneAtATime::init(), b"The quick brown fox jumps over the lazy dog", "519e91f5";
);
