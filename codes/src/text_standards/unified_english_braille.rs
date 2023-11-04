use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

use crate::{errors::CodeError, traits::Code};

pub const LINE1: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚";
pub const LINE2: &'static str = "⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞";
pub const LINE3: &'static str = "⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾";
pub const LINE4: &'static str = "⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺";
pub const LINE5: &'static str = "⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴";
pub const LINE6: &'static str = "⠌⠬⠼⠜⠄⠤";
pub const LINE7: &'static str = "⠈⠘⠸⠐⠨⠰⠠";

pub const BRAILLE_ORDER: [&'static str; 7] = [LINE1, LINE2, LINE3, LINE4, LINE5, LINE6, LINE7];

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

// const SYMBOLS_ENGLISH [&'static str; 20] = [
//     "→", "↓", "←",
// ]

// const SYMBOLS_BRAILLE [&'static str; 20] = [
//     "→", "↓", "←",
// ]

const LETTERS_LATIN: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LETTERS_BRAILLE: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵";

lazy_static! {
    pub static ref LETTER_MAP: BiMap<char, char> =
        bimap_from_iter(LETTERS_LATIN.chars().zip(LETTERS_BRAILLE.chars()));
    pub static ref MODE_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(UebMode::LATIN.into_iter().zip(UebMode::BRAILLE.into_iter()));
    pub static ref INDICATOR_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        UebIndicator::LATIN
            .into_iter()
            .zip(UebIndicator::BRAILLE.into_iter())
    );
}

pub struct UnifiedEnglishBraille {}

impl Default for UnifiedEnglishBraille {
    fn default() -> Self {
        Self {}
    }
}

impl Code for UnifiedEnglishBraille {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}
