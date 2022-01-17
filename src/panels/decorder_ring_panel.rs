use eframe::egui;
use crate::ciphers::LATIN;
use crate::ciphers::DecoderRing;
use super::randomize_button;
use super::{cipher_windows::View, Mode, display_panel, clear_button, mode_selector, run_button, input_alphabet};


pub struct DecoderRingWindow {
    plaintext: String,
    ciphertext: String,
    cipher: DecoderRing,
    mode: Mode,
}

impl Default for DecoderRingWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            cipher: DecoderRing::new(0, LATIN),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for DecoderRingWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, cipher, mode } = self;

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

            mode_selector(ui, mode);
            ui.add_space(16.0);

            run_button(ui, mode, cipher, plaintext, ciphertext);
            ui.add_space(32.0);

            clear_button(ui, plaintext, ciphertext);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });

        display_panel(ui, 
            "The Decoder Ring is a simplified variation of the Caesar cipher. Each letter of the alphabet along with a space character is assigned a numbe. Then some other number is added to each with the value wrapping around at the greatest value. Additional spaces are needed unless the number are displayed with a fixed width.",
            plaintext, 
            ciphertext, 
        );
    }
}




impl crate::panels::cipher_windows::CipherFrame for DecoderRingWindow {
    fn name(&self) -> &'static str {
        "DecoderRing Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("DecoderRing Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}