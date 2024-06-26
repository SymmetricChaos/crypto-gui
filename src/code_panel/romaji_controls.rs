use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::romaji::romaji::{Romaji, RomajiVariant};
use egui::RichText;

pub struct RomajiFrame {
    code: Romaji,
}

impl Default for RomajiFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for RomajiFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/codes/src/romaji",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Variant");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.variant, RomajiVariant::Kunrei, "Kunrei");
                ui.selectable_value(&mut self.code.variant, RomajiVariant::Hepbern, "Hepbern");
                ui.selectable_value(&mut self.code.variant, RomajiVariant::Nihon, "Nihon");
            });
        });
        ui.add_space(16.0);

        match self.code.variant {
            RomajiVariant::Nihon => ui.label("Nihon-shiki is a now obsolete style of romanization. It keeps closely to morphology of Japanese with two letter per kana excepting ん (n) and the single vowel sounds and different consonants for every row of the gojūon, making it a one-to-one encoding. It does not reflect modern Japanese pronunciation of some kana."),
            RomajiVariant::Kunrei => ui.label("Kunrei-shiki is the style of romanization prefered by the government of Japan. While similar to Nihon-shiki it does not distinguish some kana because they are usually pronounced identically. For instance ぢ and じ are both romanized as 'zi'."),
            RomajiVariant::Hepbern => ui.label("Hepbern-shiki is the style of romanization commonly seen outside of Japan as it attempts to give English phonetic spellings for kana, making it easier to read. In particular し is written 'shi', ち is written 'chi, and つ is written 'tsu'. Like Kunrei-shiki it does not distinguish some kana."),
        };
        ui.add_space(16.0);

        ui.subheading("Gojūon with dakuten, handakuten, and yōon.");
        ui.add_space(16.0);
        egui::Grid::new("romaji_grid")
            .num_columns(8)
            .striped(true)
            .show(ui, |ui| {
                let mut pairs = self.code.hiragana_codes();
                for _row in 0..16 {
                    for _col in 0..8 {
                        let (kana, romaji) = pairs.next().unwrap();
                        ui.label(RichText::new(format!("{} {}  ", kana, romaji)).size(16.0));
                    }
                    ui.end_row()
                }
            });
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
