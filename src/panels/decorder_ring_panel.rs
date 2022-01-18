use eframe::egui;
use crate::ciphers::LATIN;
use crate::ciphers::DecoderRing;
use super::decrypt_button;
use super::encrypt_button;
use super::randomize_button;
use super::{cipher_windows::View, display_panel, clear_button, input_alphabet};


pub struct DecoderRingWindow {
    input: String,
    output: String,
    cipher: DecoderRing,
}

impl Default for DecoderRingWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: DecoderRing::new(0, LATIN),
        }
    }
}


impl crate::panels::cipher_windows::View for DecoderRingWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ input, output, cipher } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Key");
            let alpha_range = 0..=((cipher.length()-1));
            ui.add(egui::Slider::new(&mut cipher.index, alpha_range));
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui.button("Annie").clicked() {
                    cipher.annie();
                }
                if ui.button("Midnight").clicked() {
                    cipher.midnight();
                }
            });

            ui.horizontal(|ui| {
                encrypt_button(ui, cipher, input, output);
                decrypt_button(ui, cipher, input, output);
            });
            ui.add_space(32.0);

            clear_button(ui, input, output);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });

        display_panel(ui, 
            "The Decoder Ring is a simplified variation of the Caesar cipher. Each letter of the alphabet along with a space character is assigned a numbe. Then some other number is added to each with the value wrapping around at the greatest value. Additional spaces are needed unless the number are displayed with a fixed width.",
            input, 
            output, 
        );
    }
}




impl crate::panels::cipher_windows::CipherWindow for DecoderRingWindow {
    fn name(&self) -> &'static str {
        "Decoder Ring"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Decoder Ring")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}