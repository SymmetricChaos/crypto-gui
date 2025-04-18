use crate::traits::StatefulHasher;

pub struct Skein1024 {
    pub key: [u64; Self::WORDS],
    pub tweak: [u64; 2],
}

impl Default for Skein1024 {
    fn default() -> Self {
        Self {
            key: [0; Self::WORDS],
            tweak: [0; 2],
        }
    }
}

impl Skein1024 {
    const WORDS: usize = 16;
    const ROUNDS: usize = 80;

    pub fn init128() -> Self {
        todo!()
    }

    pub fn init160() -> Self {
        todo!()
    }

    pub fn init224() -> Self {
        todo!()
    }

    pub fn init256() -> Self {
        todo!()
    }

    pub fn init384() -> Self {
        todo!()
    }

    pub fn init512() -> Self {
        todo!()
    }
}

impl StatefulHasher for Skein1024 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}
