// https://en.wikipedia.org/wiki/Elias_omega_coding

use crate::errors::CodeError;
use num::Zero;
use utils::bits::{bits_to_u32_lower, Bit};

pub struct OmegaGen {
    pub n: u32,
}

impl OmegaGen {
    pub fn new() -> Self {
        OmegaGen { n: 0 }
    }
}

impl Iterator for OmegaGen {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
        self.n += 1;
        let mut temp_n = self.n;
        let mut out = String::from("0");
        while temp_n > 1 {
            out.insert_str(0, &format!("{:b}", temp_n));
            temp_n = temp_n.ilog2();
        }
        Some((self.n, out))
    }
}

pub fn omega_to_u32(bits: &mut dyn Iterator<Item = Bit>) -> Result<Vec<u32>, CodeError> {
    let mut out = Vec::new();
    let mut buffer = Vec::new();
    let mut n = 1;
    loop {
        if let Some(b) = bits.next() {
            buffer.push(b);
            // If we reach a zero stop and return n
            if b.is_zero() {
                out.push(n);
                // Reset n
                n = 1;
            } else {
                // If we reached a 1 take the next n bits as a number and make them the new value of n
                for _ in 0..n {
                    if let Some(b) = bits.next() {
                        buffer.push(b)
                    } else {
                        return Err(CodeError::input("partial or malformed input"));
                    }
                }
                n = bits_to_u32_lower(&buffer);
                buffer.clear();
            }
        } else {
            break;
        }
    }
    Ok(out)
}

pub fn recognize_omega(text: &str) -> Vec<Option<u32>> {
    let mut out = Vec::new();
    let mut buffer = Vec::new();
    let mut n = 1;
    let mut bits = text.chars().filter(|c| !c.is_whitespace()).map(|c| {
        if c == '0' {
            Some(Bit::Zero)
        } else if c == '1' {
            Some(Bit::One)
        } else {
            None
        }
    });
    'outer: loop {
        if let Some(bit) = bits.next() {
            let b = if let Some(b) = bit {
                buffer.push(b);
                b
            } else {
                // If we get an invalid symbol interrupt and restart
                out.push(None);
                buffer.clear();
                n = 1;
                continue;
            };

            // If we reach a zero stop and return n
            if b.is_zero() {
                out.push(Some(n));
                n = 1;
            } else {
                // If we reached a 1 take the next n bits as a number and make them the new value of n
                for _ in 0..n {
                    if let Some(bit) = bits.next() {
                        if let Some(b) = bit {
                            buffer.push(b);
                        } else {
                            out.push(None);
                            buffer.clear();
                            n = 1;
                            continue 'outer;
                        };
                    } else {
                        out.push(None);
                        continue 'outer;
                    };
                }
                n = bits_to_u32_lower(&buffer);
                buffer.clear();
            }
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use utils::bits::bits_from_str;

    use super::*;

    #[test]
    fn omega_codes() {
        let codes = OmegaGen::new();
        for ((_, code), check) in codes.zip([
            "0", "100", "110", "101000", "101010", "101100", "101110", "1110000", "1110010",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn omega_decode_u32() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            omega_to_u32(
                &mut bits_from_str("010011010100010101010110010111011100001110010").unwrap()
            )
            .unwrap()
        );
    }

    // #[test]
    // fn omega_decode_u32_str() {
    //     println!(
    //         "{:?}",
    //         recognize_omega("01001101010x0010101010110010111011100001110010")
    //     );
    // }
}
