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

fn mix_seed(mut seed: u64, i: usize) -> u64 {
    seed ^= rapid_mix(seed ^ DEFAULT_SECRETS[2], DEFAULT_SECRETS[i], false);

    // Force the seed to not be all zeroes
    const HI: u64 = 0xFFFF << 48;
    const MI: u64 = 0xFFFF << 24;
    const LO: u64 = 0xFFFF;

    if (seed & HI) == 0 {
        seed |= 1u64 << 63;
    }

    if (seed & MI) == 0 {
        seed |= 1u64 << 31;
    }

    if (seed & LO) == 0 {
        seed |= 1u64;
    }

    seed
}

pub struct RapidHashV3 {
    state: [u64; 7],
    pub avalanche: bool,
    pub protected: bool,
    secrets: [u64; 7],
    buffer: Vec<u8>,
    last_read: Vec<u8>,
    long_hash: bool,
}

impl Default for RapidHashV3 {
    fn default() -> Self {
        Self {
            state: [0; 7],
            avalanche: true,
            protected: false,
            secrets: DEFAULT_SECRETS,
            buffer: Vec::with_capacity(112),
            last_read: Vec::with_capacity(112),
            long_hash: false,
        }
    }
}

impl RapidHashV3 {
    // Reference spec
    pub fn with_seed(seed: u64) -> Self {
        let seed = mix_seed(seed, 0);
        let mut secrets = [0; 7];
        secrets[0] = mix_seed(seed, 0);
        secrets[1] = mix_seed(secrets[0], 1);
        secrets[2] = mix_seed(secrets[1], 2);
        secrets[3] = mix_seed(secrets[2], 3);
        secrets[4] = mix_seed(secrets[3], 4);
        secrets[5] = mix_seed(secrets[4], 5);
        secrets[6] = mix_seed(secrets[5], 6);
        Self {
            state: [seed; 7],
            buffer: Vec::with_capacity(112),
            secrets: secrets,
            ..Default::default()
        }
    }

    // Original spec
    pub fn with_seed_simple(seed: u64) -> Self {
        let seed = seed ^ rapid_mix(seed ^ DEFAULT_SECRETS[2], DEFAULT_SECRETS[1], false);
        Self {
            state: [seed; 7],
            buffer: Vec::with_capacity(112),
            ..Default::default()
        }
    }

    pub fn avalanche(mut self, avalanche: bool) -> Self {
        self.avalanche = avalanche;
        self
    }

    pub fn protected(mut self, protected: bool) -> Self {
        self.protected = protected;
        self
    }
}

impl StatefulHasher for RapidHashV3 {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, 112, {
            self.long_hash = true;
            self.last_read = self.buffer.clone();
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
                self.last_read = buffer.to_vec();
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

            self.last_read.extend_from_slice(buffer);

            a ^= read_u64(&self.last_read, self.last_read.len() - 16) ^ (buffer.len() as u64);
            b ^= read_u64(&self.last_read, self.last_read.len() - 8);
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
        .to_be_bytes()
        .to_vec()
    }
}

// Calculated from reference crate
crate::stateful_hash_tests!(
    test0, RapidHashV3::with_seed(0x123456), b"hello world",
    "A1B8913D9926ED57";
    test1, RapidHashV3::with_seed(0x123456), b"hello",
    "41C86949D9461B4E";
    test2, RapidHashV3::with_seed(0x123456), b"he",
    "59D459F6E4A1BC44";
    test3, RapidHashV3::with_seed(0x123456), b"It is a truth universally acknowledged",
    "67F45C74C90B7124";
    test4, RapidHashV3::with_seed(0x123456), b"It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.",
    "183D019073C64BE1";
    test5, RapidHashV3::with_seed(0x123456), b"It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife. However little known the feelings or views of such a man may be on his first entering a neighbourhood, this truth is so well fixed in the minds of the surrounding families, that he is considered as the rightful property of some one or other of their daughters.",
    "7A4A4CEA4C05E144";
);
