use ciphers::{polyalphabetic::M94, Cipher};
use egui::{Color32, FontFamily, RichText, Slider, Ui};
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::randomize_reset};

#[derive(Default)]
pub struct M94Frame {
    cipher: M94,
}

impl CipherFrame for M94Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label(mono(Alphabet::BasicLatin.slice()).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Offset");
        ui.add(Slider::new(&mut self.cipher.offset, 0..=25));
        ui.add_space(16.0);

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

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.wheels.shuffle(&mut rng);
        self.cipher.offset = rng.gen_range(1..25);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
