use pest::iterators::Pairs;
use pest_derive::Parser;
use unicode_normalization::UnicodeNormalization;

use super::unified_english_braille_maps::{
    ALPHABETIC_WORDSIGNS_MAP, DIACRITIC_MAP, LETTER_MAP, NUMERIC_MAP, PUNCTUATION_MAP, SYMBOL_MAP,
};

#[derive(Parser)]
#[grammar = "braille/ueb.pest"] // relative to src
struct UebParser;

pub fn visualize_tree(pairs: Pairs<'_, Rule>, space: String) {
    for pair in pairs.into_iter() {
        println!("{space}{:?}({})", pair.as_rule(), pair.as_str());
        visualize_tree(pair.into_inner(), format!("{space} "))
    }
}

pub fn decode_passage(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::passage => out.push_str(&decode_passage(pair.into_inner())),
            Rule::WHITESPACE => out.push_str(" "),
            Rule::character => decode_character(pair.into_inner(), &mut out),
            Rule::capital_sequence => decode_capital_sequence(pair.into_inner(), &mut out),
            Rule::capital_passage => decode_capital_passage(pair.into_inner(), &mut out),
            Rule::numeric_sequence => decode_numeric_sequence(pair.into_inner(), &mut out),
            Rule::numeric_passage => decode_numeric_passage(pair.into_inner(), &mut out),
            // Rule::alpha_ws_alone => decode_alphabetic_wordsign(pair.into_inner(), &mut out),
            Rule::unknown => out.push_str(pair.as_str()),
            _ => unreachable!("unexpected Rule in Rule::passage {:?}", pair.as_rule()),
        }
    }
    out.nfc().collect()
}

pub fn decode_character(pairs: Pairs<'_, Rule>, string: &mut String) {
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::letter => decode_letter(pair.into_inner(), string),
            Rule::punctuation => {
                string.push_str(*PUNCTUATION_MAP.get_by_right(pair.as_str()).unwrap())
            }
            Rule::symbol => string.push_str(SYMBOL_MAP.get_by_right(pair.as_str()).unwrap()),
            _ => unreachable!("unexpected Rule in Rule::character {:?}", pair.as_rule()),
        }
    }
}

// pub fn decode_alphabetic_wordsign(pairs: Pairs<'_, Rule>, string: &mut String) {
//     for pair in pairs.into_iter() {
//         match pair.as_rule() {
//             Rule::WHITESPACE => string.push_str(" "),
//             //Rule::spacer => string.push_str(PUNCTUATION_MAP.get_by_right(pair.as_str()).unwrap()),
//             Rule::alphabetic_wordsign => string.push_str(
//                 *ALPHABETIC_WORDSIGNS_MAP
//                     .get_by_right(pair.as_str())
//                     .unwrap(),
//             ),
//             _ => unreachable!(
//                 "unexpected Rule in Rule::alphabetic_wordsign {:?}",
//                 pair.as_rule()
//             ),
//         }
//     }
// }

pub fn decode_numeric_sequence(pairs: Pairs<'_, Rule>, string: &mut String) {
    for pair in pairs.into_iter() {
        if pair.as_rule() == Rule::WHITESPACE {
            continue;
        }
        string.push_str(NUMERIC_MAP.get_by_right(pair.as_str()).unwrap())
    }
}

pub fn decode_numeric_passage(pairs: Pairs<'_, Rule>, string: &mut String) {
    for pair in pairs.into_iter() {
        if pair.as_rule() == Rule::WHITESPACE {
            continue;
        }
        string.push_str(NUMERIC_MAP.get_by_right(pair.as_str()).unwrap())
    }
}

pub fn decode_letter(pairs: Pairs<'_, Rule>, string: &mut String) {
    let mut capital = false;
    let mut diacritics = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::basic_letter => {
                let letter = if capital {
                    LETTER_MAP
                        .get_by_right(&pair.as_str())
                        .unwrap()
                        .to_uppercase()
                } else {
                    LETTER_MAP.get_by_right(&pair.as_str()).unwrap().to_string()
                };
                string.push_str(&letter);
                string.push_str(&diacritics);
            }
            Rule::capitalize => {
                capital = true;
            }
            Rule::diacritic => {
                diacritics.push_str(DIACRITIC_MAP.get_by_right(pair.as_str()).unwrap())
            }
            _ => string.push_str(pair.as_str()),
        }
    }
}

pub fn decode_capital_sequence(pairs: Pairs<'_, Rule>, string: &mut String) {
    let mut diacritics = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::basic_letter => {
                let letter = LETTER_MAP
                    .get_by_right(&pair.as_str())
                    .unwrap()
                    .to_uppercase();
                string.push_str(&letter);
                string.push_str(&diacritics);
                diacritics.clear();
            }
            Rule::diacritic => {
                diacritics.push_str(DIACRITIC_MAP.get_by_right(pair.as_str()).unwrap())
            }
            _ => unreachable!(
                "unexpected Rule in Rule::capital_sequence {:?}",
                pair.as_rule()
            ),
        }
    }
}

pub fn decode_capital_passage(pairs: Pairs<'_, Rule>, string: &mut String) {
    let mut diacritics = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::WHITESPACE => string.push_str(" "),
            Rule::basic_letter => {
                let letter = LETTER_MAP
                                .get_by_right(&pair.as_str())
                                .unwrap()
                                .to_uppercase();
                string.push_str(&letter);
                string.push_str(&diacritics);
                diacritics.clear();
            }
            Rule::diacritic => {
                diacritics.push_str(DIACRITIC_MAP.get_by_right(pair.as_str()).unwrap())
            }
            _ => unreachable!(
                "capital passage should only contain the rules: WHITESPACE, basic_letter, and diacritic; found {:?}", pair.as_rule()
            ),
        }
    }
}

#[cfg(test)]
mod ueb_parser_tests {

    use super::*;

    const TESTS: &[(&'static str, &'static str)] = &[
        // Capitalization
        (
            "ΣAŨB  Xyz CAPITAL PASSAGE WITH SPACES",
            "⠠⠠⠨⠎⠁⠘⠻⠥⠃  ⠠⠭⠽⠵ ⠠⠠⠠⠉⠁⠏⠊⠞⠁⠇⠀⠏⠁⠎⠎⠁⠛⠑⠀⠺⠊⠞⠓⠀⠎⠏⠁⠉⠑⠎⠠⠄",
        ),
        // Diacritics and Greek
        (
            "Étienne! háček Im-Frühling Ω σ",
            "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠖ ⠓⠘⠌⠁⠘⠬⠉⠑⠅ ⠠⠊⠍⠤⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛ ⠠⠨⠺ ⠨⠎",
        ),
        // Numbers
        // Note spaced numeric indicator at start
        (
            "123 1€ = 6.55957₣ 9 7:30 a.m",
            "⠼  ⠁⠃⠉⠀⠼⠁⠈⠑⠀⠐⠶⠀⠼⠋⠲⠑⠑⠊⠑⠛⠈⠋⠀⠼⠊⠀⠼⠛⠒⠼⠉⠚⠀⠁⠲⠍",
        ),
        // // Use wordsigns
        // (
        //     "pizza more people like pizza than will say so",
        //     "⠏⠊⠵⠵⠁  ⠍⠀⠏ ⠇⠀ ⠏⠊⠵⠵⠁⠀⠞⠓⠁⠝ ⠺ ⠎⠁⠽⠀⠎",
        // ),
    ];

    use pest::Parser;

    #[test]
    #[ignore = "parsing experiment"]
    fn parse_tree() {
        for (_sighted, braille) in TESTS.into_iter().copied() {
            let pairs = UebParser::parse(Rule::passage, braille).unwrap();
            visualize_tree(pairs, String::new());
            println!();
        }
    }

    #[test]
    fn decode() {
        for (sighted, braille) in TESTS.into_iter().copied() {
            let pairs = UebParser::parse(Rule::passage, braille).unwrap();
            let decoded = decode_passage(pairs);
            println!("{}", decoded);
            assert_eq!(sighted, decoded)
        }
    }
}
