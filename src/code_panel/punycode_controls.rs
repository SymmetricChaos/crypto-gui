use egui::Slider;

use super::{View, ViewableCode};
use crate::codes::Punycode;

impl ViewableCode for Punycode {}

// tmin: 1,
// tmax: 26,
// damp: 700,
// skew: 38,
// init_bias: 72,
// init_n: 128,
// base: 36,
// delim: 'c',

impl View for Punycode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        let tmin_range = 1..=(self.tmax - 1);
        let tmax_range = 2..=2000;
        ui.label("Min Threshold");
        ui.add(Slider::new(&mut self.tmin, tmin_range));
        ui.label("Max Threshold");
        ui.add(Slider::new(&mut self.tmin, tmax_range));

        ui.add_space(16.0);

        let damp_range = 1..=2000;
        ui.label("Damping Factor");
        ui.add(Slider::new(&mut self.damp, damp_range));

        ui.add_space(16.0);

        let skew_range = 1..=2000;
        ui.label("Skew Factor");
        ui.add(Slider::new(&mut self.skew, skew_range));

        let base_range = 1..=2000;
        ui.label("Base");
        ui.add(Slider::new(&mut self.base, base_range));

        let bias_range = 1..=2000;
        ui.label("Initial Bias");
        ui.add(Slider::new(&mut self.init_bias, bias_range));

        let n_range = 1..=2000;
        ui.label("Initial N");
        ui.add(Slider::new(&mut self.init_bias, n_range));
    }
}
