use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

pub const LETTERS: [&'static str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "ŋ", "ə", "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ",
    "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω",
];
pub const LETTERS_BRAILLE: [&'static str; 52] = [
    "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊", "⠚", "⠅", "⠇", "⠍", "⠝", "⠕", "⠏", "⠟", "⠗", "⠎",
    "⠞", "⠥", "⠧", "⠺", "⠭", "⠽", "⠵", "⠘⠝", "⠸⠢", "⠨⠁", "⠨⠃", "⠨⠛", "⠨⠙", "⠨⠑", "⠨⠵", "⠨⠱", "⠨⠹",
    "⠨⠊", "⠨⠅", "⠨⠇", "⠨⠍", "⠨⠝", "⠨⠭", "⠨⠕", "⠨⠏", "⠨⠗", "⠨⠎", "⠨⠞", "⠨⠥", "⠨⠋", "⠨⠯", "⠨⠽", "⠨⠺",
];

pub const LETTERS_UPPER: [&'static str; 52] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "Ŋ", "Ə", "Α", "Β", "Γ", "Δ", "Ε", "Ζ", "Η", "Θ", "Ι", "Κ",
    "Λ", "Μ", "Ν", "Ξ", "Ο", "Π", "Ρ", "Σ", "Τ", "Υ", "Φ", "Χ", "Ψ", "Ω",
];
// const LETTERS_UPPER_BRAILLE: [&'static str; 52] = [
//     "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊", "⠚", "⠅", "⠇", "⠍", "⠝", "⠕", "⠏", "⠟", "⠗", "⠎",
//     "⠞", "⠥", "⠧", "⠺", "⠭", "⠽", "⠵", "⠘⠝", "⠸⠢", "⠨⠁", "⠨⠃", "⠨⠛", "⠨⠙", "⠨⠑", "⠨⠵", "⠨⠱", "⠨⠹",
//     "⠨⠊", "⠨⠅", "⠨⠇", "⠨⠍", "⠨⠝", "⠨⠭", "⠨⠕", "⠨⠏", "⠨⠗", "⠨⠎", "⠨⠞", "⠨⠥", "⠨⠋", "⠨⠯", "⠨⠽", "⠨⠺",
// ];

pub const DIACRITIC: [&'static str; 12] = ["̸", "̶", "̆", "̄", "̧", "̀", "̂", "̊", "̃", "̈", "́", "̌"];
pub const DIACRITIC_DISPLAY: [&'static str; 12] =
    ["̸◌", "̶◌", "̆◌", "̄◌", "̧◌", "̀◌", "̂◌", "̊◌", "̃◌", "̈◌", "́◌", "̌◌"];
pub const DIACRITIC_BRAILLE: [&'static str; 12] = [
    "⠈⠡", "⠈⠒", "⠈⠬", "⠈⠤", "⠘⠯", "⠘⠡", "⠘⠩", "⠘⠫", "⠘⠻", "⠘⠒", "⠘⠌", "⠘⠬",
];

pub const SYMBOLS: [&'static str; 45] = [
    "→", "↓", "←", "↑", "∶", "∷", "′", "″", "♮", "♭", "♯", "@", "¢", "€", "₣", "£", "₦", "$", "¥",
    "&", "<", "^", "~", ">", "†", "‡", "©", "°", "¶", "®", "§", "™", "♀", "♂", "#", "•", "〃", "+",
    "=", "×", "*", "÷", "-", "%", "✓",
];
pub const SYMBOLS_BRAILLE: [&'static str; 45] = [
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
    "⠈⠩",
];

// The dashes are Unicode specified: hyphen, em-dash, and double em-dash.
pub const PUNCTUATION: [&'static str; 29] = [
    ",", ";", ":", ".", "!", "?", "“", "”", "‘", "’", "«", "»", "\"", "'", "(", ")", "[", "]", "<",
    ">", "{", "}", "/", "\\", "-", "—", "⸺", "_", "…",
];
pub const PUNCTUATION_BRAILLE: [&'static str; 29] = [
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
    "⠲⠲⠲",
];

pub const NUMERIC: [&'static str; 23] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", ",", ".", "/", " 1", " 2", " 3", " 4", " 5",
    " 6", " 7", " 8", " 9", " 0",
];
pub const NUMERIC_BRAILLE: [&'static str; 23] = [
    "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊", "⠚", "⠂", "⠲", "⠌", "⠐⠁", "⠐⠃", "⠐⠉", "⠐⠙", "⠐⠑",
    "⠐⠋", "⠐⠛", "⠐⠓", "⠐⠊", "⠐⠚",
];

pub const TYPEFORM_PREFIX: [&'static str; 4] = ["italic", "boldface", "underlined", "script"];
pub const TYPEFORM_PREFIX_BRAILLE: [&'static str; 4] = ["⠨", "⠘", "⠸", "⠈"];

pub const TYPEFORM_ROOT: [&'static str; 4] = ["symbol", "word", "passage", "terminator"];
pub const TYPEFORM_ROOT_BRAILLE: [&'static str; 4] = ["⠆", "⠂", "⠶", "⠄"];

// pub const TYPEFORMS: [&'static str; 16] = [
//     "italic symbol",
//     "italic word",
//     "italic passage",
//     "italic terminator",
//     "bold symbol",
//     "bold word",
//     "bold passage",
//     "bold terminator",
//     "underlined symbol",
//     "underlined word",
//     "underlined passage",
//     "underlined terminator",
//     "script symbol",
//     "script word",
//     "script passage",
//     "script terminator",
// ];
// pub const TYPEFORMS_BRAILLE: [&'static str; 16] = [
//     "⠨⠆", "⠨⠂", "⠨⠶", "⠨⠄", "⠘⠆", "⠘⠂", "⠘⠶", "⠘⠄", "⠸⠆", "⠸⠂", "⠸⠶", "⠸⠄", "⠈⠆", "⠈⠂", "⠈⠶", "⠈⠄",
// ];

pub const ALPHABETIC_WORDSIGNS: [&'static str; 23] = [
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
pub const ALPHABETIC_WORDSIGNS_BRAILLE: [&'static str; 23] = [
    "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠚", "⠅", "⠇", "⠍", "⠝", "⠏", "⠟", "⠗", "⠎", "⠞", "⠥", "⠧",
    "⠭", "⠽", "⠵", "⠺",
];

pub const STRONG_WORDSIGNS: [&'static str; 6] = ["child", "shall", "this", "which", "out", "still"];
pub const STRONG_WORDSIGNS_BRAILLE: [&'static str; 6] = ["⠡", "⠩", "⠹", "⠱", "⠳", "⠌"];

pub const STRONG_CONTRACTIONS: [&'static str; 5] = ["and", "for", "of", "the", "with"];
pub const STRONG_CONTRACTIONS_BRAILLE: [&'static str; 5] = ["⠽", "⠿", "⠷", "⠮", "⠾"];

pub const STRONG_GROUPSIGNS: [&'static str; 12] = [
    "ch", "gh", "sh", "th", "wh", "ed", "er", "ou", "ow", "st", "ing", "ar",
];
pub const STRONG_GROUPSIGNS_BRAILLE: [&'static str; 12] =
    ["⠡", "⠣", "⠩", "⠹", "⠱", "⠫", "⠻", "⠳", "⠪", "⠌", "⠬", "⠜"];

// Only used within a word meaning: preceeded and followed by a letter, contraction, or modified letter
pub const LOWER_GROUPSIGNS: [&'static str; 7] = ["ea", "bb", "cc", "en", "ff", "gg", "in"];
pub const LOWER_GROUPSIGNS_BRAILLE: [&'static str; 7] = ["⠂", "⠆", "⠒", "⠢", "⠖", "⠶", "⠔"];

// Only used used at the start of a world followed by a letter, contraction, or modified letter
pub const LOWER_GROUPSIGNS_STARTING: [&'static str; 3] = ["be", "con", "dis"];
pub const LOWER_GROUPSIGNS_STARTING_BRAILLE: [&'static str; 3] = ["⠆", "⠆", "⠲"];

pub const INITIAL_LETTER_CONTRACTIONS: [&'static str; 33] = [
    "upon",
    "these",
    "those",
    "whose",
    "word",
    "cannot",
    "had",
    "many",
    "spirit",
    "their",
    "world",
    "day",
    "ever",
    "father",
    "here",
    "know",
    "lord",
    "mother",
    "name",
    "one",
    "part",
    "question",
    "right",
    "some",
    "time",
    "under",
    "young",
    "there",
    "character",
    "through",
    "where",
    "ought",
    "work",
];

pub const FINAL_LETTER_GROUPSIGNS: [&'static str; 12] = [
    "ound", "ance", "sion", "less", "ount", "ence", "ong", "ful", "tion", "ness", "ment", "ity",
];

pub const SHORTFORMS: [&'static str; 75] = [
    "about",
    "above",
    "according",
    "accross",
    "after",
    "afternoon",
    "afterward",
    "again",
    "against",
    "also",
    "almost",
    "already",
    "altogether",
    "although",
    "always",
    "blind",
    "braille",
    "could",
    "declare",
    "declaring",
    "deceive",
    "deceiving",
    "either",
    "friend",
    "first",
    "good",
    "great",
    "him",
    "himself",
    "herself",
    "immediate",
    "little",
    "letter",
    "myself",
    "much",
    "must",
    "necessary",
    "neither",
    "paid",
    "perceive",
    "perceiving",
    "perhaps",
    "quick",
    "receive",
    "receiving",
    "rejoice",
    "rejoicing",
    "said",
    "such",
    "today",
    "together",
    "tomorrow",
    "tonight",
    "itself",
    "its",
    "your",
    "yourself",
    "yourselves",
    "themselves",
    "children",
    "should",
    "thyself",
    "ourselves",
    "would",
    "because",
    "before",
    "behind",
    "below",
    "beneath",
    "beside",
    "between",
    "beyond",
    "conceive",
    "conceiving",
    "oneself",
];

lazy_static! {
    pub static ref LETTER_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(LETTERS.into_iter().zip(LETTERS_BRAILLE.into_iter()));
    pub static ref LETTER_UPPER_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(LETTERS_UPPER.into_iter().zip(LETTERS_BRAILLE.into_iter()));
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
