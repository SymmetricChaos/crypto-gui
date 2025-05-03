use crate::traits::StatefulHasher;

// https://eprint.iacr.org/2012/351.pdf

const BLOCK_LEN: usize = 8;

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

#[derive(Debug, Clone)]
pub struct SipHash {
    compression_rounds: usize,
    finalization_rounds: usize,
    state: [u64; 4],
    buffer: Vec<u8>,
    final_byte: u8,
}

impl SipHash {
    pub fn init(key: [u64; 2], compression_rounds: usize, finalization_rounds: usize) -> Self {
        let state: [u64; 4] = [
            key[0].to_be() ^ 0x736f6d6570736575,
            key[1].to_be() ^ 0x646f72616e646f6d,
            key[0].to_be() ^ 0x6c7967656e657261,
            key[1].to_be() ^ 0x7465646279746573,
        ];

        Self {
            compression_rounds,
            finalization_rounds,
            state,
            buffer: Vec::with_capacity(BLOCK_LEN),
            final_byte: 0,
        }
    }

    pub fn init_2_4(key: [u64; 2]) -> Self {
        Self::init(key, 2, 4)
    }

    pub fn init_1_3(key: [u64; 2]) -> Self {
        Self::init(key, 1, 3)
    }
}

impl StatefulHasher for SipHash {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            let mi: u64 = u64::from_le_bytes(self.buffer.clone().try_into().unwrap());
            self.state[3] ^= mi;
            for _ in 0..self.compression_rounds {
                self.state = sip_round(self.state);
            }
            self.state[0] ^= mi;
            self.final_byte = self.final_byte.wrapping_add(8);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        self.final_byte = self.final_byte.wrapping_add(self.buffer.len() as u8);
        while self.buffer.len() % 8 != 7 {
            self.buffer.push(0x00);
        }
        self.buffer.push(self.final_byte);

        for chunk in self.buffer.chunks_exact(8) {
            let mi: u64 = u64::from_le_bytes(chunk.try_into().unwrap());
            self.state[3] ^= mi;
            for _ in 0..self.compression_rounds {
                self.state = sip_round(self.state);
            }
            self.state[0] ^= mi;
            self.final_byte = self.final_byte.wrapping_add(8);
        }

        self.state[2] ^= 0xff;

        for _ in 0..self.finalization_rounds {
            self.state = sip_round(self.state);
        }

        (self.state[0] ^ self.state[1] ^ self.state[2] ^ self.state[3])
            .to_be_bytes()
            .to_vec()
    }
}

crate::stateful_hash_tests!(
    test_1, SipHash::init_2_4([0x0001020304050607, 0x08090a0b0c0d0e0f]), &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e],
    "a129ca6149be45e5";
);
