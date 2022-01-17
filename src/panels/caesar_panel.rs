use eframe::egui;
use crate::ciphers::LATIN;
use crate::ciphers::Caesar;
use super::randomize_button;
use super::{cipher_windows::View, Mode, display_panel, clear_button, mode_selector, run_button, input_alphabet};


pub struct CaesarWindow {
    plaintext: String,
    ciphertext: String,
    cipher: Caesar,
    mode: Mode,
}

impl Default for CaesarWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            cipher: Caesar::new(0, LATIN),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for CaesarWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, cipher, mode } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Key");
            let alpha_range = 0..=((cipher.length()-1));
            ui.add(egui::Slider::new(&mut cipher.shift, alpha_range));
            ui.add_space(16.0);

            mode_selector(ui, mode);
            ui.add_space(16.0);

            run_button(ui, mode, cipher, plaintext, ciphertext);
            ui.add_space(32.0);

            clear_button(ui, plaintext, ciphertext);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });

        display_panel(ui, 
            "The Caesar Cipher is one of the oldest and simplest forms of cryptography. The key is any positive whole number. Each letter of the plaintext is shifted that many positions in the alphabet, wrapping around at the end.",
            plaintext, 
            ciphertext, 
        );
    }
}




impl crate::panels::cipher_windows::CipherFrame for CaesarWindow {
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