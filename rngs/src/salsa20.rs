use crate::ClassicRng;

// https://cr.yp.to/snuffle/salsafamily-20071225.pdf
pub struct Salsa20 {
    pub key: [u32; 8],
    pub nonce: [u32; 2],
    pub ctr: u64,
    pub rounds: u8,
    pub big_endian: bool,
    pub saved_keystream: Vec<u32>,
}

impl Default for Salsa20 {
    fn default() -> Self {
        Self {
            key: [
                0x04030201, 0x08070605, 0x0c0b0a09, 0x100f0e0d, 0x14131211, 0x18171615, 0x1c1b1a19,
                0x201f1e1d,
            ],
            nonce: [0x01040103, 0x06020905],
            ctr: 0,
            rounds: 20,
            big_endian: true,
            saved_keystream: Vec::new(),
        }
    }
}

impl Salsa20 {
    pub fn create_state(&self) -> [u32; 16] {
        [
            0x61707865,
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            0x3320646e,
            self.nonce[0],
            self.nonce[1],
            self.ctr as u32,
            (self.ctr >> 32) as u32,
            0x79622d32,
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            0x6b206574,
        ]
    }

    pub fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[b] ^= (state[a].wrapping_add(state[d])).rotate_left(7);
        state[c] ^= (state[b].wrapping_add(state[a])).rotate_left(9);
        state[d] ^= (state[c].wrapping_add(state[b])).rotate_left(13);
        state[a] ^= (state[d].wrapping_add(state[c])).rotate_left(18);
    }

    // Acts on columns
    pub fn odd_round(state: &mut [u32; 16]) {
        Self::quarter_round(state, 0, 4, 8, 12);
        Self::quarter_round(state, 5, 9, 13, 1);
        Self::quarter_round(state, 10, 14, 2, 6);
        Self::quarter_round(state, 15, 3, 7, 11);
    }

    // Acts on rows
    pub fn even_round(state: &mut [u32; 16]) {
        Self::quarter_round(state, 0, 1, 2, 3);
        Self::quarter_round(state, 5, 6, 7, 4);
        Self::quarter_round(state, 10, 11, 8, 9);
        Self::quarter_round(state, 15, 12, 13, 14);
    }

    pub fn double_round(state: &mut [u32; 16]) {
        Self::odd_round(state);
        Self::even_round(state);
    }

    pub fn next_block(&mut self) -> Vec<u32> {
        let state = self.create_state();
        self.ctr += 1;

        // Temporary state
        let mut t_state = state.clone();

        // Only Salsa20/20, Salsa20/12, and Salsa20/8 are official but any number is usable
        for _round in 0..self.rounds / 2 {
            Self::double_round(&mut t_state);
        }
        if self.rounds % 2 == 1 {
            Self::odd_round(&mut t_state)
        }

        // XOR the current state into the temporary state
        for (i, word) in t_state.iter_mut().enumerate() {
            *word = word.wrapping_add(state[i])
        }

        // Output the block
        t_state.to_vec()
    }
}

impl ClassicRng for Salsa20 {
    fn next_u32(&mut self) -> u32 {
        if self.saved_keystream.is_empty() {
            self.next_block();
        }

        if self.big_endian {
            self.saved_keystream
                .pop()
                .expect("saved keystream should be filled")
                .to_be()
        } else {
            self.saved_keystream
                .pop()
                .expect("saved keystream should be filled")
                .to_le()
        }
    }
}
