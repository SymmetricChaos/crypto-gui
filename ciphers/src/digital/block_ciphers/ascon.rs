use crate::{digital::block_ciphers::block_cipher::BCMode, errors::CipherError, Cipher};
use utils::{byte_formatting::ByteFormat, padding::bit_padding};

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

fn padded_bytes_to_u64_be(bytes: &[u8]) -> u64 {
    if bytes.len() > 8 {
        panic!("input block was too large")
    } else if bytes.len() == 8 {
        u64::from_be_bytes(bytes.try_into().unwrap())
    } else {
        let mut word_bytes: [u8; 8] = [0; 8];
        for (word_byte, input_byte) in word_bytes.iter_mut().zip(bytes.iter()) {
            *word_byte = *input_byte;
        }
        word_bytes[bytes.len()] = 0x80;
        u64::from_be_bytes(word_bytes)
    }
}

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone, Default)]
pub struct AsconState {
    state: [u64; 5],
    _k: u8, // not used in this implementation
    _r: u8,
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
            _r: 64,
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
            _r: 128,
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
}

pub struct Ascon128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub mode: BCMode,
    // pub padding: BCPadding, // only bit padding is allowed
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

    pub fn with_ad(mut self, ad: &[u8]) -> Self {
        self.associated_data = ad.to_owned();
        self
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = AsconState::ascon_128(self.subkeys, self.nonce);

        // Absorb associated data if it is provided
        if !self.associated_data.is_empty() {
            for chunk in self.associated_data.chunks(8) {
                println!("{:016x?}", padded_bytes_to_u64_be(chunk));
                state[0] ^= padded_bytes_to_u64_be(chunk);
                state.rounds_b();
            }
        }
        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Encrypt the plaintext treating the last block specially
        let mut ctext = Vec::new();
        let chunks = bytes.chunks(8);
        let n_chunks = chunks.len();

        if n_chunks == 0 {
            state[0] ^= padded_bytes_to_u64_be(&[]);
        } else {
            for chunk in chunks.clone().take(n_chunks - 1) {
                state[0] ^= padded_bytes_to_u64_be(chunk);
                ctext.extend(state[0].to_be_bytes());
                state.rounds_b();
            }

            // Encrypt the last block then truncate it to the length of the input
            let last_chunk = chunks.last().expect("there is always at least one block");
            let last_chunk_len = last_chunk.len();
            state[0] ^= padded_bytes_to_u64_be(last_chunk);
            ctext.extend_from_slice(&state[0].to_be_bytes()[0..last_chunk_len]);
        }

        // Finalize and create the authentication tag
        state[1] ^= self.subkeys[0];
        state[2] ^= self.subkeys[1];
        state.rounds_a();
        ctext.extend((state[3] ^ self.subkeys[0]).to_be_bytes());
        ctext.extend((state[4] ^ self.subkeys[1]).to_be_bytes());

        ctext
    }

    // pub fn decrypt_bytes(&self, bytes: &[u8]) {}
}

impl Cipher for Ascon128 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.encrypt_bytes(&bytes)))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        let mut state = AsconState::ascon_128(self.subkeys, self.nonce);

        // Absorb associated data if it is provided
        if !self.associated_data.is_empty() {
            for chunk in self.associated_data.chunks(8) {
                state[0] ^= padded_bytes_to_u64_be(chunk);
                state.rounds_b();
            }
        }
        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Decrypt the plaintext except the last block
        let last_block_len = bytes.len() % 8;
        bit_padding(&mut bytes, 8).map_err(|e| CipherError::General(e.to_string()))?;

        let mut ptext = Vec::new();
        let ctext = bytes_to_u64_be(&bytes);
        for chunk in ctext.iter().take(ctext.len() - 1) {
            state[0] ^= chunk;
            ptext.extend(state[0].to_be_bytes());
            state.rounds_b();
        }
        // Decrypt the last block then truncate it to the length of the input
        state[0] ^= ctext.last().expect("there is always at least one chunk");
        ptext.extend_from_slice(&state[0].to_be_bytes()[0..last_block_len]);

        // Finalize, check, and remove the authentication tag
        let (message, tag) = ptext.split_at(ptext.len() - 16);
        state[1] ^= self.subkeys[0];
        state[2] ^= self.subkeys[1];
        state.rounds_a();

        let x = (state[3] ^ self.subkeys[0]).to_be_bytes();
        let y = (state[4] ^ self.subkeys[1]).to_be_bytes();
        println!("{:02x?} {:02x?}", x, y);
        println!("{:02x?}", tag);
        // TODO

        // Output as text
        Ok(self.output_format.byte_slice_to_text(message))
    }
}

#[cfg(test)]
mod ascon_tests {

    use super::*;

    #[test]
    fn ascon128_encrypt_0_0() {
        let cipher = Ascon128::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ]);
        let ptext = "";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!("e355159f292911f794cb1432a0103a8a", ctext);
    }

    #[test]
    fn ascon128_encrypt_2_0() {
        let cipher = Ascon128::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ]);
        let ptext = "0001";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!("bc82d5bde868f7494f57d81e06facbf70ce1", ctext);
    }

    #[test]
    fn ascon128_encrypt_7_0() {
        let cipher = Ascon128::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ]);
        let ptext = "00010203040506";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!("bc820dbdf7a463ce9985966c40bc56a9c5180e23f7086c", ctext);
    }

    #[test]
    fn ascon128_encrypt_8_0() {
        let cipher = Ascon128::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ]);
        let ptext = "0001020304050607";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!("bc820dbdf7a4631c01a8807a44254b42ac6bb490da1e000a", ctext);
    }

    #[test]
    fn ascon128_encrypt_12_0() {
        let cipher = Ascon128::default()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ]);
        let ptext = "000102030405060708090A0B";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(
            "bc820dbdf7a4631c5b29884a7d1c07dc8d0d5ed48e64d7dcb25c325f",
            ctext
        );
    }

    // #[test]
    // fn ascon128_decrypt_0_0() {
    //     let cipher = Ascon128::default()
    //         .with_key([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_nonce([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ]);
    //     let ctext = "e355159f292911f794cb1432a0103a8a";
    //     let ptext = cipher.decrypt(ctext).unwrap();
    //     assert_eq!("", ptext);
    // }

    // #[test]
    // fn ascon128_encrypt_0_1() {
    //     let cipher = Ascon128::default()
    //         .with_key([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_nonce([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_ad(&[0x00]);
    //     let ptext = "";
    //     let ctext = cipher.encrypt(ptext).unwrap();
    //     assert_eq!("944df887cd4901614c5dedbc42fc0da0", ctext);
    // }

    // #[test]
    // fn ascon128_decrypt_2_0() {
    //     let cipher = Ascon128::default()
    //         .with_key([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_nonce([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ]);
    //     let ctext = "bc82d5bde868f7494f57d81e06facbf70ce1";
    //     let ptext = cipher.decrypt(ctext).unwrap();
    //     assert_eq!("0001", ptext);
    // }

    // #[test]
    // fn ascon128_encrypt_2_2() {
    //     let cipher = Ascon128::default()
    //         .with_key([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_nonce([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_ad(&[0x00, 0x01]);
    //     let ptext = "0001";
    //     let ctext = cipher.encrypt(ptext).unwrap();
    //     assert_eq!("6e9f373c0b74264c1ce4d705d995915fcccd", ctext);
    // }

    // #[test]
    // fn ascon128_encrypt_64_64() {
    //     let cipher = Ascon128::default()
    //         .with_key([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_nonce([
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F,
    //         ])
    //         .with_ad(&[
    //             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
    //             0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
    //             0x1C, 0x1D, 0x1E, 0x1F,
    //         ]);
    //     let ptext = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    //     let ctext = cipher.encrypt(ptext).unwrap();
    //     assert_eq!("b96c78651b6246b0c3b1a5d373b0d5168dca4a96734cf0ddf5f92f8d15e30270279bf6a6cc3f2fc9350b915c292bdb8d", ctext);
    // }
}
