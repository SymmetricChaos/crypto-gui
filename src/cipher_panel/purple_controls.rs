use super::{View, ViewableCipher};
use crate::ciphers::{purple::switch::SwitchSpeed, Purple};
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};
use egui::{Color32, RichText};

impl ViewableCipher for Purple {}

impl View for Purple {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.label("Sixes Position");
        ui.horizontal(|ui| {
            ui.add(Slider::new(&mut self.switches.sixes.position, 0..=24).clamp_to_range(true));
            ui.label(self.switches.sixes.to_string());
        });

        ui.add_space(16.0);

        ui.label("Twenties Positions");
        for switch in self.switches.twenties.iter_mut() {
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
                        &mut self.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Slow,
                        "Slow",
                    )
                    .clicked()
                {
                    self.switches.set_slow(self.switches.twenties[n].clone());
                };
                if ui
                    .selectable_value(
                        &mut self.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Middle,
                        "Middle",
                    )
                    .clicked()
                {
                    self.switches.set_middle(self.switches.twenties[n].clone());
                };
                if ui
                    .selectable_value(
                        &mut self.switches.twenties[n].borrow_mut().speed,
                        SwitchSpeed::Fast,
                        "Fast",
                    )
                    .clicked()
                {
                    self.switches.set_fast(self.switches.twenties[n].clone());
                };
            });
        }
        if let Err(e) = self.switches.validate_switches() {
            ui.label(
                RichText::new(e.to_string())
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        };

        ui.add_space(10.0);
        ui.label("Plugboard");
        if ui
            .add(TextEdit::singleline(&mut self.plugboard_string).font(TextStyle::Monospace))
            .changed()
        {
            match self.set_plugboard() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        };
    }
}
