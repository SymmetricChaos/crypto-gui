use crypto_bigint::{NonZero, U256};
// use paste::paste;

// macro_rules! large_point {
//     ($t:ty, $n:ident) => {
//         paste! {
//             #[derive(Debug)]
//             pub struct  [<Point $t >] {
//                 x: $t,
//                 y: $t,
//             }

//             impl [<Point $t >] {
//                 pub const fn from_be_hex(x: &str, y: &str) -> Self {
//                     Self {
//                         x: <$t>::from_be_hex(x),
//                         y: <$t>::from_be_hex(y),
//                     }
//                 }

//                 pub const fn from_u64(x: u64, y: u64) -> Self {
//                     Self {
//                         x: <$t>::from_u64(x),
//                         y: <$t>::from_u64(y),
//                     }
//                 }

//                 pub fn to_string(&self) -> String {
//                     format!(
//                         "({},{})",
//                         self.x.to_string().trim_matches('0'),
//                         self.y.to_string().trim_matches('0')
//                     )
//                 }
//             }
//             pub fn [< on_curve_ $n >] (n: &[<Point $t >], a: &$t, b: &$t, m: &$t) -> bool {
//                 let p = NonZero::new(*m).unwrap();
//                 let x3 = n.x.mul_mod(&n.x, &p).mul_mod(&n.x, &p);
//                 x3.add_mod(&n.x.mul_mod(&a, &p), &p).add_mod(&b, &p) == n.y.mul_mod(&n.y, &p)
//             }

//             pub fn [< ec_add_ $n >](lhs: &[<Point $t >], rhs: &[<Point $t >], m: &$t) -> [<Point $t >] {
//                 let modulus = NonZero::new(*m).unwrap();
//                 let dx = rhs.x.sub_mod(&lhs.x, &modulus);
//                 let dy = rhs.y.sub_mod(&lhs.y, &modulus);
//                 let s: $t = dy.mul_mod(&dx.inv_mod(&modulus).unwrap(), &modulus);
//                 let x = s
//                     .mul_mod(&s, &modulus)
//                     .sub_mod(&lhs.x.add_mod(&rhs.x, &modulus), &modulus);
//                 let y = s
//                     .mul_mod(&lhs.x.sub_mod(&x, &modulus), &modulus)
//                     .sub_mod(&lhs.y, &modulus);
//                 [<Point $t >] { x, y }
//             }

//             pub fn [< ec_mul_ $n >](lhs: &[<Point $t >], rhs: &[<Point $t >], m: &$t) -> [<Point $t >] {
//                 let modulus = NonZero::new(*m).unwrap();
//                 let bits = lhs.bits();
//                 [<Point $t >] { x, y }
//             }
//         }
//     };
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteEllipticCurve {
    a: U256,
    b: U256,
    m: NonZero<U256>,
}

impl FiniteEllipticCurve {
    pub fn on_curve(&self, n: &PointU256) -> bool {
        let x3 = n.x.mul_mod(&n.x, &self.m).mul_mod(&n.x, &self.m);
        x3.add_mod(&n.x.mul_mod(&self.a, &self.m), &self.m)
            .add_mod(&self.b, &self.m)
            == n.y.mul_mod(&n.y, &self.m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointU256 {
    x: U256,
    y: U256,
}
impl PointU256 {
    pub const fn from_be_hex(x: &str, y: &str) -> Self {
        Self {
            x: <U256>::from_be_hex(x),
            y: <U256>::from_be_hex(y),
        }
    }
    pub const fn from_u64(x: u64, y: u64) -> Self {
        Self {
            x: <U256>::from_u64(x),
            y: <U256>::from_u64(y),
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "({},{})",
            self.x.to_string().trim_matches('0'),
            self.y.to_string().trim_matches('0')
        )
    }
}

pub fn ec_add_u256(lhs: &PointU256, rhs: &PointU256, m: &U256) -> PointU256 {
    if lhs == rhs {
        panic!("lhs and rhs must not be the same, use ec_double_u256 instead")
    }
    let modulus = NonZero::new(*m).expect("modulus was zero");
    let dx = rhs.x.sub_mod(&lhs.x, &modulus);
    let dy = rhs.y.sub_mod(&lhs.y, &modulus);
    let s: U256 = dy.mul_mod(
        &dx.inv_mod(&modulus)
            .expect("unable to find modular multiplicative inverse"),
        &modulus,
    );
    let x = s
        .mul_mod(&s, &modulus)
        .sub_mod(&lhs.x.add_mod(&rhs.x, &modulus), &modulus);
    let y = s
        .mul_mod(&lhs.x.sub_mod(&x, &modulus), &modulus)
        .sub_mod(&lhs.y, &modulus);
    PointU256 { x, y }
}

pub fn ec_double_u256(n: &PointU256, a: &U256, m: &U256) -> PointU256 {
    let modulus = NonZero::new(*m).expect("modulus was zero");
    let num =
        n.x.mul_mod(&n.x, &modulus)
            .mul_mod(&U256::from_u64(3), &modulus)
            .add_mod(&a, &modulus);
    let den =
        n.y.add_mod(&n.y, &modulus)
            .inv_mod(&modulus)
            .expect("unable to find modular multiplicative inverse");
    let s = num.mul_mod(&den, &modulus);
    let x = s
        .mul_mod(&s, &modulus)
        .sub_mod(&n.x.add_mod(&n.x, &modulus), &modulus);
    let y = s
        .mul_mod(&n.x.sub_mod(&x, &modulus), &modulus)
        .sub_mod(&n.y, &modulus);
    PointU256 { x, y }
}

pub fn ec_scalar_mul_u256(lhs: &PointU256, rhs: &U256, a: &U256, m: &U256) -> PointU256 {
    let mut out = PointU256::from_u64(0, 0);
    let mut temp = lhs.clone();
    for i in 0..256 {
        if rhs.bit(i).into() {
            out = ec_add_u256(&out, &temp, &m);
        }
        temp = ec_double_u256(&temp, &a, &m)
    }
    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_addition() {
        let curve = FiniteEllipticCurve {
            a: U256::ONE,
            b: U256::ZERO,
            m: NonZero::new(U256::from_u64(257)).unwrap(),
        };

        let p = PointU256::from_u64(1, 60);
        let q = PointU256::from_u64(15, 7);
        assert!(curve.on_curve(&p));
        assert!(curve.on_curve(&q));
        // assert_eq!("(12,F3)", ec_add_u256(&p, &q, &modulus).to_string());
    }

    #[test]
    fn test_multiplication() {
        let a = U256::ONE;
        let b = U256::ZERO;
        let p = PointU256::from_u64(1, 60);
        let modulus = U256::from_u64(257);
        // let p2 = ec_scalar_mul_u256(&p, &U256::from_u64(2), &a, &modulus);
        let double = ec_double_u256(&p, &a, &modulus);
        // println!("{}\n{}", a.to_string(), b.to_string());
        // assert!(a == b);
    }
}
