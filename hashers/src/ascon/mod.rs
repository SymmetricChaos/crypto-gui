use strum::EnumIter;

pub mod hash;
pub mod mac;
pub mod tests;

const DEBUG: bool = false;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Variant {
    Hash,
    Hasha,
    Xof,
    Xofa,
    Mac,
    Maca,
    Prf,
    Prfa,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hash => write!(f, "Ascon-Hash"),
            Self::Hasha => write!(f, "Ascon-Hasha"),
            Self::Xof => write!(f, "Ascon-XOF"),
            Self::Xofa => write!(f, "Ascon-XOFa"),
            Self::Mac => write!(f, "Ascon-MAC"),
            Self::Maca => write!(f, "Ascon-MACa"),
            Self::Prf => write!(f, "Ascon-XOF"),
            Self::Prfa => write!(f, "Ascon-XOFa"),
        }
    }
}

impl Variant {
    pub fn a(&self) -> usize {
        match self {
            Variant::Hash => 12,
            Variant::Hasha => 8,
            Variant::Xof => 12,
            Variant::Xofa => 8,
            Variant::Mac => 12,
            Variant::Maca => 8,
            Variant::Prf => 12,
            Variant::Prfa => 8,
        }
    }

    pub fn b(&self) -> usize {
        match self {
            Variant::Hash => 12,
            Variant::Hasha => 8,
            Variant::Xof => 12,
            Variant::Xofa => 8,
            Variant::Mac => 12,
            Variant::Maca => 12,
            Variant::Prf => 12,
            Variant::Prfa => 12,
        }
    }

    pub fn rate(&self) -> usize {
        match self {
            Variant::Hash => 8,
            Variant::Hasha => 8,
            Variant::Xof => 8,
            Variant::Xofa => 8,
            Variant::Mac => 32,
            Variant::Maca => 40,
            Variant::Prf => 32,
            Variant::Prfa => 40,
        }
    }
}

fn padded_bytes_64(bytes: &[u8]) -> u64 {
    assert!(bytes.len() <= 8);
    if bytes.len() == 8 {
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

fn unpadded_bytes_64(bytes: &[u8]) -> u64 {
    assert!(bytes.len() <= 8);
    if bytes.len() == 8 {
        u64::from_be_bytes(bytes.try_into().unwrap())
    } else {
        let mut word_bytes: [u8; 8] = [0; 8];
        for (word_byte, input_byte) in word_bytes.iter_mut().zip(bytes.iter()) {
            *word_byte = *input_byte;
        }
        u64::from_be_bytes(word_bytes)
    }
}

fn unpadded_bytes_128(bytes: &[u8]) -> [u64; 2] {
    assert!(bytes.len() <= 16);
    if bytes.len() == 16 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
        ]
    } else if bytes.len() >= 8 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            unpadded_bytes_64(&bytes[8..]),
        ]
    } else {
        [unpadded_bytes_64(&bytes[0..]), 0]
    }
}

// fn padded_bytes_128(bytes: &[u8]) -> [u64; 2] {
//     if bytes.len() > 16 {
//         panic!("input block was too large")
//     } else if bytes.len() == 16 {
//         [
//             u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
//             u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
//         ]
//     } else if bytes.len() >= 8 {
//         let word_0 = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
//         [word_0, padded_bytes_64(&bytes[8..])]
//     } else {
//         [padded_bytes_64(&bytes[0..]), 0x0000000000000000_u64]
//     }
// }

fn padded_bytes_256(bytes: &[u8]) -> [u64; 4] {
    assert!(bytes.len() <= 32);
    if bytes.len() == 32 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_be_bytes(bytes[24..32].try_into().unwrap()),
        ]
    } else if bytes.len() >= 24 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            padded_bytes_64(&bytes[24..]),
        ]
    } else if bytes.len() >= 16 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            padded_bytes_64(&bytes[16..]),
            0,
        ]
    } else if bytes.len() >= 8 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            padded_bytes_64(&bytes[8..]),
            0,
            0,
        ]
    } else {
        [padded_bytes_64(&bytes[0..]), 0, 0, 0]
    }
}

fn padded_bytes_320(bytes: &[u8]) -> [u64; 5] {
    assert!(bytes.len() <= 40);
    if bytes.len() == 40 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_be_bytes(bytes[24..32].try_into().unwrap()),
            u64::from_be_bytes(bytes[32..40].try_into().unwrap()),
        ]
    } else if bytes.len() >= 32 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_be_bytes(bytes[24..32].try_into().unwrap()),
            padded_bytes_64(&bytes[32..]),
        ]
    } else if bytes.len() >= 24 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            padded_bytes_64(&bytes[24..]),
            0,
        ]
    } else if bytes.len() >= 16 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            padded_bytes_64(&bytes[16..]),
            0,
            0,
        ]
    } else if bytes.len() >= 8 {
        [
            u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            padded_bytes_64(&bytes[8..]),
            0,
            0,
            0,
        ]
    } else {
        [padded_bytes_64(&bytes[0..]), 0, 0, 0, 0]
    }
}

const C: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

const ROTS: [(u32, u32); 5] = [(19, 28), (61, 39), (1, 6), (10, 17), (7, 41)];
const RATE: usize = 8; // number of bytes absorbed at a time
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
    // Initial state for Ascon-Hash
    pub fn initialize(iv: u64) -> Self {
        let mut out = Self([iv, 0, 0, 0, 0]);
        if DEBUG {
            println!("initial val: {:016x?}", out);
        }
        out.rounds_12();
        if DEBUG {
            println!("initialized: {:016x?}", out);
        }
        out
    }

    pub fn initialize_full(state: [u64; 5]) -> Self {
        let mut out = Self(state);
        if DEBUG {
            println!("initial val: {:016x?}", out);
        }
        out.rounds_12();
        if DEBUG {
            println!("initialized: {:016x?}", out);
        }
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

    pub fn rounds(&mut self, n: usize) {
        match n {
            6 => self.rounds_6(),
            8 => self.rounds_8(),
            12 => self.rounds_12(),
            _ => panic!("only round counts of 6, 8, and 12 are allowed"),
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

    pub fn absorb_64_hash(&mut self, message: &[u8], a: usize) {
        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= RATE {
            self[0] ^= padded_bytes_64(&message[ptr..ptr + RATE]);
            ptr += RATE;
            mlen -= RATE;
            self.rounds(a);

            if DEBUG {
                println!("medial_absorb_128:   {:016x?}", self);
            }
        }
        // Absorb the last padded block
        self[0] ^= padded_bytes_64(&message[ptr..]);
        self.rounds_12();
        if DEBUG {
            println!("final_absorb_128: {:016x?}", self);
        }
    }

    pub fn absorb_256_prf(&mut self, message: &[u8], a: usize) {
        let rate = 32;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            let [x0, x1, x2, x3] = padded_bytes_256(&message[ptr..ptr + RATE]);
            self[0] ^= x0;
            self[1] ^= x1;
            self[2] ^= x2;
            self[3] ^= x3;
            if DEBUG {
                println!("msg xored in: {:016x?}", self);
            }
            ptr += rate;
            mlen -= rate;
            self.rounds(a);
            if DEBUG {
                println!("medial_absorb_256:   {:016x?}", self);
            }
        }
        // Absorb the last padded block
        let [x0, x1, x2, x3] = padded_bytes_256(&message[ptr..]);
        self[0] ^= x0;
        self[1] ^= x1;
        self[2] ^= x2;
        self[3] ^= x3;
        if DEBUG {
            println!("msg xored in: {:016x?}", self);
        }
        self[4] ^= 1;
        if DEBUG {
            println!("final_absorb_256: {:016x?}", self);
        }
    }

    pub fn absorb_320_prf(&mut self, message: &[u8], a: usize) {
        let rate = 40;

        // Encrypt the plaintext treating the last block specially
        let mut mlen = message.len();
        let mut ptr = 0;
        // Absorb full blocks
        while mlen >= rate {
            let [x0, x1, x2, x3, x4] = padded_bytes_320(&message[ptr..ptr + rate]);
            self[0] ^= x0;
            self[1] ^= x1;
            self[2] ^= x2;
            self[3] ^= x3;
            self[4] ^= x4;
            if DEBUG {
                println!("msg xored in: {:016x?}", self);
            }
            ptr += rate;
            mlen -= rate;
            self.rounds(a);
            if DEBUG {
                println!("medial_absorb_{}_320:   {:016x?}", a, self);
            }
        }
        // Absorb the last padded block
        let [x0, x1, x2, x3, x4] = padded_bytes_320(&message[ptr..]);
        self[0] ^= x0;
        self[1] ^= x1;
        self[2] ^= x2;
        self[3] ^= x3;
        self[4] ^= x4;
        if DEBUG {
            println!("msg xored in: {:016x?}", self);
        }
        self[4] ^= 1;
        if DEBUG {
            println!("final_absorb_{}_320: {:016x?}", a, self);
        }
    }

    pub fn squeeze_64_hash(&mut self, hash_len: usize, a: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            output.extend_from_slice(&self[0].to_be_bytes());
            self.rounds(a);
            if DEBUG {
                println!("medial_squeeze_64: {:016x?}", self);
            }
        }

        output.truncate(hash_len);
        output
    }

    pub fn squeeze_128_prf(&mut self, hash_len: usize, a: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        while output.len() < hash_len {
            self.rounds(a);
            if DEBUG {
                println!("squeeze_{}_128: {:016x?}", a, self);
            }
            output.extend_from_slice(&self[0].to_be_bytes());
            output.extend_from_slice(&self[1].to_be_bytes());
        }

        output.truncate(hash_len);
        output
    }

    pub fn squeeze_128_prfshort(&mut self, hash_len: usize, key: [u64; 2]) -> Vec<u8> {
        let mut output = Vec::with_capacity(hash_len);

        self[3] ^= key[0];
        self[4] ^= key[1];
        if DEBUG {
            println!("squeeze_128_prfshort: {:016x?}", self);
        }
        output.extend_from_slice(&self[3].to_be_bytes());
        output.extend_from_slice(&self[4].to_be_bytes());

        output.truncate(hash_len);
        output
    }
}
