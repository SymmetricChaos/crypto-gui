use itertools::zip;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::errors::CodeError;

/*
There are numerous romanization of the Japanese syllabary, the kana. The two
most common are the Hebern romanization, often seen in the west, and the
official Kunrei-shiki romanization (訓令式ローマ字). However neither of
these are reversible without significant context because some kana are
written in exactly the same way due to being pronounced in the same way,
occasionally depending on usage. For this encoding we use the the
Nihon-shiki romanization (日本式ローマ字) because it is highly regularized,
giving the 'vowel' kana a single letter, the 'n' kana a single letter, and
all others exactly two letters. This encoding does not always use correspond
to its English pronunciation. For instance こんにちは is written and
pronounced "kon'nichiwa" in the Hepburn system but is written "kon'nitiha" in
the Nihon-shiki system, with the same pronunciation.
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
    "n", 
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

    pub static ref LATIN_REGEX: Regex = Regex::new(r"(([kstnhmrgzdbp]y[auo])|([kstnhmrgzdbp][aiueo])|(w[aueo])|(y[auo])|([aiueo])|(n'|n)|.+)").unwrap();
    pub static ref HIRAGANA_REGEX: Regex = Regex::new(r"((\p{hira}[ゃゅょ])|(\p{hira})|.+)").unwrap();
    pub static ref KATAKANA_REGEX: Regex = Regex::new(r"((\p{kata}[ャュョ])|(\p{kata})|.+)").unwrap();


    // LATIN to HIRAGANA
    pub static ref L_TO_H: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), HIRAGANA.iter()) {
            map.insert(*l,*h);
        }
        map.insert("n'","ん");
        map
    };

    // LATIN to KATAKANA
    pub static ref L_TO_K: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), KATAKANA.iter()) {
            map.insert(*l,*h);
        }
        map.insert("n'","ン");
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
    // regex is ordered
    // first we match the 'kya' type chunks (note w and y excluded)
    // then the 'ka' types (note w and y excluded)
    // then the 'wa' types
    // then the 'ya', 'yu', and 'yo'
    // the the 'a' types
    // then the two n types, always checking for the n with apostophe first, otherwise it would never be matched
    // finally we capture everything else in order to catch malformed strings when converting

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

    fn kana_to_latin(text: &str, map: &HashMap<&str, &str>) -> Result<String, CodeError> {
        let mut out = Vec::new();
        let words = text.split_whitespace();
        for word in words {
            let mut temp_word = String::with_capacity(12);
            for m in HIRAGANA_REGEX.find_iter(word) {
                let group = m.as_str();
                if let Some(s) = map.get(group) {
                    temp_word.push_str(s)
                } else {
                    return Err(CodeError::Input(format!("invalid kana found: {}", group)));
                }
            }
            out.push(temp_word)
        }
        Ok(out.join(" "))
    }

    pub fn hiragana_to_romaji(&self, text: &str) -> Result<String, CodeError> {
        Self::kana_to_latin(text, &H_TO_L)
    }

    pub fn katakana_to_romaji(&self, text: &str) -> Result<String, CodeError> {
        Self::kana_to_latin(text, &K_TO_L)
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
    let latin = "konnitiwa hiragana kyouto oosaka toukyo yokohama ren'ai"; // kippu";
    let hiragana = "こんにちわ ひらがな きょうと おおさか とうきょ よこはま れんあい"; // きっぷ";
    let katakana = "コンニチワ ヒラガナ キョウト オオサカ トウキョ ヨコハマ レンアイ"; // きっぷ";

    assert_eq!(ns.romaji_to_hiragana(latin).unwrap(), hiragana);
    assert_eq!(ns.romaji_to_katakana(latin).unwrap(), katakana);

    println!("{:?}", ns.hiragana_to_romaji(hiragana));
    println!("{:?}", ns.katakana_to_romaji(katakana));
}
