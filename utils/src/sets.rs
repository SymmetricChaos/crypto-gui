use crate::errors::GeneralError;

pub fn paren_ranges_nonoverlapping_subsets(s: &str) -> Result<Vec<(usize, usize)>, GeneralError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '{' {
            starts.push(i);
        } else if c == '}' {
            if starts.is_empty() {
                return Err(GeneralError::input("brackets in the set do not match"));
            } else {
                let pair = (starts.pop().unwrap(), i + 1);
                if pair.0 == 0 {
                    break;
                }
                pairs.retain(|x| x.0 < pair.0 && x.1 < pair.1);
                pairs.push(pair);
            }
        } else {
            return Err(GeneralError::input("invalid character"));
        }
    }

    Ok(pairs)
}
