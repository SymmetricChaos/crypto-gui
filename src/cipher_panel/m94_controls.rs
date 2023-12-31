use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polyalphabetic::M94, Cipher};
use egui::{FontFamily, RichText, Slider, Ui};
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

#[derive(Default)]
pub struct M94Frame {
    cipher: M94,
}

impl CipherFrame for M94Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        ui.label(Alphabet::BasicLatin.slice());
        ui.add_space(16.0);

        ui.subheading("Offset");
        ui.add(Slider::new(&mut self.cipher.offset, 0..=25));
        ui.add_space(16.0);

        ui.subheading("Wheels");
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
        ui.add_space(16.0);
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
