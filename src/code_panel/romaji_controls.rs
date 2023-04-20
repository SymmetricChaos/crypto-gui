use super::{View, ViewableCode};
use crate::{
    codes::{romaji::romaji::RomajiVariant, Romaji},
    egui_aux::mono_strong,
};

impl ViewableCode for Romaji {}

impl View for Romaji {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, RomajiVariant::Kunrei, "Kunrei");
            ui.selectable_value(&mut self.variant, RomajiVariant::Hepbern, "Hepbern");
            ui.selectable_value(&mut self.variant, RomajiVariant::Nihon, "Nihon");
        });

        match self.variant {
            RomajiVariant::Nihon => ui.label("Nihon-shiki is a now obsolete style of romanization. It keeps closely to morphology of Japanese with two letter per kana excepting ん (n) and the single vowel sounds and different consonants for every row of the . It does not reflect modern Japanese pronunciation of some kana."),
            RomajiVariant::Kunrei => ui.label("Kunrei-shiki is the style of romanization prefered by the government of Japan. While similar to Nihon-shiki it does not distinguish some kana because they are usually pronounced identically. For instance ぢ and じ are both romanized as 'zi'."),
            RomajiVariant::Hepbern => ui.label("Hepbern-shiki is commonly seen outside of Japan as it attempts to give phonetic spellings for kana, making it easier to read. In particular し is written 'shi', ち is written 'chi, and つ is written 'tsu'. Like Kunrein-shiki it does not distinguish some kana."),
        };

        ui.add_space(10.0);

        egui::Grid::new("romaji_grid")
            .num_columns(8)
            .striped(true)
            .show(ui, |ui| {
                let mut pairs = self.hiragana_codes();
                for _row in 0..16 {
                    for _col in 0..8 {
                        let (kana, romaji) = pairs.next().unwrap();
                        mono_strong(ui, &format!("{} {}  ", kana, romaji), Some(16.0));
                    }
                    ui.end_row()
                }
            });
    }
}
