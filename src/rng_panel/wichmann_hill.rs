use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::wichmann_hill::WichmannHill;

pub struct WichmannHillFrame {
    rng: WichmannHill,
    randoms: String,
    n_random: usize,
    wh_randoms: String,
    wh_n_random: usize,
}

impl Default for WichmannHillFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 1,
            wh_randoms: String::new(),
            wh_n_random: 1,
        }
    }
}

impl ClassicRngFrame for WichmannHillFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/wichmann_hill.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.subheading("State Variables");
        if ui.u32_drag_value_dec(&mut self.rng.s1).lost_focus() {
            self.rng.s1 = self.rng.s1 % 30269;
        };
        if ui.u32_drag_value_dec(&mut self.rng.s2).lost_focus() {
            self.rng.s2 = self.rng.s2 % 30307;
        };
        if ui.u32_drag_value_dec(&mut self.rng.s3).lost_focus() {
            self.rng.s3 = self.rng.s3 % 30323;
        };
        ui.add_space(8.0);

        ui.subheading("Intended Usage");
        ui.label("Wichmann-Hill is designed specifically to produce floating point values in the range [0,1].");
        if ui.button("Create Floats").clicked() {
            for _ in 0..self.wh_n_random {
                if !self.wh_randoms.is_empty() {
                    self.wh_randoms.push_str(", ");
                }
                self.wh_randoms.push_str(&self.rng.next_f32().to_string());
            }
        }
        ui.add(DragValue::new(&mut self.wh_n_random).range(1..=100));
        ui.add_space(4.0);
        ui.text_edit_multiline(&mut self.wh_randoms);
        ui.add_space(16.0);

        ui.subheading("Derived Usage");
        ui.label("These buttons (common for all pages here) are somewhat weak because they create a 32-bit value from two Wichmann-Hill outputs and have less than 32 bits of range.");
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.s1 = rng.gen_range(0..30269);
        self.rng.s2 = rng.gen_range(0..30307);
        self.rng.s3 = rng.gen_range(0..30323);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
