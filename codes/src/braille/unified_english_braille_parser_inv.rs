use super::unified_english_braille_maps::{
    DIACRITIC_MAP, LETTER_MAP, LETTER_UPPER_MAP, NUMERIC_MAP, PUNCTUATION_MAP, SYMBOL_MAP,
};
use pest::iterators::Pairs;
use pest_derive::Parser;
use unicode_normalization::UnicodeNormalization;

#[derive(Parser)]
#[grammar = "braille/ueb_inv.pest"] // relative to src
struct UebInvParser;

pub fn visualize_tree(pairs: Pairs<'_, Rule>, space: String) {
    for pair in pairs.into_iter() {
        println!("{space}{:?}({})", pair.as_rule(), pair.as_str());
        visualize_tree(pair.into_inner(), format!("{space} "))
    }
}

pub fn encode_g1_braille(text: &str) -> String {
    use pest::Parser;
    let decomposed: String = text.nfd().collect();
    let pairs = UebInvParser::parse(Rule::g1_passage, &decomposed).unwrap();
    encode_passage(pairs)
}

pub fn encode_passage(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::g1_passage => out.push_str(&encode_passage(pair.into_inner())),
            Rule::WHITESPACE => out.push_str("⠀"),
            Rule::upper_letter => {
                out.push('⠠');
                encode_letter(pair.into_inner(), &mut out)
            }
            Rule::lower_letter => encode_letter(pair.into_inner(), &mut out),
            Rule::punctuation => out.push_str(PUNCTUATION_MAP.get_by_left(pair.as_str()).unwrap()),
            Rule::symbol => out.push_str(SYMBOL_MAP.get_by_left(pair.as_str()).unwrap()),
            Rule::capital_sequence => encode_capital_sequence(pair.into_inner(), &mut out),
            Rule::numeric_sequence => encode_numeric_sequence(pair.into_inner(), &mut out),
            Rule::unknown => out.push_str(pair.as_str()),
            _ => unreachable!("unexpected Rule in Rule::g1_passage {:?}", pair.as_rule()),
        }
    }
    out.nfc().collect()
}

pub fn encode_numeric_sequence(pairs: Pairs<'_, Rule>, string: &mut String) {
    string.push('⠼');
    for pair in pairs.into_iter() {
        string.push_str(NUMERIC_MAP.get_by_left(pair.as_str()).unwrap())
    }
}

pub fn encode_letter(pairs: Pairs<'_, Rule>, string: &mut String) {
    let mut letter = String::new();
    let mut diacritics = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::basic_upper_letter => {
                letter.push_str(LETTER_UPPER_MAP.get_by_left(pair.as_str()).unwrap())
            }
            Rule::basic_lower_letter => {
                letter.push_str(LETTER_MAP.get_by_left(pair.as_str()).unwrap())
            }
            Rule::diacritic => {
                diacritics.push_str(DIACRITIC_MAP.get_by_left(pair.as_str()).unwrap())
            }
            _ => unreachable!("unexpected Rule in Rule::letter {:?}", pair.as_rule()),
        }
    }
    string.push_str(&diacritics);
    string.push_str(&letter);
}

pub fn encode_capital_sequence(pairs: Pairs<'_, Rule>, string: &mut String) {
    let mut letter = String::new();
    let mut diacritics = String::new();
    string.push_str("⠠⠠");
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::upper_letter => encode_letter(pair.into_inner(), string),
            Rule::diacritic => {
                diacritics.push_str(DIACRITIC_MAP.get_by_left(pair.as_str()).unwrap())
            }
            _ => unreachable!(
                "unexpected Rule in Rule::capital_sequence {:?}",
                pair.as_rule()
            ),
        }
        string.push_str(&diacritics);
        string.push_str(&letter);
        diacritics.clear();
        letter.clear();
    }
}

#[cfg(test)]
mod ueb_parser_tests {

    use super::*;

    const TESTS: &[(&'static str, &'static str)] = &[
        // Diacritics and Greek
        (
            "Étienne! háček Im-Frühling Ω σ",
            "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠖⠀⠓⠘⠌⠁⠘⠬⠉⠑⠅⠀⠠⠊⠍⠤⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛⠀⠠⠨⠺⠀⠨⠎",
        ),
        // Capitalization
        ("ΣAŨB  Xyz", "⠠⠠⠨⠎⠁⠘⠻⠥⠃⠀⠀⠠⠭⠽⠵"),
        // Numbers
        (
            "123 1€ = 6.55957₣ 9 7:30 a.m.",
            "⠼⠁⠃⠉⠀⠼⠁⠈⠑⠀⠐⠶⠀⠼⠋⠲⠑⠑⠊⠑⠛⠈⠋⠀⠼⠊⠀⠼⠛⠒⠼⠉⠚⠀⠁⠲⠍⠲",
        ),
    ];

    use pest::Parser;

    #[test]
    #[ignore = "parsing experiment"]
    fn parse_tree() {
        for (print, _braille) in TESTS.into_iter().copied() {
            let decomposed: String = print.nfd().collect();
            let pairs = UebInvParser::parse(Rule::g1_passage, &decomposed).unwrap();
            visualize_tree(pairs, String::new());
        }
    }

    #[test]
    fn encode() {
        for (print, braille) in TESTS.into_iter().copied() {
            let encoded = encode_g1_braille(print);
            println!("{}", encoded);
            assert_eq!(braille, encoded)
        }
    }
}
