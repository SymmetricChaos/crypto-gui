pub mod ascon128;
pub mod ascon80pq;

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];

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

    // Initializae Ascon-80pq with a key and nonce
    pub fn ascon_80pq(key: [u64; 3], nonce: [u64; 2]) -> Self {
        let mut out = Self([
            0xa0400c0600000000 | key[0],
            key[1],
            key[2],
            nonce[0],
            nonce[1],
        ]);
        out.rounds_12();
        out[2] ^= key[0];
        out[3] ^= key[1];
        out[4] ^= key[2];
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

#[derive(Debug, PartialEq, Eq)]
pub enum Ascon128Variant {
    Ascon128,
    Ascon128a,
}

impl std::fmt::Display for Ascon128Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascon128 => write!(f, "Ascon-128"),
            Self::Ascon128a => write!(f, "Ascon-128a"),
        }
    }
}

impl Ascon128Variant {
    pub fn initialize(&self, key: [u64; 2], nonce: [u64; 2]) -> AsconState {
        match self {
            Self::Ascon128 => AsconState::ascon_128(key, nonce),
            Self::Ascon128a => AsconState::ascon_128a(key, nonce),
        }
    }

    pub fn rate(&self) -> usize {
        match self {
            Ascon128Variant::Ascon128 => 8,
            Ascon128Variant::Ascon128a => 16,
        }
    }

    pub fn key_size(&self) -> usize {
        match self {
            Ascon128Variant::Ascon128 => 16,
            Ascon128Variant::Ascon128a => 16,
        }
    }
}
