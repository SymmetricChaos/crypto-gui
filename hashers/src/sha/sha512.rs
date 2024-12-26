use crate::traits::StatefulHasher;
use itertools::Itertools;
use std::ops::Shr;

#[rustfmt::skip] 
pub const K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

pub fn sigma_0(a: u64) -> u64 {
    (a.rotate_right(1)) ^ (a.rotate_right(8)) ^ (a.shr(7))
}

pub fn sigma_1(a: u64) -> u64 {
    (a.rotate_right(19)) ^ (a.rotate_right(61)) ^ (a.shr(6))
}

pub fn sum_0(a: u64) -> u64 {
    (a.rotate_right(28)) ^ (a.rotate_right(34)) ^ (a.rotate_right(39))
}

pub fn sum_1(a: u64) -> u64 {
    (a.rotate_right(14)) ^ (a.rotate_right(18)) ^ (a.rotate_right(41))
}

pub fn choice(a: u64, b: u64, c: u64) -> u64 {
    (a & b) ^ (!a & c)
}

pub fn majority(a: u64, b: u64, c: u64) -> u64 {
    (a & b) ^ (a & c) ^ (b & c)
}

fn compress(state: &mut [u64; 8], chunk: &[u8]) {
    // Copy variable values into working variables
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    // Array of 64 words
    let mut x = [0u64; 80];

    // Copy the first words into the array
    // Each word is 4 bytes and 16 are taken in total
    utils::byte_formatting::fill_u64s_be(&mut x[0..16], &chunk);

    // Extend the 16 words already in the array into a total of 64 words
    for i in 16..80 {
        x[i] = x[i - 16]
            .wrapping_add(sigma_0(x[i - 15]))
            .wrapping_add(x[i - 7])
            .wrapping_add(sigma_1(x[i - 2]));
    }

    for i in 0..80 {
        let temp1 = h
            .wrapping_add(sum_1(e))
            .wrapping_add(choice(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(x[i]);
        let temp2 = sum_0(a).wrapping_add(majority(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

macro_rules! sha2_512 {
    ($name: ident, $iv: expr, $output_len: literal) => {
        pub struct $name {
            state: [u64; 8],
            buffer: Vec<u8>,
            bits_taken: u128,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    state: $iv,
                    buffer: Vec::new(),
                    bits_taken: 0,
                }
            }
        }

        impl $name {
            pub fn init() -> Self {
                Self::default()
            }
        }

        impl StatefulHasher for $name {
            fn update(&mut self, bytes: &[u8]) {
                self.buffer.extend_from_slice(bytes);
                let chunks = self.buffer.chunks_exact(128);
                let rem = chunks.remainder().to_vec();
                for chunk in chunks {
                    self.bits_taken += 512;
                    compress(&mut self.state, chunk);
                }
                self.buffer = rem;
            }

            fn finalize(mut self) -> Vec<u8> {
                self.bits_taken += self.buffer.len() as u128 * 8;
                self.buffer.push(0x80);
                while (self.buffer.len() % 128) != 112 {
                    self.buffer.push(0x00)
                }
                for b in self.bits_taken.to_be_bytes() {
                    self.buffer.push(b)
                }

                // There can be multiple final blocks after padding
                for chunk in self.buffer.chunks_exact(128) {
                    compress(&mut self.state, &chunk);
                }

                let mut out = self
                    .state
                    .iter()
                    .map(|x| x.to_be_bytes())
                    .flatten()
                    .collect_vec();
                out.truncate($output_len);
                out
            }

            crate::stateful_hash_helpers!();
        }
    };
}

sha2_512!(
    Sha2_384,
    [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ],
    48
);

sha2_512!(
    Sha2_512,
    [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ],
    64
);

sha2_512!(
    Sha2_512_224,
    [
        0x8c3d37c819544da2,
        0x73e1996689dcd4d6,
        0x1dfab7ae32ff9c82,
        0x679dd514582f9fcf,
        0x0f6d2b697bd44da8,
        0x77e36f7304c48942,
        0x3f9d85a86a1d36c8,
        0x1112e6ad91d692a1,
    ],
    28
);

sha2_512!(
    Sha2_512_256,
    [
        0x22312194fc2bf72c,
        0x9f555fa3c84c64c2,
        0x2393b86b6f53b151,
        0x963877195940eabd,
        0x96283ee2a88effe3,
        0xbe5e1e2553863992,
        0x2b0199fc2c85b8aa,
        0x0eb72ddc81c52ca2,
    ],
    32
);
