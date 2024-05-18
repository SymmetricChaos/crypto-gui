use ciphers::{transposition::Scytale, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct ScytaleFrame {
    cipher: Scytale,
    example: String,
    example_rows: Vec<String>,
}

impl Default for ScytaleFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            example: String::from("SCYTALEEXAMPLE"),
            example_rows: vec!["SCYT".into(), "ALEE".into(), "XAMP".into(), "LEXX".into()],
        }
    }
}

impl ScytaleFrame {
    fn set_example(&mut self) {
        let n_cols = num::Integer::div_ceil(&self.example.chars().count(), &self.cipher.num_rails);
        let mut rows = vec![String::new(); self.cipher.num_rails];
        let mut symbols = self.example.chars();

        for row in rows.iter_mut() {
            for _ in 0..n_cols {
                row.push(symbols.next().unwrap_or(self.cipher.padding))
            }
        }
    }
}

impl CipherFrame for ScytaleFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/transposition/syctale.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.label("Wraps");
        if ui
            .add(Slider::new(&mut self.cipher.num_rails, 2..=12))
            .changed()
        {
            self.set_example();
        };
        ui.add_space(16.0);

        ui.collapsing("Example", |ui| {
            if ui.control_string(&mut self.example).changed() {
                self.set_example();
            }

            ui.add_space(4.0);

            for row in self.example_rows.iter() {
                ui.mono(row);
            }

            ui.add_space(4.0);

            ui.mono(self.cipher.encrypt(&self.example).unwrap());
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.num_rails = thread_rng().gen_range(2..12);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
