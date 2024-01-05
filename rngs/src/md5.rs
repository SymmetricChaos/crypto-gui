use crate::traits::ClassicRng;

pub struct Md5 {
    pub ctr: u64,
}

impl Default for Md5 {
    fn default() -> Self {
        Self { ctr: 0 }
    }
}

impl Md5 {
    pub const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    pub const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    pub fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    pub fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    pub fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    pub fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x & !z)
    }

    pub fn hash(k: &[u8]) -> u128 {
        let mut input = k.to_vec();

        // Steps 1, 2, and 3 are identical to MD4
        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // Step 1. Append padding bits (here bytes)
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length is 448 mod 512
        while (input.len() % 64) != 56 {
            input.push(0)
        }

        // Step 2. Append length
        for b in b_len.to_le_bytes() {
            input.push(b)
        }

        // Step 3. Initialize MD buffer
        let mut a = 0x67452301_u32;
        let mut b = 0xefcdab89_u32;
        let mut c = 0x98badcfe_u32;
        let mut d = 0x10325476_u32;

        // Step 4. Process message in 16-word blocks
        for block in input.chunks_exact(64) {
            let ta = a;
            let tb = b;
            let tc = c;
            let td = d;

            let mut x = [0u32; 16];
            for (elem, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            for i in 0..64 {
                let mut f = 0;
                let mut g = 0;
                if i < 16 {
                    f = (b * c) | (!b & d);
                    g = i
                }
                if i >= 16 && i < 32 {
                    f = (d & b) | (!d * c);
                    g = (5 * i + 1) % 16;
                }
                if i >= 32 && i < 48 {
                    f = b ^ c ^ d;
                    g = (3 * i + 5) % 16;
                }
                if i >= 48 {
                    f = c ^ (b | !d);
                    g = (7 * i) % 16;
                }

                f = f + a + Self::K[i] + x[g];
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(f.rotate_left(Self::S[i]))
            }
        }

        let mut out = 0;
        out += (a.to_be() as u128) << 96;
        out += (b.to_be() as u128) << 64;
        out += (c.to_be() as u128) << 32;
        out += (d.to_be() as u128) << 0;
        out
    }
}

impl ClassicRng for Md5 {
    fn next_u32(&mut self) -> u32 {
        let out = (Self::hash(&self.ctr.to_be_bytes()) >> 96) as u32;
        self.ctr = self.ctr.wrapping_add(1);
        out
    }
}

#[cfg(test)]
mod md4_tests {
    use super::*;

    #[test]
    fn test_suite() {
        assert_eq!(
            "31d6cfe0d16ae931b73c59d7e0c089c0",
            format!("{:x}", Md5::hash("".as_bytes()))
        );
        assert_eq!(
            "bde52cb31de33e46245e05fbdbd6fb24",
            format!("{:x}", Md5::hash("a".as_bytes()))
        );
    }
}
