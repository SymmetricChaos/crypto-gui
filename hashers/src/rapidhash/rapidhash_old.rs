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
            (a, b, s0) = short_hash(bytes, a, b, s0, len);
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
            (a, b, s0) = short_hash(bytes, a, b, s0, len);
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
