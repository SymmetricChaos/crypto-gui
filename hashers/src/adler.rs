use crate::traits::StatefulHasher;

const MODULUS: u32 = 65521;

pub struct Adler32 {
    a: u32,
    b: u32,
}

impl Adler32 {
    pub fn init() -> Self {
        Self { a: 1, b: 0 }
    }
}

impl StatefulHasher for Adler32 {
    fn update(&mut self, bytes: &[u8]) {
        // The modulo operation can be deferred for 5552 bytes (after which b may overflow a u32) if optimizing for speed
        for byte in bytes {
            self.a = self.a.wrapping_add(*byte as u32) % MODULUS;
            self.b = self.b.wrapping_add(self.a) % MODULUS;
        }
    }

    fn finalize(self) -> Vec<u8> {
        (self.b << 16 | self.a).to_be_bytes().into()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1,
    Adler32::init(),
    b"Wikipedia",
    "11e60398";
    test2, // From an online calculator
    Adler32::init(),
    b"RelativelyLongTextInputInOrderToReachTheModulus",
    "c36712ca";
);
