use ciphers::{transposition::RailFence, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

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
        self.example_rails = vec![String::new(); self.cipher.num_rails];

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
        ui.randomize_reset(self);

        ui.add_space(16.0);
        ui.subheading("Number of Rails");
        if ui
            .add(Slider::new(&mut self.cipher.num_rails, 2..=12))
            .changed()
        {
            if self.cipher.start_rail >= self.cipher.num_rails {
                self.cipher.start_rail = self.cipher.num_rails - 1;
            }
            self.set_rail_example()
        }
        ui.add_space(8.0);

        ui.subheading("Starting Rail");
        if ui
            .add(
                Slider::new(&mut self.cipher.start_rail, 0..=12)
                    .custom_formatter(|n, _| format!("{}", n + 1.0)),
            )
            .changed()
        {
            if self.cipher.start_rail >= self.cipher.num_rails {
                self.cipher.start_rail = self.cipher.num_rails - 1;
            }
            self.set_rail_example()
        }
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut self.cipher.falling, true, "Falling")
                .clicked()
            {
                self.set_rail_example();
            };
            if ui
                .selectable_value(&mut self.cipher.falling, false, "Rising")
                .clicked()
            {
                self.set_rail_example();
            };
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Example");
            if ui.control_string(&mut self.example).changed() {
                self.set_rail_example()
            }
            ui.add_space(4.0);

            for rail in self.example_rails.iter() {
                ui.mono(rail);
            }
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label(self.cipher.encrypt(&self.example).unwrap());
                ui.copy_to_clipboard(self.cipher.encrypt(&self.example).unwrap());
            });
            ui.add_space(2.0);
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.num_rails = thread_rng().gen_range(2..12);
        self.cipher.start_rail = thread_rng().gen_range(0..self.cipher.num_rails - 1);
        self.cipher.falling = thread_rng().gen_bool(0.5);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
