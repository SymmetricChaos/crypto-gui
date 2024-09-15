pub mod ascon_hash;
pub mod ascon_prf;

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

#[derive(Debug, Clone)]
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
    const RATE: usize = 8; // number of bytes absorbed at a time

    // Initial state for Ascon-Hash
    pub fn initialize(iv: u64) -> Self {
        let mut out = Self([iv, 0, 0, 0, 0]);
        out.rounds_12();
        out
    }

    pub fn initialize_full(state: [u64; 5]) -> Self {
        let mut out = Self(state);
        out.rounds_12();
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

    pub fn absorb_8_128(&mut self, message: &[u8]) {
        let rate = Self::RATE;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            self[0] ^= padded_bytes_to_u64_be(&message[ptr..ptr + rate]);
            ptr += rate;
            mlen -= rate;
            self.rounds_8()
        }
        // Absorb the last padded block
        self[0] ^= padded_bytes_to_u64_be(&message[ptr..]);
        self.rounds_12();
    }

    pub fn absorb_12_128(&mut self, message: &[u8]) {
        let rate = Self::RATE;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            self[0] ^= padded_bytes_to_u64_be(&message[ptr..ptr + rate]);
            ptr += rate;
            mlen -= rate;
            self.rounds_12()
        }
        // Absorb the last padded block
        self[0] ^= padded_bytes_to_u64_be(&message[ptr..]);
        self.rounds_12();
    }

    pub fn absorb_12_256(&mut self, message: &[u8]) {
        let rate = Self::RATE * 2;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            let [a, b] = padded_bytes_to_u64s_be(&message[ptr..ptr + rate]);
            self[0] ^= a;
            self[1] ^= b;
            ptr += rate;
            mlen -= rate;
            self.rounds_12()
        }
        // Absorb the last padded block
        let [a, b] = padded_bytes_to_u64s_be(&message[ptr..]);
        self[0] ^= a;
        self[1] ^= b;
        self.rounds_12();
    }

    pub fn squeeze_8(&mut self, hash_len: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            output.extend_from_slice(&self[0].to_be_bytes());
            self.rounds_8();
        }

        output.truncate(hash_len);
        output
    }

    pub fn squeeze_12(&mut self, hash_len: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            output.extend_from_slice(&self[0].to_be_bytes());
            self.rounds_12();
        }

        output.truncate(hash_len);
        output
    }
}
