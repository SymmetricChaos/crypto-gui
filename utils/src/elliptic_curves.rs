use crypto_bigint::{NonZero, U256};

#[derive(Debug)]
pub struct Point256 {
    x: U256,
    y: U256,
}

impl Point256 {
    pub const fn from_be_hex(x: &str, y: &str) -> Self {
        Self {
            x: U256::from_be_hex(x),
            y: U256::from_be_hex(y),
        }
    }

    pub const fn from_u64(x: u64, y: u64) -> Self {
        Self {
            x: U256::from_u64(x),
            y: U256::from_u64(y),
        }
    }
}

const P: Point256 = Point256::from_be_hex(
    "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
    "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
);

const Q: Point256 = Point256::from_be_hex(
    "c97445f45cdef9f0d3e05e1e585fc297235b82b5be8ff3efca67c59852018192",
    "b28ef557ba31dfcbdd21ac46e2a91e3c304f44cb87058ada2cb815151e610046",
);

pub fn curve_256(x: U256, a: U256, b: U256, p: U256) -> U256 {
    let p = NonZero::new(p).unwrap();
    let x3 = x.mul_mod(&x, &p).mul_mod(&x, &p);
    x3.add_mod(&x.mul_mod(&a, &p), &p).add_mod(&b, &p)
}

pub fn add_p256(n1: &Point256, n2: &Point256, p: &U256) -> Point256 {
    let modulus = NonZero::new(*p).unwrap();
    let dx = n2.x.sub_mod(&n1.x, &modulus);
    let dy = n2.y.sub_mod(&n1.y, &modulus);
    let s: U256 = dy.mul_mod(&dx, &modulus);
    let x = s
        .mul_mod(&s, &modulus)
        .sub_mod(&n1.x.add_mod(&n2.x, &modulus), &p);
    let y = s
        .mul_mod(&n1.x.sub_mod(&x, &p), &modulus)
        .sub_mod(&n1.y, &modulus);
    Point256 { x, y }
}

pub struct CurveP256 {
    p: U256,
    n: U256,
    b: U256,
}

impl Default for CurveP256 {
    fn default() -> Self {
        Self {
            // 115792089210356248762697446949407573530086143415290314195533631308867097853951
            p: U256::from_be_hex(
                "FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF",
            ),
            // 115792089210356248762697446949407573529996955224135760342422259061068512044369
            n: U256::from_be_hex(
                "FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551",
            ),
            b: U256::from_be_hex(
                "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_addition() {
        let p = Point256::from_u64(1, 60);
        let q = Point256::from_u64(15, 7);
        let modulus = U256::from_u64(257);
        println!("{:?}", add_p256(&p, &q, &modulus));
    }
}
