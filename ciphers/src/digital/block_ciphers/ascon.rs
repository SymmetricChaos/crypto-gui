use crate::{
    digital::block_ciphers::block_cipher::{BCMode, BlockCipher},
    errors::CipherError,
    Cipher,
};
use utils::{
    byte_formatting::{u64s_to_bytes_be, ByteFormat},
    padding::bit_padding,
};

fn bytes_to_u64_be(bytes: &[u8]) -> Vec<u64> {
    assert!(
        bytes.len() % 8 == 0,
        "must have a length that is a multiple of eight bytes"
    );
    let output_len = bytes.len() / 8;
    let mut out = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let mut word_bits: [u8; 8] = Default::default();
        word_bits.copy_from_slice(&bytes[(i * 8)..(i * 8 + 8)]);
        out.push(u64::from_be_bytes(word_bits));
    }
    out
}

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone, Default)]
pub struct AsconState {
    state: [u64; 5],
    _k: u8, // not used in this implementation
    r: u8,
    a: u8,
    b: u8,
}

// Shortcut indexing
impl std::ops::Index<usize> for AsconState {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}

impl std::ops::IndexMut<usize> for AsconState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.state[index]
    }
}

impl AsconState {
    // Initializae Ascon-128 with a key and nonce
    pub fn ascon_128(key: [u64; 2], nonce: [u64; 2]) -> Self {
        let mut out = Self {
            state: [0x80400c0600000000, key[0], key[1], nonce[0], nonce[1]],
            _k: 128,
            r: 64,
            a: 12,
            b: 6,
        };
        out.rounds_a();
        out[3] ^= key[0];
        out[4] ^= key[1];
        out
    }

    // Initializae Ascon-128a with a key and nonce
    pub fn ascon_128a(key: [u64; 2], nonce: [u64; 2]) -> Self {
        let mut out = Self {
            state: [0x80800c0800000000, key[0], key[1], nonce[0], nonce[1]],
            _k: 128,
            r: 128,
            a: 12,
            b: 8,
        };
        out.rounds_a();
        out[3] ^= key[0];
        out[4] ^= key[1];
        out
    }

    pub fn rounds_a(&mut self) {
        for i in 0..self.a {
            self.transform(i as usize);
        }
    }

    pub fn rounds_b(&mut self) {
        for i in 0..self.b {
            self.transform(i as usize);
        }
    }

    pub fn transform(&mut self, i: usize) {
        // round constant
        self[2] ^= C[i];
        // substitution
        self.sbox();
        // linear diffusion
        self.linear_diffusor();
    }

    // The sbox works across words
    // It effectively take the nth bit of each word, interprets it as a 5-bit word, then substitutes it
    pub fn sbox(&mut self) {
        self[0] ^= self[4];
        self[4] ^= self[3];
        self[2] ^= self[1];

        let mut t = self.clone();
        for i in 0..5 {
            t[i] ^= !self[(i + 1) % 5] & self[(i + 2) % 5];
        }

        t[1] ^= t[0];
        t[0] ^= t[4];
        t[3] ^= t[2];
        t[2] = !t[2];

        *self = t;
    }

    // This diffuses bits within each word of state
    pub fn linear_diffusor(&mut self) {
        for i in 0..5 {
            self[i] ^= self[i].rotate_right(ROTS[i].0) ^ self[i].rotate_right(ROTS[i].1);
        }
    }

    // pub fn absorb(&mut self, message: &[u8]) {
    //     assert!(
    //         message.len() % self.r as usize == 0,
    //         "message length in bytes must be a multiple of {}",
    //         self.r
    //     );
    //     let words = bytes_to_u64_be(message);

    //     for word in words {
    //         self[0] ^= word;
    //         self.rounds_a();
    //     }
    // }

    // pub fn squeeze(&mut self, hash_len: usize) -> Vec<u8> {
    //     let mut output = Vec::with_capacity(hash_len);

    //     while output.len() < hash_len {
    //         output.extend_from_slice(&self[0].to_be_bytes());
    //         self.rounds_a();
    //     }

    //     output.truncate(hash_len);
    //     output
    // }
}

pub struct Ascon128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    // pub padding: BCPadding, // only bit padding is allowed
    pub iv: u64,
    pub state: AsconState,
    pub associated_data: Vec<u8>,
    pub subkeys: [u64; 2],
    pub nonce: [u64; 2],
}

impl Default for Ascon128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            iv: 0,

            state: Default::default(),
            associated_data: Default::default(),
            subkeys: Default::default(),
            nonce: Default::default(),
        }
    }
}

impl Ascon128 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn mode(mut self, mode: BCMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn ksa(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u64s_be(&mut self.subkeys, &bytes);
    }

    pub fn with_key(mut self, key: [u8; 16]) -> Self {
        self.ksa(key);
        self
    }

    pub fn nonce(&mut self, bytes: [u8; 16]) {
        utils::byte_formatting::fill_u64s_be(&mut self.nonce, &bytes);
    }

    pub fn with_nonce(mut self, key: [u8; 16]) -> Self {
        self.nonce(key);
        self
    }
}

// impl BlockCipher<8> for Ascon128 {
//     fn encrypt_block(&self, bytes: &mut [u8]) {
//         let mut block =
//             u64::from_be_bytes(bytes.try_into().expect("invalid bytes for block encrypt"));

//         u64s_to_bytes_be(bytes, &[block]);
//     }

//     fn decrypt_block(&self, bytes: &mut [u8]) {
//         let mut block =
//             u64::from_be_bytes(bytes.try_into().expect("invalid bytes for block encrypt"));

//         u64s_to_bytes_be(bytes, &[block]);
//     }
// }

impl Cipher for Ascon128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let last_block_len = bytes.len() % 8;
        bit_padding(&mut bytes, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut ad = self.associated_data.clone();
        bit_padding(&mut ad, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut state = AsconState::ascon_128(self.subkeys, self.nonce);

        // Absorb associated data
        for chunk in bytes_to_u64_be(&ad) {
            state[0] ^= chunk;
            state.rounds_b();
        }
        state[5] ^= 1;

        // Encrypt the plaintext except the last block
        let mut ctext = Vec::new();
        let ptext = bytes_to_u64_be(&bytes);
        for chunk in ptext.iter().take(ptext.len() - 1) {
            state[0] ^= chunk;
            ctext.extend(state[0].to_be_bytes());
            state.rounds_b();
        }
        // Encrypt the last block then truncate it to the length of the input
        state[0] ^= ptext.last().expect("there is alwways at least one chunk");
        ctext.extend_from_slice(&state[0].to_be_bytes()[0..last_block_len]);

        // Finalize and create the authentication tag

        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        let last_block_len = bytes.len() % 8;
        bit_padding(&mut bytes, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut ad = self.associated_data.clone();
        bit_padding(&mut ad, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut state = AsconState::ascon_128(self.subkeys, self.nonce);

        // Absorb associated data
        for chunk in bytes_to_u64_be(&ad) {
            state[0] ^= chunk;
            state.rounds_b();
        }
        state[5] ^= 1;

        // Decrypt the plaintext except the last block
        let mut ptext = Vec::new();
        let ctext = bytes_to_u64_be(&bytes);
        for chunk in ctext.iter().take(ctext.len() - 1) {
            state[0] ^= chunk;
            ptext.extend(state[0].to_be_bytes());
            state.rounds_b();
        }
        // Decrypt the last block then truncate it to the length of the input
        state[0] ^= ctext.last().expect("there is alwways at least one chunk");
        ptext.extend_from_slice(&state[0].to_be_bytes()[0..last_block_len]);

        // Finalize and check the authentication tag
        todo!()
    }
}
