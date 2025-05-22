fn f1(n: u32) -> u32 {
    n.rotate_right(7) ^ n.rotate_right(18) ^ n.rotate_right(3)
}

fn f2(n: u32) -> u32 {
    n.rotate_right(17) ^ n.rotate_right(19) ^ n.rotate_right(10)
}

pub struct Hc256 {
    p: [u32; 1024],
    q: [u32; 1024],
    ctr: usize,
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
        (x.rotate_right(10) ^ y.rotate_right(25)).wrapping_add(self.q[i])
    }

    fn g2(&self, x: u32, y: u32) -> u32 {
        let i = ((x ^ y) as usize) % 1024;
        (x.rotate_right(10) ^ y.rotate_right(25)).wrapping_add(self.p[i])
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

        for i in 0..4096 {
            out.step();
        }

        out
    }

    fn step(&mut self) -> u32 {}
}
