use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text_standards/ueb.pest"] // relative to src
struct UebParser;

#[cfg(test)]
mod ueb_parser_tests {
    use super::*;
    use pest::iterators::Pairs;
    use pest::Parser;
    pub fn descend(pairs: Pairs<'_, Rule>, space: String) {
        for pair in pairs.into_iter() {
            let mut space = space.clone();
            space.push(' ');
            match pair.as_rule() {
                Rule::WHITESPACE => println!("{space}WHITESPACE({})", pair.as_str()),
                Rule::basic_letter => println!("{space}basic_letter({})", pair.as_str()),
                Rule::capitalize => println!("{space}capitalize({})", pair.as_str()),
                Rule::other_modifier => println!("{space}other_modifier({})", pair.as_str()),
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
            }
        }
    }

    //Étienne háček Im Frühling
    const TEXT: &'static str = "⠠⠘⠌⠑⠞⠊⠑⠝⠝⠑⠀⠓⠘⠌⠁⠘⠬⠉⠑⠅⠀⠠⠊⠍⠀⠠⠋⠗⠘⠒⠥⠓⠇⠊⠝⠛";

    #[test]
    #[ignore = "parsing experiment"]
    fn example_parse() {
        let pairs = UebParser::parse(Rule::passage, TEXT).unwrap();
        descend(pairs, String::new())
    }
}
