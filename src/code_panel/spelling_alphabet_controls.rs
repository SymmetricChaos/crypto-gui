use codes::text_standards::spelling_alphabet::{SpellingAlphabet, SpellingAlphabetMode};

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct SpellingAlphabetFrame {
    code: SpellingAlphabet,
}

impl Default for SpellingAlphabetFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for SpellingAlphabetFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/spelling_alphabet.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Alphabet");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Nato,
                    "NATO/ICAO (1956)",
                );
                ui.selectable_value(&mut self.code.variant, SpellingAlphabetMode::Ccb, "CCB");
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Uka1904,
                    "UK Army (1904)",
                );
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Usn1908,
                    "US Navy, long (1908)",
                );
            });
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Usn1908Alt,
                    "US Navy, short (1908)",
                );
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Wu1912,
                    "Western Union (1912)",
                );
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Wu1942,
                    "Western Union (1942)",
                );
            });
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::Us1941,
                    "US Joint Army/Navy (1941)",
                );

                ui.selectable_value(
                    &mut self.code.variant,
                    SpellingAlphabetMode::FirstLetter,
                    "First Character",
                );
            })
        });

        ui.add_space(16.0);

        match self.code.variant {
            SpellingAlphabetMode::Nato => ui.label("The most widely used international standard today is the one specified by the ICAO, though it is commonly called the NATO Phonetic Alphabet. Note the intentionally unusual spellings for ALFA (for non-English speakers generally) and JULIETT (for French speakers)."),
            SpellingAlphabetMode::Ccb => ui.label("The Combined Communications Board was formed during World War II to improve the interoperability of communications systems for UK and US forces. It was the immediate predececessor to the NATO/ICAO standard."),
            SpellingAlphabetMode::Wu1912 => ui.label("In 1912 Western Union introduced a spelling alphabet for its employees that mostly used place names to represent letters."),
            SpellingAlphabetMode::Wu1942 => ui.label("The 1942 update to the Western Union standard."),
            SpellingAlphabetMode::Usn1908 => ui.label("The first of two recommendations by the US Navy in 1908, this version using two syllable words."),
            SpellingAlphabetMode::Usn1908Alt => ui.label("The first of two recommendations by the US Navy in 1908, this version using one syllable words."),
            SpellingAlphabetMode::Us1941 => ui.label("A US military standard created for joint use by the Army and Navy."),
            SpellingAlphabetMode::FirstLetter => ui.label("Arbitrary words can be used for a phonetic alphabet. Most commonly the first letter of each word is the letter being represented. This decodes any sequence of words into their first letters but doesn't provide encoding."),
            SpellingAlphabetMode::Uka1904 => ui.label("This very early standard from the UK Army Signalling Regulations in 1904 is unusual in several ways. The non-word 'ACK' is used for A, rhyming words EDDY and FREDDY are both present, EMMA and ESSES do not start with the letter they represent."),
        };
        ui.add_space(16.0);
        ui.fill_code_columns(9, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
