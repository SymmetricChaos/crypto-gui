// https://en.wikipedia.org/wiki/Elias_delta_coding

use crate::errors::CodeError;
use num::Zero;
use utils::bits::{bits_to_u32_lower, Bit};

pub struct DeltaGen {
    pub n: u32,
}

impl DeltaGen {
    pub fn new() -> Self {
        DeltaGen { n: 0 }
    }
}

impl Iterator for DeltaGen {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
        self.n += 1;
        if self.n == 1 {
            Some((self.n, "1".into()))
        } else {
            let p = self.n.ilog2() as usize;
            let l = (p + 1).ilog2() as usize;
            let mut out = "0".repeat(l);
            out.push_str(&format!("{:b}", p + 1));
            out.push_str(&format!("{:b}", self.n)[1..]);
            Some((self.n, out))
        }
    }
}

pub fn delta_to_u32(bits: &mut dyn Iterator<Item = Bit>) -> Result<Vec<u32>, CodeError> {
    let mut out = Vec::new();
    let mut buffer = Vec::new();
    let mut zero_ctr = 0;
    loop {
        if let Some(b) = bits.next() {
            buffer.push(b);
            // Count up zeroes until a one is reached
            if b.is_zero() {
                zero_ctr += 1;
                continue;
            } else {
                // Once we reach a one get extra bits equal to the zeroes seen
                for _ in 0..zero_ctr {
                    if let Some(b) = bits.next() {
                        buffer.push(b)
                    } else {
                        return Err(CodeError::input("partial or malformed input"));
                    }
                }
                // Convert the bits into an integer
                let remaining = bits_to_u32_lower(&buffer) - 1;

                // Take that many more bits
                buffer.clear();
                for _ in 0..remaining {
                    if let Some(b) = bits.next() {
                        buffer.push(b)
                    } else {
                        return Err(CodeError::input("partial or malformed input"));
                    }
                }

                let f = bits_to_u32_lower(&buffer);

                out.push(2_u32.pow(remaining) + f);
                // Clear buffer and counter
                buffer.clear();
                zero_ctr = 0;
            }
        } else {
            break;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use utils::bits::bits_from_str;

    use super::*;
    #[test]
    fn delta_codes() {
        let codes = DeltaGen::new();
        for ((_, code), check) in codes.zip([
            "1", "0100", "0101", "01100", "01101", "01110", "01111", "00100000", "00100001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn delta_decode_u32() {
        assert_eq!(
            vec![19],
            delta_to_u32(&mut bits_from_str("001010011").unwrap()).unwrap()
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            delta_to_u32(
                &mut bits_from_str("101000101011000110101110011110010000000100001").unwrap()
            )
            .unwrap()
        );
    }
}
