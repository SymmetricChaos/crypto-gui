use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

use crate::{errors::CodeError, traits::Code};

const LINE1: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚";
const LINE2: &'static str = "⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞";
const LINE3: &'static str = "⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾";
const LINE4: &'static str = "⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺";
const LINE5: &'static str = "⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴";
const LINE6: &'static str = "⠌⠬⠼⠜⠄⠤";
const LINE7: &'static str = "⠈⠘⠸⠐⠨⠰⠠";

const MODE_ENGLISH: [&'static str; 5] = [
    "[shape]",
    "[arrow]",
    "[numeric]",
    "[horizontal-line]",
    "[grade-1]",
];
const MODE_BRAILLE: [&'static str; 5] = ["⠫", "⠳", "⠼", "⠐⠒", "⠰"];

const INDIC_ENGLISH: [&'static str; 8] = [
    "[subscript]",
    "[superscript]",
    "[script]",
    "[bold]",
    "[ligature]",
    "[underline]",
    "[italic]",
    "[capital]",
];
const INDIC_BRAILLE: [&'static str; 8] = ["⠢", "⠔", "⠈⠆", "⠘⠆", "⠘⠖", "⠸⠆", "⠨⠆", "⠠⠠"];

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
