use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text_standards/ueb.pest"] // relative to src
struct UebParser;

#[cfg(test)]
mod ueb_parser_tests {
    use super::*;
    use pest::iterators::Pairs;
    use pest::Parser;
    pub fn descend(pairs: Pairs<'_, Rule>) {
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::WHITESPACE => println!("WHITESPACE({})", pair.as_str()),
                Rule::basic_letter => println!("basic_letter({})", pair.as_str()),
                Rule::capitalize => println!("capitalize({})", pair.as_str()),
                Rule::other_modifier => println!("other_modifier({})", pair.as_str()),
                Rule::modifier => {
                    println!("modifier({})", pair.as_str());
                    descend(pair.into_inner())
                }
                Rule::letter => {
                    println!("letter({})", pair.as_str());
                    descend(pair.into_inner())
                }
                Rule::letter_sequence => {
                    println!("letter_sequence({})", pair.as_str());
                    descend(pair.into_inner())
                }
                Rule::passage => {
                    println!("passage({})", pair.as_str());
                    descend(pair.into_inner())
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
        descend(pairs)
    }
}
