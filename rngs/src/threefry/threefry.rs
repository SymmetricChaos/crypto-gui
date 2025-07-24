use crate::{
    threefry::{threefry_64_4_12, threefry_64_4_20},
    ClassicRng,
};

// pub struct Threefry32_2 {}

// impl Threefry32_2 {}

// pub struct Threefry32_4 {}

// impl Threefry32_4 {}

// pub struct Threefry64_2 {}

// impl Threefry64_2 {}

pub struct Threefry4_64_12 {
    ctr: [u64; 4],
    key: [u64; 4],
}

impl Default for Threefry4_64_12 {
    fn default() -> Self {
        Self {
            ctr: [0; 4],
            key: [0; 4],
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
        todo!()
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
}
