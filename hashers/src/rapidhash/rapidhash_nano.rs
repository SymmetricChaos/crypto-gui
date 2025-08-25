use crate::{
    rapidhash::{finalize, mix_seed, rapid_mix, read_u64, short_hash, DEFAULT_SECRETS},
    traits::StatefulHasher,
};

fn compress(bytes: &[u8], state: &mut [u64; 3], secrets: &[u64; 7], protected: bool) {
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
}

pub struct RapidHashNanoV3 {
    state: [u64; 3],
    pub avalanche: bool,
    pub protected: bool,
    secrets: [u64; 7],
    buffer: Vec<u8>,
    last_read: Vec<u8>, // this is awkward but the stateful reference version does the same thing
    long_hash: bool,
}

impl Default for RapidHashNanoV3 {
    fn default() -> Self {
        Self {
            state: [0; 3],
            avalanche: true,
            protected: false,
            secrets: super::DEFAULT_SECRETS,
            buffer: Vec::with_capacity(80),
            last_read: Vec::with_capacity(80),
            long_hash: false,
        }
    }
}

impl RapidHashNanoV3 {
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
            state: [seed; 3],
            secrets: secrets,
            ..Default::default()
        }
    }

    // Original spec
    pub fn with_seed_simple(seed: u64) -> Self {
        let seed = seed ^ rapid_mix(seed ^ DEFAULT_SECRETS[2], DEFAULT_SECRETS[1], false);
        Self {
            state: [seed; 3],
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

impl StatefulHasher for RapidHashNanoV3 {
    // This can't use the typical macro because compression is only called when
    // the buffer is GREATER than 48 bytes, rather than equal to 48 bytes
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend(bytes);

        while self.buffer.len() > 48 {
            self.long_hash = true;
            self.last_read = self.buffer[..48].to_vec();
            compress(
                &self.buffer[..48],
                &mut self.state,
                &self.secrets,
                self.protected,
            );
            self.buffer = self.buffer[48..].to_vec();
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        let mut buffer = &self.buffer[..];
        let mut a = 0;
        let mut b = 0;
        let rem = buffer.len() as u64;

        if !self.long_hash && buffer.len() <= 16 {
            (a, b, self.state[0]) = short_hash(&buffer, a, b, self.state[0], buffer.len());
        } else {
            while buffer.len() > 48 {
                self.last_read = buffer.to_vec();
                compress(&buffer, &mut self.state, &self.secrets, self.protected);
                let (_, split) = buffer.split_at(48);
                buffer = split;
            }
            self.state[0] ^= self.state[1];
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
    // All of the short paths
    test_2, RapidHashNanoV3::with_seed(0x123456), b"he",
    "59D459F6E4A1BC44";
    test_5, RapidHashNanoV3::with_seed(0x123456), b"hello",
    "41C86949D9461B4E";
    test_11, RapidHashNanoV3::with_seed(0x123456), b"hello world",
    "A1B8913D9926ED57";

    // Long path with no update compressions
    test_38, RapidHashNanoV3::with_seed(0x123456), b"It is a truth universally acknowledged",
    "67F45C74C90B7124";
    // Long path with one update compressions
    test_117, RapidHashNanoV3::with_seed(0x123456), b"It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.",
    "6CAA66288DF8BF00";
    // Long path with multiple update compressions
    test_378, RapidHashNanoV3::with_seed(0x123456), b"It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife. However little known the feelings or views of such a man may be on his first entering a neighbourhood, this truth is so well fixed in the minds of the surrounding families, that he is considered as the rightful property of some one or other of their daughters.",
    "7093C0D49CCD3F49";

    // Exactly one block
    test_48, RapidHashNanoV3::with_seed(0x123456), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "D72D72B69419BEDE";
    // Exactly two blocks
    test_96, RapidHashNanoV3::with_seed(0x123456), b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "6E51198F2F035CFB";
);
