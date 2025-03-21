use crypto_bigint::U256;
use utils::elliptic_curves::PointU256;

const CURVEP256_MODULUS: U256 =
    U256::from_be_hex("FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF");

const P: PointU256 = PointU256::from_be_hex(
    "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
    "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
);

const Q: PointU256 = PointU256::from_be_hex(
    "c97445f45cdef9f0d3e05e1e585fc297235b82b5be8ff3efca67c59852018192",
    "b28ef557ba31dfcbdd21ac46e2a91e3c304f44cb87058ada2cb815151e610046",
);

pub struct DualEcDrbgP256 {}

#[cfg(test)]
mod tests {

    use super::*;
}
