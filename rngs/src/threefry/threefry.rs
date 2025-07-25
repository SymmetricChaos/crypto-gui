use crate::{
    threefry::{threefry_32_4_r, threefry_64_4_r},
    ClassicRng,
};

// pub struct Threefry32_2 {}

// impl Threefry32_2 {}

pub struct Threefry4_32 {
    pub ctr: [u32; 4],
    pub key: [u32; 4],
    pub rounds: usize,
    saved: [u32; 4],
    idx: usize,
}

impl Default for Threefry4_32 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            rounds: 20,
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Threefry4_32 {
    pub fn array(&self) -> [u32; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key: [u32; 5] = [0; 4 + 1];
        ex_key[4] = super::C240_32;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        threefry_32_4_r(&mut arr, &ex_key, self.rounds);
        arr
    }
}

impl ClassicRng for Threefry4_32 {
    fn next_u32(&mut self) -> u32 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
                if self.ctr[1] == 0 {
                    self.ctr[2] = self.ctr[2].wrapping_add(1);
                    if self.ctr[2] == 0 {
                        self.ctr[3] = self.ctr[3].wrapping_add(1);
                    }
                }
            }
        }
        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 4;
        out
    }
}

// pub struct Threefry2_64 {
//     pub ctr: [u64; 2],
//     pub key: [u64; 2],
//     pub rounds: usize,
//     saved: [u64; 2],
//     idx: usize,
// }

// impl Default for Threefry2_64 {
//     fn default() -> Self {
//         Self {
//             ctr: [0; 2],
//             key: [0; 2],
//             rounds: 20,
//             saved: [0; 2],
//             idx: 0,
//         }
//     }
// }

// impl Threefry2_64 {
//     pub fn array(&self) -> [u64; 2] {
//         let mut arr = self.ctr.clone();
//         let mut ex_key = [0; 4 + 1];
//         ex_key[4] = super::C240;
//         for i in 0..4 {
//             ex_key[i] = self.key[i];
//             ex_key[4] ^= self.key[i];
//         }
//         threefry_64_4_r(&mut arr, &ex_key, self.rounds);
//         arr
//     }
// }

pub struct Threefry4_64 {
    pub ctr: [u64; 4],
    pub key: [u64; 4],
    pub rounds: usize,
    saved: [u64; 4],
    idx: usize,
}

impl Default for Threefry4_64 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            rounds: 20,
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Threefry4_64 {
    pub fn array(&self) -> [u64; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key = [0; 4 + 1];
        ex_key[4] = super::C240_64;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        threefry_64_4_r(&mut arr, &ex_key, self.rounds);
        arr
    }
}

impl ClassicRng for Threefry4_64 {
    /// The 64-bit Threefry is meant to produce 64-bit random numbers and this methods ignores the upper bits
    /// To make use of all the bits for smaller values extract them from .next_u64() or from .array()
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx == 0 {
            self.saved = self.array();
            self.ctr[0] = self.ctr[0].wrapping_add(1);
            if self.ctr[0] == 0 {
                self.ctr[1] = self.ctr[1].wrapping_add(1);
                if self.ctr[1] == 0 {
                    self.ctr[2] = self.ctr[2].wrapping_add(1);
                    if self.ctr[2] == 0 {
                        self.ctr[3] = self.ctr[3].wrapping_add(1);
                    }
                }
            }
        }
        let out = self.saved[self.idx];
        self.idx = (self.idx + 1) % 4;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sequence4_64() {
        let mut rng = Threefry4_64::default();

        rng.ctr = [0, 0, 0, 0];
        rng.key = [0, 0, 0, 0];
        assert_eq!(
            [
                0x09218ebde6c85537,
                0x55941f5266d86105,
                0x4bd25e16282434dc,
                0xee29ec846bd2e40b
            ],
            rng.array()
        );

        rng.ctr = [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ];
        rng.key = [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ];
        assert_eq!(
            [
                0x29c24097942bba1b,
                0x0371bbfb0f6f4e11,
                0x3c231ffa33f83a1c,
                0xcd29113fde32d168
            ],
            rng.array()
        );

        rng.ctr = [
            0x243f6a8885a308d3,
            0x13198a2e03707344,
            0xa4093822299f31d0,
            0x082efa98ec4e6c89,
        ];
        rng.key = [
            0x452821e638d01377,
            0xbe5466cf34e90c6c,
            0xbe5466cf34e90c6c,
            0xc0ac29b7c97c50dd,
        ];
        assert_eq!(
            [
                0xa7e8fde591651bd9,
                0xbaafd0c30138319b,
                0x84a5c1a729e685b9,
                0x901d406ccebc1ba4
            ],
            rng.array()
        );

        rng.rounds = 13;

        rng.ctr = [0, 0, 0, 0];
        rng.key = [0, 0, 0, 0];
        assert_eq!(
            [
                0x4071fabee1dc8e05,
                0x02ed3113695c9c62,
                0x397311b5b89f9d49,
                0xe21292c3258024bc
            ],
            rng.array()
        );

        rng.ctr = [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ];
        rng.key = [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ];
        assert_eq!(
            [
                0x7eaed935479722b5,
                0x90994358c429f31c,
                0x496381083e07a75b,
                0x627ed0d746821121
            ],
            rng.array()
        );

        rng.ctr = [
            0x243f6a8885a308d3,
            0x13198a2e03707344,
            0xa4093822299f31d0,
            0x082efa98ec4e6c89,
        ];
        rng.key = [
            0x452821e638d01377,
            0xbe5466cf34e90c6c,
            0xc0ac29b7c97c50dd,
            0x3f84d5b5b5470917,
        ];
        assert_eq!(
            [
                0x4361288ef9c1900c,
                0x8717291521782833,
                0x0d19db18c20cf47e,
                0xa0b41d63ac8581e5
            ],
            rng.array()
        );
    }

    #[test]
    fn sequence4_32() {
        let mut rng = Threefry4_32::default();

        rng.ctr = [0, 0, 0, 0];
        rng.key = [0, 0, 0, 0];
        assert_eq!(
            [0x9c6ca96a, 0xe17eae66, 0xfc10ecd4, 0x5256a7d8],
            rng.array()
        );

        rng.ctr = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        rng.key = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        assert_eq!(
            [0x2a881696, 0x57012287, 0xf6c7446e, 0xa16a6732],
            rng.array()
        );

        rng.ctr = [0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344];
        rng.key = [0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89];
        assert_eq!(
            [0x59cd1dbb, 0xb8879579, 0x86b5d00c, 0xac8b6d84],
            rng.array()
        );

        rng.rounds = 13;

        // rng.ctr = [0, 0, 0, 0];
        // rng.key = [0, 0, 0, 0];
        // assert_eq!(
        //     [0x9c6ca96a, 0xe17eae66, 0xfc10ecd4, 0x5256a7d8],
        //     rng.array()
        // );

        // rng.ctr = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        // rng.key = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
        // assert_eq!(
        //     [0x2a881696, 0x57012287, 0xf6c7446e, 0xa16a6732],
        //     rng.array()
        // );

        // rng.ctr = [0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344];
        // rng.key = [0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89];
        // assert_eq!(
        //     [0x59cd1dbb, 0xb8879579, 0x86b5d00c, 0xac8b6d84],
        //     rng.array()
        // );
    }
}
