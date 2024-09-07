use utils::byte_formatting::ByteFormat;

use super::block_cipher::{BCMode, BCPadding};

const C: [u64; 12] = [
    0xf0, 0x96, 0xe1, 0x87, 0xd2, 0x78, 0xc3, 0x69, 0xb4, 0x5a, 0xa5, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

pub struct Ascon128State {
    state: [u64; 5],
}

impl Ascon128State {
    const A: usize = 12; // initialization rounds
    const B: usize = 6; // block rounds
    const R: usize = 1; // 64 bits, a single word of state

    // Initialize with a constant derived from the key length in bits, rate in bits, initialization and finalization rounds, and intermediate rounds
    pub fn initialize(&mut self, key: [u64; 2]) {
        self.state[0] = 0x80400c0600000000;
        self.rounds_a(key);
    }

    pub fn rounds_a(&mut self, key: [u64; 2]) {
        for i in 0..Self::A {
            self.transform(i);
        }
        self.state[3] ^= key[3];
        self.state[4] ^= key[4];
    }

    pub fn rounds_b(&mut self, key: [u64; 2]) {
        for i in 0..Self::B {
            self.transform(i + 6);
        }
        self.state[3] ^= key[3];
        self.state[4] ^= key[4];
    }

    pub fn transform(&mut self, i: usize) {
        // round constant
        self.state[2] ^= C[i];
        // substitution
        self.sbox();
        // linear diffusion
        self.linear_diffusor();
    }

    pub fn sbox(&mut self) {
        self.state[0] ^= self.state[4];
        self.state[4] ^= self.state[3];
        self.state[2] ^= self.state[1];

        let mut t = self.state.clone();
        t = t.map(|w| !w);
        for i in 0..5 {
            t[i] &= self.state[(i + 1) % 5];
        }
        for i in 0..5 {
            self.state[i] &= t[(i + 1) % 5];
        }

        self.state[1] ^= self.state[0];
        self.state[0] ^= self.state[4];
        self.state[3] ^= self.state[2];
        self.state[2] = !self.state[2];
    }

    pub fn linear_diffusor(&mut self) {
        for i in 0..5 {
            self.state[i] ^=
                self.state[i].rotate_right(ROTS[i].0) ^ self.state[i].rotate_right(ROTS[i].1);
        }
    }
}

pub struct Ascon128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Ascon128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}
