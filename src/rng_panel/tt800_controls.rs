use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::mersenne_twister::tt800::Tt800;

pub struct Tt800Frame {
    rng: Tt800,
    randoms: String,
    n_random: usize,
    key: u32,
}

impl Default for Tt800Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
            key: 0,
        }
    }
}

impl Tt800Frame {}

impl ClassicRngFrame for Tt800Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/mersenne_twister/tt800.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("The internal state can be built from a 32-bit key. Specifying all of the state is better, however.");
        if ui.u32_hex_edit(&mut self.key).changed() {
            self.rng = Tt800::from_u32(self.key);
        }
        ui.add_space(8.0);

        ui.subheading("Internal State");
        ui.label(format!("Index: {}", self.rng.index));
        ui.collapsing("Array of 25 32-bit words", |ui| {
            egui::Grid::new("tt800_array")
                .num_columns(5)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.arr.iter_mut().enumerate() {
                        if n % 5 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.u32_hex_edit(b);
                    }
                });
        });

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = rng.gen();
        self.rng = Tt800::from_u32(self.key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
