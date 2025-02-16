use std::num::Wrapping;

use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

macro_rules! mix(
    ($a:expr) => (
    {
        $a[0] ^= $a[1] << 11; $a[3] += $a[0]; $a[1] += $a[2];
        $a[1] ^= $a[2] >> 2;  $a[4] += $a[1]; $a[2] += $a[3];
        $a[2] ^= $a[3] << 8;  $a[5] += $a[2]; $a[3] += $a[4];
        $a[3] ^= $a[4] >> 16; $a[6] += $a[3]; $a[4] += $a[5];
        $a[4] ^= $a[5] << 10; $a[7] += $a[4]; $a[5] += $a[6];
        $a[5] ^= $a[6] >> 4;  $a[0] += $a[5]; $a[6] += $a[7];
        $a[6] ^= $a[7] << 8;  $a[1] += $a[6]; $a[7] += $a[0];
        $a[7] ^= $a[0] >> 9;  $a[2] += $a[7]; $a[0] += $a[1];
    } );
);

#[derive(Clone)]
pub struct Isaac {
    array: [Wrapping<u32>; 256],
    a: Wrapping<u32>,
    b: Wrapping<u32>,
    c: Wrapping<u32>,
    rand_rsl: [Wrapping<u32>; 256], // effectively the output state (I do not know why it is called this)
    ctr: usize,                     // point to the current position in rand_rsl
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Isaac {
    fn default() -> Self {
        Self {
            array: [Wrapping(0); 256],
            a: Wrapping(0),
            b: Wrapping(0),
            c: Wrapping(0),
            rand_rsl: [Wrapping(0); 256],
            ctr: 0,
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl Isaac {
    fn isaac(&mut self) {
        self.c += Wrapping(1);
        self.b += self.c;
        for i in 0..256 {
            let x = self.array[i];
            match i % 4 {
                0 => self.a ^= self.a << 13,
                1 => self.a ^= self.a >> 6,
                2 => self.a ^= self.a << 2,
                3 => self.a ^= self.a >> 16,
                _ => unreachable!(),
            }
            self.a += self.array[(i + 128) % 256];
            self.array[i] = self.array[(x.0 as usize >> 2) % 256] + self.a + self.b;
            let y = self.array[i].0 as usize;
            self.b = self.array[(y >> 10) % 256] + x;
            self.rand_rsl[i] = self.b;
        }
        self.ctr = 0;
    }

    fn init(&mut self, extra_pass: bool) {
        self.a = Wrapping(0);
        self.b = Wrapping(0);
        self.c = Wrapping(0);

        // Golden Ratio
        let mut arr = [Wrapping(0x9e37_79b9u32); 8];

        for _ in 0..4 {
            mix!(arr)
        }

        for i in (0..256).step_by(8) {
            if extra_pass {
                for j in 0..8 {
                    arr[j] += self.rand_rsl[i + j];
                }
            }
            mix!(arr);
            for j in 0..8 {
                self.array[i + j] = arr[j]
            }
        }

        if extra_pass {
            for i in (0..256).step_by(8) {
                for j in 0..8 {
                    arr[j] += self.array[i + j];
                }
                mix!(arr);
                for j in 0..8 {
                    self.array[i + j] = arr[j]
                }
            }
        }

        self.isaac();
    }

    pub fn seed(&mut self, seed: &[u8], extra_pass: bool) {
        assert!(seed.len() <= 256, "seed cannot have more than 256 bytes");
        self.array = [Wrapping(0); 256];
        self.rand_rsl = [Wrapping(0); 256];
        for i in 0..seed.len() {
            self.rand_rsl[i] = Wrapping(u32::from(seed[i]));
        }
        self.init(extra_pass);
    }

    pub fn init_with_seed(seed: &[u8], extra_pass: bool) -> Self {
        let mut rng = Self::default();
        rng.seed(seed, extra_pass);
        rng
    }

    fn next_u32(&mut self) -> u32 {
        if self.ctr > 255 {
            self.isaac();
        }
        let n = self.rand_rsl[self.ctr].0;
        self.ctr += 1;
        n
    }

    // Used by Rosetta code but not recommended as it severely reduces security
    fn next_ascii(&mut self) -> u8 {
        (self.next_u32() % 95 + 32) as u8
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut rng = self.clone();
        bytes
            .into_iter()
            .map(|b| (rng.next_u32() as u8 ^ b))
            .collect_vec()
    }

    // Used by Rosetta code but not recommended as it severely reduces security
    pub fn encrypt_bytes_ascii(&self, bytes: &[u8]) -> Vec<u8> {
        let mut rng = self.clone();
        bytes
            .into_iter()
            .map(|b| (rng.next_ascii() ^ b))
            .collect_vec()
    }
}

crate::impl_cipher_for_stream_cipher!(Isaac);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rosetta_test() {
        let msg = "a Top Secret secret";
        let key = b"this is my secret key";
        let mut cipher = Isaac::init_with_seed(key, true);
        cipher.input_format = ByteFormat::Utf8;
        assert_eq!(
            "1c0636190b1260233b35125f1e1d0e2f4c5422",
            ByteFormat::Hex.byte_slice_to_text(cipher.encrypt_bytes_ascii(msg.as_bytes()))
        );
    }
}
