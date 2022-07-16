use itertools::zip;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::errors::CodeError;

/*
This converter uses the Nihon-shiki romaji (日本式ローマ字, Japanese-style romanization)
to convert between Japanese kana and the Latin alphabet. This is no longer a common
romanization because it does not reflect actual pronunciation. However it is highly
regular and allows clean conversion between the two writing systems. Kana are always
and only written according to their position in the gojyuu-on. So こんにちは is
romanized as "kon'nitiha" rather than as it is pronounced "kon'nichiwa".

Simple regex is used to tokenize strings so some aspects of writing kana are not yet supported.
These are: sokuon, chouonpu (kana should be used), iteration marks (kana should be used).
*/


// the organization of the array should be preserved for legibility
#[rustfmt::skip] 
const LATIN: [&str; 109] = [
     "a",  "i",  "u",  "e",  "o", 
    "ka", "ki", "ku", "ke", "ko",   "kya", "kyu", "kyo",
    "sa", "si", "su", "se", "so",   "sya", "syu", "syo",
    "ta", "ti", "tu", "te", "to",   "tya", "tyu", "tyo",
    "na", "ni", "nu", "ne", "no",   "nya", "nyu", "nyo",
    "ha", "hi", "hu", "he", "ho",   "hya", "hyu", "hyo",
    "ma", "mi", "mu", "me", "mo",   "mya", "myu", "myo",
    "ya",       "yu",       "yo", 
    "ra", "ri", "ru", "re", "ro",   "rya", "ryu", "ryo",
    "wa", "wi",       "we", "wo",
    "n'", 
    "ga", "gi", "gu", "ge", "go",   "gya", "gyu", "gyo",
    "za", "zi", "zu", "ze", "zo",   "zya", "zyu", "zyo",
    "da", "di", "du", "de", "do",   "dya", "dyu", "dyo",
    "ba", "bi", "bu", "be", "bo",   "bya", "byu", "byo",
    "pa", "pi", "pu", "pe", "po",   "pya", "pyu", "pyo",
];

#[rustfmt::skip] 
const HIRAGANA: [&str; 109] = [
    "あ", "い", "う", "え", "お", 
    "か", "き", "く", "け", "こ",   "きゃ", "きゅ", "きょ",
    "さ", "し", "す", "せ", "そ",   "しゃ", "しゅ", "しょ",
    "た", "ち", "つ", "て", "と",   "ちゃ", "ちゅ", "ちょ",
    "な", "に", "ぬ", "ね", "の",   "にゃ", "にゅ", "にょ",
    "は", "ひ", "ふ", "へ", "ほ",   "ひゃ", "ひゅ", "ひょ",
    "ま", "み", "む", "め", "も",   "みゃ", "みゅ", "みょ",
    "や",       "ゆ",       "よ", 
    "ら", "り", "る", "れ", "ろ",   "りゃ", "りゅ", "りょ",
    "わ", "ゐ",       "ゑ", "を",
    "ん", 
    "が", "ぎ", "ぐ", "げ", "ご",   "ぎゃ", "ぎゅ", "ぎょ",
    "ざ", "じ", "ず", "ぜ", "ぞ",   "じゃ", "じゅ", "じょ",
    "だ", "ぢ", "づ", "で", "ど",   "ぢゃ", "ぢゅ", "ぢょ",
    "ば", "び", "ぶ", "べ", "ぼ",   "びゃ", "びゅ", "びょ",
    "ぱ", "ぴ", "ぷ", "ぺ", "ぽ",   "ぴゃ", "ぴゅ", "ぴょ",
];

#[rustfmt::skip] 
const KATAKANA: [&str; 109] = [
    "ア", "イ", "ウ", "エ", "オ", 
    "カ", "キ", "ク", "ケ", "コ",   "キャ", "キュ", "キョ",
    "サ", "シ", "ス", "セ", "ソ",   "シャ", "シュ", "ショ",
    "タ", "チ", "ツ", "テ", "ト",   "チャ", "チュ", "チョ",
    "ナ", "ニ", "ヌ", "ネ", "ノ",   "ニャ", "ニュ", "ニョ",
    "ハ", "ヒ", "フ", "ヘ", "ホ",   "ヒャ", "ヒュ", "ヒョ",
    "マ", "ミ", "ム", "メ", "モ",   "ミャ", "ミュ", "ミョ",
    "ヤ",       "ユ",       "ヨ", 
    "ラ", "リ", "ル", "レ", "ロ",   "リャ", "リュ", "リョ",
    "ワ", "ヰ",       "ヱ", "ヲ",
    "ン", 
    "ガ", "ギ", "グ", "ゲ", "ゴ",   "ギャ", "ギュ", "ギョ",
    "ザ", "ジ", "ズ", "ゼ", "ゾ",   "ジャ", "ジュ", "ジョ",
    "ダ", "ヂ", "ヅ", "デ", "ド",   "ヂャ", "ヂュ", "ヂョ",
    "バ", "ビ", "ブ", "ベ", "ボ",   "ビャ", "ビュ", "ビョ",
    "パ", "ピ", "プ", "ペ", "ポ",   "ピャ", "ピュ", "ピョ",
];

lazy_static! {

    // regex is ordered
    // first we match the 'kya' type chunks (note w and y excluded)
    // then the 'ka' types (note w and y excluded)
    // then the 'wa' types
    // then the 'ya', 'yu', and 'yo'
    // the the 'a' types
    // then the two n types, always checking for the n with apostophe first, otherwise it would never be matched
    // finally we capture everything else in order to catch malformed strings when converting
    pub static ref LATIN_REGEX: Regex = Regex::new(r"(([kstnhmrgzdbp]y[auo])|([kstnhmrgzdbp][aiueo])|(w[aueo])|(y[auo])|([aiueo])|(n'|n)|.+)").unwrap();
    
    // here match the kya type chunks, then all single kana, then the sokuon, then everything else
    pub static ref HIRAGANA_REGEX: Regex = Regex::new(r"((\p{hira}[ゃゅょ])|(\p{hira})|っ|.+)").unwrap();
    pub static ref KATAKANA_REGEX: Regex = Regex::new(r"((\p{katakana}[ャュョ])|(\p{katakana})|ッ|.+)").unwrap();


    // LATIN to HIRAGANA
    pub static ref L_TO_H: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), HIRAGANA.iter()) {
            map.insert(*l,*h);
        }
        map.insert("n","ん");
        map
    };

    // LATIN to KATAKANA
    pub static ref L_TO_K: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), KATAKANA.iter()) {
            map.insert(*l,*h);
        }
        map.insert("n","ン");
        map
    };

    // HIRAGANA to LATIN
    pub static ref H_TO_L: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), HIRAGANA.iter()) {
            map.insert(*h,*l);
        }
        map
    };

    // KATAKANA to LATIN
    pub static ref K_TO_L: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,k) in zip(LATIN.iter(), KATAKANA.iter()) {
            map.insert(*k,*l);
        }
        map
    };
}

pub struct NihonShiki {}

impl Default for NihonShiki {
    fn default() -> Self {
        Self {}
    }
}

impl NihonShiki {

    fn latin_to_kana(text: &str, map: &HashMap<&str, &str>) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let words = text.split_whitespace();
        for word in words {
            let mut temp_word = String::with_capacity(12);
            for m in LATIN_REGEX.find_iter(word) {
                let group = m.as_str();
                if let Some(s) = map.get(group) {
                    temp_word.push_str(s)
                } else {
                    return Err(CodeError::Input(format!("invalid romaji found: {}", group)));
                }
            }
            out.push(temp_word)
        }
        Ok(out.join(" "))
    }

    fn hiragana_to_latin(text: &str, map: &HashMap<&str, &str>) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let mut sokuon = false;
        let words = text.split_whitespace();
        for word in words {
            let mut temp_word = String::with_capacity(12);
            for m in HIRAGANA_REGEX.find_iter(word) {
                let group = m.as_str();
                if let Some(s) = map.get(group) {
                    if sokuon {
                        
                    } else {
                        temp_word.push_str(s)
                    }
                    
                } else {
                    return Err(CodeError::Input(format!("invalid kana found: {}", group)));
                }
                if group == "っ" {
                    sokuon = true;
                }
            }
            // The apostrophe after n is unnecessary at the end of a word
            if temp_word.chars().last() == Some('\'') {
                temp_word.pop();
            }
            out.push(temp_word)
        }
        Ok(out.join(" "))
    }

    fn katakana_to_latin(text: &str, map: &HashMap<&str, &str>) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let words = text.split_whitespace();
        for word in words {
            let mut temp_word = String::with_capacity(12);
            for m in KATAKANA_REGEX.find_iter(word) {
                let group = m.as_str();
                if let Some(s) = map.get(group) {
                    temp_word.push_str(s)
                } else {
                    return Err(CodeError::Input(format!("invalid kana found: {}", group)));
                }
            }
            if temp_word.chars().last() == Some('\'') {
                temp_word.pop();
            }
            out.push(temp_word)
        }
        Ok(out.join(" "))
    }

    pub fn hiragana_to_romaji(&self, text: &str) -> Result<String, CodeError> {
        Self::hiragana_to_latin(text, &H_TO_L)
    }

    pub fn katakana_to_romaji(&self, text: &str) -> Result<String, CodeError> {
        Self::katakana_to_latin(text, &K_TO_L)
    }

    pub fn romaji_to_hiragana(&self, text: &str) -> Result<String, CodeError> {
        Self::latin_to_kana(text, &L_TO_H)
    }

    pub fn romaji_to_katakana(&self, text: &str) -> Result<String, CodeError> {
        Self::latin_to_kana(text, &L_TO_K)
    }
}

#[test]
fn nihon_shiki_hiragana() {

    let ns = NihonShiki::default();
    let latin = "kon'nitiha hiragana kyouto oosaka toukyo yokohama ren'ai ken"; // kippu";
    let hiragana = "こんにちは ひらがな きょうと おおさか とうきょ よこはま れんあい けん"; // きっぷ";
    let katakana = "コンニチハ ヒラガナ キョウト オオサカ トウキョ ヨコハマ レンアイ ケン"; // きっぷ";

    assert_eq!(ns.romaji_to_hiragana(latin).unwrap(), hiragana);
    assert_eq!(ns.romaji_to_katakana(latin).unwrap(), katakana);

    assert_eq!(ns.hiragana_to_romaji(hiragana).unwrap(), latin);
    assert_eq!(ns.katakana_to_romaji(katakana).unwrap(), latin);
}
