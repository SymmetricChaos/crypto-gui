use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;

// pub const ITU_LETTERS: &'static str = "ABCDEÉFGHIJKLMNOPQRSTUVWXYZ1234567890.,:?'-/()\"=+@";

pub const ITU_SIGNS: [&'static str; 56] = [
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
    "[UNDERSTOOD]",
    "[ERROR]",
    "[WAIT]",
    "[OUT]",
    "[ATTENTION]",
    "[VERIFIED]",
];
pub const ITU_ASCII: [&'static str; 56] = [
    ".-", "-...", "-.-.", "-..", ".", "..-..", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
    "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
    "-----", ".-.-.-", "--..--", "---...", "..--..", ".---.", "-...-", "-..-.", "-.--.", "-.--.-",
    ".-..-.", "-...-", ".-.-.", ".--.-.", "...-.", "........", ".-...", "...-.-", "-.-.-", "...-.",
];
pub const ITU_WORD: [&'static str; 56] = [
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
    "di di di dah dit",
];
pub const ITU_HALFBLOCK: [&'static str; 56] = [
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
    "▄ ▄ ▄ ▄▄▄ ▄",
];

pub const GERKE_LETTERS: [&'static str; 40] = [
    "CH", "A", "Ä", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "O", "Ö", "P", "Q",
    "R", "S", "T", "U", "Ü", "V", "W", "X", "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "0", "?",
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

pub const GREEK_SIGNS: [&'static str; 24] = [
    "Α", "Β", "Γ", "Δ", "Ε", "Ζ", "Η", "Θ", "Ι", "Κ", "Λ", "Μ", "Ν", "Ξ", "Ο", "Π", "Ρ", "Σ", "Τ",
    "Υ", "Φ", "Χ", "Ψ", "Ω",
];
pub const GREEK_ASCII: [&'static str; 24] = [
    ".-", "-...", "--.", "-..", ".", "--..", "....", "-.-.", "..", "-.-", ".-..", "--", "-.",
    "-..-", "---", ".--.", ".-.", "...", "-", "-.--", "..-.", "----", "--.-", ".--",
];
pub const GREEK_WORD: [&'static str; 24] = [
    "di dah",
    "dah di di dit",
    "dah dah dit",
    "dah di dit",
    "dit",
    "dah dah di dit",
    "di di di dit",
    "dah di dah dit",
    "di dit",
    "dah di dah",
    "di dah di dit",
    "dah dah",
    "dah dit",
    "dah di di dah",
    "dah dah dah",
    "di dah dah dit",
    "di dah dit",
    "di di dit",
    "dah",
    "dah di dah dah",
    "di di dah dit",
    "dah dah dah dah",
    "dah dah di dah",
    "di dah dah",
];
pub const GREEK_HALFBLOCK: [&'static str; 24] = [
    "▄ ▄▄▄",
    "▄▄▄ ▄ ▄ ▄",
    "▄▄▄ ▄▄▄ ▄",
    "▄▄▄ ▄ ▄",
    "▄",
    "▄▄▄ ▄▄▄ ▄ ▄",
    "▄ ▄ ▄ ▄",
    "▄▄▄ ▄ ▄▄▄ ▄",
    "▄ ▄",
    "▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄ ▄",
    "▄▄▄ ▄▄▄",
    "▄▄▄ ▄",
    "▄▄▄ ▄ ▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄ ▄",
    "▄ ▄▄▄ ▄",
    "▄ ▄ ▄",
    "▄▄▄",
    "▄▄▄ ▄ ▄▄▄ ▄▄▄",
    "▄ ▄ ▄▄▄ ▄",
    "▄▄▄ ▄▄▄ ▄▄▄ ▄▄▄",
    "▄▄▄ ▄▄▄ ▄ ▄▄▄",
    "▄ ▄▄▄ ▄▄▄",
];

lazy_static! {
    pub static ref ITU_ASCII_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_ASCII.into_iter()));
    pub static ref ITU_WORD_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_WORD.into_iter()));
    pub static ref ITU_HALFBLOCK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(ITU_SIGNS.into_iter().zip(ITU_HALFBLOCK.into_iter()));
    pub static ref AMERICAN_HALFBLOCK_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        AMERICAN_LETTERS
            .into_iter()
            .zip(AMERICAN_HALFBLOCK.into_iter())
    );
    pub static ref GERKE_HALFBLOCK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GERKE_LETTERS.into_iter().zip(GERKE_HALFBLOCK.into_iter()));
    pub static ref GREEK_ASCII_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GREEK_SIGNS.into_iter().zip(GREEK_ASCII.into_iter()));
    pub static ref GREEK_WORD_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GREEK_SIGNS.into_iter().zip(GREEK_WORD.into_iter()));
    pub static ref GREEK_HALFBLOCK_MAP: BiMap<&'static str, &'static str> =
        bimap_from_iter(GREEK_SIGNS.into_iter().zip(GREEK_HALFBLOCK.into_iter()));
}

#[cfg(test)]
mod morseitu_tests {
    use super::*;

    fn convert_ascii(text: &[&'static str]) {
        // let mut out = Vec::new();
        // for i in text {
        //     let mut t = i.replace(".", "10");
        //     t = t.replace("-", "1110");
        //     t.replace(" ", "00");
        //     t.pop();
        //     out.push(t)
        // }
        // println!("\nBINARY:\n{:?}", out);

        let mut out = Vec::new();
        for i in text {
            let mut t = i.replace(". ", "dit   ");
            t = t.replace("- ", "dah   ");
            t = t.replace(".", "di ");
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
            let mut t = i.replace(" ", "  ");
            t = t.replace(".", "▄ ");
            t = t.replace("-", "▄▄▄ ");
            t.pop();
            out.push(t)
        }
        println!("\nHALFBLOCK:\n{:?}", out)
    }

    #[test]
    #[ignore = "conversions"]
    fn convert() {
        convert_ascii(&[
            ".-.-.", "-.-.-", "-...-", "-..-.", "-..--", "---.-", "..-.-", "..-..", ".-.--",
            "--..-", ".---.", "--.-.", ".--..", ".-..-", "-.-..", "-.--.", "-.---", ".-...",
            ".--.", "..-.", "---.", "----", "-.--", "-.-.", "...-", ".-..", "--.-", "-..-", ".---",
            "....", ".-.-", ".-.-.", "--..", "..--", "-...", "--.--", "---", "--.", "-.-", ".-.",
            "...", "..-", "-..", ".--", ".-", "--", "-.", ".", "-",
        ]);
        // convert_binary_to_halfblock(&ITU_BINARY);
        // convert_binary_to_halfblock(&AMERICAN_BINARY);
        // convert_binary_to_halfblock(&GERKE_BINARY);
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
