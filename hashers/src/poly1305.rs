use crate::{stateful_hash_tests, traits::StatefulHasher};
use num::{BigUint, Zero};

const BLOCK_LEN: usize = 16;

#[derive(Debug, Clone)]
pub struct Poly1305 {
    key_r: BigUint, // point at which the polynomial is evaluated
    key_s: BigUint, // nonce that is added at the end
    modulus: BigUint,
    accumulator: BigUint,
    buffer: Vec<u8>,
}

impl Poly1305 {
    pub fn init(key_r: &[u8], key_s: &[u8]) -> Self {
        // Prime modulus (2**130 - 5) initialized from array
        let modulus = BigUint::from_bytes_be(&[
            0x03_u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xfb,
        ]);
        assert!(key_s.len() == 16, "key_s must be exactly 16 bytes");
        // Restrictions on key_r
        let mut key_r: [u8; 16] = key_r.try_into().expect("key_r must be exactly 16 bytes");
        for i in [3, 7, 11, 15] {
            // The top four bits must be 0
            key_r[i] &= 0b00001111;
        }
        for i in [4, 8, 12] {
            // The lower two bits must be 0
            key_r[i] &= 0b11111100;
        }
        Self {
            key_r: BigUint::from_bytes_le(&key_r),
            key_s: BigUint::from_bytes_le(&key_s),
            modulus,
            accumulator: BigUint::zero(),
            buffer: Vec::with_capacity(BLOCK_LEN),
        }
    }
}

impl StatefulHasher for Poly1305 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(BLOCK_LEN);
        let rem = chunks.remainder().to_vec();

        for chunk in chunks {
            let mut block = chunk.to_vec();
            block.push(0x01);
            block.reverse();
            self.accumulator += BigUint::from_bytes_be(&block);
            self.accumulator *= &self.key_r;
            self.accumulator %= &self.modulus;
        }
        self.buffer = rem;
    }

    fn finalize(mut self) -> Vec<u8> {
        // Take the partial last block so long as it is not empty
        if !self.buffer.is_empty() {
            if self.buffer.len() != 16 {
                self.buffer.push(0x01);
            }
            while self.buffer.len() != 17 {
                self.buffer.push(0x00);
            }
            self.buffer.reverse();
            self.accumulator += BigUint::from_bytes_be(&self.buffer);
            self.accumulator *= &self.key_r;
            self.accumulator %= &self.modulus;
        }

        // Finalization
        self.accumulator += &self.key_s;

        let mut out = self.accumulator.to_bytes_le();
        while out.len() < 16 {
            out.push(0x00);
        }
        out[0..16].to_vec()
    }

    crate::stateful_hash_helpers!();
}

stateful_hash_tests!(
    test1, Poly1305::init(
        &[0x12, 0x97, 0x6a, 0x08, 0xc4, 0x42, 0x6d, 0x0c, 0xe8, 0xa8, 0x24, 0x07, 0xc4, 0xf4, 0x82, 0x07],
        &[0x80, 0xf8, 0xc2, 0x0a, 0xa7, 0x12, 0x02, 0xd1, 0xe2, 0x91, 0x79, 0xcb, 0xcb, 0x55, 0x5a, 0x57]
    ),
    &[0xab, 0x08, 0x12, 0x72, 0x4a, 0x7f, 0x1e, 0x34, 0x27, 0x42, 0xcb, 0xed, 0x37, 0x4d, 0x94, 0xd1, 0x36, 0xc6, 0xb8, 0x79, 0x5d, 0x45, 0xb3, 0x81, 0x98, 0x30, 0xf2, 0xc0, 0x44, 0x91, 0xfa, 0xf0, 0x99, 0x0c, 0x62, 0xe4, 0x8b, 0x80, 0x18, 0xb2, 0xc3, 0xe4, 0xa0, 0xfa, 0x31, 0x34, 0xcb, 0x67, 0xfa, 0x83, 0xe1, 0x58, 0xc9, 0x94, 0xd9, 0x61, 0xc4, 0xcb, 0x21, 0x09, 0x5c, 0x1b, 0xf9],
    "5154ad0d2cb26e01274fc51148491f1b";

    test2, Poly1305::init(
        &[0x85, 0x1f, 0xc4, 0x0c, 0x34, 0x67, 0xac, 0x0b, 0xe0, 0x5c, 0xc2, 0x04, 0x04, 0xf3, 0xf7, 0x00],
        &[0x58, 0x0b, 0x3b, 0x0f, 0x94, 0x47, 0xbb, 0x1e, 0x69, 0xd0, 0x95, 0xb5, 0x92, 0x8b, 0x6d, 0xbc]
    ),
    &[0xf3, 0xf6],
    "f4c633c3044fc145f84f335cb81953de";
);
