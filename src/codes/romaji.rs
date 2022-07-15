use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex; 
use itertools::zip;

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
    pub static ref H_TO_R: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana, syll) in HIRAGANA.chars().zip(ROMAN.iter()) {
            m.insert(kana, *syll);
        }
        m
    };
    pub static ref R_TO_H: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (kana, syll) in HIRAGANA.chars().zip(ROMAN.iter()) {
            m.insert(*syll, kana);
        }
        m
    };
    pub static ref K_TO_R: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana, syll) in KATAKANA.chars().zip(ROMAN.iter()) {
            m.insert(kana, *syll);
        }
        m
    };
    pub static ref R_TO_K: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (kana, syll) in KATAKANA.chars().zip(ROMAN.iter()) {
            m.insert(*syll, kana);
        }
        m
    };

    pub static ref L_TO_H: HashMap<&'static str, &'static str> = {
        let mut hiragana_map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), HIRAGANA.iter()) {
            hiragana_map.insert(*l,*h);
        }
        hiragana_map.insert("n'","ん");
        hiragana_map
    };

    pub static ref L_TO_K: HashMap<&'static str, &'static str> = {
        let mut katakana_map = HashMap::<&str,&str>::new();
        for (l,h) in zip(LATIN.iter(), KATAKANA.iter()) {
            katakana_map.insert(*l,*h);
        }
        katakana_map.insert("n'","ン");
        katakana_map
    };
}

pub struct NihonShiki {

}

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
 
    fn latin_to_kana(text: &str, map: HashMap<&str,&str>) -> String {
        let re = Regex::new(r"(([kstnhmrgzdbp]y[auo])|([kstnhmrgzdbp][aiueo])|(w[aueo])|(y[auo])|([aiueo])|(n'|n)|.+)").unwrap();
 
        let mut out = Vec::new();
        let words = text.split_whitespace();
        for word in words {
            let mut temp_word = String::with_capacity(12);
            for m in re.find_iter(word) {
                let group = m.as_str();
                if let Some(s) = map.get(group) {
                    temp_word.push_str(s)
                } else {
                    temp_word = format!("INVALID({})",word);
                    break
                }
            }
            out.push(temp_word)
        }
        out.join(" ")
    }

    pub fn hirigana_to_romaji(&self, text: &str) -> Result<String,CodeError> {
        let mut symbols = text.chars().peekable();
        let mut out = String::with_capacity(text.chars().count() * 2);

        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['あ', 'い', 'う', 'え', 'お', 'や', 'ゆ', 'よ'];

        // kana start start with n when romanized
        let n_kana = ['な', 'に', 'ぬ', 'ね', 'の'];

        // The small y-kana
        let small_y = ['ゃ', 'ゅ', 'ょ'];

        loop {
            let s = match symbols.next() {
                Some(symbol) => symbol,
                None => break,
            };
            // Don't modify whitespace
            if s.is_whitespace() {
                out.push(s);
            // handle apostophe after ん
            } else if s == 'ん' {
                let next_kana = symbols.peek();
                if next_kana.is_none() {
                    out.push('n')
                } else {
                    let k = next_kana.unwrap();
                    if vowels.contains(k) || n_kana.contains(k) {
                        out.push_str("n'");
                    } else {
                        out.push('n');
                    }
                }
            // handle sokuon
            } else if s == 'っ' {
                let next_kana = symbols.peek().unwrap();
                let romaji = H_TO_R[next_kana].chars().nth(0).unwrap();
                out.push(romaji);
            // handle yoon
            } else if small_y.contains(&s) {
                let prev_char = out.pop().unwrap();
                if prev_char == 'i' {
                    out.push_str(&H_TO_R[&s])
                } else {
                    return Err(CodeError::Input("small y kana must be preceeded by a i-column kana".to_string()))
                }
            // everything else
            } else {
                out.push_str(&H_TO_R[&s])
            }
        }
        Ok(out)
    }

    pub fn romaji_to_hirigana(&self, text: &str) -> Result<String,CodeError> {
        let mut symbols = text.chars().peekable();
        let mut out = String::with_capacity(text.chars().count() / 2);

        let mut buffer = String::with_capacity(12);

        // For each roman letter:
        //  push to the buffer
        //  if the buffer contains only a vowel then push that kana to out and clear the buffer
        //  if the buffer contains a consonant and a non-'y' vowel push that kana to out and clear the buffer
        //  if the buffer contains a non-'n' consonant then restart the loop
        //  if the buffer contains 'n' check the next symbol
        //      if it is an apostrophe then push 'ん' to out and clear the buffer
        //      if that is a vowel then restart the loop
        //      otherwise return an error
        //  if the buffer contains a consonant and 'y' continue then restart the loop
        //  if the buffer contains a constant, a 'y', and a non-'y' vowel push that kana to out and clear the buffer
        //  in all other cases return an error
        loop {
            match symbols.next() {
                Some(s) => buffer.push(s),
                None => break,
            }
            if buffer == "n" {
                if let Some(c) = symbols.peek() {

                }
            }

        }


        Ok(out)
    }

    pub fn katakana_to_romaji(&self, text: &str) -> Result<String,CodeError> {
        let mut symbols = text.chars().peekable();
        let mut out = String::new();

        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['ア', 'イ', 'ウ', 'エ', 'オ', 'ユ', 'ヨ', 'ラ'];
        // kana start start with n when romanized
        let n_kana = ['ナ', 'ニ', 'ヌ', 'ネ', 'ノ'];
        // The small y-kana
        let small_y = ['ャ', 'ュ', 'ョ'];

        loop {
            let s = match symbols.next() {
                Some(kana) => kana,
                None => break,
            };
            if s.is_whitespace() {
                out.push(s);
            // handle apostophe after ン
            } else if s == 'ン' {
                let next_kana = symbols.peek();
                if next_kana.is_none() {
                    out.push('n')
                } else {
                    let k = next_kana.unwrap();
                    if vowels.contains(k) || n_kana.contains(k) {
                        out.push_str("n'");
                    } else {
                        out.push('n');
                    }
                }
            // handle chōonpu
            } else if s == 'ー' {
                let vowel = out.pop().unwrap();
                out.push(vowel);
                out.push(vowel);

            // handle sokuon
            } else if s == 'ッ' {
                let next_kana = symbols.peek().unwrap();
                let romaji = K_TO_R[next_kana].chars().nth(0).unwrap();
                out.push(romaji);
            // handle yoon
            } else if small_y.contains(&s) {
                let prev_char = out.pop().unwrap();
                if prev_char == 'i' {
                    out.push_str(&K_TO_R[&s])
                } else {
                    return Err(CodeError::Input("small y kana must be preceeded by a i-column kana".to_string()))
                }
            // everything else
            } else {
                out.push_str(&K_TO_R[&s])
            }
        }
        Ok(out)
    }

    // pub fn romaji_to_katakana(&self, text: &str) -> Result<String,CodeError> {
    //     todo!()
    // }
}

#[test]
fn nihon_shiki_hiragana() {
    let ns = NihonShiki::default();
    let plaintext = "ひらがな かたかな しんよう きっぷ きよう きょう にほん　こんにちは";
    let coded = ns.hirigana_to_romaji(plaintext);
    println!("{}", coded);
}
