use crypto_bigint::U1024;
use num::One;
use utils::primality::PrimeSieve;

fn block_size(n: U1024) -> U1024 {
    let mut out = U1024::one();
    for p in PrimeSieve::new() {
        if out * U1024::from(p as u32) > n {
            break;
        } else {
            out *= U1024::from(p as u32)
        }
    }
    out
}

pub struct Vsh {
    pub n: U1024,
}

impl Default for Vsh {
    fn default() -> Self {
        Self {
            n: Default::default(),
        }
    }
}

impl Vsh {
    /// Create a new hasher from a valid RSA number
    pub fn new(n: U1024) -> Self {
        Self { n }
    }
}
