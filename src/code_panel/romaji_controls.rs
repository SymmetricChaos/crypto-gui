use super::{View, ViewableCode};
use crate::codes::{romaji::romaji::RomajiVariant, Romaji};

impl ViewableCode for Romaji {}

impl View for Romaji {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, RomajiVariant::Kunrei, "Kunrei");
            ui.selectable_value(&mut self.variant, RomajiVariant::Hepbern, "Hepbern");
            ui.selectable_value(&mut self.variant, RomajiVariant::Nihon, "Nihon");
        });

        match self.variant {
            RomajiVariant::Nihon => ui.label("Nihon-shiki is a now obsolete style of romanization. It keeps closely to morphology of Japanese with two letter per kana excepting ん (n) and the single vowel sounds. It does not reflect modern Japanese pronunciation of some kana."),
            RomajiVariant::Kunrei => ui.label("Kunrei-shiki is the style of romanization prefered by the government of Japan. While similar to Nihon-shiki it does not distinguish some kana because they are usually pronounced identically. For instance ぢ and じ are both romanized as `zi` whereas Nihon-shiki renders them as `di` and ``zi`."),
            RomajiVariant::Hepbern => ui.label("Hepbern-shiki is commonly seen outside of Japan as it attempts to give phonetic spellings for kana, making it easier to read. In particular instance ち is written `chi`, し is written `shi`, and つ is written `tsu`. Like Kunrein-shiki it does not distinguish some kana."),
        };
    }
}
