use eframe::egui;
use eframe::egui::Response;
use eframe::egui::TextStyle;
use crate::ciphers::LATIN;
use crate::ciphers::Caesar;
use super::decrypt_button;
use super::encrypt_button;
use super::randomize_button;
use super::{clear_button, input_alphabet};


pub struct CaesarWidget {
    input: String,
    output: String,
    cipher: Caesar,
}

impl Default for CaesarWidget {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Caesar::new(0, LATIN),
        }
    }
}


impl egui::Widget for &mut CaesarWidget {
    fn ui(self, ui: &mut egui::Ui) -> Response {

        let cipher = &mut self.cipher;
        let input = &mut self.input;
        let output = &mut self.output;

        egui::SidePanel::left("caesar_control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Key");
            let alpha_range = 0..=((cipher.length()-1));
            ui.add(egui::Slider::new(&mut cipher.shift, alpha_range));
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                encrypt_button(ui, cipher, input, output);
                decrypt_button(ui, cipher, input, output);
            });
            ui.add_space(32.0);

            clear_button(ui, input, output);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });

        
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {

            //ui.label(format!{"Description:\n{}",description});

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            ui.label("INPUT TEXT");
            ui.add(egui::TextEdit::multiline(input).hint_text("").text_style(TextStyle::Monospace));
            ui.add_space(16.0);
            ui.label("OUTPUT TEST");
            ui.add(egui::TextEdit::multiline(output).hint_text("").text_style(TextStyle::Monospace));
        }).response
    }
}