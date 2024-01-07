use crate::traits::ClassicHasher;

// https://eprint.iacr.org/2012/351.pdf

pub struct SipHash {
    k0: u64,
    k1: u64,
    pub compression_rounds: usize,
    pub finalization_rounds: usize,
}

impl Default for SipHash {
    fn default() -> Self {
        Self {
            k0: 0,
            k1: 0,
            compression_rounds: 2,
            finalization_rounds: 4,
        }
    }
}

impl SipHash {
    // Confirmed from spec
    pub fn set_keys(&mut self, k0: u64, k1: u64) {
        self.k0 = k0.to_be();
        self.k1 = k1.to_be();
    }

    pub fn sip_round(mut v: [u64; 4]) -> [u64; 4] {
        v[0] = v[0].wrapping_add(v[1]);
        v[1] = v[1].rotate_left(13);
        v[1] ^= v[0];
        v[0] = v[0].rotate_left(32);
        v[2] = v[2].wrapping_add(v[3]);
        v[3] = v[3].rotate_left(16);
        v[3] ^= v[2];
        v[0] = v[0].wrapping_add(v[3]);
        v[3] = v[3].rotate_left(21);
        v[3] ^= v[0];
        v[2] = v[2].wrapping_add(v[1]);
        v[1] = v[1].rotate_left(17);
        v[1] ^= v[2];
        v[2] = v[2].rotate_left(32);

        // v[0] = v[0].wrapping_add(v[1]);
        // v[2] = v[2].wrapping_add(v[3]);
        // v[1] = v[1].rotate_left(13);
        // v[3] = v[3].rotate_left(16);
        // v[1] ^= v[0];
        // v[3] ^= v[2];
        // v[0] = v[0].rotate_left(32);
        // v[2] = v[2].wrapping_add(v[1]);
        // v[0] = v[0].wrapping_add(v[3]);
        // v[1] = v[1].rotate_left(17);
        // v[3] = v[3].rotate_left(21);
        // v[1] ^= v[2];
        // v[3] ^= v[0];
        // v[2] = v[2].rotate_left(32);
        v
    }
}

impl ClassicHasher for SipHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Initialization is four 64-bit words xored with each half of the 128-bit key
        // Confirmed from spec
        let mut state: [u64; 4] = [
            self.k0 ^ 0x736f6d6570736575,
            self.k1 ^ 0x646f72616e646f6d,
            self.k0 ^ 0x6c7967656e657261,
            self.k1 ^ 0x7465646279746573,
        ];

        // Padding
        // Confirmed from spec
        let final_byte = (input.len() % 256) as u8;
        let total_len = (input.len() + 1).div_ceil(8);
        while input.len() < total_len - 1 {
            input.push(0);
        }
        input.push(final_byte);

        // Compression
        for block in input.chunks_exact(8) {
            // Confirmed from spec
            let mi: u64 = u64::from_le_bytes(block.try_into().unwrap());

            println!("\n\n<<<BEGIN BLOCK>>>");
            println!("\nmessage word: {:016x}", mi);
            println!("\nCurrent state");
            for s in state {
                println!("{:016x}", s);
            }
            // Confirmed from spec
            state[3] ^= mi;
            println!("\nmessage word XORed with state[3");
            for s in state {
                println!("{:016x}", s);
            }

            for _ in 0..self.compression_rounds {
                state = Self::sip_round(state);
            }
            println!("\ncompression rounds run");
            for s in state {
                println!("{:016x}", s);
            }

            state[0] ^= mi;
            println!("\nmessage word XORed with state[0]");
            for s in state {
                println!("{:016x}", s);
            }
            println!("\n<<<END BLOCK>>>");
        }

        // Finalization
        state[2] ^= 0xff;
        println!("After XOR with 0xff");
        for s in state {
            println!("{:016x}", s);
        }
        print!("\n");

        for _ in 0..self.finalization_rounds {
            state = Self::sip_round(state);
        }
        println!("After Finalization Rounds");
        for s in state {
            println!("{:016x}", s);
        }
        print!("\n");

        (state[0] ^ state[1] ^ state[2] ^ state[3])
            .to_le_bytes()
            .to_vec()
    }
}

#[cfg(test)]
mod siphash_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = SipHash::default();
        hasher.set_keys(0x0010020304050607, 0x08090a0b0c0d0e0f);

        assert_eq!(
            "a129ca6149be45e5",
            hasher.hash_to_string(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
        );
    }
}
