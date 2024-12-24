use crate::traits::StatefulHasher;
use itertools::Itertools;
use utils::byte_formatting::fill_u64s_be;

// Constants for compression function, beginning digits of pi
const C: [u64; 16] = [
    0x243f6a8885a308d3,
    0x13198a2e03707344,
    0xa4093822299f31d0,
    0x082efa98ec4e6c89,
    0x452821e638d01377,
    0xbe5466cf34e90c6c,
    0xc0ac29b7c97c50dd,
    0x3f84d5b5b5470917,
    0x9216d5d98979fb1b,
    0xd1310ba698dfb5ac,
    0x2ffd72dbd01adfb7,
    0xb8e1afed6a267e96,
    0xba7c9045f12c7f99,
    0x24a19947b3916cf7,
    0x0801f2e2858efc16,
    0x636920d871574e69,
];

const IV_384: [u64; 8] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

const IV_512: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

// https://decred.org/research/aumasson2010.pdf
pub fn compress(state: &mut [u64; 8], chunk: &[u64; 16], counter: u128, salt: &[u64; 4]) {
    // create a working vector starting with the current state and then following it with C xored with the salt, then C xored with the counter
    let mut work = [0_u64; 16];
    for i in 0..8 {
        work[i] = state[i];
    }
    for i in 0..4 {
        work[i + 8] = C[i] ^ salt[i]
    }
    work[12] = C[4] ^ (counter as u64); // Lower bits
    work[13] = C[5] ^ (counter as u64);
    work[14] = C[6] ^ (counter >> 64) as u64; // Upper bits
    work[15] = C[7] ^ (counter >> 64) as u64;

    crate::blake_compress!(&mut work, chunk, [32, 25, 16, 11], C, 16);

    for i in 0..8 {
        state[i] ^= salt[i % 4] ^ work[i] ^ work[i + 8];
    }
}

fn create_chunk(bytes: &[u8]) -> [u64; 16] {
    let mut k = [0u64; 16];
    fill_u64s_be(&mut k, &bytes);
    k
}

#[derive(Debug, Clone)]
pub struct Blake384 {
    state: [u64; 8],
    salt: [u64; 4],
    bits_taken: u128,
    buffer: Vec<u8>,
}

impl Blake384 {
    pub fn init() -> Self {
        let hasher = Self {
            state: IV_384,
            salt: [0; 4],
            bits_taken: 0,
            buffer: Vec::new(),
        };

        hasher
    }

    pub fn init_mac(salt: [u64; 4]) -> Self {
        let hasher = Self {
            state: IV_384,
            salt,
            bits_taken: 0,
            buffer: Vec::new(),
        };

        hasher
    }
}

impl StatefulHasher for Blake384 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);

        let chunks = self.buffer.chunks_exact(128);
        let last = chunks.remainder().to_vec();
        for chunk in chunks {
            self.bits_taken += 1024;
            let c = create_chunk(chunk);
            compress(&mut self.state, &c, self.bits_taken, &self.salt);
        }
        self.buffer = last;
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        let total_bits = self.bits_taken + (self.buffer.len() * 8) as u128;

        // Push a 1 bit followed by zeroes until and then a 1 bit again to reach 896 bits (112 bytes)
        if self.buffer.len() == 111 {
            self.buffer.push(0x81);
        } else {
            self.buffer.push(0x80);
            while self.buffer.len() % 128 != 111 {
                self.buffer.push(0x00);
            }
            self.buffer.push(0x00); // different from BLAKE512
        }
        // Then push the total input length onto the buffer at the last 128 bits
        for b in total_bits.to_be_bytes() {
            self.buffer.push(b);
        }

        // There could be either one or two blocks to finalize
        if self.buffer.len() > 128 {
            let c = create_chunk(&self.buffer[0..128]);
            compress(&mut self.state, &c, total_bits, &self.salt);
            let c = create_chunk(&self.buffer[128..256]);
            compress(&mut self.state, &c, 0, &self.salt); // in the two final block case the last block has its counter set to zero
        } else {
            let c = create_chunk(&self.buffer);
            compress(&mut self.state, &c, total_bits, &self.salt);
        }

        self.state
            .iter()
            .take(6)
            .map(|x| x.to_be_bytes())
            .flatten()
            .collect_vec()
    }

    fn hash(mut self, bytes: &[u8]) -> Vec<u8> {
        self.update(bytes);
        self.finalize()
    }
}

#[derive(Debug, Clone)]
pub struct Blake512 {
    state: [u64; 8],
    salt: [u64; 4],
    bits_taken: u128,
    buffer: Vec<u8>,
}

impl Blake512 {
    pub fn init() -> Self {
        let hasher = Self {
            state: IV_512,
            salt: [0; 4],
            bits_taken: 0,
            buffer: Vec::new(),
        };

        hasher
    }

    pub fn init_mac(salt: [u64; 4]) -> Self {
        let hasher = Self {
            state: IV_512,
            salt,
            bits_taken: 0,
            buffer: Vec::new(),
        };

        hasher
    }
}

impl StatefulHasher for Blake512 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);

        let chunks = self.buffer.chunks_exact(128);
        let last = chunks.remainder().to_vec();
        for chunk in chunks {
            self.bits_taken += 1024;
            let c = create_chunk(chunk);
            compress(&mut self.state, &c, self.bits_taken, &self.salt);
        }
        self.buffer = last;
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        let total_bits = self.bits_taken + (self.buffer.len() * 8) as u128;

        // Push a 1 bit followed by zeroes until and then a 1 bit again to reach 896 bits (112 bytes)
        if self.buffer.len() == 111 {
            self.buffer.push(0x81);
        } else {
            self.buffer.push(0x80);
            while self.buffer.len() % 128 != 111 {
                self.buffer.push(0x00);
            }
            self.buffer.push(0x01);
        }
        // Then push the total input length onto the buffer at the last 128 bits
        for b in total_bits.to_be_bytes() {
            self.buffer.push(b);
        }

        // There could be either one or two blocks to finalize
        if self.buffer.len() > 128 {
            let c = create_chunk(&self.buffer[0..128]);
            compress(&mut self.state, &c, total_bits, &self.salt);
            let c = create_chunk(&self.buffer[128..256]);
            compress(&mut self.state, &c, 0, &self.salt); // in the two final block case the last block has its counter set to zero
        } else {
            let c = create_chunk(&self.buffer);
            compress(&mut self.state, &c, total_bits, &self.salt);
        }

        self.state
            .iter()
            .map(|x| x.to_be_bytes())
            .flatten()
            .collect_vec()
    }

    fn hash(mut self, bytes: &[u8]) -> Vec<u8> {
        self.update(bytes);
        self.finalize()
    }
}

crate::stateful_hash_tests!(
    test_512_one_byte, Blake512::init(),
    &[0x00],
    "97961587f6d970faba6d2478045de6d1fabd09b61ae50932054d52bc29d31be4ff9102b9f69e2bbdb83be13d4b9c06091e5fa0b48bd081b634058be0ec49beb3";

    test_512_pangram, Blake512::init(),
    b"The quick brown fox jumps over the lazy dog",
    "1f7e26f63b6ad25a0896fd978fd050a1766391d2fd0471a77afb975e5034b7ad2d9ccf8dfb47abbbe656e1b82fbc634ba42ce186e8dc5e1ce09a885d41f43451";

    test_384_letters, Blake384::init(),
    b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "3b21598562da076378b2b8794e919172502d8a6661a503a6b846457376ce2ba546f4d4a7df2c4d8a875a89b0b4647e10";

    test_384_pangram, Blake384::init(),
    b"The quick brown fox jumps over the lazy dog",
    "67c9e8ef665d11b5b57a1d99c96adffb3034d8768c0827d1c6e60b54871e8673651767a2c6c43d0ba2a9bb2500227406";
);
