use std::ops::Range;

use crate::errors::GeneralError;

// Start and end points of every matching pair of parentheses
pub fn paren_ranges(s: &str, l: char, r: char) -> Result<Vec<Range<usize>>, GeneralError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<Range<usize>> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == l {
            starts.push(i);
        } else if c == r {
            if starts.is_empty() {
                return Err(GeneralError::input("parentheses are not correctly matched"));
            } else {
                let r = Range {
                    start: starts.pop().unwrap(),
                    end: i + 1,
                };
                pairs.push(r);
            }
        } else {
            continue;
        }
    }
    if !starts.is_empty() {
        return Err(GeneralError::input("parentheses are not correctly matched"));
    }

    Ok(pairs)
}

// Start and end points of the largest nonoverlapping subsets of a single set
pub fn paren_ranges_nonoverlapping_subsets(
    s: &str,
    l: char,
    r: char,
) -> Result<Vec<Range<usize>>, GeneralError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<Range<usize>> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == l {
            starts.push(i);
        } else if c == r {
            if starts.is_empty() {
                return Err(GeneralError::input("parentheses are not correctly matched"));
            } else {
                let pair = (starts.pop().unwrap(), i + 1);
                if pair.0 == 0 {
                    break;
                }
                pairs.retain(|x| x.start < pair.0 && x.end < pair.1);
                let r = Range {
                    start: starts.pop().unwrap(),
                    end: i + 1,
                };
                pairs.push(r);
            }
        } else {
            return Err(GeneralError::input("invalid character"));
        }
    }

    Ok(pairs)
}
