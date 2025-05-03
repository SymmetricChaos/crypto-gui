use crate::{blake::Blake2b, traits::StatefulHasher};

// Identical to Blake2b but allowing a hash of any length. This variant is specific to Argon2.
#[derive(Debug, Clone)]
pub struct Blake2bLong {
    state: Blake2b,
    hash_len: u64, // length of output in bytes, 1 to 64
}

impl Blake2bLong {
    pub fn init<T: AsRef<[u8]>>(key: T, hash_len: u64) -> Self {
        let mut h = if hash_len <= 64 {
            Blake2b::init(key, hash_len)
        } else {
            Blake2b::init(key, 64)
        };

        h.update(&(hash_len as u32).to_le_bytes());
        Self { state: h, hash_len }
    }

    pub fn init_hash(hash_len: u64) -> Self {
        let mut h = if hash_len <= 64 {
            Blake2b::init(&[], hash_len)
        } else {
            Blake2b::init(&[], 64)
        };

        h.update(&(hash_len as u32).to_le_bytes());
        Self { state: h, hash_len }
    }

    pub fn hash_len(&self) -> u64 {
        self.hash_len
    }
}

impl StatefulHasher for Blake2bLong {
    fn update(&mut self, bytes: &[u8]) {
        self.state.update(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        if self.hash_len <= 64 {
            return self.state.finalize();
        } else {
            let mut out = Vec::with_capacity(self.hash_len as usize);
            let mut ctr = self.hash_len;
            let mut v = self.state.finalize();

            while ctr > 32 {
                // Take 32 bytes of the temporary value then hash the whole vector
                // This is presumably related to length extension type attacks
                out.extend_from_slice(&v[0..32]);
                ctr -= 32;
                v = Blake2b::hash_512(&v)
            }

            // Final bytes change the hash length of Blake2b, which alters its state, so truncation is not used
            let mut h = Blake2b::init(&[], ctr as u64);
            h.update(&v);
            out.extend_from_slice(&h.finalize());

            out
        }
    }
}
