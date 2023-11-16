use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

const LETTERS: [&'static str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "ŋ", "ə", "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ",
    "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω",
];
const LETTERS_BRAILLE: [&'static str; 52] = [
    "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊", "⠚", "⠅", "⠇", "⠍", "⠝", "⠕", "⠏", "⠟", "⠗", "⠎",
    "⠞", "⠥", "⠧", "⠺", "⠭", "⠽", "⠵", "⠘⠝", "⠸⠢", "⠨⠁", "⠨⠃", "⠨⠛", "⠨⠙", "⠨⠑", "⠨⠵", "⠨⠱", "⠨⠹",
    "⠨⠊", "⠨⠅", "⠨⠇", "⠨⠍", "⠨⠝", "⠨⠭", "⠨⠕", "⠨⠏", "⠨⠗", "⠨⠎", "⠨⠞", "⠨⠥", "⠨⠋", "⠨⠯", "⠨⠽", "⠨⠺",
];

const SYMBOLS: [&'static str; 44] = [
    "→", "↓", "←", "↑", "∶", "∷", "′", "″", "♮", "♭", "♯", "@", "¢", "€", "₣", "£", "₦", "$", "¥",
    "&", "<", "^", "~", ">", "†", "‡", "©", "°", "¶", "®", "§", "™", "♀", "♂", "#", "•", "〃", "+",
    "=", "×", "*", "÷", "-", "%",
];
const SYMBOLS_BRAILLE: [&'static str; 44] = [
    "⠳⠕",
    "⠳⠩",
    "⠳⠪",
    "⠳⠬",
    "⠒",
    "⠒⠒",
    "⠶",
    "⠶⠶",
    "⠼⠡",
    "⠼⠣",
    "⠼⠩",
    "⠈⠁",
    "⠈⠉",
    "⠈⠑",
    "⠈⠋",
    "⠈⠇",
    "⠈⠝",
    "⠈⠎",
    "⠈⠽",
    "⠈⠯",
    "⠈⠣",
    "⠈⠢",
    "⠈⠔",
    "⠈⠜",
    "⠈⠠⠹",
    "⠈⠠⠻",
    "⠘⠉",
    "⠘⠚",
    "⠘⠏",
    "⠘⠗",
    "⠘⠎",
    "⠘⠞",
    "⠘⠭",
    "⠘⠽",
    "⠸⠹",
    "⠸⠲",
    "⠐⠂",
    "⠐⠖",
    "⠐⠶",
    "⠐⠦",
    "⠐⠔",
    "⠐⠌",
    "⠐⠤",
    "⠨⠴",
];

const DIACRITIC: [&'static str; 12] = ["̸", "̶", "̆", "̄", "̧", "̀", "̂", "̊", "̃", "̈", "́", "̌"];
const DIACRITIC_BRAILLE: [&'static str; 12] = [
    "⠈⠡", "⠈⠒", "⠈⠬", "⠈⠤", "⠘⠯", "⠘⠡", "⠘⠩", "⠘⠫", "⠘⠻", "⠘⠒", "⠘⠌", "⠘⠬",
];

// Ellipsis is a specific punctuation symbol listed for UEB but it written as a sequence of full stops
// The dashes are Unicode specified: hyphen, em-dash, and double em-dash.
const PUNCTUATION: [&'static str; 28] = [
    ",", ";", ":", ".", "!", "?", "“", "”", "‘", "’", "«", "»", "\"", "'", "(", ")", "[", "]", "<",
    ">", "{", "}", "/", "\\", "-", "—", "⸺", "_",
];
const PUNCTUATION_BRAILLE: [&'static str; 28] = [
    "⠂",
    "⠆",
    "⠒",
    "⠲",
    "⠖",
    "⠦",
    "⠘⠦",
    "⠘⠴",
    "⠠⠦",
    "⠠⠴",
    "⠸⠦",
    "⠸⠴",
    "⠠⠶",
    "⠄",
    "⠐⠣",
    "⠐⠜",
    "⠨⠣",
    "⠨⠜",
    "⠈⠣",
    "⠈⠜",
    "⠸⠣",
    "⠸⠜",
    "⠸⠌",
    "⠸⠡",
    "⠤",
    "⠠⠤",
    "⠐⠠⠤",
    "⠨⠤",
];

const NUMERIC: [&'static str; 23] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", ",", ".", " 1", " 2", " 3", " 4", " 5", " 6",
    " 7", " 8", " 9", " 0", "/",
];
const NUMERIC_BRAILLE: [&'static str; 23] = [
    "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊", "⠚", "⠂", "⠲", "⠐⠁", "⠐⠃", "⠐⠉", "⠐⠙", "⠐⠑", "⠐⠋",
    "⠐⠛", "⠐⠓", "⠐⠊", "⠐⠚", "⠌",
];

const ALPHABETIC_WORDSIGNS: [&'static str; 23] = [
    "but",
    "can",
    "do",
    "every",
    "from",
    "go",
    "have",
    "just",
    "knowledge",
    "like",
    "more",
    "not",
    "people",
    "quite",
    "rather",
    "so",
    "that",
    "us",
    "very",
    "it",
    "you",
    "as",
    "will",
];
const ALPHABETIC_WORDSIGNS_BRAILLE: [&'static str; 23] = [
    "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠚", "⠅", "⠇", "⠍", "⠝", "⠏", "⠟", "⠗", "⠎", "⠞", "⠥", "⠧",
    "⠭", "⠽", "⠵", "⠺",
];

// const STRONG_CONTRACTIONS: [&'static str; 5] = ["and", "for", "of", "the", "with"];
// const STRONG_GROUPSIGNS: [&'static str; 12] = [
//     "ch", "gh", "sh", "th", "wh", "ed", "er", "ou", "ow", "st", "ing", "ar",
// ];

lazy_static! {
    pub static ref LETTER_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(LETTERS.into_iter().zip(LETTERS_BRAILLE.into_iter()));
    pub static ref SYMBOL_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(SYMBOLS.into_iter().zip(SYMBOLS_BRAILLE.into_iter()));
    pub static ref PUNCTUATION_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(PUNCTUATION.into_iter().zip(PUNCTUATION_BRAILLE.into_iter()));
    pub static ref DIACRITIC_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(DIACRITIC.into_iter().zip(DIACRITIC_BRAILLE.into_iter()));
    pub static ref NUMERIC_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(NUMERIC.into_iter().zip(NUMERIC_BRAILLE.into_iter()));
    pub static ref ALPHABETIC_WORDSIGNS_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        ALPHABETIC_WORDSIGNS
            .into_iter()
            .zip(ALPHABETIC_WORDSIGNS_BRAILLE.into_iter())
    );
}

#[cfg(test)]
mod ueb_pairing_tests {
    use super::*;

    #[test]
    #[ignore = "symbol pairing test"]
    fn symbols() {
        println!("Symbols");
        for (a, b) in SYMBOLS.into_iter().zip(SYMBOLS_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "letter pairing test"]
    fn letters() {
        println!("Letters");
        for (a, b) in LETTERS.into_iter().zip(LETTERS_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "punctuation pairing test"]
    fn punctuation() {
        println!("Punctuation");
        for (a, b) in PUNCTUATION.into_iter().zip(PUNCTUATION_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "letter modifier pairing test"]
    fn letter_modifiers() {
        println!("Diacritics");
        for (a, b) in DIACRITIC.into_iter().zip(DIACRITIC_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "numeric pairing test"]
    fn numeric() {
        println!("Numeric");
        for (a, b) in NUMERIC.into_iter().zip(NUMERIC_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }
}
