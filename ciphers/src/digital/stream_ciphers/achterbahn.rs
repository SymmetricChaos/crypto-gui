const LENS: [usize; 13] = [21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33];
const MASKS: [u64; 13] = [
    0x001FFFFF,
    0x003FFFFF,
    0x007FFFFF,
    0x00FFFFFF,
    0x01FFFFFF,
    0x03FFFFFF,
    0x07FFFFFF,
    0x0FFFFFFF,
    0x1FFFFFFF,
    0x3FFFFFFF,
    0x7FFFFFFF,
    0xFFFFFFFF,
    0x1FFFFFFFF,
];

macro_rules! and3 {
    ($a: expr, $b: expr, $c: expr) => {
        $a & $b & $c
    };
}

macro_rules! and2 {
    ($a: expr, $b: expr) => {
        $a & $b
    };
}

macro_rules! xor3 {
    ($a: expr, $b: expr, $c: expr) => {
        $a ^ $b ^ $c
    };
}

// macro_rules! xor2 {
//     ($a: expr, $b: expr) => {
//         $a ^ $b
//     };
// }

macro_rules! mux3 {
    ($a: expr, $b: expr, $c: expr) => {
        ($c & ($a ^ $b)) ^ $a
    };
}

macro_rules! maj3 {
    ($a: expr, $b: expr, $c: expr) => {
        ($a & ($b ^ $c)) ^ ($b & $c)
    };
}

fn step_a0(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 15, xor3!(x >> 3, x >> 2, x)),
                xor3!(
                    and2!(x >> 4, x >> 7),
                    xor3!(x >> 5, x >> 6, x >> 8),
                    mux3!(x >> 4, x >> 5, x >> 6)
                ),
                mux3!(
                    mux3!(x >> 11, x >> 12, x >> 2),
                    and3!(x >> 6, x >> 2, x >> 13),
                    mux3!(x >> 1, x >> 10, x >> 9)
                )
            )))
            << 20)
}

fn step_a1(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 15, xor3!(x >> 8, x >> 5, x)),
                xor3!(
                    and2!(x >> 5, x >> 11),
                    mux3!(x >> 13, x >> 3, x >> 1),
                    mux3!(x >> 6, x >> 4, x >> 12)
                ),
                mux3!(
                    mux3!(x >> 1, x >> 9, x >> 7),
                    mux3!(x >> 4, x >> 12, x >> 10),
                    and3!(x >> 1, x >> 11, x >> 14)
                )
            )))
            << 21)
}

fn step_a2(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 16, xor3!(x >> 13, x >> 4, x)),
                xor3!(
                    and2!(x >> 12, x >> 14),
                    mux3!(x >> 1, x >> 9, x >> 7),
                    mux3!(x >> 1, x >> 4, x >> 6)
                ),
                mux3!(
                    mux3!(x >> 5, x >> 8, x >> 11),
                    mux3!(x >> 10, x >> 3, x >> 11),
                    and3!(x >> 1, x >> 9, x >> 15)
                )
            )))
            << 22)
}

fn step_a3(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 18, xor3!(x >> 8, x >> 3, x)),
                xor3!(
                    and2!(x >> 1, x >> 11),
                    mux3!(x >> 2, x >> 14, x >> 13),
                    mux3!(x >> 12, x >> 4, x >> 13)
                ),
                mux3!(
                    mux3!(x >> 6, x >> 1, x >> 15),
                    mux3!(x >> 14, x >> 16, x >> 9),
                    maj3!(x >> 2, x >> 5, x >> 7)
                )
            )))
            << 23)
}

fn step_a4(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 20, xor3!(x >> 11, x >> 1, x)),
                xor3!(
                    and2!(x >> 4, x >> 12),
                    mux3!(x >> 1, x >> 3, x >> 5),
                    mux3!(x >> 6, x >> 7, x >> 16)
                ),
                mux3!(
                    maj3!(x >> 8, x >> 15, x >> 17),
                    mux3!(x >> 14, x >> 13, x >> 12),
                    mux3!(x >> 5, x >> 3, x >> 2)
                )
            )))
            << 24)
}

fn step_a5(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 21, xor3!(x >> 17, x >> 16, x >> 15)),
                xor3!(
                    xor3!(x >> 5, x >> 4, x),
                    and2!(x >> 3, x >> 6),
                    mux3!(x >> 4, x >> 18, x >> 2)
                ),
                mux3!(
                    mux3!(x >> 4, x >> 12, x >> 13),
                    mux3!(x >> 14, x >> 11, x >> 7),
                    maj3!(x >> 3, x >> 10, x >> 15)
                )
            )))
            << 25)
}

fn step_a6(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 15, x >> 4, x)),
                xor3!(
                    and2!(x >> 1, x >> 12),
                    mux3!(x >> 10, x >> 6, x >> 17),
                    mux3!(x >> 3, x >> 8, x >> 1)
                ),
                mux3!(
                    mux3!(x >> 10, x >> 14, x >> 13),
                    maj3!(x >> 17, x >> 2, x >> 16),
                    and3!(x >> 18, x >> 11, x >> 5)
                )
            )))
            << 26)
}

fn step_a7(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 18, x >> 5, x)),
                xor3!(
                    and2!(x >> 4, x >> 12),
                    mux3!(x >> 1, x >> 17, x >> 2),
                    mux3!(x >> 20, x >> 14, x >> 16)
                ),
                mux3!(
                    mux3!(x >> 18, x >> 15, x >> 10),
                    and3!(x >> 1, x >> 2, x >> 13),
                    and3!(x >> 7, x >> 9, x >> 19)
                )
            )))
            << 27)
}

fn step_a8(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 24, xor3!(x >> 21, x >> 18, x >> 17)),
                xor3!(
                    and2!(x >> 1, x >> 4),
                    xor3!(x >> 11, x >> 2, x),
                    mux3!(x >> 10, x >> 8, x >> 21)
                ),
                mux3!(
                    and3!(x >> 8, x >> 18, x >> 9),
                    mux3!(x >> 13, x >> 6, x >> 15),
                    mux3!(x >> 19, x >> 16, x >> 14)
                )
            )))
            << 28)
}
fn step_a9(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 28, xor3!(x >> 18, x >> 1, x)),
                xor3!(
                    and2!(x >> 2, x >> 8),
                    mux3!(x >> 12, x >> 19, x >> 10),
                    mux3!(x >> 10, x >> 14, x >> 22)
                ),
                mux3!(
                    mux3!(x >> 7, x >> 18, x >> 4),
                    maj3!(x >> 21, x >> 9, x >> 1),
                    maj3!(x >> 8, x >> 5, x >> 3)
                )
            )))
            << 29)
}
fn step_a10(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 18, x >> 15, x >> 6)),
                xor3!(
                    xor3!(x >> 5, x >> 2, x),
                    and2!(x >> 19, x >> 14),
                    mux3!(x >> 17, x >> 12, x >> 21)
                ),
                mux3!(
                    mux3!(x >> 20, x >> 18, x >> 8),
                    maj3!(x >> 4, x >> 12, x >> 19),
                    mux3!(x >> 22, x >> 7, x >> 21)
                )
            )))
            << 30)
}

fn step_a11(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 28, xor3!(x >> 22, x >> 17, x >> 8)),
                xor3!(
                    and2!(x >> 13, x >> 15),
                    xor3!(x >> 5, x >> 3, x),
                    mux3!(x >> 5, x >> 7, x >> 19)
                ),
                mux3!(
                    mux3!(x >> 8, x >> 2, x >> 13),
                    and3!(x >> 4, x >> 11, x >> 24),
                    mux3!(x >> 12, x >> 14, x >> 7)
                )
            )))
            << 31)
}

fn step_a12(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 30, xor3!(x >> 23, x >> 10, x >> 9)),
                xor3!(
                    xor3!(x >> 7, x >> 2, x),
                    and2!(x >> 15, x >> 16),
                    mux3!(x >> 25, x >> 15, x >> 13)
                ),
                mux3!(
                    mux3!(x >> 15, x >> 12, x >> 16),
                    maj3!(x >> 14, x >> 1, x >> 18),
                    mux3!(x >> 8, x >> 24, x >> 17)
                )
            )))
            << 32)
}

fn step_a_n(n: usize, x: u64, feedin: u64) -> u64 {
    match n {
        0 => step_a0(x, feedin),
        1 => step_a1(x, feedin),
        2 => step_a2(x, feedin),
        3 => step_a3(x, feedin),
        4 => step_a4(x, feedin),
        5 => step_a5(x, feedin),
        6 => step_a6(x, feedin),
        7 => step_a7(x, feedin),
        8 => step_a8(x, feedin),
        9 => step_a9(x, feedin),
        10 => step_a10(x, feedin),
        11 => step_a11(x, feedin),
        12 => step_a12(x, feedin),
        _ => unreachable!("invalid NLFSR chosen"),
    }
}
// u32 F (u32 x0, u32 x1, u32 x2, u32 x3, u32 x4, u32 x5,
//        u32 x6, u32 x7, u32 x8, u32 x9, u32 x10, u32 x11, u32 x12)
// {
//   u32 A = x1^x2,
//       C = x2^x9,
//       H = x3^x7,
//       T = x4^x9,
//       E = ((x0^x6)&x5)^x6,
//       R = ((x1^x4)&C)^T,
//       b = (R^(A&x5)^x2)&H,
//       a = ((x10^x11)&(C^(A&T)^E))^E,
//       h = (x8^x12)&(b^a^R^x7^x10),
//       n = H^A^T^a^h^x0^x5^x6^x11^x12;
//   return (n);
// }

fn combining_function(x: [u64; 13]) -> u64 {
    let a = x[1] ^ x[2];
    let b = x[2] ^ x[9];
    let c = x[3] ^ x[7];
    let d = x[4] ^ x[9];
    let e = ((x[0] ^ x[6]) & x[5]) ^ x[6];
    let f = ((x[1] ^ x[4]) & b) ^ d;
    let g = (f ^ (a & x[5]) ^ x[2]) & c;
    let h = ((x[10] ^ x[11]) & (b ^ (a & d) ^ e)) ^ e;
    let i = (x[8] ^ x[12]) & (g ^ h ^ f ^ x[7] ^ x[10]);
    c ^ a ^ d ^ h ^ i ^ x[0] ^ x[5] ^ x[6] ^ x[11] ^ x[12]
}

fn keystream_bits(x: &[u64; 13]) -> u64 {
    combining_function([
        x[0] >> (LENS[0] - 16),
        x[1] >> (LENS[1] - 16),
        x[2] >> (LENS[2] - 16),
        x[3] >> (LENS[3] - 16),
        x[4] >> (LENS[4] - 16),
        x[5] >> (LENS[5] - 16),
        x[6] >> (LENS[6] - 16),
        x[7] >> (LENS[7] - 16),
        x[8] >> (LENS[8] - 16),
        x[9] >> (LENS[9] - 16),
        x[10] >> (LENS[10] - 16),
        x[11] >> (LENS[11] - 16),
        x[12] >> (LENS[12] - 16),
    ])
}

#[derive(Debug, Clone)]
pub struct Achterbahn128 {
    nlfsrs: [u64; 13],
}
impl Default for Achterbahn128 {
    fn default() -> Self {
        let mut out = Self {
            nlfsrs: Default::default(),
        };
        out.ksa(
            [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
        );
        out
    }
}

impl Achterbahn128 {
    pub fn step(&mut self, i: usize, feedin: u64) {
        self.nlfsrs[i] = step_a_n(i, self.nlfsrs[i], feedin);
    }

    pub fn step_all(&mut self, feedin: u64) {
        for i in 0..13 {
            self.step(i, feedin);
        }
    }

    pub fn ksa(&mut self, key: [u8; 16], iv: [u8; 16]) {
        // This is actual 48 bits but it will be masked to not more than 33 when used
        let key33 = (key[0] as u64)
            | (key[1] as u64) << 8
            | (key[2] as u64) << 16
            | (key[3] as u64) << 24
            | (key[4] as u64) << 32;

        // Makes life easier later
        let ky = {
            let mut bits = [0; 128];
            for i in 0..128 {
                bits[i] = (1 & (key[i / 8] >> (i % 8))) as u64;
            }
            bits
        };

        // Makes life easier later
        let iv = {
            let mut bits = [0; 128];
            for i in 0..128 {
                bits[i] = (1 & (iv[i / 8] >> (i % 8))) as u64;
            }
            bits
        };

        // 1: Load all NLFSRs with the first key bits
        for i in 0..13 {
            self.step(i, key33 & MASKS[i]);
        }

        // 2: For each NLFSRS feed-in the key bits not loaded in step 1
        for j in 0..13 {
            for i in LENS[j]..128 {
                self.step(j, ky[i]);
            }
        }

        // 3: for each NLFSR feed-in all IV bits
        for j in 0..13 {
            for i in 0..128 {
                self.step(j, iv[i]);
            }
        }

        // 4: for each NLFSR feed-in the keystream output
        for _ in 0..32 {
            let z = keystream_bits(&self.nlfsrs);
            self.step_all(z);
        }

        // 5: set the least significant bit of each NLFSR to 1
        for nlfsr in self.nlfsrs.iter_mut() {
            *nlfsr |= 1;
        }

        // 6: warm up
        for _ in 0..64 {
            self.step_all(0);
        }
    }

    pub fn keystream(&mut self, bytes: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for _ in 0..bytes {
            out.push(keystream_bits(&self.nlfsrs) as u8);
            for _ in 0..8 {
                self.step_all(0);
            }
        }
        out
    }

    pub fn encrypt_bytes_mut(&mut self, bytes: &mut [u8]) {
        for byte in bytes {
            *byte ^= keystream_bits(&self.nlfsrs) as u8;
            for _ in 0..8 {
                self.step_all(0);
            }
        }
    }

    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        self.clone().encrypt_bytes_mut(bytes);
    }

    pub fn decrypt_bytes(&self, bytes: &mut [u8]) {
        self.encrypt_bytes(bytes);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_keystream() {
        let reference_steam = [
            0xDF, 0x71, 0xF0, 0x42, 0x73, 0x8F, 0x6D, 0x9E, 0xC2, 0x1D, 0x89, 0x6D, 0x0C, 0xC1,
            0x2B, 0xAF, 0x54, 0xC8, 0xCE, 0x55, 0xA6, 0x50, 0x7A, 0x12, 0x43, 0xB4, 0x71, 0xC2,
            0xCD, 0xF0, 0xEC, 0x42, 0x86, 0xFC, 0x01, 0x45, 0x43, 0x80, 0x90, 0x13, 0xDC, 0xA4,
            0xCE, 0xDB, 0x0F, 0x11, 0xDA, 0xF4, 0x52, 0xBD, 0xCA, 0x14, 0x6F, 0x6B, 0x65, 0x72,
            0x1D, 0xBC, 0x79, 0x2C, 0xD2, 0xB6, 0x36, 0x14, 0xF8, 0xDB, 0xE7, 0xCB, 0x7B, 0x16,
            0x35, 0x61, 0xE8, 0x10, 0x4A, 0x75, 0xBD, 0x4B, 0x92, 0x9E, 0xA8, 0x02, 0x87, 0x96,
            0x13, 0x93, 0x4A, 0xB5, 0xFD, 0x91, 0x16, 0x1F, 0x35, 0x00, 0xE5, 0x3F, 0x36, 0x69,
            0x85, 0x34, 0xCC, 0x9C, 0xE6, 0xE9, 0xA5, 0xC2, 0x74, 0xC8, 0x2D, 0x04, 0x93, 0x2E,
            0x75, 0x06, 0x40, 0x7A, 0xDE, 0xAF, 0x61, 0x1F, 0x00, 0xF7, 0xD3, 0x8D, 0x2F, 0x1D,
            0x38, 0x94, 0xE2, 0x12, 0xCE, 0x9F, 0xF8, 0xCD, 0x9B, 0x93, 0x70, 0x18, 0xE2, 0x56,
            0x9B, 0xB5, 0x54, 0x45, 0xEC, 0x60, 0xB8, 0x52, 0xB3, 0xE6, 0x6D, 0x53, 0x68, 0x9B,
            0x3E, 0x40, 0x61, 0x0A, 0x22, 0xA8, 0xD3, 0xA1, 0x03, 0x5C, 0xC1, 0x76, 0x1D, 0x50,
            0xB0, 0x51, 0xF4, 0xDA, 0xB4, 0xE9, 0x2E, 0xA1, 0x57, 0xA3, 0xD0, 0x5F, 0x11, 0xDD,
            0x54, 0xCF, 0x6A, 0x07, 0xA4, 0x4A, 0x21, 0x56, 0x51, 0xA6, 0x91, 0x5F, 0xF4, 0x77,
            0x58, 0x81, 0x90, 0x29, 0x50, 0xE4, 0x92, 0xEA, 0xFB, 0x6C, 0x66, 0x28, 0x85, 0x06,
            0xF2, 0xBD, 0x60, 0xF8, 0x1A, 0x73, 0x68, 0x4A, 0xF7, 0xCD, 0xEC, 0xCF, 0xD0, 0x1A,
            0xCD, 0x09, 0xA8, 0x6B, 0xC6, 0x43, 0x21, 0x8A, 0xEF, 0xDF, 0x27, 0xC1, 0x47, 0x25,
            0xB9, 0x06, 0xA9, 0x40, 0x44, 0x62, 0x86, 0x61, 0x20, 0x66, 0x1E, 0x70, 0x76, 0xED,
            0x93, 0xEC, 0x31, 0xA9,
        ];
        let mut cipher = Achterbahn128::default();
        let keystream = cipher.keystream(256);
        for i in 0..256 {
            assert_eq!(
                reference_steam[i], keystream[i],
                "\nmismatch at index {i} {:02X?} != {:02X?}",
                reference_steam[i], keystream[i]
            )
        }
    }

    #[test]
    fn check_nlfsrs_lengths() {
        let seed = 0xc1a0be1a;
        for i in 0..13 {
            let mut state = seed & MASKS[i];
            let mut ctr = 0;
            loop {
                ctr += 1;
                state = step_a_n(i, state, 0);
                if seed & MASKS[i] == state {
                    assert!(ctr == MASKS[i]);
                    // println!("NLFSR_{i} correctly has period {ctr}");
                    break;
                }
            }
        }
    }

    #[test]
    fn check_boolean_combining_func() {
        let n13 = 1 << 13;
        for x in 0..n13 {
            let r = (x >> 0)
                ^ (x >> 1)
                ^ (x >> 2)
                ^ (x >> 3)
                ^ (x >> 4)
                ^ (x >> 5)
                ^ (x >> 7)
                ^ (x >> 9)
                ^ (x >> 11)
                ^ (x >> 12)
                ^ ((x >> 0) & (x >> 5))
                ^ ((x >> 2) & (x >> 10))
                ^ ((x >> 2) & (x >> 11))
                ^ ((x >> 4) & (x >> 8))
                ^ ((x >> 4) & (x >> 12))
                ^ ((x >> 5) & (x >> 6))
                ^ ((x >> 6) & (x >> 10))
                ^ ((x >> 6) & (x >> 8))
                ^ ((x >> 6) & (x >> 11))
                ^ ((x >> 6) & (x >> 12))
                ^ ((x >> 7) & (x >> 8))
                ^ ((x >> 7) & (x >> 12))
                ^ ((x >> 10) & (x >> 9))
                ^ ((x >> 10) & (x >> 8))
                ^ ((x >> 10) & (x >> 12))
                ^ ((x >> 9) & (x >> 8))
                ^ ((x >> 9) & (x >> 11))
                ^ ((x >> 9) & (x >> 12))
                ^ ((x >> 0) & (x >> 5) & (x >> 10))
                ^ ((x >> 0) & (x >> 5) & (x >> 8))
                ^ ((x >> 0) & (x >> 5) & (x >> 11))
                ^ ((x >> 0) & (x >> 5) & (x >> 12))
                ^ ((x >> 1) & (x >> 2) & (x >> 8))
                ^ ((x >> 1) & (x >> 2) & (x >> 12))
                ^ ((x >> 1) & (x >> 4) & (x >> 10))
                ^ ((x >> 1) & (x >> 4) & (x >> 11))
                ^ ((x >> 1) & (x >> 10) & (x >> 9))
                ^ ((x >> 1) & (x >> 9) & (x >> 8))
                ^ ((x >> 1) & (x >> 9) & (x >> 11))
                ^ ((x >> 1) & (x >> 9) & (x >> 12))
                ^ ((x >> 2) & (x >> 3) & (x >> 8))
                ^ ((x >> 2) & (x >> 3) & (x >> 12))
                ^ ((x >> 2) & (x >> 4) & (x >> 10))
                ^ ((x >> 2) & (x >> 4) & (x >> 8))
                ^ ((x >> 2) & (x >> 4) & (x >> 11))
                ^ ((x >> 2) & (x >> 4) & (x >> 12))
                ^ ((x >> 2) & (x >> 7) & (x >> 8))
                ^ ((x >> 2) & (x >> 7) & (x >> 12))
                ^ ((x >> 2) & (x >> 10) & (x >> 9))
                ^ ((x >> 2) & (x >> 10) & (x >> 8))
                ^ ((x >> 2) & (x >> 10) & (x >> 12))
                ^ ((x >> 2) & (x >> 9) & (x >> 11))
                ^ ((x >> 2) & (x >> 8) & (x >> 11))
                ^ ((x >> 2) & (x >> 11) & (x >> 12))
                ^ ((x >> 3) & (x >> 4) & (x >> 8))
                ^ ((x >> 3) & (x >> 4) & (x >> 12))
                ^ ((x >> 3) & (x >> 9) & (x >> 8))
                ^ ((x >> 3) & (x >> 9) & (x >> 12))
                ^ ((x >> 4) & (x >> 7) & (x >> 8))
                ^ ((x >> 4) & (x >> 7) & (x >> 12))
                ^ ((x >> 4) & (x >> 9) & (x >> 8))
                ^ ((x >> 4) & (x >> 9) & (x >> 12))
                ^ ((x >> 5) & (x >> 6) & (x >> 10))
                ^ ((x >> 5) & (x >> 6) & (x >> 8))
                ^ ((x >> 5) & (x >> 6) & (x >> 11))
                ^ ((x >> 5) & (x >> 6) & (x >> 12))
                ^ ((x >> 6) & (x >> 10) & (x >> 8))
                ^ ((x >> 6) & (x >> 10) & (x >> 12))
                ^ ((x >> 6) & (x >> 8) & (x >> 11))
                ^ ((x >> 6) & (x >> 11) & (x >> 12))
                ^ ((x >> 7) & (x >> 9) & (x >> 8))
                ^ ((x >> 7) & (x >> 9) & (x >> 12))
                ^ ((x >> 10) & (x >> 9) & (x >> 8))
                ^ ((x >> 10) & (x >> 9) & (x >> 12))
                ^ ((x >> 9) & (x >> 8) & (x >> 11))
                ^ ((x >> 9) & (x >> 11) & (x >> 12))
                ^ ((x >> 0) & (x >> 5) & (x >> 10) & (x >> 8))
                ^ ((x >> 0) & (x >> 5) & (x >> 10) & (x >> 12))
                ^ ((x >> 0) & (x >> 5) & (x >> 8) & (x >> 11))
                ^ ((x >> 0) & (x >> 5) & (x >> 11) & (x >> 12))
                ^ ((x >> 1) & (x >> 2) & (x >> 3) & (x >> 8))
                ^ ((x >> 1) & (x >> 2) & (x >> 3) & (x >> 12))
                ^ ((x >> 1) & (x >> 2) & (x >> 7) & (x >> 8))
                ^ ((x >> 1) & (x >> 2) & (x >> 7) & (x >> 12))
                ^ ((x >> 1) & (x >> 3) & (x >> 5) & (x >> 8))
                ^ ((x >> 1) & (x >> 3) & (x >> 5) & (x >> 12))
                ^ ((x >> 1) & (x >> 3) & (x >> 9) & (x >> 8))
                ^ ((x >> 1) & (x >> 3) & (x >> 9) & (x >> 12))
                ^ ((x >> 1) & (x >> 4) & (x >> 10) & (x >> 8))
                ^ ((x >> 1) & (x >> 4) & (x >> 10) & (x >> 12))
                ^ ((x >> 1) & (x >> 4) & (x >> 8) & (x >> 11))
                ^ ((x >> 1) & (x >> 4) & (x >> 11) & (x >> 12))
                ^ ((x >> 1) & (x >> 5) & (x >> 7) & (x >> 8))
                ^ ((x >> 1) & (x >> 5) & (x >> 7) & (x >> 12))
                ^ ((x >> 1) & (x >> 7) & (x >> 9) & (x >> 8))
                ^ ((x >> 1) & (x >> 7) & (x >> 9) & (x >> 12))
                ^ ((x >> 1) & (x >> 10) & (x >> 9) & (x >> 8))
                ^ ((x >> 1) & (x >> 10) & (x >> 9) & (x >> 12))
                ^ ((x >> 1) & (x >> 9) & (x >> 8) & (x >> 11))
                ^ ((x >> 1) & (x >> 9) & (x >> 11) & (x >> 12))
                ^ ((x >> 2) & (x >> 3) & (x >> 4) & (x >> 8))
                ^ ((x >> 2) & (x >> 3) & (x >> 4) & (x >> 12))
                ^ ((x >> 2) & (x >> 3) & (x >> 5) & (x >> 8))
                ^ ((x >> 2) & (x >> 3) & (x >> 5) & (x >> 12))
                ^ ((x >> 2) & (x >> 4) & (x >> 7) & (x >> 8))
                ^ ((x >> 2) & (x >> 4) & (x >> 7) & (x >> 12))
                ^ ((x >> 2) & (x >> 4) & (x >> 10) & (x >> 8))
                ^ ((x >> 2) & (x >> 4) & (x >> 10) & (x >> 12))
                ^ ((x >> 2) & (x >> 4) & (x >> 8) & (x >> 11))
                ^ ((x >> 2) & (x >> 4) & (x >> 11) & (x >> 12))
                ^ ((x >> 2) & (x >> 5) & (x >> 7) & (x >> 8))
                ^ ((x >> 2) & (x >> 5) & (x >> 7) & (x >> 12))
                ^ ((x >> 2) & (x >> 10) & (x >> 9) & (x >> 8))
                ^ ((x >> 2) & (x >> 10) & (x >> 9) & (x >> 12))
                ^ ((x >> 2) & (x >> 9) & (x >> 8) & (x >> 11))
                ^ ((x >> 2) & (x >> 9) & (x >> 11) & (x >> 12))
                ^ ((x >> 3) & (x >> 4) & (x >> 9) & (x >> 8))
                ^ ((x >> 3) & (x >> 4) & (x >> 9) & (x >> 12))
                ^ ((x >> 4) & (x >> 7) & (x >> 9) & (x >> 8))
                ^ ((x >> 4) & (x >> 7) & (x >> 9) & (x >> 12))
                ^ ((x >> 5) & (x >> 6) & (x >> 10) & (x >> 8))
                ^ ((x >> 5) & (x >> 6) & (x >> 10) & (x >> 12))
                ^ ((x >> 5) & (x >> 6) & (x >> 8) & (x >> 11))
                ^ ((x >> 5) & (x >> 6) & (x >> 11) & (x >> 12));

            let t = combining_function([
                (x >> 0),
                (x >> 1),
                (x >> 2),
                (x >> 3),
                (x >> 4),
                (x >> 5),
                (x >> 6),
                (x >> 7),
                (x >> 8),
                (x >> 9),
                (x >> 10),
                (x >> 11),
                (x >> 12),
            ]);

            assert_eq!(r, t)
        }
    }
}
