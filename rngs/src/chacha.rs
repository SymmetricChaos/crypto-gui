use crate::ClassicRng;
use itertools::Itertools;
use std::num::Wrapping;

pub struct ChaCha {
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub ctr: u64,
    pub rounds: u8,
    pub saved_keystream: Vec<u32>,
}

impl Default for ChaCha {
    fn default() -> Self {
        Self {
            // default for key and nonce taken from test vector here: https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
            key: [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            nonce: [0x03020100, 0x07060504],
            ctr: 0,
            rounds: 20,
            saved_keystream: Vec::new(),
        }
    }
}

impl ChaCha {
    pub fn create_state(&self) -> [u32; 16] {
        [
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            self.ctr as u32,
            (self.ctr >> 32) as u32,
            self.nonce[0],
            self.nonce[1],
        ]
    }

    pub fn quarter_round(state: &mut [Wrapping<u32>; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] += state[b];
        state[d] ^= state[a];
        state[d] = Wrapping(state[d].0.rotate_left(16));

        state[c] += state[d];
        state[b] ^= state[c];
        state[b] = Wrapping(state[b].0.rotate_left(12));

        state[a] += state[b];
        state[d] ^= state[a];
        state[d] = Wrapping(state[d].0.rotate_left(8));

        state[c] += state[d];
        state[b] ^= state[c];
        state[b] = Wrapping(state[b].0.rotate_left(7));
    }

    pub fn column_round(state: &mut [Wrapping<u32>; 16]) {
        Self::quarter_round(state, 0, 4, 8, 12);
        Self::quarter_round(state, 1, 5, 9, 13);
        Self::quarter_round(state, 2, 6, 10, 14);
        Self::quarter_round(state, 3, 7, 11, 15);
    }

    pub fn diag_round(state: &mut [Wrapping<u32>; 16]) {
        Self::quarter_round(state, 0, 5, 10, 15);
        Self::quarter_round(state, 1, 6, 11, 12);
        Self::quarter_round(state, 2, 7, 8, 13);
        Self::quarter_round(state, 3, 4, 9, 14);
    }

    pub fn double_round(state: &mut [Wrapping<u32>; 16]) {
        Self::column_round(state);
        Self::diag_round(state);
    }

    pub fn next_block(&mut self) -> Vec<u32> {
        let state = [
            Wrapping(0x61707865),
            Wrapping(0x3320646e),
            Wrapping(0x79622d32),
            Wrapping(0x6b206574),
            Wrapping(self.key[0]),
            Wrapping(self.key[1]),
            Wrapping(self.key[2]),
            Wrapping(self.key[3]),
            Wrapping(self.key[4]),
            Wrapping(self.key[5]),
            Wrapping(self.key[6]),
            Wrapping(self.key[7]),
            Wrapping(self.ctr as u32),
            Wrapping((self.ctr >> 32) as u32),
            Wrapping(self.nonce[0]),
            Wrapping(self.nonce[1]),
        ];
        self.ctr += 1;

        // Temporary state
        let mut t_state = state.clone();

        // Only ChaCha20, ChaCha12, and ChaCha8 are official but any number is usable
        for _round in 0..self.rounds / 2 {
            Self::double_round(&mut t_state);
        }
        if self.rounds % 2 == 1 {
            Self::column_round(&mut t_state)
        }

        // XOR the current state into the temporary state
        for (i, word) in t_state.iter_mut().enumerate() {
            *word += state[i]
        }

        // Create a byte stream
        t_state.iter().map(|w| w.0.to_be()).rev().collect_vec()
    }
}

impl ClassicRng for ChaCha {
    fn next_u32(&mut self) -> u32 {
        if self.saved_keystream.is_empty() {
            self.saved_keystream = self.next_block();
        }
        self.saved_keystream
            .pop()
            .expect("saved keystream should be filled")
    }
}

#[cfg(test)]
mod chacha_rng_tests {

    use super::*;

    #[test]
    fn state_test_empty() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let mut cipher = ChaCha::default();
        cipher.key = [0, 0, 0, 0, 0, 0, 0, 0];
        cipher.nonce = [0, 0];

        for word in [
            0x76b8e0ad, 0xa0f13d90, 0x405d6ae5, 0x5386bd28, 0xbdd219b8, 0xa08ded1a, 0xa836efcc,
            0x8b770dc7, 0xda41597c, 0x5157488d, 0x7724e03f, 0xb8d84a37, 0x6a43b8f4, 0x1518a11c,
            0xc387b669, 0xb2ee6586,
        ] {
            assert_eq!(word, cipher.next_u32());
        }
    }

    #[test]
    fn state_test() {
        // https://datatracker.ietf.org/doc/html/draft-agl-tls-chacha20poly1305-04#section-7
        let mut cipher = ChaCha::default();

        for word in [
            0xf798a189, 0xf195e669, 0x82105ffb, 0x640bb775, 0x7f579da3, 0x1602fc93, 0xec01ac56,
            0xf85ac3c1, 0x34a4547b, 0x733b4641, 0x3042c944, 0x00491769, 0x05d3be59, 0xea1c53f1,
            0x5916155c, 0x2be8241a,
        ] {
            assert_eq!(word, cipher.next_u32());
        }
    }
}
