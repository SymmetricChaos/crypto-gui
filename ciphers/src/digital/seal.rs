use crate::{Cipher, CipherError};

// https://www.cs.ucdavis.edu/~rogaway/papers/seal.pdf
pub struct Seal {
    registers: [u32; 4],
    pub key: [u32; 5], // 160-bit key
    n: u32,
    pub r: [u32; 256],
    pub t: [u32; 512],
    pub s: [u32; 256],
}

impl Default for Seal {
    fn default() -> Self {
        Self {
            registers: [0; 4],
            key: [0; 5],
            n: 0,
            r: [0; 256],
            t: [0; 512],
            s: [0; 256],
        }
    }
}

impl Seal {
    pub fn g(&self, i: u32) -> [u32; 5] {
        let mut a = self.key[0];
        let mut b = self.key[1];
        let mut c = self.key[2];
        let mut d = self.key[3];
        let mut e = self.key[4];

        // Initial 512 bit state
        let mut w = [0u32; 80];
        w[0] = i;

        // Extend the 16 words to 80 words
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1)
        }

        for i in 0..80 {
            let mut f = 0;
            let mut g = 0;
            if i < 20 {
                f = (b & c) | (!b & d);
                g = 0x5a827999;
            }
            if i >= 20 && i < 40 {
                f = b ^ c ^ d;
                g = 0x6ed9eba1;
            }
            if i >= 40 && i < 60 {
                f = (b & c) | (b & d) | (c & d);
                g = 0x8f1bbcdc;
            }
            if i >= 60 {
                f = b ^ c ^ d;
                g = 0xca62c1d6;
            }

            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(g)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        a = a.wrapping_add(a);
        b = b.wrapping_add(b);
        c = c.wrapping_add(c);
        d = d.wrapping_add(d);
        e = e.wrapping_add(e);

        [a, b, c, d, e]
    }

    pub fn gamma(&self, i: u32) -> u32 {
        self.g(i / 5)[(i as usize) % 5]
    }

    pub fn derive_tables(&mut self) {
        for i in 0..512 {
            self.t[i] = self.gamma(i as u32);
        }
        for i in 0..256 {
            self.s[i] = self.gamma(0x1000 + i as u32);
        }
        for i in 0..256 {
            self.r[i] = self.gamma(0x2000 + i as u32);
        }
    }

    pub fn initialize() {}
}

impl Cipher for Seal {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    // Decryption is identical
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt(text)
    }
}
#[cfg(test)]
mod seal_tests {

    use super::*;

    #[test]
    fn tables() {
        let mut cipher = Seal::default();
        cipher.key = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

        cipher.derive_tables();

        println!("{:08x?}", cipher.r);
    }
}
