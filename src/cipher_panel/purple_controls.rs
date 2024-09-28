use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::machines::purple::{switch::SwitchSpeed, Purple};
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};

pub struct PurpleFrame {
    cipher: Purple,
    plugboard_string: String,
}

impl Default for PurpleFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            plugboard_string: String::from("NOKTYUXEQLHBRMPDICJASVWGZF"),
        }
    }
}

impl PurpleFrame {
    fn unused_speed(&mut self) -> Option<SwitchSpeed> {
        for speed in [SwitchSpeed::Slow, SwitchSpeed::Middle, SwitchSpeed::Fast].iter() {
            match self
                .cipher
                .switches
                .twenties
                .iter()
                .position(|s| s.speed == *speed)
            {
                Some(_) => (),
                None => return Some(*speed),
            }
        }
        None
    }
    fn swap_switches(&mut self, speed: SwitchSpeed, switch_idx: usize) {
        let other_switch_idx = self
            .cipher
            .switches
            .twenties
            .iter()
            .enumerate()
            .position(|(n, s)| s.speed == speed && n != switch_idx)
            .unwrap();

        match self.unused_speed() {
            Some(s) => self.cipher.switches.twenties[other_switch_idx].speed = s,
            None => (),
        }
    }
}

impl CipherFrame for PurpleFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/machines/purple",
        );
        ui.add_space(8.0);

        ui.subheading("Sixes Position");
        ui.horizontal(|ui| {
            ui.add(Slider::new(
                &mut self.cipher.switches.sixes.position,
                0..=24,
            ));
            ui.label(self.cipher.switches.sixes.to_string());
        });

        ui.add_space(16.0);
        ui.subheading("Twenties Positions");
        for switch in self.cipher.switches.twenties.iter_mut() {
            ui.horizontal(|ui| {
                ui.add(Slider::new(&mut switch.position, 0..=24));
                ui.label(switch.to_string());
            });
        }

        ui.add_space(16.0);
        ui.subheading("Twenties Speeds");
        for n in 0..3 {
            ui.horizontal(|ui| {
                ui.label(format!("\nSwitch {}", n + 1));
                for speed in [SwitchSpeed::Slow, SwitchSpeed::Middle, SwitchSpeed::Fast] {
                    if ui
                        .selectable_value(
                            &mut self.cipher.switches.twenties[n].speed,
                            speed,
                            speed.name(),
                        )
                        .clicked()
                    {
                        self.swap_switches(speed, n);
                    };
                }
            });
        }

        ui.add_space(16.0);
        ui.subheading("Plugboard");
        if ui
            .add(TextEdit::singleline(&mut self.plugboard_string).font(TextStyle::Monospace))
            .changed()
        {
            match self.cipher.set_plugboard(&self.plugboard_string) {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        };
        ui.add_space(16.0);
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
