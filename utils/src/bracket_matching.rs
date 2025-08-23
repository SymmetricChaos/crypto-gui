use std::ops::Range;

use itertools::Itertools;

use crate::errors::GeneralError;

/// Start and end points of every matching pair of parentheses
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

/// Start and end points of every matching pair of parentheses. Accepts multiple kinds of parentheses.
pub fn paren_ranges_multi(s: &str, p: &[(char, char)]) -> Result<Vec<Range<usize>>, GeneralError> {
    let mut starts = Vec::new();
    let mut pairs: Vec<Range<usize>> = Vec::new();

    // Unzip the pairs for easy matching
    let l = p.iter().map(|x| x.0).collect_vec();
    let r = p.iter().map(|x| x.1).collect_vec();

    for (i, c) in s.chars().enumerate() {
        if l.contains(&c) {
            starts.push((i, c));
        } else if r.contains(&c) {
            if starts.is_empty() {
                return Err(GeneralError::input("parentheses are not correctly matched"));
            } else {
                let m = starts.pop().unwrap();
                if p.contains(&(m.1, c)) {
                    let r = Range {
                        start: m.0,
                        end: i + 1,
                    };
                    pairs.push(r);
                } else {
                    return Err(GeneralError::input("parentheses are not correctly matched"));
                }
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

/// Start and end points of the largest nonoverlapping subsets of a single set. Assumes that in the input is a single set such that the first open paren matches the final closed paren
/// For example {{}{{}{{{}}}}} is a valid input but {}{{}{{{}}}} is not
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
                let r = Range {
                    start: starts.pop().unwrap(),
                    end: i + 1,
                };
                if r.start == 0 {
                    break;
                }
                pairs.retain(|x| x.start < r.start && x.end < r.end);
                pairs.push(r);
            }
        } else {
            return Err(GeneralError::input("invalid character"));
        }
    }
    if !starts.is_empty() {
        return Err(GeneralError::input("parentheses are not correctly matched"));
    }

    Ok(pairs)
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn sets() {
//         let set = "{}{{{}}{}}";
//         for range in paren_ranges(set, '{', '}').unwrap() {
//             let s = range.start;
//             let e = range.end;
//             println!("{} [{}..{}]", &set[range], s, e)
//         }
//     }
// }
