use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

// The 64 possible Braille cells as organized by UEB specification, excluding the space
// Unicode Braille space: "⠀" <- right there
pub const LINE1: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚";
pub const LINE2: &'static str = "⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞";
pub const LINE3: &'static str = "⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾";
pub const LINE4: &'static str = "⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺";
pub const LINE5: &'static str = "⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴";
pub const LINE6: &'static str = "⠌⠬⠼⠜⠄⠤";
pub const LINE7: &'static str = "⠈⠘⠸⠐⠨⠰⠠";

pub const BRAILLE_ORDER: [&'static str; 7] = [LINE1, LINE2, LINE3, LINE4, LINE5, LINE6, LINE7];

// These eight characters are the UEB prefixes. All others characters are called roots as is the space.
const PREFIXES: &'static str = "⠼⠈⠘⠸⠐⠨⠰⠠";

pub enum LetterModifier {
    Soldius,
    HorizontalStroke,
    Breve,
    Macron,
    Cedilla,
    GraveAccent,
    Circumflex,
    Ring,
    Tilde,
    Diaeresis,
    AcuteAccent,
    Caron,
}

impl LetterModifier {
    pub fn to_symbol(&self) -> char {
        match self {
            Self::Soldius => '̸',
            Self::HorizontalStroke => '̶',
            Self::Breve => '̆',
            Self::Macron => '̄',
            Self::Cedilla => '̧',
            Self::GraveAccent => '̀',
            Self::Circumflex => '̂',
            Self::Ring => '̊',
            Self::Tilde => '̃',
            Self::Diaeresis => '̈',
            Self::AcuteAccent => '́',
            Self::Caron => '̌',
        }
    }
}

pub enum UebMode {
    Shape,
    Arrow,
    Numeric,
    HorizontalLine,
    GradeOne,
}

impl UebMode {
    pub const BRAILLE: [&'static str; 5] = ["⠫", "⠳", "⠼", "⠐⠒", "⠰"];
    pub const LATIN: [&'static str; 5] = [
        "[shape]",
        "[arrow]",
        "[numeric]",
        "[horizontal-line]",
        "[grade-1]",
    ];

    pub fn as_braille(&self) -> &'static str {
        match self {
            Self::Shape => Self::BRAILLE[0],
            Self::Arrow => Self::BRAILLE[1],
            Self::Numeric => Self::BRAILLE[2],
            Self::HorizontalLine => Self::BRAILLE[3],
            Self::GradeOne => Self::BRAILLE[4],
        }
    }

    pub fn from_braille(s: &str) -> Option<Self> {
        match s {
            "⠫" => Some(Self::Shape),
            "⠳" => Some(Self::Arrow),
            "⠼" => Some(Self::Numeric),
            "⠐⠒" => Some(Self::HorizontalLine),
            "⠰" => Some(Self::GradeOne),
            _ => None,
        }
    }

    pub fn as_latin(&self) -> &'static str {
        match self {
            Self::Shape => Self::LATIN[0],
            Self::Arrow => Self::LATIN[1],
            Self::Numeric => Self::LATIN[2],
            Self::HorizontalLine => Self::LATIN[3],
            Self::GradeOne => Self::LATIN[4],
        }
    }

    pub fn from_latin(s: &str) -> Option<Self> {
        match s {
            "[shape]" => Some(Self::Shape),
            "[arrow]" => Some(Self::Arrow),
            "[numeric]" => Some(Self::Numeric),
            "[horizontal-line]" => Some(Self::HorizontalLine),
            "[grade-1]" => Some(Self::GradeOne),
            _ => None,
        }
    }
}

pub enum UebIndicator {
    Subscript,
    Superscript,
    Script,
    Bold,
    Ligature,
    Underline,
    Italic,
    Capital,
}

impl UebIndicator {
    pub const BRAILLE: [&'static str; 8] = ["⠢", "⠔", "⠈⠆", "⠘⠆", "⠘⠖", "⠸⠆", "⠨⠆", "⠠⠠"];
    pub const LATIN: [&'static str; 8] = [
        "[subscript]",
        "[superscript]",
        "[script]",
        "[bold]",
        "[ligature]",
        "[underline]",
        "[italic]",
        "[capital]",
    ];

    pub fn as_braille(&self) -> &'static str {
        match self {
            Self::Subscript => Self::BRAILLE[0],
            Self::Superscript => Self::BRAILLE[1],
            Self::Script => Self::BRAILLE[2],
            Self::Bold => Self::BRAILLE[3],
            Self::Ligature => Self::BRAILLE[4],
            Self::Underline => Self::BRAILLE[5],
            Self::Italic => Self::BRAILLE[6],
            Self::Capital => Self::BRAILLE[7],
        }
    }

    pub fn from_braille(s: &str) -> Option<Self> {
        match s {
            "⠢" => Some(Self::Subscript),
            "⠔" => Some(Self::Superscript),
            "⠈⠆" => Some(Self::Script),
            "⠘⠆" => Some(Self::Bold),
            "⠘⠖" => Some(Self::Ligature),
            "⠸⠆" => Some(Self::Underline),
            "⠨⠆" => Some(Self::Italic),
            "⠠⠠" => Some(Self::Capital),
            _ => None,
        }
    }

    pub fn as_latin(&self) -> &'static str {
        match self {
            Self::Subscript => Self::LATIN[0],
            Self::Superscript => Self::LATIN[1],
            Self::Script => Self::LATIN[2],
            Self::Bold => Self::LATIN[3],
            Self::Ligature => Self::LATIN[4],
            Self::Underline => Self::LATIN[5],
            Self::Italic => Self::LATIN[6],
            Self::Capital => Self::LATIN[7],
        }
    }

    pub fn from_latin(s: &str) -> Option<Self> {
        match s {
            "[subscript]" => Some(Self::Subscript),
            "[superscript]" => Some(Self::Superscript),
            "[script]" => Some(Self::Script),
            "[bold]" => Some(Self::Bold),
            "[ligature]" => Some(Self::Ligature),
            "[underline]" => Some(Self::Underline),
            "[italic]" => Some(Self::Italic),
            "[capital]" => Some(Self::Capital),
            _ => None,
        }
    }
}

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LETTERS_BRAILLE: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵";

const GREEK: [&'static str; 24] = [
    "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "τ",
    "υ", "φ", "χ", "ψ", "ω",
];
// Preceeded by ⠨ prefix
const GREEK_BRAILLE: [&'static str; 24] = [
    "⠁", "⠃", "⠛", "⠙", "⠑", "⠵", "⠱", "⠹", "⠊", "⠅", "⠇", "⠍", "⠝", "⠭", "⠕", "⠏", "⠗", "⠎", "⠞",
    "⠥", "⠋", "⠯", "⠽", "⠺",
];

const SYMBOLS: &'static str = "→↓←↑∶∷′″♮♭♯@¢€₣£₦$¥&<^~>†‡©°¶®§™♀♂#•〃+=×*÷-%";
const SYMBOLS_BRAILLE: &[&'static str] = &[
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

const DIACRITIC: &[&'static str] = &["̸", "̶", "̆", "̄", "̧", "̀", "̂", "̊", "̃", "̈", "́", "̌"];
const DIACRITIC_BRAILLE: &[&'static str] = &[
    "⠈⠡", "⠈⠒", "⠈⠬", "⠈⠤", "⠘⠯", "⠘⠡", "⠘⠩", "⠘⠫", "⠘⠻", "⠘⠒", "⠘⠌", "⠘⠬",
];

// Ellipsis is a specific punctuation symbol listed for UEB but it written as a sequence of full stops
// The dashes are Unicode specifier: en-dash, em-dash, and double em-dash.
const PUNCTUATION: &'static str = ",;:.!?“”‘’«»\"'()[]<>{}/\\–—⸺_";
const PUNCTUATION_BRAILLE: &[&'static str] = &[
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

lazy_static! {
    pub static ref LETTER_MAP: BiMap<char, char> =
        bimap_from_iter(LETTERS.chars().zip(LETTERS_BRAILLE.chars()));
    pub static ref MODE_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(UebMode::LATIN.into_iter().zip(UebMode::BRAILLE.into_iter()));
    pub static ref INDICATOR_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        UebIndicator::LATIN
            .into_iter()
            .zip(UebIndicator::BRAILLE.into_iter())
    );
    pub static ref SYMBOL_MAP: BiMap<char, &'static str> =
        bimap_from_iter(SYMBOLS.chars().zip(SYMBOLS_BRAILLE.into_iter().copied()));
    pub static ref PUNCTUATION_MAP: BiMap<char, &'static str> = bimap_from_iter(
        PUNCTUATION
            .chars()
            .zip(PUNCTUATION_BRAILLE.into_iter().copied())
    );
    pub static ref DIACRITIC_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        DIACRITIC
            .into_iter()
            .copied()
            .zip(DIACRITIC_BRAILLE.into_iter().copied())
    );
    pub static ref GREEK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GREEK.into_iter().zip(GREEK_BRAILLE.into_iter()));
}

#[cfg(test)]
mod ueb_pairing_tests {
    use super::*;

    #[test]
    #[ignore = "symbol pairing test"]
    fn symbols() {
        println!("Symbols");
        for (a, b) in SYMBOLS.chars().zip(SYMBOLS_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "letter pairing test"]
    fn letters() {
        println!("Letters");
        for (a, b) in LETTERS.chars().zip(LETTERS_BRAILLE.chars()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "punctuation pairing test"]
    fn punctuation() {
        println!("Punctuation");
        for (a, b) in PUNCTUATION.chars().zip(PUNCTUATION_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }

    #[test]
    #[ignore = "letter modifier pairing test"]
    fn letter_modifiers() {
        println!("Letter Modifiers");
        for (a, b) in DIACRITIC.into_iter().zip(DIACRITIC_BRAILLE.into_iter()) {
            println!("{} {}", a, b)
        }
    }
}
