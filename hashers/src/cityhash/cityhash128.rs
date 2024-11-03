// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::ClassicHasher;
use std::ops::BitXor;
use utils::byte_formatting::ByteFormat;

use super::helpers::{city_mur, fetch_u64, hash128_64, weak_hash_128_with_seeds, P0, P1};

pub struct CityHash128 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub seeds: [u64; 2],
}

impl Default for CityHash128 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            seeds: [P0, P1],
        }
    }
}

impl CityHash128 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn with_seeds(mut self, seed0: u64, seed1: u64) -> Self {
        self.seeds = [seed0, seed1];
        self
    }
}

impl ClassicHasher for CityHash128 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let l = bytes.len();

        if l < 128 {
            return city_mur(bytes, self.seeds[0], self.seeds[1]);
        } else {
            let mut x = self.seeds[0];
            let mut y = self.seeds[1];
            let mut z = (l as u64).wrapping_mul(P1);
            let mut v1 = y
                .bitxor(P1)
                .rotate_right(49)
                .wrapping_mul(P1)
                .wrapping_add(fetch_u64(bytes, 0));
            let mut v2 = v1
                .rotate_right(42)
                .wrapping_mul(P1)
                .wrapping_add(fetch_u64(bytes, 8));
            let mut w1 = y
                .wrapping_add(z)
                .rotate_right(35)
                .wrapping_mul(P1)
                .wrapping_add(x);
            let mut w2 = x
                .wrapping_add(fetch_u64(bytes, 88))
                .rotate_right(53)
                .wrapping_mul(P1);

            let mut n = l;
            let mut offset = 0;

            while n >= 128 {
                // Same action as CityHash64
                let block = &bytes[offset..];
                x = x
                    .wrapping_add(y)
                    .wrapping_add(v1)
                    .wrapping_add(fetch_u64(block, 8))
                    .rotate_right(37)
                    .wrapping_mul(P1);
                y = y
                    .wrapping_add(v2)
                    .wrapping_add(fetch_u64(block, 48))
                    .rotate_right(42)
                    .wrapping_mul(P1);

                x ^= w2;

                y = y.wrapping_add(v1).wrapping_add(fetch_u64(block, 40));

                z = z.wrapping_add(w1).rotate_right(33).wrapping_mul(P1);

                (v1, v2) = weak_hash_128_with_seeds(block, v2.wrapping_mul(P1), x.wrapping_add(w1));
                (w1, w2) = weak_hash_128_with_seeds(
                    &block[32..],
                    z.wrapping_add(w2),
                    y.wrapping_add(fetch_u64(block, 16)),
                );

                (z, x) = (x, z);
                n -= 64;
                offset += 64;
            }

            x = v1.wrapping_add(z).rotate_right(49).wrapping_add(P0);
            y = y.wrapping_add(P0).wrapping_add(w2.rotate_right(37));
            z = z.wrapping_add(P0).wrapping_add(w1.rotate_right(27));
            w1 = w1.wrapping_add(9);
            v1 = v1.wrapping_mul(P0);

            // let mut pos = 0;
            // while pos < l {
            //     pos += 32;
            //     y = x
            //         .wrapping_add(y)
            //         .rotate_right(42)
            //         .wrapping_mul(P0)
            //         .wrapping_add(v2);
            //     w1 = fetch_u64(bytes, l - pos + offset + 16);
            //     x = x.wrapping_mul(P0).wrapping_add(w1);
            //     z = z
            //         .wrapping_add(w2)
            //         .wrapping_add(fetch_u64(bytes, l - pos + offset));
            //     w2 = w2.wrapping_add(v1);
            //     (v1, v2) = weak_hash_128_with_seeds(bytes, v1.wrapping_add(z), v2);
            //     v1 = v1.wrapping_mul(P0);
            // }

            x = hash128_64(x, v1);
            y = hash128_64(y.wrapping_add(z), w1);
            [
                hash128_64(x.wrapping_add(v2), w2).wrapping_add(y),
                hash128_64(x.wrapping_add(w2), y.wrapping_add(v2)),
            ]
            .into_iter()
            .flat_map(|w| w.to_be_bytes())
            .collect()
        }
    }

    crate::hash_bytes_from_string! {}
}
