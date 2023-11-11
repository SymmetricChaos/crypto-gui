use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text_standards/ueb.pest"] // relative to src
struct UebParser;

#[cfg(test)]
mod ueb_parser_tests {
    use crate::text_standards::unified_english_braille_maps::{
        DIACRITIC_MAP, GREEK_MAP, LETTER_MAP, PUNCTUATION_MAP,
    };

    use super::*;
    use pest::iterators::{Pair, Pairs};
    use pest::Parser;
    use unicode_normalization::UnicodeNormalization;
    pub fn descend(pairs: Pairs<'_, Rule>, space: String) {
        for pair in pairs.into_iter() {
            let mut space = space.clone();
            space.push(' ');
            match pair.as_rule() {
                Rule::WHITESPACE => println!("{space}WHITESPACE({})", pair.as_str()),
                Rule::basic_letter => println!("{space}basic_letter({})", pair.as_str()),
                Rule::capitalize => println!("{space}capitalize({})", pair.as_str()),
                Rule::letter => {
                    println!("{space}letter({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::character => {
                    println!("{space}character({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::punctuation => {
                    println!("{space}punctuation({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::passage => {
                    println!("{space}passage({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::greek => println!("{space}greek({})", pair.as_str()),
                Rule::diacritic => println!("{space}diacritic({})", pair.as_str()),
            }
        }
    }

    pub fn decode_passage(pairs: Pairs<'_, Rule>) -> String {
        let mut out = String::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::passage => out.push_str(&decode_passage(pair.into_inner())),
                Rule::WHITESPACE => out.push_str(pair.as_str()),
                Rule::character => {
                    decode_character(pair.into_inner(), &mut out);
                }
                _ => unreachable!(
                    "a passage consists only of WHITESPACE and letter_sequence at the top level"
                ),
            }
        }
        out.nfc().collect()
    }

    // pub fn decode_letter_sequence(pairs: Pairs<'_, Rule>, string: &mut String) {
    //     for letter in pairs.into_iter() {
    //         string.push_str(&decode_letter(letter.into_inner()))
    //     }
    // }

    pub fn decode_character(pairs: Pairs<'_, Rule>, string: &mut String) {
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::letter => decode_letter(pair.into_inner(), string),
                Rule::punctuation => {
                    string.push(*PUNCTUATION_MAP.get_by_right(pair.as_str()).unwrap())
                }
                _ => unreachable!("characters are only: letter and punctuation"),
            }
        }
    }

    pub fn decode_letter(pairs: Pairs<'_, Rule>, string: &mut String) {
        let mut capital = false;
        let mut greek = false;
        let mut diacritics = String::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::basic_letter => {
                    let letter = if capital {
                        if greek {

                                GREEK_MAP
                                    .get_by_right(pair.as_str())
                                    .unwrap()
                                    .to_uppercase()
                        } else {

                                LETTER_MAP
                                    .get_by_right(&pair.as_str().chars().next().unwrap())
                                    .unwrap()
                                    .to_uppercase().to_string()
                        }
                    } else {
                        if greek {
                            GREEK_MAP.get_by_right(pair.as_str()).unwrap().to_string()
                        } else {
                                LETTER_MAP
                                    .get_by_right(&pair.as_str().chars().next().unwrap())
                                    .unwrap().to_string()
                        }
                    };
                    string.push_str(&letter);
                    string.push_str(&diacritics);
                }
                Rule::capitalize => {
                    capital = true;
                }
                Rule::greek => {
                    greek = true;
                }
                Rule::diacritic => {
                    diacritics.push_str(DIACRITIC_MAP.get_by_right(pair.as_str()).unwrap())
                }
                _ => unreachable!(
                    "letters should only contain the rules: basic_letter, capitalize, greek, and diacritic"
                ),
            }
        }
    }

    //Étienne! háček Im-Frühling Ω σ
    const TEXT: &'static str = "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠖⠀⠓⠘⠌⠁⠘⠬⠉⠑⠅⠀⠠⠊⠍⠤⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛ ⠠⠨⠺ ⠨⠎";

    #[test]
    #[ignore = "parsing experiment"]
    fn example_parse() {
        let pairs = UebParser::parse(Rule::passage, TEXT).unwrap();
        descend(pairs.clone(), String::new());
        println!("{}", decode_passage(pairs));
    }
}
