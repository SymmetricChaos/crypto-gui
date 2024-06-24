use num::Integer;
use utils::byte_formatting::ByteFormat;

use crate::{Cipher, CipherError};

// https://www.cs.ucdavis.edu/~rogaway/papers/seal.pdf
pub struct Seal3 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: [u32; 5], // 160-bit key
    pub n: u32,
    pub l: u32, // number of output bits, limited to 524288 (64 * 1024 * 8)
    // registers: [u32; 4],
    // ns: [u32; 4],
    r: [u32; 256], // r is sometimes not completely used depending on the value of l
    t: [u32; 512],
    s: [u32; 256],
}

impl Default for Seal3 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: [0; 5],
            n: 0,
            l: 524288,
            // registers: [0; 4],
            // ns: [0; 4],
            r: [0; 256],
            t: [0; 512],
            s: [0; 256],
        }
    }
}

impl Seal3 {
    /// This is taken from the SHA1 compression function
    pub fn g(&self, i: u32) -> [u32; 5] {
        let mut a = self.key[0];
        let mut b = self.key[1];
        let mut c = self.key[2];
        let mut d = self.key[3];
        let mut e = self.key[4];

        let mut w = [0u32; 80];
        w[0] = i.to_le();

        // Fill all of w
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1)
        }

        for n in 0..80 {
            let mut f = 0;
            let mut g = 0;
            if n < 20 {
                f = (b & c) | (!b & d);
                g = 0x5a827999;
            }
            if n >= 20 && n < 40 {
                f = b ^ c ^ d;
                g = 0x6ed9eba1;
            }
            if n >= 40 && n < 60 {
                f = (b & c) | (b & d) | (c & d);
                g = 0x8f1bbcdc;
            }
            if n >= 60 {
                f = b ^ c ^ d;
                g = 0xca62c1d6;
            }

            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(g)
                .wrapping_add(w[n]);
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
        let (q, r) = i.div_mod_floor(&5);
        self.g(q)[r as usize]
    }

    pub fn derive_tables(&mut self) {
        for i in 0..self.t.len() {
            self.t[i] = self.gamma(i as u32);
        }
        for i in 0..self.s.len() {
            self.s[i] = self.gamma(0x1000 + i as u32);
        }
        for i in 0..self.r.len() {
            self.r[i] = self.gamma(0x2000 + i as u32);
        }
    }

    fn twist(&self, p: &mut u32, r1: &mut u32, r2: &mut u32) {
        *p = *r1 & 0x7fc_u32;
        *r2 = r2.wrapping_add(self.t[*p as usize / 4]);
        *r1 = r1.rotate_right(9);
    }

    pub fn initialize(&self, ctr: usize, registers: &mut [u32; 4], ns: &mut [u32; 4]) {
        let mut a = self.n ^ self.r[4 * ctr];
        let mut b = self.n.rotate_right(8) ^ self.r[4 * ctr + 1];
        let mut c = self.n.rotate_right(16) ^ self.r[4 * ctr + 2];
        let mut d = self.n.rotate_right(23) ^ self.r[4 * ctr + 3];

        let mut p = 0;

        for _ in 1..=2 {
            self.twist(&mut p, &mut a, &mut b);
            self.twist(&mut p, &mut b, &mut c);
            self.twist(&mut p, &mut c, &mut d);
            self.twist(&mut p, &mut d, &mut a);
        }

        *ns = [d, b, a, c];

        self.twist(&mut p, &mut a, &mut b);
        self.twist(&mut p, &mut b, &mut c);
        self.twist(&mut p, &mut c, &mut d);
        self.twist(&mut p, &mut d, &mut a);

        *registers = [a, b, c, d];
    }

    fn keystream(&self, p: &mut u32, q: &mut u32, registers: &mut [u32; 4], ns: &mut [u32; 4]) {}

    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        let mut ctr = 0;
        let mut registers = [0; 4];
        let mut ns = [0; 4];
        let mut chunks = bytes.chunks_mut(16);

        loop {
            self.initialize(ctr, &mut registers, &mut ns);
            for i in 1..=64 {
                // Generate keystream bytes
                if let Some(chunk) = chunks.next() {
                    // XOR keystream into the chunk
                } else {
                    return ();
                }
            }

            ctr += 1;
        }
    }
}

impl Cipher for Seal3 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        self.encrypt_bytes(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&bytes))
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
        let mut cipher = Seal3::default();
        cipher.key = [0x67462301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

        cipher.derive_tables();

        // assert_eq!(
        //     &cipher.r[0..16],
        //     &[
        //         0x5021758d, 0xce577c11, 0xfa5bd5dd, 0x366d1b93, 0x182cff72, 0xac06d7c6, 0x2683ead8,
        //         0xfabe3573, 0x82a10c96, 0x48c483bd, 0xca92285c, 0x71fe84c0, 0xbd76b700, 0x6fdcc20c,
        //         0x8dada151, 0x4506dd64
        //     ]
        // );

        println!("R: {:08x?}", &cipher.r[0..4]);
        println!("T: {:08x?}", &cipher.t[0..4]);
        println!("S: {:08x?}", &cipher.s[0..4]);
    }
}
