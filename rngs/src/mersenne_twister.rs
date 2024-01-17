use crate::traits::ClassicRng;

pub struct MersenneTwister {
    w: usize,
    n: usize,
    m: usize,
    a: u32,
    u: usize,
    d: u32,
    s: usize,
    b: u32,
    t: usize,
    c: u32,
    l: usize,
    index: usize,
    lower_mask: u32,
    upper_mask: u32,
    arr: [u32; 624],
}

impl Default for MersenneTwister {
    fn default() -> Self {
        let arr = [0u32; 624];
        let lower_mask = (1 << 31) - 1;
        let upper_mask = !lower_mask;
        Self {
            w: 32,         // word size
            n: 624,        // array size
            m: 397,        // middle word
            a: 0x9908b0df, // coefficients of the rational normal form twist matrix
            u: 11,         // tempering bitshift
            d: 0xffffffff, // tempering bitmask
            s: 7,          // tempering bitshift
            b: 0x9d2c5680, // tempering bitmask
            t: 15,         // tempering bitshift
            c: 0xefc60000, // tempering bitmask
            l: 18,         // tempering bitshift
            index: 0,
            lower_mask,
            upper_mask,
            arr,
        }
    }
}

impl MersenneTwister {
    pub fn ksa(&mut self, key: u32) {
        self.arr[0] = key;
        for i in 1..self.n {
            self.arr[i] = 1812433253_u32
                .wrapping_mul(self.arr[i - 1] ^ (self.arr[i - 1] >> (self.w - 2)))
                .wrapping_add(1)
        }
    }

    pub fn twist(&mut self) {
        for i in 0..self.n {
            let x =
                (self.arr[i] & self.upper_mask) | (self.arr[(i + 1) % self.n] & self.lower_mask);
            let mut x_a = x >> 1;
            if x % 2 != 0 {
                x_a = x_a ^ self.a;
            }
            self.arr[i] = self.arr[(i + self.m) % self.n] ^ x_a
        }
        self.index = 0;
    }
}

impl ClassicRng for MersenneTwister {
    fn next_u32(&mut self) -> u32 {
        if self.index >= self.n {
            if self.index > self.n {
                self.ksa(5489) // Seed from 5489
            }
            self.twist();
        }
        let mut y = self.arr[self.index];
        y = y ^ ((y >> self.u) & self.d);
        y = y ^ ((y >> self.s) & self.b);
        y = y ^ ((y >> self.t) & self.c);
        y = y ^ (y >> self.l);
        self.index += 1;

        y
    }
}

#[cfg(test)]
mod mt_tests {

    use super::*;

    #[test]
    fn keystream_test() {
        let mut rng = MersenneTwister::default();
        rng.ksa(5489);
        for _ in 0..10 {
            println!("{}", rng.next_u32())
        }
    }
}
