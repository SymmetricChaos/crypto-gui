// #define W 32
// #define R 16
// #define P 0
// #define M1 13
// #define M2 9
// #define M3 5

// #define MAT0POS(t,v) (v^(v>>t))
// #define MAT0NEG(t,v) (v^(v<<(-(t))))
// #define MAT3NEG(t,v) (v<<(-(t)))
// #define MAT4NEG(t,b,v) (v ^ ((v<<(-(t))) & b))

// #define V0            STATE[state_i                   ]
// #define VM1           STATE[(state_i+M1) & 0x0000000fU]
// #define VM2           STATE[(state_i+M2) & 0x0000000fU]
// #define VM3           STATE[(state_i+M3) & 0x0000000fU]
// #define VRm1          STATE[(state_i+15) & 0x0000000fU]
// #define VRm2          STATE[(state_i+14) & 0x0000000fU]
// #define newV0         STATE[(state_i+15) & 0x0000000fU]
// #define newV1         STATE[state_i                 ]
// #define newVRm1       STATE[(state_i+14) & 0x0000000fU]

// double WELLRNG512a (void){
//     z0    = VRm1;
//     z1    = MAT0NEG (-16,V0)    ^ MAT0NEG (-15, VM1);
//     z2    = MAT0POS (11, VM2)  ;
//     newV1 = z1                  ^ z2;
//     newV0 = MAT0NEG (-2,z0)     ^ MAT0NEG(-18,z1)    ^ MAT3NEG(-28,z2) ^ MAT4NEG(-5,0xda442d24U,newV1) ;
//     state_i = (state_i + 15) & 0x0000000fU;
//     return ((double) STATE[state_i]) * FACT;
//   }

use crate::ClassicRng;

const M1: usize = 13;
const M2: usize = 9;
const M3: usize = 5;

fn matopos(t: i32, v: u32) -> u32 {
    v ^ (v >> t)
}

fn mat0neg(t: i32, v: u32) -> u32 {
    v ^ (v << (-t))
}

fn mat3neg(t: i32, v: u32) -> u32 {
    v << (-t)
}

fn mat4neg(t: i32, b: u32, v: u32) -> u32 {
    v ^ ((v << (-t)) & b)
}

pub struct Well512a {
    state: [u32; 16],
    idx: usize,
}

impl Default for Well512a {
    fn default() -> Self {
        Self {
            state: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            idx: 0,
        }
    }
}

impl Well512a {
    pub fn from_u32(seed: u32) -> Self {
        Self {
            state: [seed; 16],
            idx: 0,
        }
    }

    fn v0(&self) -> u32 {
        self.state[self.idx & 0xF]
    }

    fn vm1(&self) -> u32 {
        self.state[(self.idx + M1) & 0xF]
    }

    fn vm2(&self) -> u32 {
        self.state[(self.idx + M2) & 0xF]
    }

    fn vm3(&self) -> u32 {
        self.state[(self.idx + M3) & 0xF]
    }

    fn vrm1(&self) -> u32 {
        self.state[(self.idx + 15) & 0xF]
    }

    fn vrm2(&self) -> u32 {
        self.state[(self.idx + 14) & 0xF]
    }
}

impl ClassicRng for Well512a {
    fn next_u32(&mut self) -> u32 {
        let z0 = self.vrm1();
        let z1 = mat0neg(-16, self.v0()) ^ mat0neg(-15, self.vm1());
        let z2 = matopos(11, self.vm2());
        self.state[self.idx] = z1 ^ z2;
        self.state[(self.idx + 15) & 0xf] = mat0neg(-2, z0)
            ^ mat0neg(-18, z1)
            ^ mat3neg(-28, z2)
            ^ mat4neg(-5, 0xda442d24, self.state[self.idx]);

        self.idx = (self.idx + 15) & 0xf;
        self.state[self.idx]
    }
}
