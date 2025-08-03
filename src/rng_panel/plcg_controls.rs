use super::ClassicRngFrame;
use crate::ui_elements::{filter_and_parse_u32, generate_randoms_box, UiElements};
use egui::TextStyle;
use rand::{thread_rng, Rng};
use rngs::{lcg::Lcg32, plcg::Plcg32, ClassicRng};

pub struct PlcgFrame {
    rng: Plcg32,
    state_string: String,
    coefs_string: String,
    modulus_string: String,
    randoms: String,
    n_random: usize,
}

impl Default for PlcgFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            state_string: String::from("1257924810"),
            coefs_string: String::from("1013904223, 1664525"),
            modulus_string: String::from("4294967295"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl PlcgFrame {
    fn input_control(ui: &mut egui::Ui, string: &mut String, n: &mut u32) {
        if ui
            .add_sized(
                [40.0, 20.0],
                egui::TextEdit::singleline(string)
                    .font(TextStyle::Monospace)
                    .clip_text(false),
            )
            .changed()
        {
            filter_and_parse_u32(n, string);
        }
    }

    fn set_all_strings(&mut self) {
        self.state_string = self.rng.state.to_string();
        self.modulus_string = self.rng.modulus.to_string();

        // self.multiplier_string = self.rng.multiplier.to_string();
        // self.increment_string = self.rng.increment.to_string();
    }
}

impl ClassicRngFrame for PlcgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/plcg.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);

        ui.subheading("State");
        ui.u32_drag_value_dec(&mut self.rng.state);

        if ui.button("step").clicked() {
            self.rng.next_u32();
            self.set_all_strings();
        }

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.coefs.clear();
        self.rng.modulus = rng.gen();
        self.rng.state = rng.gen::<u32>() % self.rng.modulus;
        for _ in 0..3 {
            self.rng.coefs.push(rng.gen::<u32>() % self.rng.modulus);
        }
        self.set_all_strings();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
