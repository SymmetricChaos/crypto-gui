use crate::traits::StatefulHasher;

pub struct Grostl {
    buffer: Vec<u8>,
}

impl StatefulHasher for Grostl {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}
