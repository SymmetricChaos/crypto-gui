use crypto_bigint::U1024;
use num::{Num, One};
use std::cell::LazyCell;
use utils::primality::PrimeSieve;

/// 1024-bits
const RSA_309: LazyCell<U1024> = LazyCell::new(|| {
    U1024::from_str_radix(
        "BDD14965645E9E42E7F658C6FC3E4C73C69DC246451C714EB182305B0FD6ED47D84BC9A610172FB56DAE2F89FA40E7C9521EC3F97EA12FF7C3248181CEBA33B55212378B579AE6627BCC082130955234E5B26A3E425BC1254326173D5F4E25A6D2E172FE62D81CED2C9F362B982F30650881CE46B7D52F14885EECF903076CA5",
        16,
    )
    .unwrap()
});
const RSA_309_BLOCK_SIZE: LazyCell<U1024> = LazyCell::new(|| block_size(&RSA_309));

fn block_size(n: &U1024) -> U1024 {
    let mut out = U1024::one();
    for p in PrimeSieve::new() {
        if out * U1024::from(p) > *n {
            break;
        } else {
            out *= U1024::from(p)
        }
    }
    out
}

pub struct Vsh {
    pub n: U1024,
    pub block_size: U1024,
}

impl Default for Vsh {
    fn default() -> Self {
        Self {
            n: RSA_309.to_owned(),
            block_size: RSA_309_BLOCK_SIZE.to_owned(),
        }
    }
}

impl Vsh {
    /// Create a new hasher. Assumes that n is a valid RSA number.
    pub fn new(n: U1024) -> Self {
        Self {
            n,
            block_size: block_size(&n),
        }
    }
}

#[test]
fn block() {
    println!("{:?}", *RSA_309_BLOCK_SIZE);
}
