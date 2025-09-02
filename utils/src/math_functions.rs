use num::{bigint::ToBigInt, integer::Roots, BigInt, Integer, One, ToPrimitive, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Odd,
    Even,
}

impl Parity {
    pub fn cycle(&self) -> std::iter::Cycle<std::array::IntoIter<Parity, 2>> {
        match self {
            Parity::Odd => [Self::Odd, Self::Even].into_iter().cycle(),
            Parity::Even => [Self::Even, Self::Odd].into_iter().cycle(),
        }
    }
}

pub fn is_square(n: usize) -> bool {
    n.sqrt().pow(2) == n
}

pub fn mul_inv<N: ToBigInt>(num: &N, modulus: &N) -> Option<BigInt> {
    let num = num.to_bigint().expect("unable to convert num to BigInt");
    let modulus = modulus
        .to_bigint()
        .expect("unable to convert modulus to BigInt");
    if num.is_zero() {
        return None;
    }

    let egcd = num.extended_gcd(&modulus);
    if !egcd.gcd.is_one() {
        None
    } else {
        Some(egcd.x.mod_floor(&modulus))
    }
}

pub fn modular_division<N: ToBigInt>(n: &N, d: &N, m: &N) -> Option<BigInt> {
    Some(
        n.to_bigint()
            .expect("unable to convert numerator to BigInt")
            * mul_inv(d, m)?,
    )
}

// We're not going to deal with big numbers so this thse crude factorizations are plenty
pub fn factors<N: Integer + Copy + ToPrimitive>(n: N) -> Vec<N> {
    let mut out = Vec::new();
    for f in num::range(N::one(), n) {
        if n.is_multiple_of(&f) {
            out.push(n);
            out.push(n.div(f));
        }
    }
    out.sort();
    out
}

pub fn mod_pow_32(base: u32, pow: u32, modulus: u32) -> u32 {
    let mut out = 1;

    for _ in 0..pow {
        out *= u64::from(base);
        out %= u64::from(modulus);
    }

    // This truncation is always valid because it has been reduced by the modulus which starts as u32
    out as u32
}

pub fn mod_pow_64(base: u64, pow: u64, modulus: u64) -> u64 {
    let mut out = 1;

    for _ in 0..pow {
        out *= u128::from(base);
        out %= u128::from(modulus);
    }

    // This truncation is always valid because it has been reduced by the modulus which starts as u64
    out as u64
}

pub fn mod_mul_32(lhs: u32, rhs: u32, modulus: u32) -> u32 {
    (u64::from(lhs) * u64::from(rhs) % u64::from(modulus)) as u32
}

pub fn mod_mul_64(lhs: u64, rhs: u64, modulus: u64) -> u64 {
    (u128::from(lhs) * u128::from(rhs) % u128::from(modulus)) as u64
}

pub fn incr_array_ctr_be(ctr: &mut [u8]) {
    for byte in ctr.iter_mut().rev() {
        match byte.checked_add(1) {
            Some(n) => {
                *byte = n;
                return ();
            }

            None => {
                *byte = 0;
            }
        }
    }
}

pub fn incr_array_ctr_le(ctr: &mut [u8]) {
    for byte in ctr.iter_mut() {
        match byte.checked_add(1) {
            Some(n) => {
                *byte = n;
                return ();
            }

            None => {
                *byte = 0;
            }
        }
    }
}

#[cfg(test)]
mod math_tests {

    use super::*;

    #[test]
    fn test_mod_pow() {
        let x = mod_pow_32(4, 5, 23);
        assert_eq!(4, x);
        let x = mod_pow_32(3, 5, 23);
        assert_eq!(10, x);
    }

    #[test]
    fn test_incr_array_ctr() {
        let mut ctr = [0x00, 0x00, 0xfe, 0xff, 0xff];
        incr_array_ctr_be(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x00], ctr);
        incr_array_ctr_be(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x01], ctr);

        let mut ctr = [0xff, 0xff, 0xfe, 0x00, 0x00];
        incr_array_ctr_le(&mut ctr);
        assert_eq!([0x00, 0x00, 0xff, 0x00, 0x00], ctr);
        incr_array_ctr_le(&mut ctr);
        assert_eq!([0x01, 0x00, 0xff, 0x00, 0x00], ctr);
    }
}
