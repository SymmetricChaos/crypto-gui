use crate::traits::StatefulHasher;

const R: u128 = 0xE1000000000000000000000000000000;
const BLOCK_LEN: usize = 16;

// Multiplication in the Galois field used for GHASH. Addition in the same is XOR.
// This implementation is not optimized at all because this project seeks clarity not real world use.
pub fn mult_gf(x: u128, y: u128) -> u128 {
    let mut out = 0;
    let mut v = x;
    for i in 0..128 {
        // Take bits of y from the left
        if y >> (127 - i) & 1 == 1 {
            out ^= v
        }
        // Check rightmost bit of v
        if v & 1 == 0 {
            v >>= 1;
        } else {
            v >>= 1;
            v ^= R;
        }
    }
    out
}

// Add the bytes of block to the accumulator (this is XOR in the Galois Field) then multiply by the value h
// This is used to implement Horner's Rule for evaluating a polynomial
pub fn add_mul(acc: &mut u128, block: &[u8], h: u128) {
    for (i, byte) in block.iter().enumerate() {
        *acc ^= (*byte as u128) << (15 - i) * 8
    }
    *acc = mult_gf(*acc, h);
}

pub fn mulx(x: u128) -> u128 {
    let mut v = x.to_be();
    let v_hi = v >> 127;
    v <<= 1;
    v ^= v_hi ^ (v_hi << 127) ^ (v_hi << 126) ^ (v_hi << 121);
    v.to_be()
}

#[derive(Debug, Clone)]
pub struct PolyVal {
    h: u128,     // usually determined by a cipher
    c: u128,     // constant term, usually determined by a cipher
    ad_len: u64, // how many bytes of input to treat as the additional data
    accumulator: u128,
    bits_taken: u64,
    buffer: Vec<u8>,
}

impl Default for PolyVal {
    fn default() -> Self {
        Self {
            h: 0,
            c: 0,
            ad_len: 0,
            accumulator: 0,
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        }
    }
}

impl PolyVal {
    pub fn init(h: &[u8], c: &[u8], ad: &[u8]) -> Self {
        let h = mulx(u128::from_be_bytes(
            h.try_into().expect("h must be exactly 16 bytes"),
        ));
        let ad_len = (ad.len() as u64) * 8;
        let mut accumulator = 0;
        for block in ad.chunks(16) {
            add_mul(&mut accumulator, block, h);
        }
        Self {
            h,
            c: u128::from_be_bytes(c.try_into().expect("c must be exactly 16 bytes")),
            ad_len,
            accumulator,
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        }
    }
}

impl StatefulHasher for PolyVal {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            self.bits_taken += 128;
            self.buffer.reverse();
            add_mul(&mut self.accumulator, &self.buffer, self.h);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        // Final block
        if !self.buffer.is_empty() {
            self.bits_taken += (self.buffer.len() * 8) as u64;
            add_mul(&mut self.accumulator, &self.buffer, self.h);
        }

        // The length of the AD and CT form the term x^1
        self.accumulator ^= (self.ad_len as u128) << 64;
        self.accumulator ^= self.bits_taken as u128;
        self.accumulator = mult_gf(self.accumulator, self.h);

        // XOR in the constant term, x^0, this is the key when used securely
        self.accumulator ^= self.c;

        self.accumulator.to_le_bytes().into()
    }
}

#[cfg(test)]
mod polyval {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn mulx_tests() {
        assert_eq!(
            0x02000000000000000000000000000000,
            mulx(0x01000000000000000000000000000000)
        );
        assert_eq!(
            0x3931819bf271fada0503eb52574ca572,
            mulx(0x9c98c04df9387ded828175a92ba652d8)
        );
    }

    #[test]
    fn rfc8452() {
        let mut hasher = PolyVal::init(
            &hex!("25629347589242761d31f826ba4b757b"),
            &hex!("00000000000000000000000000000000"),
            &[],
        );
        hasher.update(&hex!("4f4f95668c83dfb6401762bb2d01a262"));
        hasher.update(&hex!("d1a24ddd2721d006bbe45f20d3c9f362"));
        assert_eq!(
            hex!("f7a3b47b846119fae5b7866cf5e5b77e").to_vec(),
            hasher.finalize()
        );
    }
}
