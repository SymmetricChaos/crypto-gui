use lazy_static::lazy_static;
use std::collections::HashMap;

/*
There are numerous romanization of the Japanese syllabary, the kana. The two
most common are the Hebrun romanization, often seen in the west, and the
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
const HIRAGANA: &'static str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽゃゅょ";

const KATAKANA: &'static str = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポャュョ";

const ROMAN: [&'static str; 74] = [
    "a", "i", "u", "e", "o", "ka", "ki", "ku", "ke", "ko", "sa", "si", "su", "se", "so", "ta",
    "ti", "tu", "te", "to", "na", "ni", "nu", "ne", "no", "ha", "hi", "hu", "he", "ho", "ma", "mi",
    "mu", "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re", "ro", "wa", "wo", "n", "ga", "gi",
    "gu", "ge", "go", "za", "zi", "zu", "ze", "zo", "da", "di", "du", "de", "do", "ba", "bi", "bu",
    "be", "bo", "pa", "pi", "pu", "pe", "po", "ya", "yu", "yo",
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
}

pub struct NihonShiki {}

impl Default for NihonShiki {
    fn default() -> Self {
        Self {}
    }
}

impl NihonShiki {
    pub fn hirigana_to_romaji(&self, text: &str) -> String {
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
                    panic!("small y kana must be preceeded by a i-column kana")
                }
            // everything else
            } else {
                out.push_str(&H_TO_R[&s])
            }
        }
        out
    }

    // pub fn romaji_to_hirigana(&self, text: &str) -> String {
    //     todo!()
    // }

    pub fn katakana_to_romaji(&self, text: &str) -> String {
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
                    panic!("small y kana must be preceeded by a i-column kana")
                }
            // everything else
            } else {
                out.push_str(&K_TO_R[&s])
            }
        }
        out
    }

    // pub fn romaji_to_katakana(&self, text: &str) -> String {
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
