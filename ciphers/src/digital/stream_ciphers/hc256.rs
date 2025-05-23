fn f1(n: u32) -> u32 {
    n.rotate_right(7) ^ n.rotate_right(18) ^ n.rotate_right(3)
}

fn f2(n: u32) -> u32 {
    n.rotate_right(17) ^ n.rotate_right(19) ^ n.rotate_right(10)
}

pub struct Hc256 {
    p: [u32; 1024],
    q: [u32; 1024],
    ctr: u32,
}

impl Default for Hc256 {
    fn default() -> Self {
        Self {
            p: [0; 1024],
            q: [0; 1024],
            ctr: 0,
        }
    }
}

impl Hc256 {
    fn g1(&self, x: u32, y: u32) -> u32 {
        let i = ((x ^ y) as usize) % 1024;
        (x.rotate_right(10) ^ y.rotate_right(23)).wrapping_add(self.q[i])
    }

    fn g2(&self, x: u32, y: u32) -> u32 {
        let i = ((x ^ y) as usize) % 1024;
        (x.rotate_right(10) ^ y.rotate_right(23)).wrapping_add(self.p[i])
    }

    fn h1(&self, x: u32) -> u32 {
        let [x3, x2, x1, x0] = x.to_be_bytes();
        self.q[x0 as usize]
            .wrapping_add(self.q[x1 as usize + 256])
            .wrapping_add(self.q[x2 as usize + 512])
            .wrapping_add(self.q[x3 as usize + 768])
    }

    fn h2(&self, x: u32) -> u32 {
        let [x3, x2, x1, x0] = x.to_be_bytes();
        self.p[x0 as usize]
            .wrapping_add(self.p[x1 as usize + 256])
            .wrapping_add(self.p[x2 as usize + 512])
            .wrapping_add(self.p[x3 as usize + 768])
    }

    fn with_key_and_iv_u32(key: [u32; 8], iv: [u32; 8]) -> Self {
        let mut w = [0; 2560];
        w[0..8].copy_from_slice(&key);
        w[8..16].copy_from_slice(&iv);

        for i in 16..2560 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32)
        }

        let mut out = Self::default();
        out.p.copy_from_slice(&w[512..1536]);
        out.q.copy_from_slice(&w[1536..2560]);

        for _ in 0..4096 {
            out.step();
        }

        out
    }

    fn step(&mut self) -> u32 {
        let j = (self.ctr % 1024) as usize + 1024;
        let out: u32;
        if self.ctr < 1024 {
            self.p[j % 1024] = self.p[j % 1024]
                .wrapping_add(self.p[(j - 10) % 1024])
                .wrapping_add(self.g1(self.p[(j - 3) % 1024], self.p[(j - 1023) % 1024]));
            out = self.h1(self.p[(j - 12) % 1024]) ^ self.p[j % 1024]
        } else {
            self.q[j % 1024] = self.q[j % 1024]
                .wrapping_add(self.q[(j - 10) % 1024])
                .wrapping_add(self.g2(self.q[(j - 3) % 1024], self.q[(j - 1023) % 1024]));
            out = self.h2(self.q[(j - 12) % 1024]) ^ self.q[j % 1024]
        }
        self.ctr = (self.ctr + 1) % 2048;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn keystream_1() {
        let mut cipher = Hc256::with_key_and_iv_u32([0; 8], [0; 8]);
        for _ in 0..16 {
            println!("{:08x?}", cipher.step());
        }
        let mut cipher = Hc256::with_key_and_iv_u32([0; 8], [0; 8]);
        assert_eq!(0x8589075b, cipher.step());
        // assert_eq!(0x0df3f6d8, cipher.step());
        // assert_eq!(0x2fc0c542, cipher.step());
        // assert_eq!(0x5179b6a6, cipher.step());
        // assert_eq!(0x3465f053, cipher.step());
        // assert_eq!(0xf2891f80, cipher.step());
        // assert_eq!(0x8b24744e, cipher.step());
        // assert_eq!(0x18480b72, cipher.step());
        // assert_eq!(0xec2792cd, cipher.step());
        // assert_eq!(0xbf4dcfeb, cipher.step());
        // assert_eq!(0x7769bf8d, cipher.step());
        // assert_eq!(0xfa14aee4, cipher.step());
        // assert_eq!(0x7b4c50e8, cipher.step());
        // assert_eq!(0xeaf3a9c8, cipher.step());
        // assert_eq!(0xf506016c, cipher.step());
        // assert_eq!(0x81697e32, cipher.step());
    }

    #[test]
    fn keystream_2() {
        let mut cipher = Hc256::with_key_and_iv_u32([0; 8], [1, 0, 0, 0, 0, 0, 0, 0]);
        for _ in 0..16 {
            println!("{:08x?}", cipher.step());
        }
        let mut cipher = Hc256::with_key_and_iv_u32([0; 8], [1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(0xbfa2e2af, cipher.step());
        // assert_eq!(0xe9ce174f, cipher.step());
        // assert_eq!(0x8b05c2fe, cipher.step());
        // assert_eq!(0xb18bb1d1, cipher.step());
        // assert_eq!(0xee42c05f, cipher.step());
        // assert_eq!(0x01312b71, cipher.step());
        // assert_eq!(0xc61f50dd, cipher.step());
        // assert_eq!(0x502a080b, cipher.step());
        // assert_eq!(0xedfec706, cipher.step());
        // assert_eq!(0x633d9241, cipher.step());
        // assert_eq!(0xa6dac448, cipher.step());
        // assert_eq!(0xaf8561ff, cipher.step());
        // assert_eq!(0x5e04135a, cipher.step());
        // assert_eq!(0x9448c434, cipher.step());
        // assert_eq!(0x2de7e9f3, cipher.step());
        // assert_eq!(0x37520bdf, cipher.step());
    }

    #[test]
    fn keystream_3() {
        let mut cipher = Hc256::with_key_and_iv_u32([0x55, 0, 0, 0, 0, 0, 0, 0], [0; 8]);
        for _ in 0..16 {
            println!("{:08x?}", cipher.step());
        }
        let mut cipher = Hc256::with_key_and_iv_u32([0x55, 0, 0, 0, 0, 0, 0, 0], [0; 8]);
        assert_eq!(0xfe4a401c, cipher.step());
        // assert_eq!(0xed5fe24f, cipher.step());
        // assert_eq!(0xd19a8f95, cipher.step());
        // assert_eq!(0x6fc036ae, cipher.step());
        // assert_eq!(0x3c5aa688, cipher.step());
        // assert_eq!(0x23e2abc0, cipher.step());
        // assert_eq!(0x2f90b3ae, cipher.step());
        // assert_eq!(0xa8d30e42, cipher.step());
        // assert_eq!(0x59f03a6c, cipher.step());
        // assert_eq!(0x6e39eb44, cipher.step());
        // assert_eq!(0x8f7579fb, cipher.step());
        // assert_eq!(0x70137a5e, cipher.step());
        // assert_eq!(0x6d10b7d8, cipher.step());
        // assert_eq!(0xadd0f7cd, cipher.step());
        // assert_eq!(0x723423da, cipher.step());
        // assert_eq!(0xf575dde6, cipher.step());
    }
}
