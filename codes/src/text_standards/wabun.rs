use std::collections::HashMap;

use crate::{errors::CodeError, traits::Code};

use lazy_static::lazy_static;
use regex::Regex;

use super::wabun_encoding::{
    ASCII_TO_HIRA, HALFBLOCK_TO_HIRA, HIRAGANA, KANA_TO_ASCII, KANA_TO_HALFBLOCK, KANA_TO_WORD,
    WABUN_ASCII, WABUN_HALFBLOCK, WABUN_WORD, WORD_TO_HIRA,
};

lazy_static! {
    // Just a list of all kana combinations, Japanese punctuation, and two spaces commonly used
    pub static ref WABUN_KANA_REGEX: Regex = Regex::new(r"(っ|ッ|キャ|キュ|キョ|シャ|シュ|ショ|チャ|チュ|チョ|ニャ|ニュ|ニョ|ヒャ|ヒュ|ヒョ|ミャ|ミュ|ミョ|リャ|リュ|リョ|ギャ|ギュ|ギョ|ジャ|ジュ|ジョ|ヂャ|ヂュ|ヂョ|ビャ|ビュ|ビョ|ピャ|ピュ|ピョ|ア|イ|ウ|エ|オ|カ|キ|ク|ケ|コ|サ|シ|ス|セ|ソ|タ|チ|ツ|テ|ト|ナ|ニ|ヌ|ネ|ノ|ハ|ヒ|フ|ヘ|ホ|マ|ミ|ム|メ|モ|ヤ|ユ|ヨ|ラ|リ|ル|レ|ロ|ワ|ヰ|ヱ|ヲ|ン|ガ|ギ|グ|ゲ|ゴ|ザ|ジ|ズ|ゼ|ゾ|ダ|ヂ|ヅ|デ|ド|バ|ビ|ブ|ベ|ボ|パ|ピ|プ|ペ|ポ|きゃ|きゅ|きょ|しゃ|しゅ|しょ|ちゃ|ちゅ|ちょ|にゃ|にゅ|にょ|ひゃ|ひゅ|ひょ|みゃ|みゅ|みょ|りゃ|りゅ|りょ|ぎゃ|ぎゅ|ぎょ|じゃ|じゅ|じょ|ぢゃ|ぢゅ|ぢょ|びゃ|びゅ|びょ|ぴゃ|ぴゅ|ぴょ|あ|い|う|え|お|か|き|く|け|こ|さ|し|す|せ|そ|た|ち|つ|て|と|な|に|ぬ|ね|の|は|ひ|ふ|へ|ほ|ま|み|む|め|も|や|ゆ|よ|ら|り|る|れ|ろ|わ|ゐ|ゑ|を|ん|が|ぎ|ぐ|げ|ご|ざ|じ|ず|ぜ|ぞ|だ|ぢ|づ|で|ど|ば|び|ぶ|べ|ぼ|ぱ|ぴ|ぷ|ぺ|ぽ|、|。|ー|（|）|゛|゜)| |　|.").unwrap();
    // A valid Wabun code is a kana, followed by optionally one of two diacritic codes, followed optionally by one of three small kana codes, followed by a space of end of input
    pub static ref WABUN_ASCII_REGEX: Regex =
        Regex::new(r"([-\.]+( \.--| -\.\.--| --)?( \.\.| \.\.--\.)?)( |$)").unwrap();
    pub static ref WABUN_HALFBLOCK_REGEX: Regex = Regex::new(
        r"((▄ ▄▄▄ ▄ ▄▄▄ ▄|▄▄▄ ▄ ▄▄▄ ▄ ▄▄▄|▄▄▄ ▄ ▄ ▄ ▄▄▄|▄▄▄ ▄ ▄ ▄▄▄ ▄|▄▄▄ ▄ ▄ ▄▄▄ ▄▄▄|▄▄▄ ▄▄▄ ▄▄▄ ▄ ▄▄▄|▄ ▄ ▄▄▄ ▄ ▄▄▄|▄ ▄ ▄▄▄ ▄ ▄|▄ ▄▄▄ ▄ ▄▄▄ ▄▄▄|▄▄▄ ▄▄▄ ▄ ▄ ▄▄▄|▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄|▄▄▄ ▄▄▄ ▄ ▄▄▄ ▄|▄ ▄▄▄ ▄▄▄ ▄ ▄|▄ ▄▄▄ ▄ ▄ ▄▄▄|▄▄▄ ▄ ▄▄▄ ▄ ▄|▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄|▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄▄▄|▄ ▄▄▄ ▄ ▄ ▄|▄ ▄▄▄ ▄▄▄ ▄|▄ ▄ ▄▄▄ ▄|▄▄▄ ▄▄▄ ▄▄▄ ▄|▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄|▄▄▄ ▄ ▄▄▄ ▄▄▄|▄▄▄ ▄ ▄▄▄ ▄|▄ ▄ ▄ ▄▄▄|▄ ▄▄▄ ▄ ▄|▄▄▄ ▄▄▄ ▄ ▄▄▄|▄▄▄ ▄ ▄ ▄▄▄|▄ ▄▄▄ ▄▄▄ ▄▄▄|▄ ▄ ▄ ▄|▄ ▄▄▄ ▄ ▄▄▄|▄ ▄▄▄ ▄ ▄▄▄ ▄|▄▄▄ ▄▄▄ ▄ ▄|▄ ▄ ▄▄▄ ▄▄▄|▄▄▄ ▄ ▄ ▄|▄▄▄ ▄▄▄ ▄ ▄▄▄ ▄▄▄|▄▄▄ ▄▄▄ ▄▄▄|▄▄▄ ▄▄▄ ▄|▄▄▄ ▄ ▄▄▄|▄ ▄▄▄ ▄|▄ ▄ ▄|▄ ▄ ▄▄▄|▄▄▄ ▄ ▄|▄ ▄▄▄ ▄▄▄|▄ ▄▄▄|▄▄▄ ▄▄▄|▄▄▄ ▄|▄|▄▄▄)(   ▄ ▄▄▄ ▄▄▄|   ▄▄▄ ▄ ▄ ▄▄▄ ▄▄▄|   ▄▄▄ ▄▄▄)?(   ▄ ▄|   ▄ ▄ ▄▄▄ ▄▄▄ ▄)?)(   |$)"
    )
    .unwrap();
    pub static ref WABUN_WORD_REGEX: Regex = Regex::new(
        r"((di dah di dah dit|dah di dah di dah|dah di di di dah|dah di di dah dit|dah di di dah dah|dah dah dah di dah|di di dah di dah|di di dah di dit|di dah di dah dah|dah dah di di dah|di dah dah dah dit|dah dah di dah dit|di dah dah di dit|di dah di di dah|dah di dah di dit|dah di dah dah dit|dah di dah dah dah|di dah di di dit|di dah dah dit|di di
        dah dit|dah dah dah dit|dah dah dah dah|dah di dah dah|dah di dah dit|di di di dah|di dah di dit|dah dah di dah|dah di di dah|di dah dah dah|di di di dit|di dah di dah|di dah di dah dit|dah dah di dit|di di dah dah|dah di di dit|dah dah di dah dah|dah dah dah|dah dah dit|dah di dah|di dah dit|di di dit|di di dah|dah di dit|di dah 
        dah|di dah|dah dah|dah dit|dit|dah)(   di dah dah|   dah di di dah dah|   dah dah)?(   di dit|   di di dah dah dit)?)(   |$)"
    )
    .unwrap();
}

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

    pub fn kana_to_code(&self) -> &HashMap<&str, &str> {
        match self {
            Self::HalfBlock => &KANA_TO_HALFBLOCK,
            Self::Ascii => &KANA_TO_ASCII,
            Self::Word => &KANA_TO_WORD,
        }
    }

    pub fn code_to_kana(&self) -> &HashMap<&str, &str> {
        match self {
            Self::HalfBlock => &HALFBLOCK_TO_HIRA,
            Self::Ascii => &ASCII_TO_HIRA,
            Self::Word => &WORD_TO_HIRA,
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
        let map = self.representation.kana_to_code();
        let mut out: Vec<&str> = Vec::new();
        for symbol in WABUN_KANA_REGEX
            .captures_iter(text)
            .map(|cap| cap.get(0).unwrap().as_str())
        {
            if symbol == " " || symbol == "\u{3000}" {
                out.push(" ");
                continue;
            }
            match map.get(symbol) {
                Some(code) => out.push(code),
                None => out.push(symbol),
            }
        }

        Ok(out.join(self.representation.letter_sep()))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let map = self.representation.code_to_kana();
        let regex: &Regex = match self.representation {
            WabunRep::HalfBlock => &WABUN_HALFBLOCK_REGEX,
            WabunRep::Ascii => &WABUN_ASCII_REGEX,
            WabunRep::Word => &WABUN_WORD_REGEX,
        };
        let mut word_buffer = String::new();
        for word in text.split(self.representation.word_sep()) {
            for cap in regex
                .captures_iter(word)
                .map(|cap| cap.get(1).unwrap().as_str())
            {
                match map.get(cap) {
                    Some(kana) => word_buffer.push_str(kana),
                    None => return Err(CodeError::invalid_input_group(cap)),
                }
            }
            out.push(word_buffer.to_string());
            word_buffer.clear()
        }

        Ok(out.join(" "))
    }
}

#[cfg(test)]
mod wabun_tests {
    use super::*;

    const KANA: &'static str = "ひらがな にゃん";
    const ASCII: &'static str = "--..- ... .-.. .. .-.   -.-. .-- .-.-.";
    const HALFBLOCK: &'static str = "▄▄▄ ▄▄▄ ▄ ▄ ▄▄▄   ▄ ▄ ▄   ▄ ▄▄▄ ▄ ▄   ▄ ▄   ▄ ▄▄▄ ▄       ▄▄▄ ▄ ▄▄▄ ▄   ▄ ▄▄▄ ▄▄▄   ▄ ▄▄▄ ▄ ▄▄▄ ▄";
    const WORD: &'static str = "dah dah di di dah   di di dit   di dah di dit   di dit   di dah dit       dah di dah dit   di dah dah   di dah di dah dit";

    #[test]
    fn tree() {
        // for c in WABUN_KANA_REGEX.captures_iter(KANA) {
        //     println!("{:?}", c.get(0).unwrap().as_str());
        // }
        for c in WABUN_ASCII_REGEX.captures_iter(ASCII) {
            println!("{:?}", c.get(1).unwrap().as_str());
        }
        // for c in WABUN_HALFBLOCK_REGEX.captures_iter(HALFBLOCK) {
        //     println!("{:?}", c.get(1).unwrap().as_str());
        // }
        // for c in WABUN_WORD_REGEX.captures_iter(WORD) {
        //     println!("{:?}", c.get(1).unwrap().as_str());
        // }
    }

    #[test]
    fn encode_test_ascii() {
        let mut code = Wabun::default();
        code.representation = WabunRep::Ascii;
        assert_eq!(code.encode(KANA).unwrap(), ASCII);
    }

    #[test]
    fn decode_test_ascii() {
        let mut code = Wabun::default();
        code.representation = WabunRep::Ascii;
        assert_eq!(code.decode(ASCII).unwrap(), KANA);
    }

    #[test]
    fn encode_test_halfblock() {
        let mut code = Wabun::default();
        code.representation = WabunRep::HalfBlock;
        assert_eq!(code.encode(KANA).unwrap(), HALFBLOCK);
    }

    #[test]
    fn decode_test_halfblock() {
        let mut code = Wabun::default();
        code.representation = WabunRep::HalfBlock;
        assert_eq!(code.decode(HALFBLOCK).unwrap(), KANA);
    }

    #[test]
    fn encode_test_word() {
        let mut code = Wabun::default();
        code.representation = WabunRep::Word;
        assert_eq!(code.encode(KANA).unwrap(), WORD);
    }

    #[test]
    fn decode_test_word() {
        let mut code = Wabun::default();
        code.representation = WabunRep::Word;
        assert_eq!(code.decode(WORD).unwrap(), KANA);
    }
}
