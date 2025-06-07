// #define W 32
// #define R 32
// #define M1 3
// #define M2 24
// #define M3 10

// #define MAT0POS(t,v) (v^(v>>t))
// #define MAT0NEG(t,v) (v^(v<<(-(t))))
// #define Identity(v) (v)

// #define V0            STATE[state_i                   ]
// #define VM1           STATE[(state_i+M1) & 0x0000001fU]
// #define VM2           STATE[(state_i+M2) & 0x0000001fU]
// #define VM3           STATE[(state_i+M3) & 0x0000001fU]
// #define VRm1          STATE[(state_i+31) & 0x0000001fU]
// #define newV0         STATE[(state_i+31) & 0x0000001fU]
// #define newV1         STATE[state_i                   ]

// #define FACT 2.32830643653869628906e-10

// static unsigned int state_i = 0;
// static unsigned int STATE[R];
// static unsigned int z0, z1, z2;

// void InitWELLRNG1024a (unsigned int *init){
//    int j;
//    state_i = 0;
//    for (j = 0; j < R; j++)
//      STATE[j] = init[j];
// }

// double WELLRNG1024a (void){
//   z0    = VRm1;
//   z1    = Identity(V0)       ^ MAT0POS (8, VM1);
//   z2    = MAT0NEG (-19, VM2) ^ MAT0NEG(-14,VM3);
//   newV1 = z1                 ^ z2;
//   newV0 = MAT0NEG (-11,z0)   ^ MAT0NEG(-7,z1)    ^ MAT0NEG(-13,z2) ;
//   state_i = (state_i + 31) & 0x0000001fU;
//   return ((double) STATE[state_i]  * FACT);
// }

use crate::ClassicRng;

const M1: usize = 3;
const M2: usize = 24;
const M3: usize = 10;

fn mat0pos(t: i32, v: u32) -> u32 {
    v ^ (v >> t)
}

fn mat0neg(t: i32, v: u32) -> u32 {
    v ^ (v << t)
}

pub struct Well1024a {
    state: [u32; 32],
    idx: usize,
}

impl Default for Well1024a {
    fn default() -> Self {
        Self {
            state: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            idx: 0,
        }
    }
}

impl Well1024a {
    pub fn from_array(arr: &[u32]) -> Self {
        assert!(arr.len() == 32);
        Well1024a {
            state: arr.try_into().unwrap(),
            idx: 0,
        }
    }

    fn v0(&self) -> u32 {
        self.state[self.idx & 0x1f]
    }

    fn vm1(&self) -> u32 {
        self.state[(self.idx + M1) & 0x1f]
    }

    fn vm2(&self) -> u32 {
        self.state[(self.idx + M2) & 0x1f]
    }

    fn vm3(&self) -> u32 {
        self.state[(self.idx + M3) & 0x1f]
    }

    fn vrm1(&self) -> u32 {
        self.state[(self.idx + 31) & 0x1f]
    }
}

impl ClassicRng for Well1024a {
    fn next_u32(&mut self) -> u32 {
        let z0 = self.vrm1();
        let z1 = self.v0() ^ mat0pos(8, self.vm1());
        let z2 = mat0neg(19, self.vm2()) ^ mat0neg(14, self.vm3());
        self.state[self.idx] = z1 ^ z2;
        self.state[(self.idx + 31) & 0x1f] = mat0neg(11, z0) ^ mat0neg(7, z1) ^ mat0neg(13, z2);
        self.idx = (self.idx + 31) & 0x1f;
        self.state[self.idx]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Calculated from the C code above with uint32_t words
    #[test]
    fn stream() {
        let mut rng = Well1024a::default();
        assert_eq!(0x50c0398e, rng.next_u32());
        assert_eq!(0x78658f8c, rng.next_u32());
        assert_eq!(0x0a13610e, rng.next_u32());
        assert_eq!(0x5aeb8c11, rng.next_u32());
        assert_eq!(0x5aa56af3, rng.next_u32());
        assert_eq!(0x0f881563, rng.next_u32());
        assert_eq!(0xd82e8082, rng.next_u32());
        assert_eq!(0xe8b91def, rng.next_u32());
        assert_eq!(0xdef4b661, rng.next_u32());
        assert_eq!(0x1eca99ec, rng.next_u32());
        assert_eq!(0xde5ca8be, rng.next_u32());
        assert_eq!(0x0a75bf3f, rng.next_u32());
        assert_eq!(0x85cb125a, rng.next_u32());
        assert_eq!(0x4d931c5f, rng.next_u32());
        assert_eq!(0x4ffffea2, rng.next_u32());
        assert_eq!(0x88e1270a, rng.next_u32());
        assert_eq!(0x827749b2, rng.next_u32());
        assert_eq!(0xf9fbcca1, rng.next_u32());
        assert_eq!(0x0f0cf194, rng.next_u32());
        assert_eq!(0x6350625c, rng.next_u32());
        assert_eq!(0x346e6844, rng.next_u32());
        assert_eq!(0xb5dba252, rng.next_u32());
        assert_eq!(0x637399b5, rng.next_u32());
        assert_eq!(0x43b22b9e, rng.next_u32());
        assert_eq!(0x2c6628a9, rng.next_u32());
        assert_eq!(0x61bf8991, rng.next_u32());
        assert_eq!(0x0c6872ea, rng.next_u32());
        assert_eq!(0x0dd1fbf4, rng.next_u32());
        assert_eq!(0xa7f07de7, rng.next_u32());
        assert_eq!(0x24bc9d99, rng.next_u32());
        assert_eq!(0xfdb0600e, rng.next_u32());
        assert_eq!(0x3bde485b, rng.next_u32());
        assert_eq!(0xab1a196c, rng.next_u32());
    }
}
