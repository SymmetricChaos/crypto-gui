// https://eprint.iacr.org/2019/458.pdf

use crate::traits::StatefulHasher;

const RF_128: usize = 8;
const RP_128: usize = 60;

const RF_80: usize = 8;
const RP_80: usize = 35;

const RF_256: usize = 8;
const RP_256: usize = 120;

pub struct PoseidonState {}

impl PoseidonState {
    fn add_round_constants(&mut self) {
        todo!()
    }

    fn sub_words_full(&mut self) {
        todo!()
    }

    fn sub_words_partial(&mut self) {
        todo!()
    }

    fn mix_layer(&mut self) {
        todo!()
    }

    fn round_full(&mut self) {
        self.add_round_constants();
        self.sub_words_full();
        self.mix_layer();
    }

    fn round_partial(&mut self) {
        self.add_round_constants();
        self.sub_words_partial();
        self.mix_layer();
    }

    pub fn absorb() {
        todo!()
    }

    pub fn squeeze() {
        todo!()
    }
}

pub struct Poseidon {
    buffer: Vec<u8>,
}

impl Poseidon {
    pub fn init() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl StatefulHasher for Poseidon {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }
}
