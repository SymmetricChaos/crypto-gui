use egui::RichText;

use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::braille::{
    braille_data::UEB_ROWS,
    unified_english_braille::UnifiedEnglishBraille,
    unified_english_braille_maps::{
        DIACRITIC_BRAILLE, DIACRITIC_DISPLAY, NUMERIC, NUMERIC_BRAILLE, PUNCTUATION,
        PUNCTUATION_BRAILLE, SYMBOLS, SYMBOLS_BRAILLE,
    },
};

#[derive(Debug, PartialEq, Eq)]
enum UebInfo {
    Alphabet,
    Punctuation,
    Symbols,
    Capitalization,
    Numbers,
}

pub struct UebFrame {
    code: UnifiedEnglishBraille,
    info: UebInfo,
}

impl Default for UebFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            info: UebInfo::Alphabet,
        }
    }
}

impl CodeFrame for UebFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/codes/src/braille",
        );
        ui.add_space(8.0);

        ui.subheading("Braille Order");
        egui::Grid::new("columnar_grid")
            .num_columns(10)
            .min_col_width(5.0)
            .striped(true)
            .show(ui, |ui| {
                for row in 0..7 {
                    let mut cells = UEB_ROWS[row].chars();
                    for _col in 0..10 {
                        if let Some(c) = cells.next() {
                            ui.label(RichText::from(c.to_string()).monospace().size(24.0));
                        }
                    }
                    ui.end_row();
                }
            });

        ui.add_space(16.0);
        ui.group(|ui| {
            ui.subheading("Information");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.info, UebInfo::Alphabet, "Alphabet");
                ui.selectable_value(&mut self.info, UebInfo::Punctuation, "Punctuation");
                ui.selectable_value(&mut self.info, UebInfo::Symbols, "Symbols");
                ui.selectable_value(&mut self.info, UebInfo::Capitalization, "Capitalization");
                ui.selectable_value(&mut self.info, UebInfo::Numbers, "Numeric Mode");
            });
        });

        ui.add_space(8.0);
        match self.info {
            UebInfo::Alphabet => ui.subheading("A letter in UEB can be any of the 26 letters of the English alphabet, any of the 24 letters of the Greek alphabet (mainly for use in technical literature), and the characters representing the eng (ŋ) and schwa (ə). Any of these may be preceeded by diacritical marks and the capitalization symbol."),
            UebInfo::Punctuation => ui.subheading("A wide array of punctuation is included in UEB."),
            UebInfo::Symbols => ui.subheading("Various symbols are included in UEB."),
            UebInfo::Capitalization => ui.subheading("A single letter is capitalized by prepending the capitalization symbol. A sequence of letters can be capitalized by prepending the capitalization symbol twice. Finally with three capitalization symbols a capitalized passage is created, meaning every letter symbol is treated as capitalized while other symbols are included unchanged."),
            UebInfo::Numbers => ui.subheading("A sequence of symbols can be read as numeric symbols by prepending the numeric indicator. Spaces within a number, used for grouping, are represented with the Braille numeric space."),
        };

        // ui.add_space(8.0);
        // ui.subheading("Examples");
        // match self.info {
        //     UebInfo::Alphabet => ui.label("The Grand Façade\n⠠⠞⠓⠑⠀⠠⠛⠗⠁⠝⠙⠀⠠⠋⠁⠘⠯⠉⠁⠙⠑"),
        //     UebInfo::Punctuation => ui.label("<<<TODO>>>"),
        //     UebInfo::Symbols => ui.label("<<<TODO>>>"),
        //     UebInfo::Capitalization => ui.label("<<<TODO>>>"),
        //     UebInfo::Numbers => ui.label("<<<TODO>>>"),
        // };

        ui.add_space(16.0);
        match self.info {
            UebInfo::Alphabet => {
                ui.subheading("Letters");
                ui.fill_code_columns(
                    13,
                    4,
                    Box::new(
                        UnifiedEnglishBraille::alphabet_triples()
                            .map(|(b, l, u)| (b, format!("{}  {}", l, u))),
                    ),
                );
                ui.add_space(8.0);
                ui.subheading("Diacritical Marks");
                ui.fill_code_columns(
                    3,
                    4,
                    Box::new(
                        DIACRITIC_DISPLAY
                            .into_iter()
                            .zip(DIACRITIC_BRAILLE.into_iter()),
                    ),
                );
            }
            UebInfo::Punctuation => ui.fill_code_columns(
                8,
                4,
                Box::new(PUNCTUATION.into_iter().zip(PUNCTUATION_BRAILLE.into_iter())),
            ),
            UebInfo::Symbols => ui.fill_code_columns(
                12,
                4,
                Box::new(SYMBOLS.into_iter().zip(SYMBOLS_BRAILLE.into_iter())),
            ),
            UebInfo::Capitalization => {
                ui.subheading("Capital Symbol Indicator:   ⠠");
                ui.subheading("Capital Sequence Indicator: ⠠⠠");
                ui.subheading("Capital Passage Indicator:  ⠠⠠⠠");
            }
            UebInfo::Numbers => {
                ui.subheading("Numeric Indicator: ⠼");
                ui.fill_code_columns(
                    6,
                    4,
                    Box::new(NUMERIC.into_iter().zip(NUMERIC_BRAILLE.into_iter())),
                )
            }
        };
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
