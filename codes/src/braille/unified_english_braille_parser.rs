use pest::iterators::Pairs;
use pest_derive::Parser;
use unicode_normalization::UnicodeNormalization;

use super::unified_english_braille_maps::{
    DIACRITIC_MAP, LETTER_MAP, NUMERIC_MAP, PUNCTUATION_MAP, SYMBOL_MAP,
};

#[derive(Parser)]
#[grammar = "braille/ueb.pest"] // relative to src
struct UebParser;

pub fn descend(pairs: Pairs<'_, Rule>, space: String) {
    for pair in pairs.into_iter() {
        let mut space = space.clone();
        space.push(' ');
        match pair.as_rule() {
            Rule::WHITESPACE => println!("{space}WHITESPACE({})", pair.as_str()),
            Rule::basic_letter => println!("{space}basic_letter({})", pair.as_str()),
            Rule::capitalize => println!("{space}capitalize({})", pair.as_str()),
            Rule::number => println!("{space}letter({})", pair.as_str()),
            Rule::letter => println!("{space}letter({})", pair.as_str()),
            Rule::character => println!("{space}character({})", pair.as_str()),
            Rule::symbol => println!("{space}symbol({})", pair.as_str()),
            Rule::numeric_symbol => println!("{space}numeric_symbols({})", pair.as_str()),
            Rule::punctuation => println!("{space}punctuation({})", pair.as_str()),
            Rule::passage => println!("{space}passage({})", pair.as_str()),
            Rule::diacritic => println!("{space}diacritic({})", pair.as_str()),
            Rule::capital_sequence => println!("{space}capital_sequence({})", pair.as_str()),
            Rule::unknown => println!("{space}unknown({})", pair.as_str()),
        }

        descend(pair.into_inner(), space)
    }
}

pub fn decode_passage(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::passage => out.push_str(&decode_passage(pair.into_inner())),
            Rule::WHITESPACE => out.push_str(" "),
            Rule::character =>
                decode_character(pair.into_inner(), &mut out),
            Rule::capital_sequence => decode_capital_sequence(pair.into_inner(), &mut out),
            Rule::unknown => out.push_str(pair.as_str()),
            _ => unreachable!(
                "a passage consists only of WHITESPACE, unknown, character, and capital_sequence at the top level"
            ),
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
            Rule::number => decode_number(pair.into_inner(), string),
            Rule::symbol => string.push_str(SYMBOL_MAP.get_by_right(pair.as_str()).unwrap()),
            _ => unreachable!("characters are only: letter, number, symbol and punctuation"),
        }
    }
}

pub fn decode_number(pairs: Pairs<'_, Rule>, string: &mut String) {
    for pair in pairs.into_iter() {
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
                "capital sequence should only contain the rules: basic_letter, and diacritic, found {:?}", pair.as_rule()
            ),
        }
    }
}

#[cfg(test)]
mod ueb_parser_tests {

    use super::*;

    const TEXT: &'static str =
        "Étienne! 123 háček 9 ΣAŨB  Xyz 13% Im-Frühling Ω σ 7:30 a.m. 1 € = 6.55957₣";
    const BRAILLE: &'static str =
        "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠖ ⠼⠁⠃⠉⠀⠓⠘⠌⠁⠘⠬⠉⠑⠅ ⠼⠊ ⠠⠠⠨⠎⠁⠘⠻⠥⠃  ⠠⠭⠽⠵ 13%⠀⠠⠊⠍⠤⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛ ⠠⠨⠺ ⠨⠎ ⠼⠛⠒⠼⠉⠚ ⠁⠲⠍⠲ ⠼⠁ ⠈⠑ ⠐⠶ ⠼⠋⠲⠑⠑⠊⠑⠛⠈⠋";

    use pest::Parser;

    #[test]
    #[ignore = "parsing experiment"]
    fn parse_tree() {
        let pairs = UebParser::parse(Rule::passage, BRAILLE).unwrap();
        descend(pairs, String::new());
    }

    #[test]
    #[ignore = "decoding experiment"]
    fn decode() {
        let pairs = UebParser::parse(Rule::passage, BRAILLE).unwrap();
        let decoded = decode_passage(pairs);
        println!("{}", decoded);
        assert_eq!(TEXT, decoded)
    }
}
