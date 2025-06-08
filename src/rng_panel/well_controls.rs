use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};
use rand::{thread_rng, Rng};
use rngs::well::{well1024a::Well1024a, well512a::Well512a};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    Well512a,
    Well1024a,
}

pub struct WellFrame {
    rng512a: Well512a,
    rng1024a: Well1024a,
    randoms: String,
    n_random: usize,
    variant: Variant,
}

impl Default for WellFrame {
    fn default() -> Self {
        Self {
            rng512a: Default::default(),
            rng1024a: Default::default(),
            randoms: String::new(),
            n_random: 5,
            variant: Variant::Well512a,
        }
    }
}

impl WellFrame {}

impl ClassicRngFrame for WellFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/well",
        );
        ui.add_space(8.0);

        ui.subheading("State");
        match self.variant {
            Variant::Well512a => {
                ui.collapsing("Array of 32-bit words", |ui| {
                    egui::Grid::new("well512_array")
                        .num_columns(8)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.rng512a.state.iter_mut().enumerate() {
                                if n % 8 == 0 && n != 0 {
                                    ui.end_row()
                                }
                                ui.u32_hex_edit(b);
                            }
                        });
                });
                ui.add_space(16.0);
                generate_randoms_box(ui, &mut self.rng512a, &mut self.n_random, &mut self.randoms);
                ui.add_space(16.0);
            }
            Variant::Well1024a => {
                ui.collapsing("Array of 32-bit words", |ui| {
                    egui::Grid::new("well1024_array")
                        .num_columns(8)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.rng1024a.state.iter_mut().enumerate() {
                                if n % 8 == 0 && n != 0 {
                                    ui.end_row()
                                }
                                ui.u32_hex_edit(b);
                            }
                        });
                });
                ui.add_space(16.0);
                generate_randoms_box(
                    ui,
                    &mut self.rng1024a,
                    &mut self.n_random,
                    &mut self.randoms,
                );
                ui.add_space(16.0);
            }
        }
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.variant {
            Variant::Well512a => &mut self.rng512a,
            Variant::Well1024a => &mut self.rng1024a,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        match self.variant {
            Variant::Well512a => rng.fill(&mut self.rng512a.state),
            Variant::Well1024a => rng.fill(&mut self.rng1024a.state),
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
