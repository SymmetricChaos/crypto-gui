// Utilities for elliptic curves over finite fields.

use std::ops::Add;

use num::Integer;

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

pub fn add_mod(p: u32, q: u32, m: u32) -> u32 {
    ((p as u64 + q as u64) % m as u64) as u32
}

pub fn sub_mod(p: u32, q: u32, m: u32) -> u32 {
    ((m as u64 + p as u64 - q as u64) % m as u64) as u32
}

pub fn mul_mod(p: u32, q: u32, m: u32) -> u32 {
    ((p as u64 * q as u64) % m as u64) as u32
}

pub fn div_mod(p: u32, q: u32, m: u32) -> Option<u32> {
    let inv = mul_inv(q, m)?;
    Some(((p as u64 * inv as u64) % m as u64) as u32)
}

pub fn mul_inv(num: u32, modulus: u32) -> Option<u32> {
    if num == 0 {
        return None;
    }

    let egcd = num.extended_gcd(&modulus);
    if !egcd.gcd == 1 {
        None
    } else {
        Some(egcd.x.mod_floor(&modulus))
    }
}

/// Point in a finite field of an elliptic curve
#[derive(Debug, Clone, Copy)]
pub struct EcPoint(Option<(u32, u32)>);

impl EcPoint {
    /// Point at infinity
    pub fn inf() -> EcPoint {
        EcPoint(None)
    }

    /// Is this the point at infinity?
    pub fn is_inf(&self) -> bool {
        self.0.is_none()
    }

    /// Left component
    pub fn x(&self) -> Option<u32> {
        Some(self.0?.0)
    }

    // Right component
    pub fn y(&self) -> Option<u32> {
        Some(self.0?.1)
    }

    /// Addition inverse modulo some field size
    pub fn add(&self, rhs: EcPoint, modulus: u32) -> Self {
        if self.is_inf() || rhs.is_inf() {
            return EcPoint::inf();
        } else {
            let xp = self.x().unwrap();
            let yp = self.y().unwrap();
            let xq = rhs.x().unwrap();
            let yq = rhs.y().unwrap();
            let ydiff = sub_mod(yp, yq, modulus);
            let xdiff = sub_mod(xp, xq, modulus);
            let lambda = div_mod(ydiff, xdiff, modulus).expect("modular inverse failed");
            let xr = sub_mod(
                sub_mod(add_mod(lambda, lambda, modulus), xp, modulus),
                xq,
                modulus,
            );
            let yr = sub_mod(
                mul_mod(lambda, sub_mod(xp, xr, modulus), modulus),
                yp,
                modulus,
            );
            EcPoint(Some((xr, yr)))
        }
    }

    // /// Multiplicative inverse modulo some field size
    // pub fn inv(&self, modulus: u32) -> Self {}
}
