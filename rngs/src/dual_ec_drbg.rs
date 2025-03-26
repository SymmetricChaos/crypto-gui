use crate::ClassicRng;
use crypto_bigint::U256;
use std::sync::LazyLock;
use utils::elliptic_curves::{EcPoint, FiniteEllipticCurve};

// https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-90a.pdf

// The order of P in P256
// static N: U256 =
//     U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");

pub static P: EcPoint = EcPoint::from_be_hex(
    "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
    "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
);

pub static Q: EcPoint = EcPoint::from_be_hex(
    "c97445f45cdef9f0d3e05e1e585fc297235b82b5be8ff3efca67c59852018192",
    "b28ef557ba31dfcbdd21ac46e2a91e3c304f44cb87058ada2cb815151e610046",
);

pub static P256: LazyLock<FiniteEllipticCurve> = LazyLock::new(|| FiniteEllipticCurve::p256());

pub struct DualEcDrbgP256 {
    pub state: U256,
    pub ctr: u64,
}

impl Default for DualEcDrbgP256 {
    fn default() -> Self {
        Self {
            state: U256::from_u64(1),
            ctr: 0,
        }
    }
}

impl DualEcDrbgP256 {
    // pub fn instantiate(entropy: &[u8], nonce: &[u8], personalization: &[u8]) -> Self {
    //     let mut hasher = todo!();
    //     hasher.update(&[1_u8]);
    //     hasher.update(256_u32.to_be_bytes());
    //     hasher.update(entropy);
    //     hasher.update(nonce);
    //     hasher.update(personalization);
    //     let bytes: Vec<u8> = hasher.finalize();
    //     Self {
    //         state: U256::from_be_slice(&bytes),
    //         ctr: 0,
    //     }
    // }

    pub fn step(&mut self) {
        self.state = P256.scalar_mul(&P, &self.state).x.unwrap();
    }
}

impl ClassicRng for DualEcDrbgP256 {
    fn next_u32(&mut self) -> u32 {
        self.step();
        P256.scalar_mul(&Q, &self.state).x.unwrap().as_words()[3] as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.step();
        P256.scalar_mul(&Q, &self.state).x.unwrap().as_words()[3] as u64
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;
// }
