// https://eprint.iacr.org/2019/458.pdf

use crate::traits::StatefulHasher;

pub struct Poseidon {
    buffer: Vec<u8>,
}

impl Poseidon {
    pub fn init() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl StatefulHasher for Poseidon {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    
}
