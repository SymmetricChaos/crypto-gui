use itertools::Itertools;

use crate::traits::ClassicHasher;

pub struct Sha512 {
    reduced: bool,
}

impl Default for Sha512 {
    fn default() -> Self {
        Self { reduced: false }
    }
}

impl Sha512 {
    pub const K: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];

    // Initialization for SHA512
    pub const SHA512: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    // Initialization for SHA384
    pub const SHA384: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];
}

impl ClassicHasher for Sha512 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding and appending length is identical to MD4 and MD5
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // Step 1.Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 448 mod 512
        // equivalently until the length in bytes is 56 mod 64
        while (input.len() % 64) != 56 {
            input.push(0)
        }

        // Step 2. Append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        // println!("{:0x?}", input);

        // Step 3. Initialize variables
        let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7) = if self.reduced {
            Self::SHA384.iter().copied().collect_tuple().unwrap()
        } else {
            Self::SHA512.iter().copied().collect_tuple().unwrap()
        };
        // Step 4. Process message in 32-word blocks
        for block in input.chunks_exact(128) {
            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;
            let mut f = h5;
            let mut g = h6;
            let mut h = h7;

            let mut x = [0u64; 80];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)).take(32) {
                *elem = u64::from_be_bytes(chunk.try_into().unwrap());
            }

            // println!("{:0x?}", x);

            // Extend the 16 words to 80 words
            for i in 16..80 {
                let s0 = (x[i - 15].rotate_right(1))
                    ^ (x[i - 15].rotate_right(8))
                    ^ (x[i - 15].rotate_right(7));
                let s1 = (x[i - 2].rotate_right(19))
                    ^ (x[i - 2].rotate_right(61))
                    ^ (x[i - 2].rotate_right(6));
                x[i] = x[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(x[i - 7])
                    .wrapping_add(s1);
            }

            println!("{:0x?}", x);

            for i in 0..64 {
                let s1 = e.rotate_right(28) ^ e.rotate_right(34) ^ e.rotate_right(39);
                let ch = (e & f) & (!e & g);
                let temp1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(Self::K[i])
                    .wrapping_add(x[i]);
                let s0 = a.rotate_right(14) ^ a.rotate_right(18) ^ a.rotate_right(41);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
            h5 = h5.wrapping_add(f);
            h6 = h6.wrapping_add(g);
            h7 = h7.wrapping_add(h);
        }

        if self.reduced {
            let mut out = vec![0; 48];
            for (offset, word) in [h0, h1, h2, h3, h4, h5].iter().enumerate() {
                for (i, byte) in word.to_be_bytes().iter().enumerate() {
                    out[i + offset * 8] = *byte
                }
            }
            out
        } else {
            let mut out = vec![0; 64];
            for (offset, word) in [h0, h1, h2, h3, h4, h5, h6, h7].iter().enumerate() {
                for (i, byte) in word.to_be_bytes().iter().enumerate() {
                    out[i + offset * 8] = *byte
                }
            }
            out
        }
    }
}

#[cfg(test)]
mod sha512_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Sha512::default();
        // assert_eq!(
        //     "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
        //     hasher.hash_to_string("".as_bytes())
        // );
        hasher.reduced = true;
        assert_eq!(
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
            hasher.hash_to_string("".as_bytes())
        );
    }
}
