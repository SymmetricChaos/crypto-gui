use crate::traits::StatefulHasher;

pub fn wy_mum(a: u64, b: u64) -> (u64, u64) {
    let r = (a as u128).wrapping_mul(b as u128);
    ((r as u64), (r >> 64) as u64)
}

pub fn wy_mix(a: u64, b: u64) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);
    (r as u64) ^ (r >> 64) as u64
}

pub fn finalize(a: u64, b: u64, rem: u64, seed: u64, secrets: &[u64; 4]) -> u64 {
    wy_mix(secrets[1] ^ rem, wy_mix(a ^ secrets[1], b ^ seed))
}

pub fn read_u64(slice: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(slice[offset..offset + 8].try_into().unwrap())
}

pub fn read_u32(slice: &[u8]) -> u64 {
    u32::from_le_bytes(slice.try_into().unwrap()) as u64
}

fn read_u24(bytes: &[u8]) -> u64 {
    u64::from(bytes[0]) << 16
        | u64::from(bytes[bytes.len() >> 1]) << 8
        | u64::from(bytes[bytes.len() - 1])
}

fn compress(bytes: &[u8], state: &mut [u64; 3], secrets: &[u64; 4]) {
    state[0] = wy_mix(
        read_u64(bytes, 0) ^ secrets[0],
        read_u64(bytes, 8) ^ state[0],
    );
    state[1] = wy_mix(
        read_u64(bytes, 16) ^ secrets[1],
        read_u64(bytes, 24) ^ state[1],
    );
    state[2] = wy_mix(
        read_u64(bytes, 32) ^ secrets[2],
        read_u64(bytes, 40) ^ state[2],
    );
}

const P0: u64 = 0xa076_1d64_78bd_642f;
const P1: u64 = 0xe703_7ed1_a0b4_28db;

const C: [u8; 70] = [
    15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86, 89, 90,
    92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149, 150, 153,
    154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201, 202, 204, 209,
    210, 212, 216, 225, 226, 228, 232, 240,
];

// Weyl sequence transition function with wy_mix as the output function
fn wy_rand(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(P0);
    wy_mix(*seed, *seed ^ P1)
}

pub struct Wyhash {
    buffer: Vec<u8>,
    last_read: Vec<u8>,
    state: [u64; 3],
    secrets: [u64; 4],
    long_hash: bool,
}

impl Default for Wyhash {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(48),
            last_read: Vec::with_capacity(48),
            state: [0; 3],
            secrets: [0; 4],
            long_hash: false,
        }
    }
}

impl Wyhash {
    pub fn init() -> Self {
        Self::with_seed(0)
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut secrets = [0; 4];
        let mut tseed = seed;

        for i in 0..4 {
            loop {
                secrets[i] = 0;
                for j in (0..64).step_by(8) {
                    secrets[i] |= u64::from((C[wy_rand(&mut tseed) as usize % 70]) << j)
                }
                // ???
                if secrets[i] % 2 == 0 {
                    continue;
                }
                // Ensure the secrets have sufficient difference from each other?
                if (0..i)
                    .find(|n| (secrets[*n] ^ secrets[i]).count_ones() != 32)
                    .is_none()
                {
                    break;
                }
            }
        }

        Self {
            state: [seed; 3],
            secrets: secrets,
            ..Default::default()
        }
    }
}

impl StatefulHasher for Wyhash {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend(bytes);

        while self.buffer.len() > 48 {
            self.long_hash = true;
            self.last_read = self.buffer[..48].to_vec();
            compress(&self.buffer[..48], &mut self.state, &self.secrets);
            self.buffer = self.buffer[48..].to_vec();
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        let mut a: u64 = 0;
        let mut b: u64 = 0;
        let mut rem = self.buffer.len() as u64;

        if !self.long_hash && self.buffer.len() <= 16 {
            let len = self.buffer.len();
            if len >= 4 {
                a = read_u32(&self.buffer) << 32 | read_u32(&self.buffer[((len >> 3) << 2)..]);
                b = read_u32(&self.buffer[(len - 4)..]) << 32
                    | read_u32(&self.buffer[(len - 4 - ((len >> 3) << 2))..]);
            } else if len > 0 {
                a = read_u24(&self.buffer);
            }
        } else {
            // TODO: COMPRESS
            self.state[0] ^= self.state[1];
            self.state[0] ^= self.state[2];
            while self.buffer.len() > 16 {
                self.state[0] = wy_mix(
                    read_u64(&self.buffer, 0) ^ self.secrets[1],
                    read_u64(&self.buffer, 8) ^ self.state[0],
                );
                let (_, split) = self.buffer.split_at(16);
                self.buffer = split.to_vec();
            }

            a = read_u64(&self.last_read[self.last_read.len() - 16..], 0);
            b = read_u64(&self.last_read[self.last_read.len() - 8..], 0);
        }
        finalize(a, b, rem, self.state[0], &self.secrets)
            .to_be_bytes()
            .to_vec()
    }
}
