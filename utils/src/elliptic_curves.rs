use crypto_bigint::{NonZero, Zero, U256};
use num::One;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPoint {
    pub x: Option<U256>,
    pub y: Option<U256>,
}

impl EcPoint {
    pub const fn from_be_hex(x: &str, y: &str) -> Self {
        Self {
            x: Some(<U256>::from_be_hex(x)),
            y: Some(<U256>::from_be_hex(y)),
        }
    }

    pub const fn from_u256(x: U256, y: U256) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
        }
    }

    pub const fn from_u64(x: u64, y: u64) -> Self {
        Self {
            x: Some(<U256>::from_u64(x)),
            y: Some(<U256>::from_u64(y)),
        }
    }

    pub const fn point_at_inf() -> Self {
        Self { x: None, y: None }
    }

    pub fn is_inf(&self) -> bool {
        self.x == None && self.y == None
    }

    pub fn is_valid(&self) -> bool {
        // Point at infinity is always valid
        if self.is_inf() {
            return true;
        } else {
            // Having a None value other than in the point at infiniy is invalid
            if self.x.is_none() || self.y.is_none() {
                return false;
            }
        }
        true
    }
}

impl Display for EcPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_inf() {
            write!(f, "Inf")
        } else if self.x.is_some() && self.y.is_some() {
            let x = if self.x.unwrap().is_zero().into() {
                "0".to_string()
            } else {
                self.x.unwrap().to_string().trim_matches('0').to_string()
            };
            let y = if self.y.unwrap().is_zero().into() {
                "0".to_string()
            } else {
                self.y.unwrap().to_string().trim_matches('0').to_string()
            };

            write!(f, "({}, {})", x, y)
        } else {
            write!(f, "ERROR")
        }
    }
}

/// Elliptic curve of the form y^2 = x^3 + ax + b (mod m)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteEllipticCurve {
    pub a: U256,
    pub b: U256,
    pub m: NonZero<U256>,
}

impl FiniteEllipticCurve {
    /// NIST P256
    pub fn p256() -> Self {
        Self::from_be_hex(
            "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
            "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
        )
    }

    pub fn from_be_hex(a: &str, b: &str, m: &str) -> Self {
        Self {
            a: U256::from_be_hex(a),
            b: U256::from_be_hex(b),
            m: NonZero::new(U256::from_be_hex(m)).expect("zero modulus"),
        }
    }

    pub fn from_u256(a: U256, b: U256, m: U256) -> Self {
        Self {
            a,
            b,
            m: NonZero::new(m).expect("zero modulus"),
        }
    }

    pub fn from_u64(a: u64, b: u64, m: u64) -> Self {
        Self {
            a: U256::from_u64(a),
            b: U256::from_u64(b),
            m: NonZero::new(U256::from_u64(m)).expect("zero modulus"),
        }
    }

    pub fn on_curve(&self, p: &EcPoint) -> bool {
        if !p.is_valid() {
            panic!("invalid EcPoint encountered")
        }
        if p.is_inf() {
            return true;
        } else {
            let x = p.x.unwrap();
            let y = p.y.unwrap();
            let x3 = x.mul_mod(&x, &self.m).mul_mod(&x, &self.m);
            x3.add_mod(&x.mul_mod(&self.a, &self.m), &self.m)
                .add_mod(&self.b, &self.m)
                == y.mul_mod(&y, &self.m)
        }
    }

    pub fn inverse(&self, p: &EcPoint) -> EcPoint {
        if !p.is_valid() {
            panic!("invalid EcPoint encountered")
        }
        if p.is_inf() {
            return EcPoint::point_at_inf();
        } else {
            EcPoint {
                x: p.x,
                y: Some(p.y.unwrap().neg_mod(&self.m)),
            }
        }
    }

    pub fn add(&self, p: &EcPoint, q: &EcPoint) -> EcPoint {
        if !p.is_valid() || !q.is_valid() {
            panic!("invalid EcPoint encountered")
        }
        if p.is_inf() {
            return q.clone();
        }
        if q.is_inf() {
            return p.clone();
        }
        if *p == self.inverse(q) {
            return EcPoint::point_at_inf();
        }
        if p == q {
            return self.double(p);
        }

        let px = p.x.unwrap();
        let py = p.y.unwrap();

        let qx = q.x.unwrap();
        let qy = q.y.unwrap();

        // (q_x - p_x)
        let dx = qx.sub_mod(&px, &self.m);
        // (q_y - p_y)
        let dy = qy.sub_mod(&py, &self.m);
        // (q_y - p_y) / (q_x - p_x)
        let s: U256 = dy.mul_mod(
            &dx.inv_mod(&self.m)
                .expect("unable to find modular multiplicative inverse when adding"),
            &self.m,
        );
        let x = s
            .mul_mod(&s, &self.m)
            .sub_mod(&px.add_mod(&qx, &self.m), &self.m);
        let y = s
            .mul_mod(&px.sub_mod(&x, &self.m), &self.m)
            .sub_mod(&py, &self.m);

        EcPoint::from_u256(x, y)
    }

    pub fn double(&self, p: &EcPoint) -> EcPoint {
        if !p.is_valid() {
            panic!("invalid EcPoint encountered")
        }
        if p.is_inf() {
            return p.clone();
        }
        if *p == self.inverse(p) {
            return EcPoint::point_at_inf();
        }
        if p.y.unwrap().is_zero().into() {
            return EcPoint::point_at_inf();
        }

        let px = p.x.unwrap();
        let py = p.y.unwrap();

        // 3x^3 + a
        let num = px
            .mul_mod(&px, &self.m)
            .mul_mod(&U256::from_u64(3), &self.m)
            .add_mod(&self.a, &self.m);
        // 1 / 2y
        let den = py
            .add_mod(&py, &self.m)
            .inv_mod(&self.m)
            .expect("unable to find modular multiplicative inverse when doubling");
        // (2x^3 + a) / 2y
        let s = num.mul_mod(&den, &self.m);
        let x = s
            .mul_mod(&s, &self.m)
            .sub_mod(&px.add_mod(&px, &self.m), &self.m);
        let y = s
            .mul_mod(&px.sub_mod(&x, &self.m), &self.m)
            .sub_mod(&py, &self.m);

        EcPoint::from_u256(x, y)
    }

    pub fn scalar_mul(&self, p: &EcPoint, s: &U256) -> EcPoint {
        if s.is_zero().into() {
            return EcPoint::point_at_inf();
        }
        if s.is_one() {
            return p.clone();
        }
        let mut out = EcPoint::point_at_inf();
        let mut temp = p.clone();
        for i in 0..256 {
            if s.bit(i).into() {
                out = self.add(&out, &temp);
            }
            temp = self.double(&temp)
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generating_point() {
        let curve = FiniteEllipticCurve::from_u64(0, 3, 11);
        let g = EcPoint::from_u64(4, 10);
        let mut p = g.clone();
        let points = [
            "(4, A)", "(7, 7)", "(1, 9)", "(0, 6)", "(8, 8)", "(2, 0)", "(8, 3)", "(0, 5)",
            "(1, 2)", "(7, 4)", "(4, 1)", "Inf",
        ];
        for i in 0..12 {
            assert!(curve.on_curve(&p));
            assert_eq!(points[i], p.to_string());
            p = curve.add(&p, &g);
        }
    }

    #[test]
    fn multiplication() {
        let curve = FiniteEllipticCurve::from_u64(0, 3, 11);
        let g = EcPoint::from_u64(4, 10);
        let points = [
            "(4, A)", "(7, 7)", "(1, 9)", "(0, 6)", "(8, 8)", "(2, 0)", "(8, 3)", "(0, 5)",
            "(1, 2)", "(7, 4)", "(4, 1)", "Inf",
        ];
        for i in 0..12 {
            assert_eq!(
                points[i],
                curve
                    .scalar_mul(&g, &U256::from_u64((i + 1) as u64))
                    .to_string()
            );
        }
    }
}
