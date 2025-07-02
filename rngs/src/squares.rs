use crate::ClassicRng;

pub struct Squares {
    pub key: u64,
    pub ctr: u64,
}

impl Default for Squares {
    fn default() -> Self {
        Self {
            key: Default::default(),
            ctr: Default::default(),
        }
    }
}

impl Squares {}

impl ClassicRng for Squares {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.ctr.wrapping_mul(self.key);
        self.ctr = self.ctr.wrapping_add(1);
        let y = x;
        let z = y.wrapping_add(self.key);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        (x.wrapping_mul(x).wrapping_add(z) >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.ctr.wrapping_mul(self.key);
        self.ctr = self.ctr.wrapping_add(1);
        let y = x;
        let z = y.wrapping_add(self.key);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(y);
        x = x.rotate_right(32);
        x = x.wrapping_mul(x).wrapping_add(z);
        let t = x;
        x = x.rotate_right(32);
        t ^ (x.wrapping_mul(x).wrapping_add(y) >> 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_twenty_u32() {
        let mut rng = Squares::default();
        rng.key = 123456;
        let ints: [u32; 20] = [
            0x00000000, 0x368cbf81, 0x850fa3b2, 0x71a85e6d, 0xb9b56435, 0x1c47d8aa, 0x0c64b4f5,
            0xd291a3e2, 0xd5361e10, 0xa78c358a, 0x23fefb23, 0x6246b07c, 0xd46010e5, 0x74f00d45,
            0xb73a5fde, 0x86485ef1, 0x0c7589bc, 0xfd2631c7, 0x6abac718, 0x42e5f448,
        ];
        for int in ints {
            assert_eq!(int, rng.next_u32());
        }
    }

    #[test]
    fn first_twenty_u64() {
        let mut rng = Squares::default();
        rng.key = 123456;
        let ints: [u64; 20] = [
            0x000000000001e240,
            0x368cbf819fd6c64f,
            0x850fa3b20c7e228e,
            0x71a85e6d21c8c173,
            0xb9b5643514cc81db,
            0x1c47d8aa2f4304bb,
            0x0c64b4f5f3b60c5f,
            0xd291a3e2f31047e9,
            0xd5361e10202cf0b6,
            0xa78c358aa0119290,
            0x23fefb23ff16f4de,
            0x6246b07c52300fe3,
            0xd46010e5f39ccab8,
            0x74f00d4587ed2fc3,
            0xb73a5fdecb453c9b,
            0x86485ef1ed42b425,
            0x0c7589bc4815c330,
            0xfd2631c7209171f5,
            0x6abac718a0e5a2f5,
            0x42e5f448c09e6f4d,
        ];
        for int in ints {
            assert_eq!(int, rng.next_u64());
        }
    }
}
