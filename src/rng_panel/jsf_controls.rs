use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::jsf::{Jsf32, Jsf64};

pub struct JsfFrame {
    rng32: Jsf32,
    rng64: Jsf64,
    big: bool,
    randoms: String,
    n_random: usize,
}

impl Default for JsfFrame {
    fn default() -> Self {
        Self {
            rng32: Default::default(),
            rng64: Default::default(),
            big: false,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl ClassicRngFrame for JsfFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/jsf.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Key");
        if self.big {
            for i in 0..4 {
                ui.add(DragValue::new(&mut self.rng64.state[i]).hexadecimal(16, false, false));
            }
        } else {
            for i in 0..4 {
                ui.add(DragValue::new(&mut self.rng32.state[i]).hexadecimal(8, false, false));
            }
        }
        ui.add_space(8.0);
        if self.big {
            generate_randoms_box(ui, &mut self.rng64, &mut self.n_random, &mut self.randoms);
        } else {
            generate_randoms_box(ui, &mut self.rng32, &mut self.n_random, &mut self.randoms);
        }
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.big {
            true => &mut self.rng64,
            false => &mut self.rng32,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        if self.big {
            rng.fill(&mut self.rng64.state)
        } else {
            rng.fill(&mut self.rng32.state)
        };
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
