use eframe::egui::{RichText,Color32};

use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::ciphers::Playfair;
use crate::ciphers::playfair::PlayfairMode;


impl View for Playfair {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() { self.set_mode(PlayfairMode::NoQ) };
            if ui.button("No J").clicked() { self.set_mode(PlayfairMode::NoJ) };
            if ui.button("Alphanumeric").clicked() { self.set_mode(PlayfairMode::AlphaNum) };
        });
        ui.add_space(10.0);
        ui.label(RichText::new(self.get_mut_input_alphabet().clone()).monospace().background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label(RichText::new(format!("Grid\n{}",self)).monospace());
        ui.add_space(16.0);

        // ui.label("Key Word");
        // ui.add(TextEdit::singleline(&mut self.key_word).text_style(TextStyle::Monospace));

        //ui.label("Spacer Character\nInserted between double letters if needed");
        //ui.add(TextEdit::singleline(&mut self.spacer.to_string()).text_style(TextStyle::Monospace).desired_width(15.0));

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
