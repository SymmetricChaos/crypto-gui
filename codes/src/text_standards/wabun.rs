use super::morse_encodings::*;
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "text_standards/wabun.pest"] // relative to src
pub struct WabunParser;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WabunRep {
    HalfBlock,
    Ascii,
    Word,
}

impl WabunRep {
    pub fn letter_sep(&self) -> &str {
        match self {
            Self::Ascii => " ",
            Self::HalfBlock => "   ",
            Self::Word => "   ",
        }
    }

    pub fn word_sep(&self) -> &str {
        match self {
            Self::Ascii => "   ",
            Self::HalfBlock => "       ",
            Self::Word => "       ",
        }
    }

    pub fn map(&self) -> &BiMap<&str, &str> {
        match self {
            Self::HalfBlock => &WABUN_ASCII_MAP,
            Self::Ascii => &WABUN_ASCII_MAP,
            Self::Word => &WABUN_ASCII_MAP,
        }
    }
}

pub struct Wabun {
    pub representation: WabunRep,
}

impl Wabun {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        match self.representation {
            WabunRep::HalfBlock => Box::new(HIRAGANA.into_iter().zip(WABUN_HALFBLOCK)),
            WabunRep::Ascii => Box::new(HIRAGANA.into_iter().zip(WABUN_ASCII)),
            WabunRep::Word => Box::new(HIRAGANA.into_iter().zip(WABUN_WORD)),
        }
    }
}

impl Default for Wabun {
    fn default() -> Self {
        Self {
            representation: WabunRep::HalfBlock,
        }
    }
}

impl Code for Wabun {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let map = self.representation.map();
        let mut out = Vec::new();
        for pair in WabunParser::parse(Rule::kana_passage, text)
            .unwrap()
            .into_iter()
            .flatten()
        {
            match pair.as_rule() {
                // Rule::unknown => out.push(pair.as_str()),
                Rule::kata | Rule::hira => out.push(*map.get_by_left(pair.as_str()).unwrap()),
                Rule::space => out.push(" "),
                _ => (),
            }
        }

        Ok(out.join(self.representation.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let map = self.representation.map();
        let rule = match self.representation {
            // WabunRep::HalfBlock => Rule::halfblock_passage,
            WabunRep::Ascii => Rule::ascii_passage,
            // WabunRep::Word => Rule::word_passage,
            _ => todo!("halfblack and word"),
        };
        for pair in WabunParser::parse(rule, text)
            .unwrap()
            .into_iter()
            .flatten()
        {
            println!("{}", pair.as_str());
            match pair.as_rule() {
                // Rule::unknown => out.push(pair.as_str()),
                Rule::ascii_kana => {
                    println!("{:?}", map.get_by_left(pair.as_str()));
                    out.push(*map.get_by_left(pair.as_str()).unwrap())
                }
                Rule::space => out.push(" "),
                _ => (),
            }
        }

        Ok(out.join(" "))
    }
}

#[cfg(test)]
mod wabun_tests {
    use super::*;

    const KANA: &'static str = "ひらがな にゃん";
    const ASCII: &'static str = "--..- ... .-.. .. .-.   -.-. .-- .-.-.";
    // const WORD: &'static str = "";

    fn visualize_tree(pairs: pest::iterators::Pairs<'_, Rule>, space: String) {
        for pair in pairs.into_iter() {
            println!("{space}{:?}({})", pair.as_rule(), pair.as_str());
            visualize_tree(pair.into_inner(), format!("{space} "))
        }
    }

    #[test]
    fn tree() {
        visualize_tree(
            WabunParser::parse(Rule::kana_passage, KANA).unwrap(),
            String::new(),
        );
        visualize_tree(
            WabunParser::parse(Rule::ascii_passage, ASCII).unwrap(),
            String::new(),
        );
    }

    #[test]
    fn encode_test_ascii() {
        // visualize_tree(
        //     WabunParser::parse(Rule::kana_passage, KANA).unwrap(),
        //     String::new(),
        // );
        let mut code = Wabun::default();
        code.representation = WabunRep::Ascii;
        assert_eq!(code.encode(KANA).unwrap(), ASCII);
    }

    #[test]
    fn decode_test_ascii() {
        visualize_tree(
            WabunParser::parse(Rule::ascii_passage, ASCII).unwrap(),
            String::new(),
        );
        let mut code = Wabun::default();
        code.representation = WabunRep::Ascii;
        assert_eq!(code.decode(ASCII).unwrap(), KANA);
    }
}
