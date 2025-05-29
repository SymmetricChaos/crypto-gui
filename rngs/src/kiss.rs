use crate::ClassicRng;

const MASK16: u32 = 0xffff;

//http://www.ciphersbyritter.com/NEWS4/RANDC.HTM#369B5E30.65A55FD1@stat.fsu.edu
/*
Subject: Random numbers for C: The END?
Date: Wed, 20 Jan 1999 10:55:14 -0500
From: George Marsaglia <geo@stat.fsu.edu>
Message-ID: <36A5FC62.17C9CC33@stat.fsu.edu>
Newsgroups: sci.stat.math,sci.math
Lines: 301
*/

/*
#define znew   (z=36969*(z&65535)+(z>>16))
#define wnew   (w=18000*(w&65535)+(w>>16))
#define MWC    ((znew<<16)+wnew )
#define SHR3  (jsr^=(jsr<<17), jsr^=(jsr>>13), jsr^=(jsr<<5))
#define CONG  (jcong=69069*jcong+1234567)
#define KISS  ((MWC^CONG)+SHR3)
 */

pub struct Kiss99 {
    w: u32,
    z: u32,
    jcong: u32,
    jsr: u32,
}

impl Default for Kiss99 {
    fn default() -> Self {
        Self {
            w: 521288629,
            z: 362436069,
            jcong: 380116160,
            jsr: 123456789,
        }
    }
}

impl Kiss99 {
    // Pair of 16 bit multiply with carry generators
    fn mwc(&mut self) -> u32 {
        self.w = 36969_u32
            .wrapping_mul(self.w & MASK16)
            .wrapping_add(self.w >> 16);
        self.z = 18000_u32
            .wrapping_mul(self.z & MASK16)
            .wrapping_add(self.z >> 16);
        (self.z << 16).wrapping_add(self.w)
    }

    // An xorshift generator
    fn shr3(&mut self) -> u32 {
        self.jsr ^= self.jsr << 17; // the 17 and 13 should be switched for a maximal length generator, likely a typo as it is corrected in later version
        self.jsr ^= self.jsr >> 13;
        self.jsr ^= self.jsr << 5;
        self.jsr
    }

    // A linear congruential generator
    fn cong(&mut self) -> u32 {
        self.jcong = 69069_u32.wrapping_mul(self.jcong).wrapping_add(1234567);
        self.jcong
    }
}

impl ClassicRng for Kiss99 {
    fn next_u32(&mut self) -> u32 {
        (self.mwc() ^ self.cong()).wrapping_add(self.shr3())
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
