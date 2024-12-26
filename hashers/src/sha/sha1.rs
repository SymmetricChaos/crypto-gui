use crate::traits::StatefulHasher;
use utils::byte_formatting::fill_u32s_be;

fn compress(state: &mut [u32; 5], chunk: &[u8]) {
    let mut v = state.clone();
    // Extract 16 words from the block and make them the first 16 values of the array
    let mut x = [0u32; 80];
    fill_u32s_be(&mut x[0..16], &chunk);

    // Extend the 16 words to 80 words
    for i in 16..80 {
        x[i] = (x[i - 3] ^ x[i - 8] ^ x[i - 14] ^ x[i - 16]).rotate_left(1)
    }

    // Apply 80 rounds of mixing
    for i in 0..80 {
        let mut f = 0;
        let mut g = 0;
        // Round functions and round constant are changed every 20 rounds
        if i < 20 {
            f = (v[1] & v[2]) | (!v[1] & v[3]);
            g = 0x5a827999;
        }
        if i >= 20 && i < 40 {
            f = v[1] ^ v[2] ^ v[3];
            g = 0x6ed9eba1;
        }
        if i >= 40 && i < 60 {
            f = (v[1] & v[2]) | (v[1] & v[3]) | (v[2] & v[3]);
            g = 0x8f1bbcdc;
        }
        if i >= 60 {
            f = v[1] ^ v[2] ^ v[3];
            g = 0xca62c1d6;
        }

        let t = v[0]
            .rotate_left(5)
            .wrapping_add(f)
            .wrapping_add(v[4])
            .wrapping_add(g)
            .wrapping_add(x[i]); // Each round a new word from the array x is added here
        v[4] = v[3];
        v[3] = v[2];
        v[2] = v[1].rotate_left(30);
        v[1] = v[0];
        v[0] = t;
    }

    for i in 0..5 {
        state[i] = state[i].wrapping_add(v[i]);
    }
}

#[derive(Debug, Clone)]
pub struct Sha1 {
    state: [u32; 5],
    buffer: Vec<u8>,
    bits_taken: u64,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            buffer: Vec::new(),
            bits_taken: 0,
        }
    }
}

impl StatefulHasher for Sha1 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(64);
        let rem = chunks.remainder().to_vec();
        for chunk in chunks {
            self.bits_taken += 512;
            compress(&mut self.state, chunk);
        }
        self.buffer = rem;
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        self.bits_taken += self.buffer.len() as u64 * 8;
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0)
        }
        for b in self.bits_taken.to_be_bytes() {
            self.buffer.push(b)
        }

        // There can be multiple final blocks after padding
        for chunk in self.buffer.chunks_exact(64) {
            compress(&mut self.state, &chunk);
        }

        let mut out = Vec::with_capacity(20);
        for word in self.state {
            out.extend(word.to_be_bytes())
        }
        out
    }

    crate::stateful_hash_helpers!();
}

impl Sha1 {
    pub fn init() -> Self {
        Self::default()
    }
}

crate::stateful_hash_tests!(
    empty,
    Sha1::init(),
    b"",
    "da39a3ee5e6b4b0d3255bfef95601890afd80709";

    abc,
    Sha1::init(),
    b"abc",
    "a9993e364706816aba3e25717850c26c9cd0d89d";

    test3,
    Sha1::init(),
    b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "84983e441c3bd26ebaae4aa1f95129e5e54670f1";

    test4,
    Sha1::init(),
    b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "a49b2446a02c645bf419f995b67091253a04a259"
);
