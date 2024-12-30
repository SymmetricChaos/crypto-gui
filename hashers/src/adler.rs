use crate::traits::StatefulHasher;

pub struct Adler32 {
    a: u16,
    b: u16,
}

impl Adler32 {
    pub fn init() -> Self {
        Self { a: 1, b: 0 }
    }
}

impl StatefulHasher for Adler32 {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.a = self.a.wrapping_add(*byte as u16);
            self.b = self.b.wrapping_add(self.a);
        }
    }

    fn finalize(self) -> Vec<u8> {
        [self.b, self.a]
            .into_iter()
            .flat_map(|w| w.to_be_bytes())
            .collect()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1,
    Adler32::init(),
    b"Wikipedia",
    "11e60398";
);
