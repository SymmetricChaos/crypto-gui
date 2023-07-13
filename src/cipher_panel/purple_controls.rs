use super::CipherFrame;
use crate::ui_elements::{error_text, subheading};
use ciphers::machines::purple::{switch::SwitchSpeed, Purple};
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};

#[derive(Default)]
pub struct PurpleFrame {
    cipher: Purple,
}

impl CipherFrame for PurpleFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.label(subheading("Sixes Position"));
        ui.horizontal(|ui| {
            ui.add(
                Slider::new(&mut self.cipher.switches.sixes.position, 0..=24).clamp_to_range(true),
            );
            ui.label(self.cipher.switches.sixes.to_string());
        });

        ui.add_space(16.0);

        ui.label(subheading("Twenties Positions"));
        for switch in self.cipher.switches.twenties.iter_mut() {
            ui.horizontal(|ui| {
                ui.add(Slider::new(&mut switch.borrow_mut().position, 0..=24).clamp_to_range(true));
                ui.label(switch.borrow().to_string());
            });
        }

        // Rather than automatically create a valid setting (which seems hard)
        // we just detect invalid state when switches are changed and report it
        // to the errors
        ui.label("Select Twenties Speeds");
        for n in 0..3 {
            ui.horizontal(|ui| {
                ui.label(format!("\nSwitch {}", n + 1));
                if ui
                    .selectable_value(
                        &mut self.cipher.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Slow,
                        "Slow",
                    )
                    .clicked()
                {
                    self.cipher
                        .switches
                        .set_slow(self.cipher.switches.twenties[n].clone());
                };
                if ui
                    .selectable_value(
                        &mut self.cipher.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Middle,
                        "Middle",
                    )
                    .clicked()
                {
                    self.cipher
                        .switches
                        .set_middle(self.cipher.switches.twenties[n].clone());
                };
                if ui
                    .selectable_value(
                        &mut self.cipher.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Fast,
                        "Fast",
                    )
                    .clicked()
                {
                    self.cipher
                        .switches
                        .set_fast(self.cipher.switches.twenties[n].clone());
                };
            });
        }
        if let Err(e) = self.cipher.switches.validate_switches() {
            ui.label(error_text(e.to_string()));
        };

        ui.add_space(10.0);
        ui.label(subheading("Plugboard"));
        if ui
            .add(TextEdit::singleline(&mut self.cipher.plugboard_string).font(TextStyle::Monospace))
            .changed()
        {
            match self.cipher.set_plugboard() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        };
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
