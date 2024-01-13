use crate::traits::ClassicHasher;

pub struct Sha256 {}

impl Default for Sha256 {
    fn default() -> Self {
        Self {}
    }
}

impl Sha256 {
    pub const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
}

impl ClassicHasher for Sha256 {
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

        // Step 3. Initialize variables
        let mut h0 = 0x6a09e667_u32;
        let mut h1 = 0xbb67ae85_u32;
        let mut h2 = 0x3c6ef372_u32;
        let mut h3 = 0xa54ff53a_u32;
        let mut h4 = 0x510e527f_u32;
        let mut h5 = 0x9b05688c_u32;
        let mut h6 = 0x1f83d9ab_u32;
        let mut h7 = 0x5be0cd19_u32;

        // Step 4. Process message in 16-word blocks
        for block in input.chunks_exact(64) {
            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;
            let mut f = h5;
            let mut g = h6;
            let mut h = h7;

            let mut x = [0u32; 64];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)).take(16) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            // Extend the 16 words to 64 words
            for i in 16..64 {
                let s0 = (x[i - 15].rotate_right(7))
                    ^ (x[i - 15].rotate_right(18))
                    ^ (x[i - 15].rotate_right(3));
                let s1 = (x[i - 2].rotate_right(17))
                    ^ (x[i - 2].rotate_right(19))
                    ^ (x[i - 2].rotate_right(10));
                x[i] = x[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(x[i - 2])
                    .wrapping_add(s1);
            }

            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) & (!e & g);
                let temp1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(Self::K[i])
                    .wrapping_add(x[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
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

        let mut out = vec![0; 32];
        for (offset, word) in [h0, h1, h2, h3, h4, h5, h6, h7].iter().enumerate() {
            for (i, byte) in word.to_be_bytes().iter().enumerate() {
                out[i + offset * 4] = *byte
            }
        }
        out
    }
}

#[cfg(test)]
mod sha256_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let hasher = Sha256::default();
        assert_eq!(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            hasher.hash_to_string("".as_bytes())
        );
    }
}
