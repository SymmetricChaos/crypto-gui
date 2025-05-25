use crate::ClassicRng;

fn f1(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

fn f2(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

fn g1(x: u32, y: u32, z: u32) -> u32 {
    (x.rotate_right(10) ^ z.rotate_right(23)).wrapping_add(y.rotate_right(8))
}

fn g2(x: u32, y: u32, z: u32) -> u32 {
    (x.rotate_left(10) ^ z.rotate_left(23)).wrapping_add(y.rotate_left(8))
}

// Subtraction modulo 512
macro_rules! sub {
    ($n: expr, $s: literal) => {
        $n.wrapping_sub($s) & 0x1FF
    };
}

#[derive(Debug, Clone)]
pub struct Hc128 {
    p: [u32; 512],
    q: [u32; 512],
    ctr: u32,
}

impl Default for Hc128 {
    fn default() -> Self {
        Self {
            p: [0; 512],
            q: [0; 512],
            ctr: 0,
        }
    }
}

impl Hc128 {
    fn h1(&self, x: u32) -> u32 {
        let x0 = x & 0xff;
        let x2 = (x >> 16) & 0xff;
        self.q[x0 as usize].wrapping_add(self.q[(256 + x2) as usize])
    }

    fn h2(&self, x: u32) -> u32 {
        let x0 = x & 0xff;
        let x2 = (x >> 16) & 0xff;
        self.p[x0 as usize].wrapping_add(self.p[(256 + x2) as usize])
    }

    pub fn with_key_and_iv_u32(key: [u32; 4], iv: [u32; 4]) -> Self {
        let mut w = [0; 1280];
        w[0..4].copy_from_slice(&key);
        w[4..8].copy_from_slice(&key);
        w[8..12].copy_from_slice(&iv);
        w[12..16].copy_from_slice(&iv);

        for i in 16..1280 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32)
        }

        let mut out = Self::default();
        out.p.copy_from_slice(&w[256..768]);
        out.q.copy_from_slice(&w[768..1280]);

        // These steps are the same as normal stepping but they XOR the output back into the Sbox
        for i in 0..512 {
            out.p[i] = out.p[i].wrapping_add(g1(
                out.p[sub!(i, 3)],
                out.p[sub!(i, 10)],
                out.p[sub!(i, 511)],
            )) ^ out.h1(out.p[sub!(i, 12)]);
        }
        for i in 0..512 {
            out.q[i] = out.q[i].wrapping_add(g2(
                out.q[sub!(i, 3)],
                out.q[sub!(i, 10)],
                out.q[sub!(i, 511)],
            )) ^ out.h2(out.q[sub!(i, 12)]);
        }

        out
    }

    pub fn step(&mut self) -> u32 {
        let j = (self.ctr % 512) as usize;
        let out: u32;
        if self.ctr < 512 {
            self.p[j] = self.p[j].wrapping_add(g1(
                self.p[sub!(j, 3)],
                self.p[sub!(j, 10)],
                self.p[sub!(j, 511)],
            ));
            out = self.h1(self.p[sub!(j, 12)]) ^ self.p[j]
        } else {
            self.q[j] = self.q[j].wrapping_add(g2(
                self.q[sub!(j, 3)],
                self.q[sub!(j, 10)],
                self.q[sub!(j, 511)],
            ));
            out = self.h2(self.q[sub!(j, 12)]) ^ self.q[j]
        }
        self.ctr = (self.ctr + 1) % 1024;
        out
    }

    pub fn next_block(&mut self) -> [u8; 4] {
        self.step().to_be_bytes()
    }

    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        self.clone().encrypt_bytes_mut(bytes);
    }

    pub fn encrypt_bytes_mut(&mut self, bytes: &mut [u8]) {
        let mut keystream: [u8; 4];
        let mut ptr = 0;

        while ptr < bytes.len() {
            keystream = self.next_block();
            xor_into_bytes(&mut bytes[ptr..(ptr + 4)], &keystream);
            ptr += 4;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn keystream_1() {
        let mut cipher = Hc128::with_key_and_iv_u32([0; 4], [0; 4]);
        assert_eq!(0x73150082, cipher.next_u32());
        assert_eq!(0x3bfd03a0, cipher.next_u32());
        assert_eq!(0xfb2fd77f, cipher.next_u32());
        assert_eq!(0xaa63af0e, cipher.next_u32());

        assert_eq!(0xde122fc6, cipher.next_u32());
        assert_eq!(0xa7dc29b6, cipher.next_u32());
        assert_eq!(0x62a68527, cipher.next_u32());
        assert_eq!(0x8b75ec68, cipher.next_u32());

        assert_eq!(0x9036db1e, cipher.next_u32());
        assert_eq!(0x81896005, cipher.next_u32());
        assert_eq!(0x00ade078, cipher.next_u32());
        assert_eq!(0x491fbf9a, cipher.next_u32());

        assert_eq!(0x1cdc3013, cipher.next_u32());
        assert_eq!(0x6c3d6e24, cipher.next_u32());
        assert_eq!(0x90f664b2, cipher.next_u32());
        assert_eq!(0x9cd57102, cipher.next_u32());
    }

    #[test]
    fn keystream_2() {
        let mut cipher = Hc128::with_key_and_iv_u32([0; 4], [1, 0, 0, 0]);
        assert_eq!(0xc01893d5, cipher.next_u32());
        assert_eq!(0xb7dbe958, cipher.next_u32());
        assert_eq!(0x8f65ec98, cipher.next_u32());
        assert_eq!(0x64176604, cipher.next_u32());

        assert_eq!(0x36fc6724, cipher.next_u32());
        assert_eq!(0xc82c6eec, cipher.next_u32());
        assert_eq!(0x1b1c38a7, cipher.next_u32());
        assert_eq!(0xc9b42a95, cipher.next_u32());

        assert_eq!(0x323ef123, cipher.next_u32());
        assert_eq!(0x0a6a908b, cipher.next_u32());
        assert_eq!(0xce757b68, cipher.next_u32());
        assert_eq!(0x9f14f7bb, cipher.next_u32());

        assert_eq!(0xe4cde011, cipher.next_u32());
        assert_eq!(0xaeb5173f, cipher.next_u32());
        assert_eq!(0x89608c94, cipher.next_u32());
        assert_eq!(0xb5cf46ca, cipher.next_u32());
    }

    #[test]
    fn keystream_3() {
        let mut cipher = Hc128::with_key_and_iv_u32([0x55, 0, 0, 0], [0; 4]);
        assert_eq!(0x518251a4, cipher.next_u32());
        assert_eq!(0x04b4930a, cipher.next_u32());
        assert_eq!(0xb02af931, cipher.next_u32());
        assert_eq!(0x0639f032, cipher.next_u32());

        assert_eq!(0xbcb4a47a, cipher.next_u32());
        assert_eq!(0x5722480b, cipher.next_u32());
        assert_eq!(0x2bf99f72, cipher.next_u32());
        assert_eq!(0xcdc0e566, cipher.next_u32());

        assert_eq!(0x310f0c56, cipher.next_u32());
        assert_eq!(0xd3cc83e8, cipher.next_u32());
        assert_eq!(0x663db8ef, cipher.next_u32());
        assert_eq!(0x62dfe07f, cipher.next_u32());

        assert_eq!(0x593e1790, cipher.next_u32());
        assert_eq!(0xc5ceaa9c, cipher.next_u32());
        assert_eq!(0xab03806f, cipher.next_u32());
        assert_eq!(0xc9a6e5a0, cipher.next_u32());
    }
}
