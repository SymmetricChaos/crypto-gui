use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text_standards/ueb.pest"] // relative to src
struct UebParser;

#[cfg(test)]
mod ueb_parser_tests {
    use crate::text_standards::unified_english_braille_maps::{
        DIACRITIC_MAP, GREEK_MAP, LETTER_MAP,
    };

    use super::*;
    use pest::iterators::Pairs;
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
                // Rule::other_modifier => println!("{space}other_modifier({})", pair.as_str()),
                Rule::modifier => {
                    println!("{space}modifier({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::letter => {
                    println!("{space}letter({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::letter_sequence => {
                    println!("{space}letter_sequence({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::passage => {
                    println!("{space}passage({})", pair.as_str());
                    descend(pair.into_inner(), space)
                }
                Rule::prefix => println!("{space}prefix({})", pair.as_str()),
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
                Rule::letter_sequence => {
                    out.push_str(&decode_letter_sequence(pair.into_inner()));
                }
                _ => unreachable!(
                    "a passage consists only of WHITESPACE and letter_sequence at the top level"
                ),
            }
        }
        out
    }

    pub fn decode_letter_sequence(pairs: Pairs<'_, Rule>) -> String {
        let mut out = String::new();
        for letter in pairs.into_iter() {
            out.push_str(&decode_letter(letter.into_inner()))
        }
        out
    }

    pub fn decode_letter(pairs: Pairs<'_, Rule>) -> String {
        let mut capital = false;
        let mut greek = false;
        let mut diacritics = String::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::basic_letter => {
                    let mut letter = if capital {
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
                    letter.push_str(&diacritics);
                    return letter.nfc().collect();
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

        unreachable!("matches should be exhaustive")
    }

    //Étienne háček Im Frühling Ω σ
    const TEXT: &'static str = "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠀⠓⠘⠌⠁⠘⠬⠉⠑⠅⠀⠠⠊⠍⠀⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛ ⠠⠨⠺ ⠨⠎";

    #[test]
    #[ignore = "parsing experiment"]
    fn example_parse() {
        let pairs = UebParser::parse(Rule::passage, TEXT).unwrap();
        descend(pairs.clone(), String::new());
        println!("{}", decode_passage(pairs));
    }
}
