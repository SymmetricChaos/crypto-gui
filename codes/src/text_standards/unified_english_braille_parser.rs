use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "text_standards/ueb.pest"] // relative to src
struct UebParser;

#[cfg(test)]
mod ueb_parser_tests {
    use super::*;

    const TEXT: &'static str = "⠠⠞⠓⠑ ⠠⠟⠥⠊⠉⠅";

    #[test]
    fn parse() {
        for x in UebParser::parse(Rule::passage, TEXT).into_iter() {
            let y = x.flatten();
            for a in y.into_iter() {
                println!("{:?} {}", a.as_rule(), a.as_str())
            }
        }
    }
}
