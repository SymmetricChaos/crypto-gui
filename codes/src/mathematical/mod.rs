use crate::errors::CodeError;

pub mod arithmetic;
pub mod balanced_ternary;
pub mod base_n;
pub mod base_n_bijective;
pub mod base_negative_two;
pub mod biquinary_decimal;
pub mod combinadic;
pub mod elias;
pub mod elias_integers;
pub mod factoradic;
pub mod fibonacci;
pub mod fibonacci_integers;
pub mod godel;
pub mod gray;
pub mod leb128;
pub mod levenshtein;
pub mod levenshtein_integers;
pub mod negative_base_n;
pub mod primorial;
pub mod roman_numeral;
pub mod symmetric_unary;
pub mod twos_complement;
pub mod unary;

pub(crate) fn string_to_u32s(s: &str, sep: &str) -> Result<Vec<u32>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n = u32::from_str_radix(group.trim(), 10)
            .map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

pub(crate) fn string_to_i32s(s: &str, sep: &str) -> Result<Vec<i32>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n = i32::from_str_radix(group.trim(), 10)
            .map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

pub(crate) fn string_to_u64s(s: &str, sep: &str) -> Result<Vec<u64>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n = u64::from_str_radix(group.trim(), 10)
            .map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}

pub(crate) fn string_to_usizes(s: &str, sep: &str) -> Result<Vec<usize>, CodeError> {
    let mut out = Vec::new();
    for group in s.split(sep).map(|x| x.trim()) {
        if group.is_empty() {
            continue;
        }
        let n = usize::from_str_radix(group.trim(), 10)
            .map_err(|_| CodeError::invalid_input_group(group))?;
        out.push(n);
    }
    Ok(out)
}
