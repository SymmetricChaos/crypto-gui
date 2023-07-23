use codes::text_standards::punycode::Punycode;
use egui::Slider;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct PunycodeFrame {
    code: Punycode,
    example: String,
}

impl Default for PunycodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            example: String::from("TạisaohọkhôngthểchỉnóitiếngViệt"),
        }
    }
}

impl CodeFrame for PunycodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Example");
        ui.text_edit_singleline(&mut self.example);
        ui.add_space(8.0);
        match self.encode(&self.example) {
            Ok(s) => ui.label(s),
            Err(e) => ui.error_text(e),
        };

        ui.collapsing("Variables", |ui| {
            let tmin_range = 1..=(self.code.tmax - 1);
            let tmax_range = 2..=2000;
            ui.label("Min Threshold");
            ui.add(Slider::new(&mut self.code.tmin, tmin_range));
            ui.label("Max Threshold");
            ui.add(Slider::new(&mut self.code.tmin, tmax_range));

            ui.add_space(16.0);

            let damp_range = 1..=2000;
            ui.label("Damping Factor");
            ui.add(Slider::new(&mut self.code.damp, damp_range));

            ui.add_space(16.0);

            let skew_range = 1..=2000;
            ui.label("Skew Factor");
            ui.add(Slider::new(&mut self.code.skew, skew_range));

            let base_range = 1..=2000;
            ui.label("Base");
            ui.add(Slider::new(&mut self.code.base, base_range));

            let bias_range = 1..=2000;
            ui.label("Initial Bias");
            ui.add(Slider::new(&mut self.code.init_bias, bias_range));

            let n_range = 1..=2000;
            ui.label("Initial N");
            ui.add(Slider::new(&mut self.code.init_n, n_range));
        });
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
