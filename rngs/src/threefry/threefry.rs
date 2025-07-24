use crate::{
    threefry::{threefry_64_4_12, threefry_64_4_13, threefry_64_4_20},
    ClassicRng,
};

// pub struct Threefry32_2 {}

// impl Threefry32_2 {}

// pub struct Threefry32_4 {}

// impl Threefry32_4 {}

// pub struct Threefry64_2 {}

// impl Threefry64_2 {}

// They only give test vectors for Threefry4_64_13 but the paper claims that Threefry4_64_12 passes the test suite
pub struct Threefry4_64_12 {
    ctr: [u64; 4],
    key: [u64; 4],
    saved: [u64; 4],
    idx: usize,
}

impl Default for Threefry4_64_12 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Threefry4_64_12 {
    pub fn array(&self) -> [u64; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key = [0; 4 + 1];
        ex_key[4] = super::C240;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        threefry_64_4_12(&mut arr, &ex_key);
        arr
    }
}

impl ClassicRng for Threefry4_64_12 {
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

pub struct Threefry4_64_13 {
    ctr: [u64; 4],
    key: [u64; 4],
    saved: [u64; 4],
    idx: usize,
}

impl Default for Threefry4_64_13 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
            saved: [0; 4],
            idx: 0,
        }
    }
}

impl Threefry4_64_13 {
    pub fn array(&self) -> [u64; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key = [0; 4 + 1];
        ex_key[4] = super::C240;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        threefry_64_4_13(&mut arr, &ex_key);
        arr
    }
}

impl ClassicRng for Threefry4_64_13 {
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

pub struct Threefry4_64_20 {
    ctr: [u64; 4],
    key: [u64; 4],
}

impl Default for Threefry4_64_20 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
        }
    }
}

impl Threefry4_64_20 {
    pub fn array(&self) -> [u64; 4] {
        let mut arr = self.ctr.clone();
        let mut ex_key = [0; 4 + 1];
        ex_key[4] = super::C240;
        for i in 0..4 {
            ex_key[i] = self.key[i];
            ex_key[4] ^= self.key[i];
        }
        threefry_64_4_20(&mut arr, &ex_key);
        arr
    }
}

impl ClassicRng for Threefry4_64_20 {
    fn next_u32(&mut self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sequence4_64_20() {
        let mut rng = Threefry4_64_20::default();

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
    }

    #[test]
    fn sequence4_64_13() {
        let mut rng = Threefry4_64_13::default();

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
}
