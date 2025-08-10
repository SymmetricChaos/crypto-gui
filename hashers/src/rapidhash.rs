// based on
// https://github.com/hoxxep/rapidhash/tree/master/rapidhash

// A "proper" implementation of any of these should be as optimized as possible.
// However that is a non-goal for this project and they are presented to be as
// easy to understand as possible.

// TODO: Can this be implemented as a stateful hasher?

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

// Little endian
fn read_u64(slice: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(slice[offset..offset + 8].try_into().unwrap())
}

// Little endian
fn read_u32(slice: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(slice[offset..offset + 4].try_into().unwrap())
}

fn rapidhash_core_cold(bytes: &[u8], seed: u64, avalanche: bool, protected: bool) -> u64 {
    todo!()
}

fn rapidhash_finish(a: u64, b: u64, rem: u64, protected: bool, secrets: &[u64; 7]) -> u64 {
    rapid_mix(a ^ 0xaaaaaaaaaaaaaaaa, b ^ secrets[1] ^ rem, protected)
}

pub struct RapidHash {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl RapidHash {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        let len = bytes.len();
        let mut seed = self.seed;
        let mut a = 0;
        let mut b = 0;
        let rem = len as u64;

        if bytes.len() <= 16 {
            if bytes.len() >= 4 {
                seed ^= rem;
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
            return rapidhash_core_cold(bytes, self.seed, self.avalanche, self.protected);
        }

        a ^= self.secrets[1];
        b ^= seed;

        (a, b) = rapid_mum(a, b, self.protected);

        if self.avalanche {
            rapidhash_finish(a, b, rem, self.protected, &self.secrets)
        } else {
            a ^ b
        }
    }
}

pub struct RapidHashMicro {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl RapidHashMicro {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        todo!()
        // let mut a = 0;
        // let mut b = 0;
        // let rem;

        // if self.avalanche {
        //     rapidhash_finish(a, b, rem, self.protected, &self.secrets)
        // } else {
        //     a ^ b
        // }
    }
}

pub struct RapidHashNano {
    pub seed: u64,
    pub avalanche: bool,
    pub protected: bool,
    pub secrets: [u64; 7],
}

impl RapidHashNano {
    pub fn hash(&self, bytes: &[u8]) -> u64 {
        todo!()
        // let mut a = 0;
        // let mut b = 0;
        // let rem;

        // if self.avalanche {
        //     rapidhash_finish(a, b, rem, self.protected, &self.secrets)
        // } else {
        //     a ^ b
        // }
    }
}
