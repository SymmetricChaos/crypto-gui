use eframe::egui;
use crate::ciphers::LATIN;
use crate::ciphers::Caesar;
use super::decrypt_button;
use super::encrypt_button;
use super::randomize_button;
use super::{cipher_windows::View, display_panel, clear_button, input_alphabet};


pub struct CaesarWindow {
    input: String,
    output: String,
    cipher: Caesar,
}

impl Default for CaesarWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Caesar::new(0, LATIN),
        }
    }
}


impl crate::panels::cipher_windows::View for CaesarWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ input, output, cipher } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
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

        display_panel(ui, 
            "The Caesar Cipher is one of the oldest and simplest forms of cryptography. The key is any positive whole number. Each letter of the input is shifted that many positions in the alphabet, wrapping around at the end.",
            input, 
            output, 
        );
    }
}




impl crate::panels::cipher_windows::CipherWindow for CaesarWindow {
    fn name(&self) -> &'static str {
        "Caesar Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Caesar Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}