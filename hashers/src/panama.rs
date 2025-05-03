// https://github.com/bitbandi/all-hash-python/blob/master/sph/panama.c

use std::ops::Deref;

use crate::traits::StatefulHasher;

struct Buffer([[u32; 8]; 32]);

impl Buffer {
    pub fn new() -> Self {
        Buffer([[0; 8]; 32])
    }

    pub fn reset(&mut self) {
        self.0 = [[0; 8]; 32]
    }

    pub fn stage(&self, n: usize) -> &[u32; 8] {
        &self.0[n]
    }

    pub fn update(&mut self, q: &[u32; 8]) {
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
    buffer: Buffer,
}

impl Default for Panama {
    fn default() -> Self {
        Self {
            state: [0; 17],
            buffer: Buffer::new(),
        }
    }
}

impl Panama {
    fn state_update_push(&mut self, p: &[u32; 8]) {
        self.gamma();
        self.pi();
        self.theta();
        self.sigma_push(p)
    }

    fn state_update_pull(&mut self) -> [u32; 8] {
        self.gamma();
        self.pi();
        self.theta();
        self.sigma_pull()
    }

    // Invertible linear transformation of the state
    fn theta(&mut self) {
        let mut t = self.state.clone();
        for i in 0..17 {
            self.state[i] = t[i] ^ t[(i + 1) % 17] ^ t[(i + 4) % 17];
        }
    }

    // Invertible nonlinear transformation of the state
    fn gamma(&mut self) {
        let mut t = self.state.clone();
        for i in 0..17 {
            self.state[i] = t[i] ^ (t[(i + 1) % 17] | !t[(i + 42) % 17]);
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
            self.state[i + 9] ^= self.buffer.stage(16)[i]
        }
    }

    // Addition
    fn sigma_pull(&mut self) -> [u32; 8] {
        self.state[0] ^= 1;
        for i in 0..8 {
            self.state[i + 1] ^= self.buffer.stage(4)[i];
            self.state[i + 9] ^= self.buffer.stage(16)[i];
        }
        let mut out = [0; 8];
        out.copy_from_slice(&self.state[9..16]);
        out
    }
}

impl StatefulHasher for Panama {
    fn update(&mut self, mut bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    
}
