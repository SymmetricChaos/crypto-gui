use crate::traits::ClassicRng;

// https://www.vmpcfunction.com/VMPC-R.pdf
pub struct Vmpcr {
    pub arr_p: [u8; Self::N],
    pub arr_s: [u8; Self::N],
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub n: u8,
    pub big_endian: bool,
}

impl Default for Vmpcr {
    fn default() -> Self {
        let mut arr = [0u8; Self::N];
        for n in 0..Self::N {
            arr[n] = n as u8;
        }
        Self {
            arr_p: arr,
            arr_s: arr,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            n: 0,
            big_endian: true,
        }
    }
}

impl Vmpcr {
    // Array size
    pub const N: usize = 256;

    // The Variably Modified Permutation Composition function for which the RNG and cipher are named
    pub fn vmpc(idx: usize, arr: &[u8; Self::N]) -> u8 {
        arr[arr[arr[idx] as usize].wrapping_add(1) as usize]
    }

    pub fn ksa(&mut self, key: &[u8], iv: &[u8]) {
        // Set both arrays to the identity permutation
        for n in 0..Self::N {
            self.arr_p[n] = n as u8;
            self.arr_s[n] = n as u8;
        }
        // Set all variables to zero
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.f = 0;
        self.n = 0;
        // Three key scheduling round rounds
        self.ksa_round(key);
        self.ksa_round(iv);
        self.ksa_round(key);
        // Set n using the VMPC
        self.n = Self::vmpc(self.c.wrapping_add(self.d) as usize, &self.arr_s);
        for _ in 0..Self::N {
            self.next_byte();
        }
    }

    pub fn ksa_subround(
        i: &mut u8,
        v: &mut u8,
        w: &mut u8,
        vec: &[u8],
        y: usize,
        arr: &[u8; Self::N],
    ) {
        *v = arr[v.wrapping_add(*w).wrapping_add(vec[*i as usize]) as usize].wrapping_add(*i as u8);
        *i = (*i + 1) % (y as u8);
    }

    pub fn ksa_round(&mut self, input: &[u8]) {
        let y = input.len();
        let rounds = Self::N * ((y * y).div_ceil(Self::N * 6));
        let mut i = 0;
        for _ in 0..rounds {
            Self::ksa_subround(&mut i, &mut self.a, &mut self.f, input, y, &mut self.arr_p);
            Self::ksa_subround(&mut i, &mut self.b, &mut self.a, input, y, &mut self.arr_s);
            Self::ksa_subround(&mut i, &mut self.c, &mut self.b, input, y, &mut self.arr_p);
            Self::ksa_subround(&mut i, &mut self.d, &mut self.c, input, y, &mut self.arr_s);
            Self::ksa_subround(&mut i, &mut self.e, &mut self.d, input, y, &mut self.arr_p);
            Self::ksa_subround(&mut i, &mut self.f, &mut self.e, input, y, &mut self.arr_s);
            self.arr_p.swap(self.n as usize, self.b as usize);
            self.arr_s.swap(self.n as usize, self.e as usize);
            self.arr_p.swap(self.d as usize, self.f as usize);
            self.arr_s.swap(self.a as usize, self.c as usize);
            self.n = self.n.wrapping_add(1);
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        self.a = self.arr_p[self
            .a
            .wrapping_add(self.c)
            .wrapping_add(self.arr_s[self.n as usize]) as usize];
        self.b = self.arr_p[self.b.wrapping_add(self.a) as usize];
        self.c = self.arr_p[self.c.wrapping_add(self.b) as usize];
        self.d = self.arr_s[self
            .d
            .wrapping_add(self.f)
            .wrapping_add(self.arr_p[self.n as usize]) as usize];
        self.e = self.arr_s[self.e.wrapping_add(self.d) as usize];
        self.f = self.arr_s[self.f.wrapping_add(self.e) as usize];

        let out = Self::vmpc(self.c.wrapping_add(self.d) as usize, &self.arr_s);

        self.arr_p.swap(self.n as usize, self.f as usize);
        self.arr_s.swap(self.n as usize, self.a as usize);

        self.n = self.n.wrapping_add(1);

        out
    }
}

impl ClassicRng for Vmpcr {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        for i in 0..4 {
            bytes[i] = self.next_byte();
        }
        match self.big_endian {
            true => u32::from_be_bytes(bytes),
            false => u32::from_le_bytes(bytes),
        }
    }
}

#[cfg(test)]
mod vmpcr_tests {

    use super::*;

    #[test]
    fn p_array_test() {
        let mut rng = Vmpcr::default();
        rng.ksa(
            &[11, 22, 33, 144, 155, 166, 233, 244, 255],
            &[255, 250, 200, 150, 100, 50, 5, 1],
        );

        // Test vectors for P array
        assert_eq!(
            ([97, 218, 106, 125], [139, 86, 36, 126]),
            (
                rng.arr_p[0..4].try_into().unwrap(),
                rng.arr_p[252..256].try_into().unwrap()
            )
        );
    }

    #[test]
    fn s_array_test() {
        let mut rng = Vmpcr::default();
        rng.ksa(
            &[11, 22, 33, 144, 155, 166, 233, 244, 255],
            &[255, 250, 200, 150, 100, 50, 5, 1],
        );

        // Test vectors for S array
        assert_eq!(
            ([152, 143, 19, 154], [92, 25, 24, 157]),
            (
                rng.arr_s[0..4].try_into().unwrap(),
                rng.arr_s[252..256].try_into().unwrap()
            )
        );
    }

    #[test]
    fn byte_stream_test() {
        let mut rng = Vmpcr::default();
        rng.ksa(
            &[11, 22, 33, 144, 155, 166, 233, 244, 255],
            &[255, 250, 200, 150, 100, 50, 5, 1],
        );

        // Test vectors bytes of output
        let mut test_vec = Vec::with_capacity(16);
        for i in 0..1000002 {
            let n = rng.next_byte();
            if [
                0, 1, 2, 3, 254, 255, 256, 257, 1000, 1001, 10000, 10001, 100000, 100001, 1000000,
                1000001,
            ]
            .contains(&i)
            {
                test_vec.push(n)
            }
        }
        assert_eq!(
            [49_u8, 161, 79, 69, 85, 237, 96, 243, 181, 184, 136, 99, 67, 27, 253, 231],
            test_vec[..]
        )
    }
}
