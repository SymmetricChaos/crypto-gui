use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

// pub const ITU_LETTERS: &'static str = "ABCDEÉFGHIJKLMNOPQRSTUVWXYZ1234567890.,:?'-/()\"=+@";

pub const ITU_SIGNS: [&'static str; 55] = [
    "A",
    "B",
    "C",
    "D",
    "E",
    "É",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    ".",
    ",",
    ":",
    "?",
    "'",
    "-",
    "/",
    "(",
    ")",
    "\"",
    "=",
    "+",
    "@",
    "[understood]",
    "[error]",
    "[wait]",
    "[end of work]",
    "[starting signal]",
];
pub const ITU_ASCII: [&'static str; 55] = [
    ".-", "-...", "-.-.", "-..", ".", "..-..", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
    "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
    "-----", ".-.-.-", "--..--", "---...", "..--..", ".---.", "-...-", "-..-.", "-.--.", "-.--.-",
    ".-..-.", "-...-", ".-.-.", ".--.-.", "...-.", "........", ".-...", "...-.-", "-.-.-",
];
pub const ITU_WORD: [&'static str; 55] = [
    "di dah",
    "dah di di dit",
    "dah di dah dit",
    "dah di dit",
    "dit",
    "di di dah di dit",
    "di di dah dit",
    "dah dah dit",
    "di di di dit",
    "di dit",
    "di dah dah dah",
    "dah di dah",
    "di dah di dit",
    "dah dah",
    "dah dit",
    "dah dah dah",
    "di dah dah dit",
    "dah dah di dah",
    "di dah dit",
    "di di dit",
    "dah",
    "di di dah",
    "di di di dah",
    "di dah dah",
    "dah di di dah",
    "dah di dah dah",
    "dah dah di dit",
    "di dah dah dah dah",
    "di di dah dah dah",
    "di di di dah dah",
    "di di di di dah",
    "di di di di dit",
    "dah di di di dit",
    "dah dah di di dit",
    "dah dah dah di dit",
    "dah dah dah dah dit",
    "dah dah dah dah dah",
    "di dah di dah di dah",
    "dah dah di di dah dah",
    "dah dah dah di di dit",
    "di di dah dah di dit",
    "di dah dah dah dit",
    "dah di di di dah",
    "dah di di dah dit",
    "dah di dah dah dit",
    "dah di dah dah di dah",
    "di dah di di dah dit",
    "dah di di di dah",
    "di dah di dah dit",
    "di dah dah di dah dit",
    "di di di dah dit",
    "di di di di di di di dit",
    "di dah di di dit",
    "di di di dah di dah",
    "dah di dah di dah",
];

pub const ITU_BINARY: [&'static str; 55] = [
    "10111",
    "111010101",
    "11101011101",
    "1110101",
    "1",
    "10101110101",
    "101011101",
    "111011101",
    "1010101",
    "101",
    "1011101110111",
    "111010111",
    "101110101",
    "1110111",
    "11101",
    "11101110111",
    "10111011101",
    "1110111010111",
    "1011101",
    "10101",
    "111",
    "1010111",
    "101010111",
    "101110111",
    "11101010111",
    "1110101110111",
    "11101110101",
    "10111011101110111",
    "101011101110111",
    "1010101110111",
    "10101010111",
    "101010101",
    "11101010101",
    "1110111010101",
    "111011101110101",
    "11101110111011101",
    "1110111011101110111",
    "10111010111010111",
    "1110111010101110111",
    "11101110111010101",
    "101011101110101",
    "101110111011101",
    "1110101010111",
    "1110101011101",
    "111010111011101",
    "1110101110111010111",
    "101110101011101",
    "1110101010111",
    "1011101011101",
    "10111011101011101",
    "10101011101",
    "101010101010101",
    "10111010101",
    "101010111010111",
    "111010111010111",
];

pub const ITU_HALFBLOCK: [&'static str; 55] = [
    "▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄",
    "▄",
    "▄ ▄ ▄▄▄ ▄ ▄",
    "▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄",
    "▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄",
    "▄ ▄ ▄",
    "▄▄▄",
    "▄ ▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄ ▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄ ▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄ ▄ ▄",
    "▄ ▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄ ▄ ▄ ▄ ▄",
    "▄ ▄▄▄ ▄ ▄ ▄",
    "▄ ▄ ▄ ▄▄▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄▄▄ ▄ ▄▄▄",
];

pub const GERKE_LETTERS: [&'static str; 40] = [
    "CH", "A", "Ä", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "O", "Ö", "P", "Q",
    "R", "S", "T", "U", "Ü", "V", "W", "X", "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "0", "?",
];

pub const GERKE_BINARY: [&'static str; 40] = [
    "111011101110111",
    "10111",
    "10111010111",
    "111010101",
    "11101011101",
    "1110101",
    "1",
    "101011101",
    "111011101",
    "1010101",
    "101",
    "111010111",
    "101110101",
    "1110111",
    "11101",
    "10111010101",
    "1110111011101",
    "101010101",
    "1110111010111",
    "1011101",
    "10101",
    "111",
    "1010111",
    "10101110111",
    "101010111",
    "101110111",
    "1010111010101",
    "1110111010101",
    "1011101110101",
    "10111011101",
    "10101110101",
    "10101011101",
    "10101010111",
    "11101110111",
    "10101010101",
    "11101110101",
    "11101010101",
    "11101010111",
    "111111",
    "1010101110101",
];
pub const GERKE_HALFBLOCK: [&'static str; 40] = [
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄",
    "▄",
    "▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄ ▄",
    "▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄",
    "▄▄▄ ▄",
    "▄ ▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄",
    "▄ ▄ ▄",
    "▄▄▄",
    "▄ ▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄▄▄ ▄▄▄ ▄",
    "▄ ▄ ▄▄▄ ▄ ▄",
    "▄ ▄ ▄ ▄▄▄ ▄",
    "▄ ▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄ ▄▄▄",
    "▄▄▄▄▄▄",
    "▄ ▄ ▄ ▄▄▄ ▄ ▄",
];

pub const AMERICAN_LETTERS: [&'static str; 41] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "&", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", ",",
    ".", "?", "!",
];
pub const AMERICAN_BINARY: [&'static str; 41] = [
    "1011",
    "11010101",
    "101001",
    "110101",
    "1",
    "101101",
    "1101101",
    "1010101",
    "101",
    "110101101",
    "1101011",
    "1111",
    "11011",
    "1101",
    "1001",
    "101010101",
    "10101101",
    "100101",
    "10101",
    "11",
    "101011",
    "10101011",
    "1011011",
    "10110101",
    "10100101",
    "10101001",
    "10010101",
    "101101101",
    "1010110101",
    "1010101101",
    "1010101011",
    "11011011",
    "10101010101",
    "110110101",
    "1101010101",
    "110101011",
    "11111",
    "101101011",
    "1010110110101",
    "11010101101",
    "1101101101",
];
pub const AMERICAN_HALFBLOCK: [&'static str; 41] = [
    "▄ ▄▄",
    "▄▄ ▄ ▄ ▄",
    "▄ ▄  ▄",
    "▄▄ ▄ ▄",
    "▄",
    "▄ ▄▄ ▄",
    "▄▄ ▄▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄ ▄",
    "▄▄ ▄ ▄▄ ▄",
    "▄▄ ▄ ▄▄",
    "▄▄▄▄",
    "▄▄ ▄▄",
    "▄▄ ▄",
    "▄  ▄",
    "▄ ▄ ▄ ▄ ▄",
    "▄ ▄ ▄▄ ▄",
    "▄  ▄ ▄",
    "▄ ▄ ▄",
    "▄▄",
    "▄ ▄ ▄▄",
    "▄ ▄ ▄ ▄▄",
    "▄ ▄▄ ▄▄",
    "▄ ▄▄ ▄ ▄",
    "▄ ▄  ▄ ▄",
    "▄ ▄ ▄  ▄",
    "▄  ▄ ▄ ▄",
    "▄ ▄▄ ▄▄ ▄",
    "▄ ▄ ▄▄ ▄ ▄",
    "▄ ▄ ▄ ▄▄ ▄",
    "▄ ▄ ▄ ▄ ▄▄",
    "▄▄ ▄▄ ▄▄",
    "▄ ▄ ▄ ▄ ▄ ▄",
    "▄▄ ▄▄ ▄ ▄",
    "▄▄ ▄ ▄ ▄ ▄",
    "▄▄ ▄ ▄ ▄▄",
    "▄▄▄▄▄",
    "▄ ▄▄ ▄ ▄▄",
    "▄ ▄ ▄▄ ▄▄ ▄ ▄",
    "▄▄ ▄ ▄ ▄▄ ▄",
    "▄▄ ▄▄ ▄▄ ▄",
];

// the organization of the array should be preserved for legibility
// #[rustfmt::skip]
// pub const HIRAGANA: [&str; 109] = [
//     "あ", "い", "う", "え", "お",
//     "か", "き", "く", "け", "こ",   "きゃ", "きゅ", "きょ",
//     "さ", "し", "す", "せ", "そ",   "しゃ", "しゅ", "しょ",
//     "た", "ち", "つ", "て", "と",   "ちゃ", "ちゅ", "ちょ",
//     "な", "に", "ぬ", "ね", "の",   "にゃ", "にゅ", "にょ",
//     "は", "ひ", "ふ", "へ", "ほ",   "ひゃ", "ひゅ", "ひょ",
//     "ま", "み", "む", "め", "も",   "みゃ", "みゅ", "みょ",
//     "や",       "ゆ",       "よ",
//     "ら", "り", "る", "れ", "ろ",   "りゃ", "りゅ", "りょ",
//     "わ", "ゐ",       "ゑ", "を",
//     "ん",
//     "が", "ぎ", "ぐ", "げ", "ご",   "ぎゃ", "ぎゅ", "ぎょ",
//     "ざ", "じ", "ず", "ぜ", "ぞ",   "じゃ", "じゅ", "じょ",
//     "だ", "ぢ", "づ", "で", "ど",   "ぢゃ", "ぢゅ", "ぢょ",
//     "ば", "び", "ぶ", "べ", "ぼ",   "びゃ", "びゅ", "びょ",
//     "ぱ", "ぴ", "ぷ", "ぺ", "ぽ",   "ぴゃ", "ぴゅ", "ぴょ",
// ];
// #[rustfmt::skip]
// pub const LATIN: [&str; 109] = [
//      "a",  "i",  "u",  "e",  "o",
//     "ka", "ki", "ku", "ke", "ko",   "kya", "kyu", "kyo",
//     "sa", "si", "su", "se", "so",   "sya", "syu", "syo",
//     "ta", "ti", "tu", "te", "to",   "tya", "tyu", "tyo",
//     "na", "ni", "nu", "ne", "no",   "nya", "nyu", "nyo",
//     "ha", "hi", "hu", "he", "ho",   "hya", "hyu", "hyo",
//     "ma", "mi", "mu", "me", "mo",   "mya", "myu", "myo",
//     "ya",       "yu",       "yo",
//     "ra", "ri", "ru", "re", "ro",   "rya", "ryu", "ryo",
//     "wa", "wi",       "we", "wo",
//     "n'",
//     "ga", "gi", "gu", "ge", "go",   "gya", "gyu", "gyo",
//     "za", "zi", "zu", "ze", "zo",   "zya", "zyu", "zyo",
//     "da", "di", "du", "de", "do",   "dya", "dyu", "dyo",
//     "ba", "bi", "bu", "be", "bo",   "bya", "byu", "byo",
//     "pa", "pi", "pu", "pe", "po",   "pya", "pyu", "pyo",
// ];

// pub const WABUN: [&str; ?] = [];

lazy_static! {
    pub static ref ITU_ASCII_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_ASCII.into_iter()));
    pub static ref ITU_WORD_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_WORD.into_iter()));
    pub static ref ITU_BINARY_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_BINARY.into_iter()));
    pub static ref ITU_HALFBLOCK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_HALFBLOCK.into_iter()));
    pub static ref AMERICAN_BINARY_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        AMERICAN_LETTERS
            .into_iter()
            .zip(AMERICAN_BINARY.into_iter())
    );
    pub static ref AMERICAN_HALFBLOCK_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        AMERICAN_LETTERS
            .into_iter()
            .zip(AMERICAN_HALFBLOCK.into_iter())
    );
    pub static ref GERKE_BINARY_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GERKE_LETTERS.into_iter().zip(GERKE_BINARY.into_iter()));
    pub static ref GERKE_HALFBLOCK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GERKE_LETTERS.into_iter().zip(GERKE_HALFBLOCK.into_iter()));
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    fn convert_binary_to_halfblock(text: &[&'static str]) {
        let mut out = Vec::new();
        for i in text {
            let mut t = i.replace("1", "▄");
            t = t.replace("0", " ");
            out.push(t)
        }
        println!("\nHALFBLOCK:\n{:?}", out)
    }

    fn convert_ascii(text: &[&'static str]) {
        let mut out = Vec::new();
        for i in text {
            let mut t = i.replace(".", "10");
            t = t.replace("-", "1110");
            t.pop();
            out.push(t)
        }
        println!("\nBINARY:\n{:?}", out);

        let mut out = Vec::new();
        for i in text {
            let mut t = i.replace(".", "di ");
            t = t.replace("-", "dah ");
            t.pop();
            if t.chars().last().unwrap() == 'i' {
                t.push('t')
            }
            out.push(t)
        }
        println!("\nWORD:\n{:?}", out);

        let mut out = Vec::new();
        for i in text {
            let mut t = i.replace(".", "▄ ");
            t = t.replace("-", "▄▄▄ ");
            t.pop();
            out.push(t)
        }
        println!("\nHALFBLOCK:\n{:?}", out)
    }

    #[test]
    #[ignore = "conversions"]
    fn convert() {
        convert_ascii(&ITU_ASCII);
        convert_binary_to_halfblock(&ITU_BINARY);
        convert_binary_to_halfblock(&AMERICAN_BINARY);
        convert_binary_to_halfblock(&GERKE_BINARY);
    }

    #[test]
    #[ignore = "visual correctness check"]
    fn itu_pairs() {
        for (letter, code) in ITU_SIGNS.into_iter().zip(ITU_ASCII) {
            println!("{letter} {code}")
        }
    }

    #[test]
    #[ignore = "visual correctness check"]
    fn gerke_pairs() {
        for (letter, code) in GERKE_LETTERS.into_iter().zip(GERKE_HALFBLOCK) {
            println!("{letter} {code}")
        }
    }

    #[test]
    #[ignore = "visual correctness check"]
    fn american_pairs() {
        for (letter, code) in AMERICAN_LETTERS.into_iter().zip(AMERICAN_HALFBLOCK) {
            println!("{letter} {code}")
        }
    }
}
