use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

// https://eprint.iacr.org/2012/351.pdf

#[derive(Debug, Clone)]

pub struct SipHash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub k0: u64,
    pub k1: u64,
    pub compression_rounds: usize,
    pub finalization_rounds: usize,
}

impl Default for SipHash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            k0: 0,
            k1: 0,
            compression_rounds: 2,
            finalization_rounds: 4,
        }
    }
}

impl SipHash {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn compression(mut self, compression_rounds: usize) -> Self {
        self.compression_rounds = compression_rounds;
        self
    }

    pub fn finalization(mut self, finalization_rounds: usize) -> Self {
        self.finalization_rounds = finalization_rounds;
        self
    }

    pub fn keys(mut self, k0: u64, k1: u64) -> Self {
        self.k0 = k0.to_be();
        self.k1 = k1.to_be();
        self
    }

    pub fn k0(mut self, k0: u64) -> Self {
        self.k0 = k0.to_be();
        self
    }

    pub fn k1(mut self, k1: u64) -> Self {
        self.k1 = k1.to_be();
        self
    }

    pub fn set_keys(&mut self, k0: u64, k1: u64) {
        self.k0 = k0.to_be();
        self.k1 = k1.to_be();
    }

    pub fn sip_round(mut v: [u64; 4]) -> [u64; 4] {
        v[0] = v[0].wrapping_add(v[1]);
        v[2] = v[2].wrapping_add(v[3]);
        v[1] = v[1].rotate_left(13);
        v[3] = v[3].rotate_left(16);
        v[1] ^= v[0];
        v[3] ^= v[2];
        v[0] = v[0].rotate_left(32);
        v[2] = v[2].wrapping_add(v[1]);
        v[0] = v[0].wrapping_add(v[3]);
        v[1] = v[1].rotate_left(17);
        v[3] = v[3].rotate_left(21);
        v[1] ^= v[2];
        v[3] ^= v[0];
        v[2] = v[2].rotate_left(32);
        v
    }
}

impl ClassicHasher for SipHash {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Initialization is four 64-bit words XORed with each half of the 128-bit key
        let mut state: [u64; 4] = [
            self.k0 ^ 0x736f6d6570736575,
            self.k1 ^ 0x646f72616e646f6d,
            self.k0 ^ 0x6c7967656e657261,
            self.k1 ^ 0x7465646279746573,
        ];

        // Padding
        let final_byte = (input.len() % 256) as u8;
        let total_len = (input.len() + 1).div_ceil(8) * 8;
        while input.len() < total_len - 1 {
            input.push(0);
        }
        input.push(final_byte);

        // Compression
        for block in input.chunks(8) {
            let mi: u64 = u64::from_le_bytes(block.try_into().unwrap());

            state[3] ^= mi;

            for _ in 0..self.compression_rounds {
                state = Self::sip_round(state);
            }

            state[0] ^= mi;
        }

        // Finalization
        state[2] ^= 0xff;

        for _ in 0..self.finalization_rounds {
            state = Self::sip_round(state);
        }

        (state[0] ^ state[1] ^ state[2] ^ state[3])
            .to_be_bytes()
            .to_vec()
    }

    crate::hash_bytes_from_string! {}
}

// impl KeyedHasher for SipHash {
//     fn set_salt(&mut self, _bytes: &[u8]) {
//         unimplemented!("SipHash does not accept a salt argument")
//     }

//     fn set_key(&mut self, bytes: &[u8]) {
//         if bytes.len() == 16 {
//             self.k0 = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
//             self.k1 = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
//         } else {
//             panic!("SipHash key must be exactly 16 bytes")
//         }
//     }
// }

#[cfg(test)]
mod siphash_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let hasher = SipHash::default()
            .input(ByteFormat::Hex)
            .keys(0x0001020304050607, 0x08090a0b0c0d0e0f);
        assert_eq!(
            "a129ca6149be45e5",
            hasher
                .hash_bytes_from_string("000102030405060708090a0b0c0d0e")
                .unwrap()
        );
    }
}
