use itertools::Itertools;

use crate::{errors::CodeError, traits::Code};

// f({}) = 0
// f(A) = sum 2^f(elem_i) for all elem in A
// implies
// f({{}}) = 2^0 = 1
// f({{{}}}) = 2^f({{}}) = 2^1 = 2
// f({{}{{}}}) = 2^f({}) + 2^f({{}}) = 2^0 + 2^1 = 1 + 2 = 3
// f({{{{}}}}) = 2^f({{{}}}) = 2^2 = 4
// f({{}{{{}}}}) = 2^f({}) + 2^f({{{}}}) = 2^0 + 2^2 = 5
// f({{{}}{{{}}}}) = 2^f({{}}) + 2^f({{{}}}) = 2^1 + 2^2 = 6
// f({{}{{}}{{{}}}}) = 2^f({}) = 2^f({{}}) + 2^f({{{}}}) = 2^0 + 2^1 + 2^2 = 7
// f({{{}{{}}}}) = 2^f({{}{{}}}) = 2^3 = 8
// 16 = 2^4 = 2^f({{{{}}}})
// 32 = 2^5 = 2^f({{}{{{}}}})
// 64 = 2^6 = 2^f({{{}}{{{}}}})

const SETS: [&str; 32] = [
    "{}",                                 // 1 = 2^0
    "{{}}",                               // 2 = 2^1
    "{{{}}}",                             // 4 = 2^2
    "{{}{{}}}",                           // 8 = 2^3 = 2^(1+2)
    "{{{{}}}}",                           // 16 = 2^4
    "{{}{{{{}}}}}",                       // 32 = 2^5 = 2^(1+4)
    "{{{}}{{{}}}}",                       // 64 = 2^6 = 2^(2+4)
    "{{}{{}}{{{}}}}",                     // 128 = 2^7 = 2(1+2+4)
    "{{{{{}}}}}",                         // 256 = 2^8
    "{{}{{{{{}}}}}}",                     // 512 = 2^9 = 2^(1+8)
    "{{{}}{{}{{}}}}",                     // 1024 = 2^10 = 2^(2+8)
    "{{}{{}}{{}{{}}}}",                   // 2048 = 2^11 = 2^(1+2+8)
    "{{{{}}}}{{}{{}}}",                   // 4096 = 2^12 = 2^(4+8)
    "{{}{{{}}}}{{}{{}}}",                 // 8192 = 2^13 = 2^(1+4+8)
    "{{{}}{{{}}}{{}{{}}}}",               // 16384 = 2^14 = 2^(2+4+8)
    "{{}{{}}{{{}}}{{}{{}}}}",             // 32768 = 2^15 = 2^(1+2+4+8)
    "{{{{{{}}}}}}",                       //       = 2^16
    "{{}{{{{{{}}}}}}}",                   //       = 2^17 = 2^(1+16)
    "{{{}}{{{{{{}}}}}}}",                 //       = 2^18 = 2^(2+16)
    "{{}{{}}{{{{{{}}}}}}}",               //       = 2^19 = 2^(1+2+16)
    "{{{{}}}{{{{{{}}}}}}}",               //       = 2^20 = 2^(4+16)
    "{{}{{{}}}{{{{{{}}}}}}}",             //       = 2^21 = 2^(1+4+16)
    "{{{}}{{{}}}{{{{{{}}}}}}}",           //       = 2^22 = 2^(2+4+16)
    "{{}{{}}{{{}}}{{{{{{}}}}}}}",         //       = 2^23 = 2^(1+2+4+16)
    "{{{}{{}}}{{{{{{}}}}}}}",             //       = 2^24 = 2^(8+16)
    "{{}{{}{{}}}{{{{{{}}}}}}}",           //       = 2^25 = 2^(1+8+16)
    "{{{}}{{}{{}}}{{{{{{}}}}}}}",         //       = 2^26 = 2^(2+8+16)
    "{{}{{}}{{}{{}}}{{{{{{}}}}}}}",       //       = 2^27 = 2^(1+2+8+16)
    "{{{{}}}{{}{{}}}{{{{{{}}}}}}}",       //       = 2^28 = 2^(4+8+16)
    "{{}{{{}}}{{}{{}}}{{{{{{}}}}}}}",     //       = 2^29 = 2^(1+4+8+16)
    "{{{}}{{{}}}{{}{{}}}{{{{{{}}}}}}}",   //       = 2^30 = 2^(2+4+8+16)
    "{{}{{}}{{{}}}{{}{{}}}{{{{{{}}}}}}}", //       = 2^31 = 2^(1+2+4+8+16)
];

pub fn number_to_set(mut n: u32) -> String {
    let mut out = String::from("{");
    for i in 0..=32 {
        if n & 1 == 1 {
            out.push_str(SETS[i]);
        }
        n >>= 1;
    }
    out.push('}');
    out
}

pub fn valid_parens(s: &str) -> bool {
    let mut lefts = 0;

    for c in s.chars() {
        if c == '{' {
            lefts += 1;
        } else if c == '}' {
            if lefts == 0 {
                return false;
            } else {
                lefts -= 1;
            }
        } else {
            return false;
        }
    }

    lefts == 0
}

pub fn paren_ranges(s: &str) -> Result<Vec<(usize, usize)>, CodeError> {
    let mut starts = Vec::new();
    let mut pairs = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '{' {
            starts.push(i);
        } else if c == '}' {
            if starts.is_empty() {
                return Err(CodeError::input("brackets in the set do not match"));
            } else {
                pairs.push((starts.pop().unwrap(), i + 1));
            }
        } else {
            return Err(CodeError::input("invalid character"));
        }
    }
    Ok(pairs)
}

pub fn paren_ranges_nonoverlapping(s: &str) -> Result<Vec<(usize, usize)>, CodeError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '{' {
            starts.push(i);
        } else if c == '}' {
            if starts.is_empty() {
                return Err(CodeError::input("brackets in the set do not match"));
            } else {
                let pair = (starts.pop().unwrap(), i + 1);
                pairs.retain(|x| x.0 > pair.0);
                pairs.push(pair);
            }
        } else {
            return Err(CodeError::input("invalid character"));
        }
    }

    Ok(pairs)
}

pub fn paren_ranges_nonoverlapping_subsets(s: &str) -> Result<Vec<(usize, usize)>, CodeError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '{' {
            starts.push(i);
        } else if c == '}' {
            if starts.is_empty() {
                return Err(CodeError::input("brackets in the set do not match"));
            } else {
                let pair = (starts.pop().unwrap(), i + 1);
                if pair.0 == 0 {
                    break;
                }
                pairs.retain(|x| x.0 < pair.0 && x.1 < pair.1);
                pairs.push(pair);
            }
        } else {
            return Err(CodeError::input("invalid character"));
        }
    }

    Ok(pairs)
}

pub struct Ackermann {}

impl Default for Ackermann {
    fn default() -> Self {
        Self {}
    }
}

impl Ackermann {}

impl Code for Ackermann {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for num in text.split(",") {
            match u32::from_str_radix(num.trim(), 10) {
                Ok(n) => {
                    out.push(number_to_set(n));
                }
                Err(_) => out.push(String::from("INVALID")),
            }
        }
        Ok(out.join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for set in text.split(",").map(|s| s.trim()) {
            let mut n = 0;
            let ranges = paren_ranges_nonoverlapping_subsets(set)?;
            for range in ranges {
                n += 2_u32.pow(
                    SETS.iter()
                        .position(|x| *x == &set[range.0..range.1])
                        .unwrap() as u32,
                )
            }
            out.push(n.to_string());
        }
        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use super::*;

    #[test]
    fn single_encode() {
        let code = Ackermann::default();

        // println!("{:?}", code.encode(&(u32::MAX).to_string()));
        // println!("{:?}", code.encode(&(23).to_string()));

        let set = "{{}{{}}{{{}}}{{}{{}}}}";
        let pairs = paren_ranges(set).unwrap();
        for p in pairs {
            println!("{:?}", &set[p.0..p.1])
        }
        println!("nonoverlapping");
        let pairs = paren_ranges_nonoverlapping_subsets(set).unwrap();
        for p in pairs {
            println!("{:?} {:?}", p, &set[p.0..p.1])
        }
    }

    #[test]
    fn encode_test() {
        let code = Ackermann::default();
        assert_eq!("", code.encode("0, 1, 2, 3").unwrap())
    }

    #[test]
    fn decode_test() {
        let code = Ackermann::default();
        assert_eq!("", code.decode("{}, {{}}, {{{}}}, {{}{{}}}").unwrap())
    }
}
