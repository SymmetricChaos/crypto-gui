use utils::{
    byte_formatting::{u64s_to_bytes_be, ByteFormat},
    padding::bit_padding,
};

use crate::{Cipher, CipherError};

use super::block_cipher::{BCMode, BlockCipher};

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Default, Clone)]
pub struct Ascon128State {
    state: [u64; 5],
}

impl Ascon128State {
    const A: usize = 12; // initialization rounds
    const B: usize = 6; // block rounds
    const R: usize = 1; // 64 bits, a single word of state

    // Initialize with a constant derived from the key length in bits, rate in bits, initialization and finalization rounds, and intermediate rounds
    pub fn initialize(&mut self, key: [u64; 2], nonce: [u64; 2]) {
        self.state[0] = 0x80400c0600000000;
        self.state[1] = key[0];
        self.state[2] = key[1];
        self.state[3] = nonce[0];
        self.state[4] = nonce[1];
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
        println!("sbox:     {:016x?}", self.state);
        // linear diffusion
        self.linear_diffusor();
    }

    // The sbox works across words
    // It effectively takes the nth bit of each word, interprets those as a 5-bit word, then substitutes it
    pub fn sbox(&mut self) {
        self.state[0] ^= self.state[4];
        self.state[4] ^= self.state[3];
        self.state[2] ^= self.state[1];

        let mut t = self.state.clone();
        for i in 0..5 {
            t[i] ^= !self.state[(i + 1) % 5] & self.state[(i + 2) % 5];
        }

        t[1] ^= t[0];
        t[0] ^= t[4];
        t[3] ^= t[2];
        t[2] = !t[2];

        self.state = t;
    }

    // This diffuses bits within each word of state
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
    // pub padding: BCPadding, // only bit padding is allowed
    pub state: Ascon128State,
    pub associated_data: Vec<u8>,
}

impl Default for Ascon128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            mode: Default::default(),
            // padding: Default::default(), // only bit padding is allowed
            state: Default::default(),
            associated_data: Default::default(),
        }
    }
}

impl BlockCipher<8> for Ascon128 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block =
            u64::from_be_bytes(bytes.try_into().expect("invalid bytes for block encrypt"));

        u64s_to_bytes_be(bytes, &[block]);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block =
            u64::from_be_bytes(bytes.try_into().expect("invalid bytes for block encrypt"));

        u64s_to_bytes_be(bytes, &[block]);
    }
}

impl Cipher for Ascon128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| crate::errors::CipherError::input("byte format error"))?;
        bit_padding(&mut bytes, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut ad = self.associated_data.clone();
        bit_padding(&mut ad, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut state = Ascon128State::default();

        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| crate::errors::CipherError::input("byte format error"))?;

        let mut ad = self.associated_data.clone();
        bit_padding(&mut ad, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut state = Ascon128State::default();

        todo!()
    }
}
