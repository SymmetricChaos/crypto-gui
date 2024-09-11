use crate::{digital::block_ciphers::block_cipher::BCMode, errors::CipherError, Cipher};
use utils::byte_formatting::ByteFormat;

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

fn padded_bytes_to_u64s_be(bytes: &[u8]) -> [u64; 2] {
    if bytes.len() > 16 {
        panic!("input block was too large")
    } else if bytes.len() == 16 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
        ]
    } else if bytes.len() >= 8 {
        let word_0 = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        [word_0, padded_bytes_to_u64_be(&bytes[8..])]
    } else {
        [padded_bytes_to_u64_be(&bytes[0..]), 0x0000000000000000_u64]
    }
}

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

#[derive(Debug, Clone, Default)]
pub struct AsconState([u64; 5]);

// Shortcut indexing
impl std::ops::Index<usize> for AsconState {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for AsconState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl AsconState {
    // Initializae Ascon-128 with a key and nonce
    pub fn ascon_128(key: [u64; 2], nonce: [u64; 2]) -> Self {
        let mut out = Self([0x80400c0600000000, key[0], key[1], nonce[0], nonce[1]]);
        out.rounds_12();
        out[3] ^= key[0];
        out[4] ^= key[1];
        out
    }

    // Initializae Ascon-128a with a key and nonce
    pub fn ascon_128a(key: [u64; 2], nonce: [u64; 2]) -> Self {
        let mut out = Self([0x80800c0800000000, key[0], key[1], nonce[0], nonce[1]]);
        out.rounds_12();
        out[3] ^= key[0];
        out[4] ^= key[1];
        out
    }

    pub fn rounds_12(&mut self) {
        for i in 0..12 {
            self.transform(i as usize);
        }
    }

    pub fn rounds_8(&mut self) {
        for i in 0..8 {
            self.transform((i + 4) as usize);
        }
    }

    pub fn rounds_6(&mut self) {
        for i in 0..6 {
            self.transform((i + 6) as usize);
        }
    }

    fn transform(&mut self, i: usize) {
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

pub enum AsconVariant {
    Ascon128,
    Ascon128a,
    // Ascon80pq,
}

impl AsconVariant {
    pub fn initialize(&self, key: [u64; 2], nonce: [u64; 2]) -> AsconState {
        let mut out = match self {
            AsconVariant::Ascon128 => {
                AsconState([0x80400c0600000000, key[0], key[1], nonce[0], nonce[1]])
            }
            AsconVariant::Ascon128a => {
                AsconState([0x80800c0800000000, key[0], key[1], nonce[0], nonce[1]])
            }
        };
        out.rounds_12();
        out[3] ^= key[0];
        out[4] ^= key[1];
        out
    }

    pub fn rate(&self) -> usize {
        match self {
            AsconVariant::Ascon128 => 8,
            AsconVariant::Ascon128a => 16,
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
    pub variant: AsconVariant,
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
            variant: AsconVariant::Ascon128,
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

    pub fn ascon128() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            associated_data: Default::default(),
            subkeys: Default::default(),
            nonce: Default::default(),
            variant: AsconVariant::Ascon128,
        }
    }

    pub fn ascon128a() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            mode: Default::default(),
            associated_data: Default::default(),
            subkeys: Default::default(),
            nonce: Default::default(),
            variant: AsconVariant::Ascon128a,
        }
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

    pub fn ad(&mut self, ad: &[u8]) {
        self.associated_data = ad.to_owned();
    }

    pub fn with_ad(mut self, ad: &[u8]) -> Self {
        self.associated_data = ad.to_owned();
        self
    }

    pub fn ad_str(&mut self, ad: &str) {
        self.associated_data = ByteFormat::Hex
            .text_to_bytes(ad)
            .expect("bytes must be given as hex");
    }

    pub fn with_ad_str(mut self, ad: &str) -> Self {
        self.associated_data = ByteFormat::Hex
            .text_to_bytes(ad)
            .expect("bytes must be given as hex");
        self
    }

    fn b_round(&self, state: &mut AsconState) {
        match self.variant {
            AsconVariant::Ascon128 => state.rounds_6(),
            AsconVariant::Ascon128a => state.rounds_8(),
        }
    }

    fn xor_into_state(&self, state: &mut AsconState, bytes: &[u8]) {
        match self.variant {
            AsconVariant::Ascon128 => {
                state[0] ^= padded_bytes_to_u64_be(bytes);
            }
            AsconVariant::Ascon128a => {
                let [a, b] = padded_bytes_to_u64s_be(bytes);
                state[0] ^= a;
                state[1] ^= b;
            }
        }
    }

    fn absorb_ad(&self, state: &mut AsconState, rate: usize) {
        if !self.associated_data.is_empty() {
            let mut adlen = self.associated_data.len();
            let mut ptr = 0;
            // Absorb full blocks
            while adlen >= rate {
                self.xor_into_state(state, &self.associated_data[ptr..ptr + rate]);
                self.b_round(state);
                ptr += rate;
                adlen -= rate;
            }
            // Absorb the last padded blcok
            self.xor_into_state(state, &self.associated_data[ptr..]);
            self.b_round(state)
        }
    }

    fn finalize(&self, state: &mut AsconState) {
        match self.variant {
            AsconVariant::Ascon128 => {
                state[1] ^= self.subkeys[0];
                state[2] ^= self.subkeys[1];
            }
            AsconVariant::Ascon128a => {
                state[2] ^= self.subkeys[0];
                state[3] ^= self.subkeys[1];
            }
        }
        state.rounds_12();
        state[3] ^= self.subkeys[0];
        state[4] ^= self.subkeys[1];
    }

    fn encrypt_block(&self, state: &mut AsconState, ctext: &mut Vec<u8>) {
        match self.variant {
            AsconVariant::Ascon128 => ctext.extend(state[0].to_be_bytes()),
            AsconVariant::Ascon128a => {
                ctext.extend(state[0].to_be_bytes());
                ctext.extend(state[1].to_be_bytes());
            }
        }
    }

    fn decrypt_block(
        &self,
        state: &mut AsconState,
        message: &[u8],
        ptext: &mut Vec<u8>,
        ptr: usize,
    ) {
        match self.variant {
            AsconVariant::Ascon128 => {
                let c = u64::from_be_bytes(message[ptr..ptr + 8].try_into().unwrap());
                let p = state[0] ^ c;
                ptext.extend(p.to_be_bytes());
                state[0] = c;
            }
            AsconVariant::Ascon128a => {
                let c0 = u64::from_be_bytes(message[ptr..ptr + 8].try_into().unwrap());
                let c1 = u64::from_be_bytes(message[ptr + 8..ptr + 16].try_into().unwrap());
                let p0 = state[0] ^ c0;
                let p1 = state[1] ^ c1;
                ptext.extend(p0.to_be_bytes());
                ptext.extend(p1.to_be_bytes());
                state[0] = c0;
                state[1] = c1;
            }
        }
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = self.variant.initialize(self.subkeys, self.nonce);
        let rate = self.variant.rate();

        // Absorb associated data if it is provided
        self.absorb_ad(&mut state, rate);

        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = bytes.len();
        let mut ptr = 0;
        let mut ctext = Vec::new();
        // Absorb full blocks
        while mlen >= rate {
            self.xor_into_state(&mut state, &bytes[ptr..ptr + rate]);
            self.encrypt_block(&mut state, &mut ctext);
            ptr += rate;
            mlen -= rate;
            self.b_round(&mut state)
        }
        // Absorb the last padded block
        self.xor_into_state(&mut state, &bytes[ptr..]);
        self.encrypt_block(&mut state, &mut ctext);
        ctext.truncate(bytes.len());

        // Finalize and create the authentication tag
        self.finalize(&mut state);
        ctext.extend(state[3].to_be_bytes());
        ctext.extend(state[4].to_be_bytes());

        ctext
    }

    pub fn decrypt_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>, CipherError> {
        if bytes.len() < 16 {
            return Err(CipherError::general(
                "authentication failed, message too short",
            ));
        }

        let mut state = self.variant.initialize(self.subkeys, self.nonce);
        let rate = self.variant.rate();

        // Absorb associated data if it is provided
        self.absorb_ad(&mut state, rate);

        // Flip the last bit, this is described as domain separation
        state[4] ^= 1;

        // Split off the tag then decrypt the ciphertext
        let (message, tag) = bytes.split_at(bytes.len() - 16);
        let mut ptext = Vec::new();
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            self.decrypt_block(&mut state, message, &mut ptext, ptr);
            ptr += rate;
            mlen -= rate;
            self.b_round(&mut state)
        }
        // Decrypt and absorb the last block. This is
        match self.variant {
            AsconVariant::Ascon128 => {
                let c = padded_bytes_to_u64_be(&message[ptr..]);
                let p = state[0] ^ c;
                ptext.extend(p.to_be_bytes());
                ptext.truncate(bytes.len() - 16);
                self.xor_into_state(&mut state, &p.to_be_bytes()[0..mlen]);
            }
            AsconVariant::Ascon128a => {
                let [c0, c1] = padded_bytes_to_u64s_be(&message[ptr..]);
                // let c0 = u64::from_be_bytes(message[ptr..ptr + 8].try_into().unwrap());
                // let c1 = u64::from_be_bytes(message[ptr + 8..ptr + 16].try_into().unwrap());
                let p0 = state[0] ^ c0;
                let p1 = state[1] ^ c1;
                ptext.extend(p0.to_be_bytes());
                ptext.extend(p1.to_be_bytes());
                ptext.truncate(bytes.len() - 16);
                let mut b = 0u128;
                b |= p0 as u128;
                b <<= 64;
                b |= p1 as u128;
                self.xor_into_state(&mut state, &b.to_be_bytes()[0..mlen]);
            }
        }

        // Finalize and check tag
        self.finalize(&mut state);

        let mut t: [u8; 16] = [0; 16];
        t[0..8].copy_from_slice(&state[3].to_be_bytes());
        t[8..16].copy_from_slice(&state[4].to_be_bytes());

        if t == tag {
            Ok(ptext)
        } else {
            // println!("{:02x?}", ptext);
            Err(CipherError::general("authentication failed"))
        }
    }
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
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.decrypt_bytes(&bytes)?))
    }
}

#[cfg(test)]
mod ascon_tests {

    use super::*;

    fn ascon128_test(ptext: &str, ctext: &str, ad: &str) {
        let cipher = Ascon128::ascon128()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_ad_str(ad);
        let otext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ctext, otext, "encrypt failed");
        let otext = cipher.decrypt(ctext).unwrap();
        assert_eq!(ptext, otext, "decrypt failed");
    }

    fn ascon128a_test(ptext: &str, ctext: &str, ad: &str) {
        let cipher = Ascon128::ascon128a()
            .with_key([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_nonce([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ])
            .with_ad_str(ad);
        let otext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ctext, otext, "encrypt failed");
        let otext = cipher.decrypt(ctext).unwrap();
        assert_eq!(ptext, otext, "decrypt failed");
    }
    #[test]
    fn ascon128_0_0() {
        ascon128_test("", "e355159f292911f794cb1432a0103a8a", "")
    }

    #[test]
    fn ascon128_2_0() {
        ascon128_test("0001", "bc82d5bde868f7494f57d81e06facbf70ce1", "")
    }

    #[test]
    fn ascon128_7_0() {
        ascon128_test(
            "00010203040506",
            "bc820dbdf7a463ce9985966c40bc56a9c5180e23f7086c",
            "",
        )
    }

    #[test]
    fn ascon128_8_0() {
        ascon128_test(
            "0001020304050607",
            "bc820dbdf7a4631c01a8807a44254b42ac6bb490da1e000a",
            "",
        )
    }

    #[test]
    fn ascon128_12_0() {
        ascon128_test(
            "000102030405060708090a0b",
            "bc820dbdf7a4631c5b29884a7d1c07dc8d0d5ed48e64d7dcb25c325f",
            "",
        )
    }

    #[test]
    fn ascon128_0_1() {
        ascon128_test("", "944df887cd4901614c5dedbc42fc0da0", "00")
    }

    #[test]
    fn ascon128_0_8() {
        ascon128_test("", "e3dcf95f869752f61cd7a2db895f918e", "0001020304050607")
    }

    #[test]
    fn ascon128_2_2() {
        ascon128_test("0001", "6e9f373c0b74264c1ce4d705d995915fcccd", "0001")
    }

    #[test]
    fn ascon128_64_64() {
        ascon128_test(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "b96c78651b6246b0c3b1a5d373b0d5168dca4a96734cf0ddf5f92f8d15e30270279bf6a6cc3f2fc9350b915c292bdb8d",
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        )
    }

    #[test]
    fn ascon128a_0_0() {
        ascon128a_test("", "7a834e6f09210957067b10fd831f0078", "")
    }

    #[test]
    fn ascon128a_2_0() {
        ascon128a_test("0001", "6e490868e32cb041a71ca5e41b615ce11c4e", "")
    }

    #[test]
    fn ascon128a_7_0() {
        ascon128a_test(
            "00010203040506",
            "6e490cfed5b35449f1bd8ab58546aa5ffa2fee5afe13a4",
            "",
        )
    }

    #[test]
    fn ascon128a_8_0() {
        ascon128a_test(
            "0001020304050607",
            "6e490cfed5b35467b89c7e12863ce5f76afc808fff786b9e",
            "",
        )
    }

    #[test]
    fn ascon128a_12_0() {
        ascon128a_test(
            "000102030405060708090a0b",
            "6e490cfed5b3546767350cd83e9b1bfeb72dd5bacf71810b946fbe03",
            "",
        )
    }

    #[test]
    fn ascon128a_0_1() {
        ascon128a_test("", "af3031b07b129ec84153373ddcaba528", "00")
    }

    #[test]
    fn ascon128a_0_8() {
        ascon128a_test("", "d60e199ffd3f9b694713dabc6d89f46f", "0001020304050607")
    }

    #[test]
    fn ascon128a_2_2() {
        ascon128a_test("0001", "abe4c55426e24a56bb77f8e0bd9212fe8d29", "0001")
    }

    #[test]
    fn ascon128a_64_64() {
        ascon128a_test(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "a55236ac020dbda74ce6ccd10c68c4d8514450a382bc87c68946d86a921dd88e2adddfbbe77d4112830e01960b9d38d5",
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        )
    }
}
