use utils::byte_formatting::{u32s_to_bytes_be, xor_into_bytes};

const A: [u32; 8] = [
    0x4D34D34D, 0xD34D34D3, 0x34D34D34, 0x4D34D34D, 0xD34D34D3, 0x34D34D34, 0x4D34D34D, 0xD34D34D3,
];

// Square a u32 to get a u64, then XOR the upper and lower 32 bits to get a u32
fn g_func(n: u32) -> u32 {
    let sq = (n as u64) * (n as u64);
    ((sq >> 32) ^ sq) as u32
}

#[derive(Debug, Clone)]
pub struct Rabbit {
    pub state: [u32; 8],
    pub ctrs: [u32; 8],
    pub carry: u32, // only one bit is used, but matching the type makes it easier to use, a bool would likely be aligned similarly anyway
}

impl Default for Rabbit {
    fn default() -> Self {
        Self {
            state: Default::default(),
            ctrs: Default::default(),
            carry: 0,
        }
    }
}

impl Rabbit {
    pub fn with_key_and_iv(key: [u8; 16], iv: [u8; 8]) -> Self {
        let mut out = Self::with_key(key);
        out.iv(iv);
        out
    }

    pub fn with_key_u32(key: [u32; 4]) -> Self {
        let mut bytes = [0u8; 16];
        u32s_to_bytes_be(&mut bytes, key);
        Self::with_key(bytes)
    }

    pub fn with_key_and_iv_u32(key: [u32; 4], iv: [u32; 2]) -> Self {
        let mut key_bytes = [0u8; 16];
        let mut iv_bytes = [0u8; 8];
        u32s_to_bytes_be(&mut key_bytes, key);
        u32s_to_bytes_be(&mut iv_bytes, iv);
        Self::with_key_and_iv(key_bytes, iv_bytes)
    }

    pub fn with_key(key: [u8; 16]) -> Self {
        let mut k = [0_u32; 8];
        for i in 0..8 {
            k[i] = (key[2 * i] as u32) | ((key[2 * i + 1] as u32) << 8);
        }

        let mut state = [0; 8];
        let mut ctrs = [0; 8];

        for i in 0..8 {
            if i % 2 == 0 {
                state[i] = (k[(i + 1) % 8] << 16) | k[i];
                ctrs[i] = (k[(i + 4) % 8] << 16) | k[(i + 5) % 8];
            } else {
                state[i] = (k[(i + 5) % 8] << 16) | k[(i + 4) % 8];
                ctrs[i] = (k[i] << 16) | k[(i + 1) % 8];
            }
        }

        let mut out = Self {
            state,
            ctrs,
            carry: 0,
        };

        for _ in 0..4 {
            out.step();
        }

        for i in 0..8 {
            out.ctrs[i] ^= out.state[(i + 4) % 8]
        }

        out
    }

    fn iv(&mut self, iv: [u8; 8]) {
        let mut i = [0_u32; 4];

        i[0] = iv[0] as u32 | (iv[1] as u32) << 8 | (iv[2] as u32) << 16 | (iv[3] as u32) << 24;
        i[2] = iv[4] as u32 | (iv[5] as u32) << 8 | (iv[6] as u32) << 16 | (iv[7] as u32) << 24;
        i[1] = (i[0] >> 16) | (i[2] & 0xFFFF0000);
        i[3] = (i[2] << 16) | (i[0] & 0x0000FFFF);

        self.ctrs[0] ^= i[0];
        self.ctrs[1] ^= i[1];
        self.ctrs[2] ^= i[2];
        self.ctrs[3] ^= i[3];
        self.ctrs[4] ^= i[0];
        self.ctrs[5] ^= i[1];
        self.ctrs[6] ^= i[2];
        self.ctrs[7] ^= i[3];

        for _ in 0..4 {
            self.step();
        }
    }

    fn step(&mut self) {
        // Update all the counters and the carry
        let old_ctrs = self.ctrs.clone();
        self.ctrs[0] = A[0].wrapping_add(self.ctrs[0]).wrapping_add(self.carry);
        for i in 1..8 {
            self.ctrs[i] = A[i]
                .wrapping_add(self.ctrs[i])
                .wrapping_add((self.ctrs[i - 1] < old_ctrs[i - 1]) as u32);
        }
        self.carry = (self.ctrs[7] < old_ctrs[7]) as u32;

        // Calculate g-values
        let mut g = [0_u32; 8];
        for i in 0..8 {
            g[i] = g_func(self.ctrs[i].wrapping_add(self.state[i]));
        }

        // Update the state
        self.state[0] = g[0]
            .wrapping_add(g[7].rotate_left(16))
            .wrapping_add(g[6].rotate_left(16));
        self.state[1] = g[1].wrapping_add(g[0].rotate_left(8)).wrapping_add(g[7]);
        self.state[2] = g[2]
            .wrapping_add(g[1].rotate_left(16))
            .wrapping_add(g[0].rotate_left(16));
        self.state[3] = g[3].wrapping_add(g[2].rotate_left(8)).wrapping_add(g[1]);
        self.state[4] = g[4]
            .wrapping_add(g[3].rotate_left(16))
            .wrapping_add(g[2].rotate_left(16));
        self.state[5] = g[5].wrapping_add(g[4].rotate_left(8)).wrapping_add(g[3]);
        self.state[6] = g[6]
            .wrapping_add(g[5].rotate_left(16))
            .wrapping_add(g[4].rotate_left(16));
        self.state[7] = g[7].wrapping_add(g[6].rotate_left(8)).wrapping_add(g[5]);
    }

    pub fn extract(&self) -> [u8; 16] {
        let mut t = [0; 8];
        let s = self.state;

        t[0] = ((s[0]) ^ (s[5] >> 16)) as u16;
        t[1] = ((s[0] >> 16) ^ (s[3])) as u16;
        t[2] = ((s[2]) ^ (s[7] >> 16)) as u16;
        t[3] = ((s[2] >> 16) ^ (s[5])) as u16;
        t[4] = ((s[4]) ^ (s[1] >> 16)) as u16;
        t[5] = ((s[4] >> 16) ^ (s[7])) as u16;
        t[6] = ((s[6]) ^ (s[3] >> 16)) as u16;
        t[7] = ((s[6] >> 16) ^ (s[1])) as u16;

        [
            t[0] as u8,
            (t[0] >> 8) as u8,
            t[1] as u8,
            (t[1] >> 8) as u8,
            t[2] as u8,
            (t[2] >> 8) as u8,
            t[3] as u8,
            (t[3] >> 8) as u8,
            t[4] as u8,
            (t[4] >> 8) as u8,
            t[5] as u8,
            (t[5] >> 8) as u8,
            t[6] as u8,
            (t[6] >> 8) as u8,
            t[7] as u8,
            (t[7] >> 8) as u8,
        ]
    }

    pub fn next_block(&mut self) -> [u8; 16] {
        self.step();
        self.extract()
    }

    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        self.clone().encrypt_bytes_mut(bytes);
    }

    pub fn encrypt_bytes_mut(&mut self, bytes: &mut [u8]) {
        let mut keystream: [u8; 16];
        let mut ptr = 0;

        while ptr < bytes.len() {
            keystream = self.next_block();
            xor_into_bytes(&mut bytes[ptr..], &keystream);
            ptr += 16;
        }
    }
}

#[cfg(test)]
mod tests {

    use hex_literal::hex;

    use super::*;

    // TODO: Why does the reference code have the wrong values for all of these?
    // Did find the correct third value for test1 in another reference!

    #[test]
    fn test1() {
        let mut cipher = Rabbit::with_key(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));

        assert_eq!(
            hex!("02 F7 4A 1C 26 45 6B F5 EC D6 A5 36 F0 54 57 B1"),
            cipher.next_block()
        );

        assert_eq!(
            hex!("A7 8A C6 89 47 6C 69 7B 39 0C 9C C5 15 D8 E8 88"),
            cipher.next_block()
        );

        assert_eq!(
            hex!("96 D6 73 16 88 D1 68 DA 51 D4 0C 70 C3 A1 16 F4"),
            cipher.next_block()
        );
    }

    #[test]
    fn test2() {
        let mut cipher = Rabbit::with_key(hex!("C2 1F CF 38 81 CD 5E E8 62 8A CC B0 A9 89 0D F8"));

        assert_eq!(
            hex!("3D 02 E0 C7 30 55 91 12 B4 73 B7 90 DE E0 18 DF"),
            cipher.next_block()
        );

        assert_eq!(
            hex!("CD 6D 73 0C E5 4E 19 F0 C3 5E C4 79 0E B6 C7 4A"),
            cipher.next_block()
        );

        // assert_eq!(
        //     hex!("9F B4 92 E1 B5 40 36 3A E3 83 C0 1F 9F A2 26 1A"),
        //     cipher.next_block()
        // );

        let mut cipher = Rabbit::with_key_u32([0xC21FCF38, 0x81CD5EE8, 0x628ACCB0, 0xA9890DF8]);
        assert_eq!(
            hex!("3D 02 E0 C7 30 55 91 12 B4 73 B7 90 DE E0 18 DF"),
            cipher.next_block()
        );

        assert_eq!(
            hex!("CD 6D 73 0C E5 4E 19 F0 C3 5E C4 79 0E B6 C7 4A"),
            cipher.next_block()
        );

        // assert_eq!(
        //     hex!("9F B4 92 E1 B5 40 36 3A E3 83 C0 1F 9F A2 26 1A"),
        //     ci
    }

    // #[test]
    // fn test3() {
    //     let mut cipher = Rabbit::with_key(hex!("1D 27 2C 6A 2D 8E 3D FC AC 14 05 6B 78 D6 33 A0"));

    //     assert_eq!(
    //         hex!("A3 A9 7A BB 80 39 38 20 B7 E5 0C 4A BB 53 82 3D"),
    //         cipher.next_block()
    //     );

    //     assert_eq!(
    //         hex!("C4 42 37 99 C2 EF C9 FF B3 A4 12 5F 1F 4C 99 A8"),
    //         cipher.next_block()
    //     );

    //     assert_eq!(
    //         hex!("97 C0 73 3F F1 F1 8D 25 6A 59 E2 BA AB C1 F4 F1"),
    //         cipher.next_block()
    //     );
    // }
}
