use ciphers::{transposition::RailFence, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};

use crate::ui_elements::{control_string, mono, randomize_reset, subheading};

use super::CipherFrame;

pub struct RailFenceFrame {
    cipher: RailFence,
    example: String,
    example_rails: Vec<String>,
}

impl Default for RailFenceFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            example: "RAILFENCEEXAMPLE".into(),
            example_rails: vec![
                "R   F   E   M   ".into(),
                " A L E C E A P E".into(),
                "  I   N   X   L ".into(),
            ],
        }
    }
}

impl RailFenceFrame {
    fn set_rail_example(&mut self) {
        self.example_rails = vec![String::new(); self.cipher.rails];

        let positions = self.cipher.positions();

        for (c, n) in self.example.chars().zip(positions) {
            for (idx, rail) in self.example_rails.iter_mut().enumerate() {
                if idx != n {
                    rail.push(' ');
                } else {
                    rail.push(c)
                }
            }
        }
    }
}

impl CipherFrame for RailFenceFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(8.0);

        ui.label(subheading("Number of Rails"));
        if ui
            .add(Slider::new(&mut self.cipher.rails, 2..=12))
            .changed()
        {
            if self.cipher.start_rail >= self.cipher.rails {
                self.cipher.start_rail = self.cipher.rails - 1;
            }
            self.set_rail_example()
        }
        ui.add_space(8.0);
        ui.label(subheading("Starting Rail"));
        if ui
            .add(Slider::new(&mut self.cipher.start_rail, 0..=12))
            .changed()
        {
            if self.cipher.start_rail >= self.cipher.rails {
                self.cipher.start_rail = self.cipher.rails - 1;
            }
            self.set_rail_example()
        }
        ui.add_space(2.0);

        if ui
            .checkbox(&mut self.cipher.start_falling, "Falling")
            .changed()
        {
            self.set_rail_example()
        };
        ui.add_space(8.0);

        ui.collapsing("Example", |ui| {
            if control_string(ui, &mut self.example).changed() {
                self.set_rail_example()
            }
            ui.add_space(4.0);
            for rail in self.example_rails.iter() {
                ui.label(mono(rail));
            }
        });

        ui.add_space(8.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.rails = thread_rng().gen_range(2..12);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
