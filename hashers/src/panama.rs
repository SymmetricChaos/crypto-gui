use crate::traits::StatefulHasher;

pub struct Panama {
    state: [u32; 17],
    buffer: Vec<u8>,
}

impl Default for Panama {
    fn default() -> Self {
        Self {
            state: [0; 17],
            buffer: Vec::new(),
        }
    }
}

impl Panama {
    fn state_update(&mut self) {}
}

impl StatefulHasher for Panama {
    fn update(&mut self, mut bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}
