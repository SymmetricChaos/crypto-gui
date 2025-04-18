use crate::traits::StatefulHasher;

pub struct Skein512 {
    pub key: [u64; Self::WORDS],
    pub tweak: [u64; 2],
}

impl Default for Skein512 {
    fn default() -> Self {
        Self {
            key: [0; Self::WORDS],
            tweak: [0; 2],
        }
    }
}

impl Skein512 {
    const WORDS: usize = 8;
    const ROUNDS: usize = 72;

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

impl StatefulHasher for Skein512 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_512_256_empty, Skein512::init256(), b"",
    "39ccc4554a8b31853b9de7a1fe638a24cce6b35a55f2431009e18780335d2621";
    test_512_512_empty, Skein512::init512(), b"",
    "bc5b4c50925519c290cc634277ae3d6257212395cba733bbad37a4af0fa06af41fca7903d06564fea7a2d3730dbdb80c1f85562dfcc070334ea4d1d9e72cba7a";
);
