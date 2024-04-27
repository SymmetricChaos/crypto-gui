use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::DragValue;
use rngs::{
    middle_square_binary::{
        MiddleSquareBinary16, MiddleSquareBinary32, MiddleSquareBinary64, MiddleSquareBinary8,
    },
    ClassicRng,
};

pub struct MiddleSquareBinaryFrame {
    rng64: MiddleSquareBinary64,
    rng32: MiddleSquareBinary32,
    rng16: MiddleSquareBinary16,
    rng8: MiddleSquareBinary8,
    randoms: String,
    n_random: usize,
}

impl Default for MiddleSquareBinaryFrame {
    fn default() -> Self {
        Self {
            rng64: MiddleSquareBinary64::default(),
            rng32: MiddleSquareBinary32::default(),
            rng16: MiddleSquareBinary16::default(),
            rng8: MiddleSquareBinary8::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl MiddleSquareBinaryFrame {}

impl ClassicRngFrame for MiddleSquareBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("State");
        ui.add(DragValue::new(&mut self.rng32.state).hexadecimal(32, false, false));

        ui.add_space(16.0);

        generate_random_u32s_box(ui, &mut self.rng32, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng32
    }

    fn randomize(&mut self) {
        // let mut rng = thread_rng();
        // self.rng.state = rng.gen_range(0..10_u64.pow((self.rng.width + 2) as u32) - 1)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
