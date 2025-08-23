// https://en.wikipedia.org/wiki/Elias_gamma_coding

use crate::next_bit_or_reset;
use num::Zero;
use utils::{
    bits::{bits_to_u32_lower, Bit},
    errors::GeneralError,
};

pub struct GammaGen {
    pub n: u32,
    prefix: String,
}

impl GammaGen {
    pub fn new() -> Self {
        GammaGen {
            n: 0,
            prefix: String::new(),
        }
    }
}

impl Iterator for GammaGen {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
        self.n += 1;
        if self.n == 1 {
            return Some((self.n, "1".to_string()));
        } else {
            if self.n.is_power_of_two() {
                self.prefix.push('0');
            }
            let out = format!("{}{:b}", self.prefix, self.n);
            Some((self.n, out))
        }
    }
}

pub fn gamma_to_u32(bits: &mut dyn Iterator<Item = Bit>) -> Result<Vec<u32>, GeneralError> {
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
                // Once we reach a one clear everything but the one from the buffer and get bits for the zeroes counted
                buffer.clear();
                buffer.push(Bit::One);
                for _ in 0..zero_ctr {
                    if let Some(b) = bits.next() {
                        buffer.push(b)
                    } else {
                        return Err(GeneralError::input("partial or malformed input"));
                    }
                }
                // Convert the bits into an integer

                out.push(bits_to_u32_lower(&buffer));
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

pub fn recognize_gamma(text: &str) -> Vec<Option<u32>> {
    let mut out = Vec::new();
    let mut buffer = Vec::new();
    let mut zero_ctr = 0;
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
                zero_ctr = 0;
                continue;
            };

            // Count up zeroes until a one is reached
            if b.is_zero() {
                zero_ctr += 1;
                continue;
            } else {
                // Once we reach a one clear everything but the one from the buffer and get bits for the zeroes counted
                buffer.clear();
                buffer.push(Bit::One);
                for _ in 0..zero_ctr {
                    next_bit_or_reset!(bits, buffer, out, zero_ctr, 'outer);
                }
                // Convert the bits into an integer

                out.push(Some(bits_to_u32_lower(&buffer)));
                // Clear buffer and counter
                buffer.clear();
                zero_ctr = 0;
            }
        } else {
            break;
        }
    }
    out
}

pub fn recognize_gamma_single(text: &str) -> Option<u32> {
    let o = recognize_gamma(text);
    if o.len() != 1 {
        return None;
    } else {
        return o[0];
    }
}

#[cfg(test)]
mod tests {
    use utils::bits::bits_from_str;

    use super::*;

    #[test]
    fn gamma_codes() {
        let codes = GammaGen::new();
        for ((_, code), check) in codes.zip([
            "1", "010", "011", "00100", "00101", "00110", "00111", "0001000", "0001001",
        ]) {
            assert_eq!(code, check)
        }
    }

    #[test]
    fn gamma_decode_u32() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            gamma_to_u32(&mut bits_from_str("10100110010000101001100011100010000001001").unwrap())
                .unwrap()
        );
    }

    // #[test]
    // fn gamma_decode_u32_str() {
    //     println!(
    //         "{:?}",
    //         recognize_gamma("1010011001000010x1001100011100010000001001")
    //     );
    // }
}
