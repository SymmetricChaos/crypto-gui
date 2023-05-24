use ciphers::{polyalphabetic::M94, Cipher};
use egui::{FontFamily, RichText, Slider, Ui};

use super::CipherFrame;

#[derive(Default)]
pub struct M94Frame {
    cipher: M94,
}

impl CipherFrame for M94Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=24;
        ui.add(Slider::new(&mut self.cipher.offset, alpha_range.clone()));
        ui.add_space(16.0);

        // if ui.button("Randomize Wheels").clicked() {
        //     self.cipher.randomize_wheels();
        // }

        ui.label("Wheels");
        for n in 0..25 {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    RichText::from(self.cipher.wheels[n]).monospace(),
                ));
                if ui
                    .small_button(RichText::from("⋀").family(FontFamily::Monospace))
                    .clicked()
                {
                    self.cipher.shift_left(n)
                }
                if ui
                    .small_button(RichText::from("⋁").family(FontFamily::Monospace))
                    .clicked()
                {
                    self.cipher.shift_right(n)
                }
            });
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
