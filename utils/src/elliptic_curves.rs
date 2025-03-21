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

#[derive(Debug, Clone)]
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
pub fn on_curve_u256(n: &PointU256, a: &U256, b: &U256, m: &U256) -> bool {
    let p = NonZero::new(*m).unwrap();
    let x3 = n.x.mul_mod(&n.x, &p).mul_mod(&n.x, &p);
    x3.add_mod(&n.x.mul_mod(&a, &p), &p).add_mod(&b, &p) == n.y.mul_mod(&n.y, &p)
}

pub fn ec_add_u256(lhs: &PointU256, rhs: &PointU256, m: &U256) -> PointU256 {
    let modulus = NonZero::new(*m).unwrap();
    let dx = rhs.x.sub_mod(&lhs.x, &modulus);
    let dy = rhs.y.sub_mod(&lhs.y, &modulus);
    let s: U256 = dy.mul_mod(&dx.inv_mod(&modulus).unwrap(), &modulus);
    let x = s
        .mul_mod(&s, &modulus)
        .sub_mod(&lhs.x.add_mod(&rhs.x, &modulus), &modulus);
    let y = s
        .mul_mod(&lhs.x.sub_mod(&x, &modulus), &modulus)
        .sub_mod(&lhs.y, &modulus);
    PointU256 { x, y }
}
pub fn ec_scalar_mul_u256(lhs: &PointU256, rhs: &U256, m: &U256) -> PointU256 {
    let mut out = PointU256::from_u64(0, 0);
    let mut temp = lhs.clone();
    for i in 0..256 {
        if rhs.bit(i).into() {
            out = ec_add_u256(&out, &temp, &m);
        }
        temp = ec_add_u256(&temp, &temp, &m)
    }
    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_addition() {
        let p = PointU256::from_u64(1, 60);
        let q = PointU256::from_u64(15, 7);
        let modulus = U256::from_u64(257);
        println!("{}", on_curve_u256(&p, &U256::ONE, &U256::ZERO, &modulus));
        println!("{}", ec_add_u256(&p, &q, &modulus).to_string());
    }
}
