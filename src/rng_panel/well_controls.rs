use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};
use rand::{thread_rng, Rng};
use rngs::well::{well1024a::Well1024a, well512a::Well512a};

pub struct WellFrame {
    rng512: Well512a,
    _rng1024: Well1024a,
    randoms: String,
    n_random: usize,
}

impl Default for WellFrame {
    fn default() -> Self {
        Self {
            rng512: Default::default(),
            _rng1024: Default::default(),
            randoms: String::new(),
            n_random: 5,
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
        ui.collapsing("Array of thirty two 32-bit words", |ui| {
            egui::Grid::new("well512_array")
                .num_columns(8)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng512.state.iter_mut().enumerate() {
                        if n % 8 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.u32_hex_edit(b);
                    }
                });
        });

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng512, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng512
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.rng512.state);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
