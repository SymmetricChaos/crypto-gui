use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::polyalphabetic::M94;
use egui::{FontFamily, RichText, Slider, Ui};
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

#[derive(Default)]
pub struct M94Frame {
    cipher: M94,
}

impl CipherFrame for M94Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/m94.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
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

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.wheels.shuffle(&mut rng);
        self.cipher.offset = rng.gen_range(1..25);
    }

    crate::simple_cipher! {}
}
