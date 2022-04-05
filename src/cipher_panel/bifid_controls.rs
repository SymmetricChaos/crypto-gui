use eframe::egui::Ui;
use eframe::egui::{RichText, Slider, TextEdit};
use eframe::epaint::Color32;
use rand::prelude::StdRng;
use super::View;
use super::generic_components::*;
use crate::ciphers::Bifid;
use crate::preset_alphabet::PresetAlphabet::*;

impl View for Bifid {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.block_size, block_size_range));

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() { self.polybius.set_alphabet(BasicLatinNoQ) };
            if ui.button("No J").clicked() { self.polybius.set_alphabet(BasicLatinNoJ) };
            if ui.button("Alphanumeric").clicked() { self.polybius.set_alphabet(BasicLatinWithDigits) };
            if ui.button("Base64").clicked() { self.polybius.set_alphabet(Base64) };
        });

        ui.add_space(10.0);
        ui.label(RichText::new(&self.polybius.alphabet).monospace().background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(self.polybius.control_key()));

        ui.label(RichText::new(format!("Grid\n{}",self.polybius)).monospace());
        ui.add_space(16.0);

    }
}
