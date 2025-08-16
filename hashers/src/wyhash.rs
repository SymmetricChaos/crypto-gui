use crate::traits::StatefulHasher;

pub fn wy_mum(a: u64, b: u64) -> (u64, u64) {
    let r = (a as u128).wrapping_mul(b as u128);
    ((r as u64), (r >> 64) as u64)
}

pub fn wy_mix(a: u64, b: u64) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);
    (r as u64) ^ (r >> 64) as u64
}

fn read_u23(bytes: &[u8]) -> u64 {
    u64::from(bytes[0]) << 16
        | u64::from(bytes[bytes.len() >> 1]) << 8
        | u64::from(bytes[bytes.len() - 1])
}

pub fn finalize(a: u64, b: u64, rem: u64, seed: u64, secrets: &[u64; 4]) -> u64 {
    wy_mix(secrets[1] ^ rem, wy_mix(a ^ secrets[1], b ^ seed))
}

pub fn short_hash(
    bytes: &[u8],
    mut a: u64,
    mut b: u64,
    mut s0: u64,
    len: usize,
) -> (u64, u64, u64) {
    assert!(bytes.len() <= 16); // guard against accidental misue
    if bytes.len() >= 4 {
        s0 ^= len as u64;
        if bytes.len() >= 8 {
            // XOR in the first full word and the last full word, these may overlap
            a ^= read_u64(bytes, 0);
            b ^= read_u64(bytes, len - 8);
        } else {
            // XOR in the first full word and the last full word, these may overlap
            // The constants are reversed in some versions of rapidhash
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

// This is called many times so the reference optimizes it a great deal
pub fn read_u64(slice: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(slice[offset..offset + 8].try_into().unwrap())
}

// This is called many times so the reference optimizes it a great deal
pub fn read_u32(slice: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(slice[offset..offset + 4].try_into().unwrap())
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

impl Wyhash {}

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

    fn finalize(self) -> Vec<u8> {
        todo!()
    }
}
