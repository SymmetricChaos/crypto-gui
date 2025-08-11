// based on
// https://github.com/hoxxep/rapidhash/tree/master/rapidhash

// A "proper" implementation of any of these should be as optimized as possible.
// The reference above, in fact, is designed to compile to highly specific
// variations to reduce size and increase speed, especially for fixed size
// inputs.
// However that is a non-goal for this project and they are presented to be as
// easy to understand as possible.

// TODO: Can this be implemented as a stateful hasher? I don't think so.

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

// Little endian
fn read_u64(slice: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(slice[offset..offset + 8].try_into().unwrap())
}

// Little endian
fn read_u32(slice: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(slice[offset..offset + 4].try_into().unwrap())
}

fn rapidhash_core_cold(
    bytes: &[u8],
    seed: u64,
    avalanche: bool,
    protected: bool,
    secrets: &[u64; 7],
) -> u64 {
    let mut a = 0;
    let mut b = 0;

    let mut slice = bytes;
    let mut s0 = seed;

    if bytes.len() > 112 {
        let mut s1 = seed;
        let mut s2 = seed;
        let mut s3 = seed;
        let mut s4 = seed;
        let mut s5 = seed;
        let mut s6 = seed;
        while bytes.len() > 112 {
            s0 = rapid_mix(
                read_u64(slice, 0) ^ secrets[0],
                read_u64(slice, 8) ^ s0,
                protected,
            );
            s1 = rapid_mix(
                read_u64(slice, 16) ^ secrets[1],
                read_u64(slice, 24) ^ s1,
                protected,
            );
            s2 = rapid_mix(
                read_u64(slice, 32) ^ secrets[2],
                read_u64(slice, 40) ^ s2,
                protected,
            );
            s3 = rapid_mix(
                read_u64(slice, 48) ^ secrets[3],
                read_u64(slice, 56) ^ s3,
                protected,
            );
            s4 = rapid_mix(
                read_u64(slice, 64) ^ secrets[4],
                read_u64(slice, 72) ^ s4,
                protected,
            );
            s5 = rapid_mix(
                read_u64(slice, 80) ^ secrets[5],
                read_u64(slice, 88) ^ s5,
                protected,
            );
            s6 = rapid_mix(
                read_u64(slice, 96) ^ secrets[6],
                read_u64(slice, 104) ^ s6,
                protected,
            );
            let (_, split) = slice.split_at(112);
            slice = split;
        }

        s0 ^= s1;
        s2 ^= s3;
        s4 ^= s5;
        s0 ^= s6;
        s2 ^= s4;
        s0 ^= s2;
    }

    if slice.len() > 16 {
        s0 = rapid_mix(
            read_u64(slice, 0) ^ secrets[2],
            read_u64(slice, 8) ^ s0,
            protected,
        );
        if slice.len() > 32 {
            s0 = rapid_mix(
                read_u64(slice, 16) ^ secrets[2],
                read_u64(slice, 24) ^ s0,
                protected,
            );
            if slice.len() > 48 {
                s0 = rapid_mix(
                    read_u64(slice, 32) ^ secrets[1],
                    read_u64(slice, 40) ^ s0,
                    protected,
                );
                if slice.len() > 64 {
                    s0 = rapid_mix(
                        read_u64(slice, 48) ^ secrets[1],
                        read_u64(slice, 56) ^ s0,
                        protected,
                    );
                    if slice.len() > 80 {
                        s0 = rapid_mix(
                            read_u64(slice, 64) ^ secrets[2],
                            read_u64(slice, 72) ^ s0,
                            protected,
                        );
                        if slice.len() > 96 {
                            s0 = rapid_mix(
                                read_u64(slice, 80) ^ secrets[1],
                                read_u64(slice, 88) ^ s0,
                                protected,
                            );
                        }
                    }
                }
            }
        }
    }

    a ^= read_u64(bytes, bytes.len() - 16) ^ (slice.len() as u64);
    b ^= read_u64(bytes, bytes.len() - 8);

    finalize(a, b, slice.len() as u64, s0, avalanche, protected, &secrets)
}

pub struct RapidHash {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl Default for RapidHash {
    fn default() -> Self {
        Self {
            seed: 0,
            avalanche: true,
            protected: true,
            secrets: DEFAULT_SECRETS,
        }
    }
}

impl RapidHash {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        let len = bytes.len();
        let mut s0 = self.seed;
        let mut a = 0;
        let mut b = 0;
        let rem = len as u64;

        if bytes.len() <= 16 {
            if bytes.len() >= 4 {
                s0 ^= rem;
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
        } else {
            return rapidhash_core_cold(bytes, s0, self.avalanche, self.protected, &self.secrets);
        }

        finalize(a, b, rem, s0, self.avalanche, self.protected, &self.secrets)
    }
}

pub struct RapidHashMicro {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl Default for RapidHashMicro {
    fn default() -> Self {
        Self {
            seed: 0,
            avalanche: true,
            protected: true,
            secrets: DEFAULT_SECRETS,
        }
    }
}

impl RapidHashMicro {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        let len = bytes.len();
        let mut a = 0;
        let mut b = 0;
        let mut rem = len as u64;
        let mut s0 = self.seed;

        if bytes.len() <= 16 {
            if bytes.len() >= 4 {
                s0 ^= rem;
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
        } else {
            let mut slice = bytes;
            if slice.len() > 80 {
                let mut s1 = s0;
                let mut s2 = s0;
                let mut s3 = s0;
                let mut s4 = s0;

                while bytes.len() > 112 {
                    s0 = rapid_mix(
                        read_u64(slice, 0) ^ self.secrets[0],
                        read_u64(slice, 8) ^ s0,
                        self.protected,
                    );
                    s1 = rapid_mix(
                        read_u64(slice, 16) ^ self.secrets[1],
                        read_u64(slice, 24) ^ s1,
                        self.protected,
                    );
                    s2 = rapid_mix(
                        read_u64(slice, 32) ^ self.secrets[2],
                        read_u64(slice, 40) ^ s2,
                        self.protected,
                    );
                    s3 = rapid_mix(
                        read_u64(slice, 48) ^ self.secrets[3],
                        read_u64(slice, 56) ^ s3,
                        self.protected,
                    );
                    s4 = rapid_mix(
                        read_u64(slice, 64) ^ self.secrets[4],
                        read_u64(slice, 72) ^ s4,
                        self.protected,
                    );

                    let (_, split) = slice.split_at(80);
                    slice = split;
                }

                s0 ^= s1;
                s2 ^= s3;
                s0 ^= s4;
                s0 ^= s2;
            }
            if slice.len() > 16 {
                s0 = rapid_mix(
                    read_u64(slice, 0) ^ self.secrets[2],
                    read_u64(slice, 8) ^ s0,
                    self.protected,
                );
                if slice.len() > 32 {
                    s0 = rapid_mix(
                        read_u64(slice, 16) ^ self.secrets[2],
                        read_u64(slice, 24) ^ s0,
                        self.protected,
                    );
                    if slice.len() > 48 {
                        s0 = rapid_mix(
                            read_u64(slice, 32) ^ self.secrets[1],
                            read_u64(slice, 40) ^ s0,
                            self.protected,
                        );
                        if slice.len() > 64 {
                            s0 = rapid_mix(
                                read_u64(slice, 48) ^ self.secrets[1],
                                read_u64(slice, 56) ^ s0,
                                self.protected,
                            );
                        }
                    }
                }
            }
            rem = slice.len() as u64;
            a ^= read_u64(bytes, len - 16) ^ rem;
            b ^= read_u64(bytes, len - 8) ^ rem;
        }

        finalize(a, b, rem, s0, self.avalanche, self.protected, &self.secrets)
    }
}

pub struct RapidHashNano {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl Default for RapidHashNano {
    fn default() -> Self {
        Self {
            seed: 0,
            avalanche: true,
            protected: true,
            secrets: DEFAULT_SECRETS,
        }
    }
}

impl RapidHashNano {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        let len = bytes.len();
        let mut a = 0;
        let mut b = 0;
        let mut rem = len as u64;
        let mut s0 = self.seed;

        if bytes.len() <= 16 {
            if bytes.len() >= 4 {
                s0 ^= rem;
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
        } else {
            let mut slice = bytes;

            if slice.len() > 48 {
                let mut s1 = s0;
                let mut s2 = s0;

                while slice.len() > 48 {
                    s0 = rapid_mix(
                        read_u64(slice, 0) ^ self.secrets[0],
                        read_u64(slice, 8) ^ s0,
                        self.protected,
                    );
                    s1 = rapid_mix(
                        read_u64(slice, 16) ^ self.secrets[1],
                        read_u64(slice, 24) ^ s1,
                        self.protected,
                    );
                    s2 = rapid_mix(
                        read_u64(slice, 32) ^ self.secrets[2],
                        read_u64(slice, 40) ^ s2,
                        self.protected,
                    );
                    let (_, split) = slice.split_at(48);
                    slice = split;
                }

                s0 ^= s1;
                s0 ^= s2;
            }

            if slice.len() > 16 {
                s0 = rapid_mix(
                    read_u64(slice, 0) ^ self.secrets[2],
                    read_u64(slice, 8) ^ s0,
                    self.protected,
                );
                if slice.len() > 32 {
                    s0 = rapid_mix(
                        read_u64(slice, 16) ^ self.secrets[2],
                        read_u64(slice, 24) ^ s0,
                        self.protected,
                    );
                }
            }

            rem = slice.len() as u64;
            a ^= read_u64(bytes, bytes.len() - 16) ^ rem;
            b ^= read_u64(bytes, bytes.len() - 8);
        }

        finalize(a, b, rem, s0, self.avalanche, self.protected, &self.secrets)
    }
}
