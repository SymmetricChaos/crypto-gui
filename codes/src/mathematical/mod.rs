use crate::errors::CodeError;

pub mod arithmetic;
pub mod balanced_ternary;
pub mod base_n;
pub mod base_n_bijective;
pub mod base_negative_two;
pub mod binary_coded_decimal;
pub mod biquinary_decimal;
pub mod combinadic;
pub mod elias;
pub mod exp_golomb;
pub mod factoradic;
pub mod fibonacci;
pub mod fibonacci_integers;
pub mod godel;
pub mod golomb;
pub mod gray;
pub mod leb128;
pub mod levenshtein;
pub mod negative_base_n;
pub mod primorial;
pub mod roman_numeral;
pub mod truncated_binary;
pub mod twos_complement;
pub mod unary;

/// TODO: This should be improvable to a single point of failure like its inverse
pub fn u32_to_i32_zigzag(n: u32) -> Option<i32> {
    if let Ok(x) = TryInto::<i32>::try_into(n) {
        if x % 2 == 0 {
            Some(x / 2)
        } else {
            Some((-(x) - 1) / 2)
        }
    } else {
        None
    }
}

pub fn i32_to_u32_zigzag(n: i32) -> Option<u32> {
    if n == i32::MIN {
        return None;
    }
    if n.is_negative() {
        Some((n.abs() as u32 * 2) - 1)
    } else {
        Some((n.abs() as u32 * 2) as u32)
    }
}

pub(super) fn string_to_u32s(s: &str, sep: &str) -> Result<Vec<u32>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n =
            u32::from_str_radix(group, 10).map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

pub(super) fn string_to_i32s(s: &str, sep: &str) -> Result<Vec<i32>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n =
            i32::from_str_radix(group, 10).map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

pub(super) fn string_to_u64s(s: &str, sep: &str) -> Result<Vec<u64>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n =
            u64::from_str_radix(group, 10).map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

// pub(super) fn string_to_i64s(s: &str, sep: &str) -> Result<Vec<i64>, CodeError> {
//     let mut out = Vec::new();
//     for group in s.split(sep).map(|x| x.trim()) {
//         if group.is_empty() {
//             continue;
//         }
//         let n =
//             i64::from_str_radix(group, 10).map_err(|_| CodeError::invalid_input_group(group))?;
//         out.push(n);
//     }
//     Ok(out)
// }

// pub(super) fn string_to_usizes(s: &str, sep: &str) -> Result<Vec<usize>, CodeError> {
//     let mut out = Vec::new();
//     for group in s.split(sep).map(|x| x.trim()) {
//         if group.is_empty() {
//             continue;
//         }
//         let n =
//             usize::from_str_radix(group, 10).map_err(|_| CodeError::invalid_input_group(group))?;
//         out.push(n);
//     }
//     Ok(out)
// }

#[cfg(test)]
mod zig_zag_tests {
    use super::*;

    #[test]
    fn to_u32() {
        for i in i32::MIN..=i32::MAX {
            i32_to_u32_zigzag(i);
        }
    }

    #[test]
    fn to_i32() {
        for i in u32::MIN..=u32::MAX {
            u32_to_i32_zigzag(i);
        }
    }
}
