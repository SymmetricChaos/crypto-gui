// based on
// https://github.com/hoxxep/rapidhash/tree/master/rapidhash

// A "proper" implementation of any of these should be as optimized as possible.
// The reference above, in fact, is designed to compile to highly specific
// variations to reduce size and increase speed, especially for fixed size
// inputs.
// However that is a non-goal for this project and they are presented to be as
// easy to understand as possible.

use crate::traits::StatefulHasher;

const DEFAULT_SECRETS: [u64; 7] = [
    0x2d358dccaa6c78a5,
    0x8bb84b93962eacc9,
    0x4b33a62ed433d4a3,
    0x4d5a2da51de1aa47,
    0xa0761d6478bd642f,
    0xe7037ed1a0b428db,
    0x90ed1765281c388c,
];

// Wide multiply then return the lower half and upper half
fn rapid_mum(a: u64, b: u64, protected: bool) -> (u64, u64) {
    let r = (a as u128).wrapping_mul(b as u128);

    if protected {
        ((a ^ r as u64), (b ^ (r >> 64) as u64))
    } else {
        ((r as u64), (r >> 64) as u64)
    }
}

// Wide multiply then XOR the lower and upper halves together
// A folded version of rapid_mum()
fn rapid_mix(a: u64, b: u64, protected: bool) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);

    if protected {
        (a ^ r as u64) ^ (b ^ (r >> 64) as u64)
    } else {
        (r as u64) ^ (r >> 64) as u64
    }
}

fn finalize(
    mut a: u64,
    mut b: u64,
    rem: u64,
    seed: u64,
    avalanche: bool,
    protected: bool,
    secrets: &[u64; 7],
) -> u64 {
    a ^= secrets[1];
    b ^= seed;

    (a, b) = rapid_mum(a, b, protected);

    if avalanche {
        rapid_mix(a ^ 0xaaaaaaaaaaaaaaaa, b ^ secrets[1] ^ rem, protected)
    } else {
        a ^ b
    }
}

fn short_hash(bytes: &[u8], mut a: u64, mut b: u64, mut s0: u64, len: usize) -> (u64, u64, u64) {
    assert!(bytes.len() <= 16); // guard against accidental misue
    if bytes.len() >= 4 {
        s0 ^= len as u64;
        if bytes.len() >= 8 {
            // XOR in the first full word and the last full word, these may overlap
            a ^= read_u64(bytes, 0);
            b ^= read_u64(bytes, len - 8);
        } else {
            // XOR in the first full word and the last full word, these may overlap
            a ^= read_u32(bytes, 0) as u64;
            b ^= read_u32(bytes, len - 4) as u64;
        }
    // Three or fewer bytes
    } else if !bytes.is_empty() {
        a ^= ((bytes[0] as u64) << 45) | bytes[len - 1] as u64;
        b ^= bytes[len >> 1] as u64;
    }
    (a, b, s0)
}

// Little endian
fn read_u64(slice: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(slice[offset..offset + 8].try_into().unwrap())
}

// Little endian
fn read_u32(slice: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(slice[offset..offset + 4].try_into().unwrap())
}

fn compress(bytes: &[u8], state: &mut [u64; 7], secrets: &[u64; 7], protected: bool) {
    state[0] = rapid_mix(
        read_u64(bytes, 0) ^ secrets[0],
        read_u64(bytes, 8) ^ state[0],
        protected,
    );
    state[1] = rapid_mix(
        read_u64(bytes, 16) ^ secrets[1],
        read_u64(bytes, 24) ^ state[1],
        protected,
    );
    state[2] = rapid_mix(
        read_u64(bytes, 32) ^ secrets[2],
        read_u64(bytes, 40) ^ state[2],
        protected,
    );
    state[3] = rapid_mix(
        read_u64(bytes, 48) ^ secrets[3],
        read_u64(bytes, 56) ^ state[3],
        protected,
    );
    state[4] = rapid_mix(
        read_u64(bytes, 64) ^ secrets[4],
        read_u64(bytes, 72) ^ state[4],
        protected,
    );
    state[5] = rapid_mix(
        read_u64(bytes, 80) ^ secrets[5],
        read_u64(bytes, 88) ^ state[5],
        protected,
    );
    state[6] = rapid_mix(
        read_u64(bytes, 96) ^ secrets[6],
        read_u64(bytes, 104) ^ state[6],
        protected,
    );
}

pub struct RapidHash {
    state: [u64; 7],
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
    pub buffer: Vec<u8>,
    long_hash: bool,
}

impl Default for RapidHash {
    fn default() -> Self {
        Self {
            state: [0; 7],
            avalanche: true,
            protected: true,
            secrets: DEFAULT_SECRETS,
            buffer: Vec::new(),
            long_hash: false,
        }
    }
}

impl StatefulHasher for RapidHash {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, 112, {
            self.long_hash = true;
            compress(&self.buffer, &mut self.state, &self.secrets, self.protected)
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        let mut buffer = &self.buffer[..];
        let mut a = 0;
        let mut b = 0;
        let rem = buffer.len() as u64;

        if !self.long_hash && buffer.len() <= 16 {
            (a, b, self.state[0]) = short_hash(&buffer, a, b, self.state[0], buffer.len());
        } else {
            while buffer.len() > 112 {
                compress(&buffer, &mut self.state, &self.secrets, self.protected);
                let (_, split) = buffer.split_at(112);
                buffer = split;
            }
            self.state[0] ^= self.state[1];
            self.state[2] ^= self.state[3];
            self.state[4] ^= self.state[5];
            self.state[0] ^= self.state[6];
            self.state[2] ^= self.state[4];
            self.state[0] ^= self.state[2];

            a ^= read_u64(buffer, buffer.len() - 16) ^ (buffer.len() as u64);
            b ^= read_u64(buffer, buffer.len() - 8);

            if buffer.len() > 16 {
                self.state[0] = rapid_mix(
                    read_u64(&buffer, 0) ^ self.secrets[2],
                    read_u64(&buffer, 8) ^ self.state[0],
                    self.protected,
                );
                if buffer.len() > 32 {
                    self.state[0] = rapid_mix(
                        read_u64(&buffer, 16) ^ self.secrets[2],
                        read_u64(&buffer, 24) ^ self.state[0],
                        self.protected,
                    );
                    if buffer.len() > 48 {
                        self.state[0] = rapid_mix(
                            read_u64(&buffer, 32) ^ self.secrets[1],
                            read_u64(&buffer, 40) ^ self.state[0],
                            self.protected,
                        );
                        if buffer.len() > 64 {
                            self.state[0] = rapid_mix(
                                read_u64(&buffer, 48) ^ self.secrets[1],
                                read_u64(&buffer, 56) ^ self.state[0],
                                self.protected,
                            );
                            if buffer.len() > 80 {
                                self.state[0] = rapid_mix(
                                    read_u64(&buffer, 64) ^ self.secrets[2],
                                    read_u64(&buffer, 72) ^ self.state[0],
                                    self.protected,
                                );
                                if buffer.len() > 96 {
                                    self.state[0] = rapid_mix(
                                        read_u64(&buffer, 80) ^ self.secrets[1],
                                        read_u64(&buffer, 88) ^ self.state[0],
                                        self.protected,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        finalize(
            a,
            b,
            rem,
            self.state[0],
            self.avalanche,
            self.protected,
            &self.secrets,
        )
        .to_le_bytes()
        .to_vec()
    }
}
