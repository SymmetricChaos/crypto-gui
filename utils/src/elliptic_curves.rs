// Utilities for elliptic curves over finite fields.

use std::ops::Add;

// FIPS 186-4 recommendes five prime fields and five binary fields
pub enum NistCurve {
    P192,
    P224,
    P256,
    P384,
    P521,
    // B162,
    // B233,
    // B283,
    // B409,
    // B571,
}

/// Point in a finite field of an elliptic curve
#[derive(Debug, Clone, Copy)]
pub struct EcPoint(Option<(u64, u64)>);

impl EcPoint {
    /// Is this the point at infinity?
    pub fn is_inf(&self) -> bool {
        self.0.is_none()
    }

    /// Multiplicative inverse modulo some field size
    pub fn inv(&self, modulus: u64) -> Self {}
}

impl Add for EcPoint {
    type Output: EcPoint;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
