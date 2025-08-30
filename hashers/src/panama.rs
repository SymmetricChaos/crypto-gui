// https://github.com/bitbandi/all-hash-python/blob/master/sph/panama.c
// https://tnlandforms.us/cns06/panama.pdf

use crate::traits::{ResettableHasher, StatefulHasher};
use utils::{
    byte_formatting::{make_u32s_le, u32s_to_bytes_be},
    padding::bit_padding,
};

struct Buffer([[u32; 8]; 32]);

impl Buffer {
    pub fn new() -> Self {
        Buffer([[0; 8]; 32])
    }

    pub fn stage(&self, n: usize) -> &[u32; 8] {
        &self.0[n]
    }

    pub fn update(&mut self, q: &[u32]) {
        self.0.rotate_right(1);
        for i in 0..8 {
            self.0[0][i] = self.0[31][i] ^ q[i]
        }
        for i in 0..8 {
            self.0[23][i] = self.0[24][i] ^ self.0[31][(i + 2) % 8]
        }
    }
}

pub struct Panama {
    state: [u32; 17],
    panama_buffer: Buffer,
    buffer: Vec<u8>,
}

impl Default for Panama {
    fn default() -> Self {
        Self {
            state: [0; 17],
            panama_buffer: Buffer::new(),
            buffer: Vec::with_capacity(32),
        }
    }
}

impl Panama {
    fn state_update_push(&mut self, p: &[u32; 8]) {
        self.panama_buffer.update(p); // gamma
        self.pi();
        self.theta();
        self.sigma_push(p)
    }

    fn state_update_pull(&mut self) -> [u32; 8] {
        self.panama_buffer.update(&self.state[0..8]); // gamma
        self.pi();
        self.theta();
        self.sigma_pull()
    }

    // Invertible linear transformation of the state
    fn theta(&mut self) {
        let t = self.state.clone();
        for i in 0..17 {
            self.state[i] = t[i] ^ t[(i + 1) % 17] ^ t[(i + 4) % 17];
        }
    }

    // Permutation
    fn pi(&mut self) {
        let t = self.state.clone();
        for i in 0..8 {
            let j = (i * 7) % 17;
            let k = ((i * (i + 1)) / 2) % 32;
            self.state[i] = t[j].rotate_left(k as u32); // TODO: check pseudocode for rotation direction
        }
    }

    // Addition
    fn sigma_push(&mut self, l: &[u32; 8]) {
        self.state[0] ^= 1;
        for i in 0..8 {
            self.state[i + 1] ^= l[i];
            self.state[i + 9] ^= self.panama_buffer.stage(16)[i]
        }
    }

    // Addition
    fn sigma_pull(&mut self) -> [u32; 8] {
        self.state[0] ^= 1;
        for i in 0..8 {
            self.state[i + 1] ^= self.panama_buffer.stage(4)[i];
            self.state[i + 9] ^= self.panama_buffer.stage(16)[i];
        }
        let mut out = [0; 8];
        out.copy_from_slice(&self.state[9..16]);
        out
    }
}

impl StatefulHasher for Panama {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, 32, {
            let block = make_u32s_le::<8>(&self.buffer);
            self.state_update_push(&block);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        bit_padding(&mut self.buffer, 32).unwrap();

        // Either one or two final blocks
        if self.buffer.len() == 32 {
            let block = make_u32s_le::<8>(&self.buffer);
            self.state_update_push(&block);
        } else {
            let block = make_u32s_le::<8>(&self.buffer[..32]);
            self.state_update_push(&block);
            let block = make_u32s_le::<8>(&self.buffer[32..]);
            self.state_update_push(&block);
        }

        for _ in 0..32 {
            self.state_update_pull();
        }

        let mut out = [0_u8; 32];
        u32s_to_bytes_be(&mut out, &self.state_update_pull());
        out.to_vec()
    }
}

// Don't update until basic methods are finalized
impl ResettableHasher for Panama {
    fn finalize_and_reset(&mut self) -> Vec<u8> {
        todo!()
    }
}
