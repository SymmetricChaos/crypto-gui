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
    pub const ENGLISH: [&'static str; 5] = [
        "[shape]",
        "[arrow]",
        "[numeric]",
        "[horizontal-line]",
        "[grade-1]",
    ];

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

    pub fn from_english(s: &str) -> Option<Self> {
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
    pub const ENGLISH: [&'static str; 8] = [
        "[subscript]",
        "[superscript]",
        "[script]",
        "[bold]",
        "[ligature]",
        "[underline]",
        "[italic]",
        "[capital]",
    ];

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

    pub fn from_english(s: &str) -> Option<Self> {
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

const LETTERS_ENGLISH: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LETTERS_BRAILLE: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵";

lazy_static! {
    pub static ref LETTER_MAP: BiMap<char, char> =
        bimap_from_iter(LETTERS_ENGLISH.chars().zip(LETTERS_BRAILLE.chars()));
    pub static ref MODE_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        UebMode::ENGLISH
            .into_iter()
            .zip(UebMode::BRAILLE.into_iter())
    );
    pub static ref INDICATOR_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        UebIndicator::ENGLISH
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
