use super::{View, ViewableCipher, _generic_components::*};
use crate::ciphers::polyalphabetic::M94;
use eframe::{
    egui::{self, RichText, Slider, Ui},
    epaint::FontFamily,
};

impl ViewableCipher for M94 {}

impl View for M94 {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=24;
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        if ui.button("Randomize Wheels").clicked() {
            self.randomize_wheels();
        }

        ui.label("Wheels");
        for n in 0..25 {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(RichText::from(self.wheels[n]).monospace()));
                if ui
                    .small_button(RichText::from("⋀").family(FontFamily::Monospace))
                    .clicked()
                {
                    self.shift_left(n)
                }
                if ui
                    .small_button(RichText::from("⋁").family(FontFamily::Monospace))
                    .clicked()
                {
                    self.shift_right(n)
                }
            });
        }
    }
}
