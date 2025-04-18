use crate::traits::StatefulHasher;

const ROWS: usize = 8;
const COLS_512: usize = 8;
const COLS_1024: usize = 16;
const ROUNDS_512: usize = 10;
const ROUNDS_1024: usize = 14;

pub const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

// #define mul1(b) ((u8)(b))
#[inline(always)]
fn mul1(byte: u8) -> u8 {
    byte
}

// #define mul2(b) ((u8)((b)>>7?((b)<<1)^0x1b:((b)<<1)))
#[inline(always)]
fn mul2(byte: u8) -> u8 {
    if byte >> 7 == 1 {
        (byte << 1) ^ 0x1b
    } else {
        byte << 1
    }
}

// #define mul3(b) (mul2(b)^mul1(b))
#[inline(always)]
fn mul3(byte: u8) -> u8 {
    mul2(byte) ^ mul1(byte)
}

// #define mul4(b) mul2(mul2(b))
#[inline(always)]
fn mul4(byte: u8) -> u8 {
    mul2(mul2(byte))
}

// #define mul5(b) (mul4(b)^mul1(b))
#[inline(always)]
fn mul5(byte: u8) -> u8 {
    mul4(byte) ^ mul1(byte)
}

// #define mul7(b) (mul4(b)^mul2(b)^mul1(b))
#[inline(always)]
fn mul7(byte: u8) -> u8 {
    mul4(byte) ^ mul2(byte) ^ mul1(byte)
}

macro_rules! groestl_state {
    ($name: ident, $cols: expr, $rounds: expr, $p_shifts: expr, $q_shifts: expr, $bytes: literal) => {
        #[derive(Clone, Debug)]
        pub struct $name([[u8; $cols]; ROWS]);

        impl $name {
            const BYTES: usize = $bytes;

            pub fn print(&self) {
                for i in 0..ROWS {
                    for j in 0..$cols {
                        print!("{:02x?} ", self.0[i][j])
                    }
                    println!()
                }
                println!();
            }

            pub fn from_array(arr: &[u8]) -> Self {
                assert!(arr.len() == ROWS * $cols);
                let mut s = [[0_u8; $cols]; ROWS];
                for i in 0..$cols {
                    for j in 0..ROWS {
                        s[j][i] = arr[i * 8 + j];
                    }
                }
                Self(s)
            }

            pub fn to_array(self) -> [u8; ROWS * $cols] {
                let mut s = [0_u8; ROWS * $cols];
                for i in 0..$cols {
                    for j in 0..ROWS {
                        s[i * 8 + j] = self.0[j][i];
                    }
                }
                s
            }

            fn add_rc_p(&mut self, round: u8) {
                for i in 0..$cols {
                    self.0[0][i] ^= ((i as u8) << 4) ^ round;
                }
            }

            fn add_rc_q(&mut self, round: u8) {
                for i in 0..$cols {
                    for j in 0..ROWS {
                        self.0[j][i] ^= 0xff
                    }
                    self.0[ROWS - 1][i] ^= ((i as u8) << 4) ^ round;
                }
            }

            fn sub_bytes(&mut self) {
                for i in 0..ROWS {
                    for j in 0..$cols {
                        self.0[i][j] = S_BOX[self.0[i][j] as usize]
                    }
                }
            }

            fn shift_bytes_p(&mut self) {
                for (i, s) in $p_shifts.into_iter().enumerate() {
                    self.0[i].rotate_left(s);
                }
            }

            fn shift_bytes_q(&mut self) {
                for (i, s) in $q_shifts.into_iter().enumerate() {
                    self.0[i].rotate_left(s);
                }
            }

            fn mix_bytes(&mut self) {
                let mut t = [0; ROWS];
                for i in 0..$cols {
                    for j in 0..ROWS {
                        t[j] = mul2(self.0[(j + 0) % ROWS][i])
                            ^ mul2(self.0[(j + 1) % ROWS][i])
                            ^ mul3(self.0[(j + 2) % ROWS][i])
                            ^ mul4(self.0[(j + 3) % ROWS][i])
                            ^ mul5(self.0[(j + 4) % ROWS][i])
                            ^ mul3(self.0[(j + 5) % ROWS][i])
                            ^ mul5(self.0[(j + 6) % ROWS][i])
                            ^ mul7(self.0[(j + 7) % ROWS][i]);
                    }
                    for j in 0..ROWS {
                        self.0[j][i] = t[j]
                    }
                }
            }

            fn p(&mut self) -> Self {
                let mut x = self.clone();
                for i in 0..$rounds {
                    x.add_rc_p(i as u8);
                    x.sub_bytes();
                    x.shift_bytes_p();
                    x.mix_bytes();
                }
                x
            }

            fn q(&mut self) -> Self {
                let mut x = self.clone();
                for i in 0..$rounds {
                    x.add_rc_q(i as u8);
                    x.sub_bytes();
                    x.shift_bytes_q();
                    x.mix_bytes();
                }
                x
            }

            pub fn compress(&mut self, message: &[u8]) {
                let mut t1 = Self::from_array(message);
                let mut t2 = Self::from_array(message);

                for i in 0..ROWS {
                    for j in 0..$cols {
                        t1.0[i][j] ^= self.0[i][j];
                    }
                }

                let p = t1.p();
                let q = t2.q();

                for i in 0..ROWS {
                    for j in 0..$cols {
                        self.0[i][j] ^= p.0[i][j] ^ q.0[i][j];
                    }
                }
            }
        }
    };
}

groestl_state!(
    State512,
    COLS_512,
    ROUNDS_512,
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 3, 5, 7, 0, 2, 4, 6],
    64
);
groestl_state!(
    State1024,
    COLS_1024,
    ROUNDS_1024,
    [0, 1, 2, 3, 4, 5, 6, 11],
    [1, 3, 5, 11, 0, 2, 4, 6],
    128
);

macro_rules! groest_hash_trait {
    ($name: ident, $cols: expr, $bytes: expr) => {
        impl StatefulHasher for $name {
            fn update(&mut self, mut bytes: &[u8]) {
                crate::compression_routine!(self.buffer, bytes, $bytes, {
                    self.blocks_taken += 1;
                    self.state.compress(&self.buffer);
                });
            }

            fn finalize(mut self) -> Vec<u8> {
                self.buffer.push(0x80);
                while self.buffer.len() % $bytes != ($bytes - 8) {
                    self.buffer.push(0x00);
                }
                self.blocks_taken += ((self.buffer.len()) / $bytes) as u64 + 1;
                self.buffer.extend(self.blocks_taken.to_be_bytes());

                for block in self.buffer.chunks_exact($bytes) {
                    self.state.compress(block);
                }

                let p = self.state.p();
                for i in 0..ROWS {
                    for j in 0..$cols {
                        self.state.0[i][j] ^= p.0[i][j];
                    }
                }

                self.state.to_array()[($bytes - self.hash_len)..].to_vec()
            }

            crate::stateful_hash_helpers!();
        }
    };
}

pub struct Groestl512 {
    hash_len: usize,
    blocks_taken: u64,
    state: State512,
    buffer: Vec<u8>,
}

impl Default for Groestl512 {
    fn default() -> Self {
        Self::init256()
    }
}

impl Groestl512 {
    pub fn init224() -> Self {
        let mut s = [0; State512::BYTES];
        s[62] = 0x00;
        s[63] = 0xe0;
        let state = State512::from_array(&s);
        Self {
            blocks_taken: 0,
            hash_len: 28,
            state,
            buffer: Vec::new(),
        }
    }

    pub fn init256() -> Self {
        let mut s = [0; State512::BYTES];
        s[62] = 0x01;
        s[63] = 0x00;
        let state = State512::from_array(&s);
        Self {
            blocks_taken: 0,
            hash_len: 32,
            state,
            buffer: Vec::new(),
        }
    }
}

groest_hash_trait!(Groestl512, COLS_512, State512::BYTES);

pub struct Groestl1024 {
    hash_len: usize,
    blocks_taken: u64,
    state: State1024,
    buffer: Vec<u8>,
}

impl Default for Groestl1024 {
    fn default() -> Self {
        Self::init512()
    }
}

impl Groestl1024 {
    pub fn init384() -> Self {
        let mut s = [0; State1024::BYTES];
        s[126] = 0x01;
        s[127] = 0x80;
        let state = State1024::from_array(&s);
        Self {
            blocks_taken: 0,
            hash_len: 48,
            state,
            buffer: Vec::new(),
        }
    }

    pub fn init512() -> Self {
        let mut s = [0; State1024::BYTES];
        s[126] = 0x02;
        s[127] = 0x00;
        let state = State1024::from_array(&s);
        Self {
            blocks_taken: 0,
            hash_len: 64,
            state,
            buffer: Vec::new(),
        }
    }
}

groest_hash_trait!(Groestl1024, COLS_1024, State1024::BYTES);

crate::stateful_hash_tests!(
    groest256_test_0, Groestl512::init256(), b"",
    "1a52d11d550039be16107f9c58db9ebcc417f16f736adb2502567119f0083467";
    groest256_test_62, Groestl512::init256(), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "ceadc17a34a70964739a7096639bed6e0fe8d63b1642d5c046f7efa630b84c15";
    groest224_test_0, Groestl512::init224(), b"",
    "f2e180fb5947be964cd584e22e496242c6a329c577fc4ce8c36d34c3";
    groest224_test_62, Groestl512::init224(), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "a79200b1e7067102128d66b3b364772117ffed049f8c902992d6768b";

    groest512_test_0, Groestl1024::init512(), b"",
    "6d3ad29d279110eef3adbd66de2a0345a77baede1557f5d099fce0c03d6dc2ba8e6d4a6633dfbd66053c20faa87d1a11f39a7fbe4a6c2f009801370308fc4ad8";
    groest512_test_124, Groestl1024::init512(), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "3dd126365a4613719194d1cff41e7cbf4b33fbea5d969b94b9b4c1e33e6011f5d2684931f9fdd396f07eab1a81bfb26f4c70220676ff29953e4cd9b459bdcb21";
    groest384_test_0, Groestl1024::init384(), b"",
    "ac353c1095ace21439251007862d6c62f829ddbe6de4f78e68d310a9205a736d8b11d99bffe448f57a1cfa2934f044a5";
    groest384_test_124, Groestl1024::init384(), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "cabfeac9351ef14c63402236978a2771422d6625ae3c0dd625c304850d5e45fb80c4e4817f2a43c96800dd699e4c0cfe";
);
