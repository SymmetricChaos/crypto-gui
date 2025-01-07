// based on: https://github.com/creachadair/cityhash/blob/v0.1.1/cityhash.go

use crate::traits::StatefulHasher;
use std::ops::BitXor;

use super::helpers::{city_mur, fetch_u64, hash128_64, weak_hash_128_with_seeds, P0, P1};

pub struct CityHash128 {
    buffer: Vec<u8>,
    pub seeds: [u64; 2],
}

impl CityHash128 {
    pub fn init(seeds: [u64; 2]) -> Self {
        Self {
            seeds,
            buffer: Vec::new(),
        }
    }
    pub fn init_unseeded() -> Self {
        Self::init([P0, P1])
    }
}

impl StatefulHasher for CityHash128 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        let l = self.buffer.len();

        if l < 128 {
            return city_mur(&self.buffer, self.seeds[0], self.seeds[1]);
        } else {
            let mut x = self.seeds[0];
            let mut y = self.seeds[1];
            let mut z = (l as u64).wrapping_mul(P1);
            let mut v1 = y
                .bitxor(P1)
                .rotate_right(49)
                .wrapping_mul(P1)
                .wrapping_add(fetch_u64(&self.buffer, 0));
            let mut v2 = v1
                .rotate_right(42)
                .wrapping_mul(P1)
                .wrapping_add(fetch_u64(&self.buffer, 8));
            let mut w1 = y
                .wrapping_add(z)
                .rotate_right(35)
                .wrapping_mul(P1)
                .wrapping_add(x);
            let mut w2 = x
                .wrapping_add(fetch_u64(&self.buffer, 88))
                .rotate_right(53)
                .wrapping_mul(P1);

            let mut n = l;
            let mut offset = 0;

            while n >= 128 {
                // Same action as CityHash64
                let block = &self.buffer[offset..];
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

            let mut pos = 0;
            while pos < l {
                pos += 32;
                y = x
                    .wrapping_add(y)
                    .rotate_right(42)
                    .wrapping_mul(P0)
                    .wrapping_add(v2);
                w1 = fetch_u64(&self.buffer, l - pos + offset + 16);
                x = x.wrapping_mul(P0).wrapping_add(w1);
                z = z
                    .wrapping_add(w2)
                    .wrapping_add(fetch_u64(&self.buffer, l - pos + offset));
                w2 = w2.wrapping_add(v1);
                (v1, v2) = weak_hash_128_with_seeds(&self.buffer, v1.wrapping_add(z), v2);
                v1 = v1.wrapping_mul(P0);
            }

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

    crate::stateful_hash_helpers!();
}
