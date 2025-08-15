pub mod rapidhash;
pub mod rapidhash_micro;

// based on
// https://github.com/hoxxep/rapidhash/tree/master/rapidhash

// A "proper" implementation of any of these should be as optimized as possible
// as that is the point of this kind of hash function.
// The reference above, in fact, is designed to compile to highly specific
// variations to reduce size and increase speed, especially for fixed size
// inputs.
// However that is a non-goal for this project and they are presented to be as
// easy to understand as possible.

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
pub fn rapid_mum(a: u64, b: u64, protected: bool) -> (u64, u64) {
    let r = (a as u128).wrapping_mul(b as u128);

    if protected {
        ((a ^ r as u64), (b ^ (r >> 64) as u64))
    } else {
        ((r as u64), (r >> 64) as u64)
    }
}

// Wide multiply then XOR the lower and upper halves together
// A folded version of rapid_mum()
pub fn rapid_mix(a: u64, b: u64, protected: bool) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);

    if protected {
        (a ^ r as u64) ^ (b ^ (r >> 64) as u64)
    } else {
        (r as u64) ^ (r >> 64) as u64
    }
}

pub fn finalize(
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

pub fn mix_seed(mut seed: u64, i: usize) -> u64 {
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
