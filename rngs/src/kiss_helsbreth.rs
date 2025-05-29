use crate::ClassicRng;

const MASK16: u32 = 0xffff;

pub struct Kiss {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    carry: u32,
}

impl Default for Kiss {
    fn default() -> Self {
        Self {
            x: 1,
            y: 2,
            z: 4,
            w: 8,
            carry: 0,
        }
    }
}

// http://www.helsbreth.org/random/unbiased.html
impl ClassicRng for Kiss {
    fn next_u32(&mut self) -> u32 {
        self.x = self.x.wrapping_mul(69069).wrapping_add(1);
        self.y ^= self.y << 13;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 5;
        let k = (self.z >> 2)
            .wrapping_add(self.w >> 3)
            .wrapping_add(self.carry >> 2);
        let m = self
            .w
            .wrapping_add(self.w)
            .wrapping_add(self.z)
            .wrapping_add(self.carry);
        self.z = self.w;
        self.w = m;
        self.carry = k >> 30;
        self.x.wrapping_add(self.y).wrapping_add(self.w)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn outputs() {
//         let mut rng = Kiss::default();
//         for _ in 0..10 {
//             println!("{:08x?}", rng.next_u32());
//         }
//     }
// }
