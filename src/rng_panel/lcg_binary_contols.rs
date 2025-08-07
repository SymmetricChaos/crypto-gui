use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    lcg::{Lcg32, Lcg64},
    ClassicRng,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Selector {
    L32,
    L64,
}

pub struct LcgBinaryFrame {
    rng32: Lcg32,
    rng64: Lcg64,
    selector: Selector,
    randoms: String,
    n_random: usize,
}

impl Default for LcgBinaryFrame {
    fn default() -> Self {
        Self {
            rng32: Default::default(),
            rng64: Default::default(),
            selector: Selector::L32,
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl LcgBinaryFrame {}

impl ClassicRngFrame for LcgBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/lcg.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.subheading("Word Size");
        ui.selectable_value(&mut self.selector, Selector::L32, "32 bits");
        ui.selectable_value(&mut self.selector, Selector::L64, "64 bits");
        ui.add_space(8.0);

        ui.subheading("State");
        match self.selector {
            Selector::L32 => ui.u32_hex_edit(&mut self.rng32.state),
            Selector::L64 => ui.u64_hex_edit(&mut self.rng64.state),
        };
        ui.add_space(4.0);
        ui.subheading("Multiplier");
        ui.label(
            "For maximum length the the multiplier must be one greater than a multiple of four.",
        );
        match self.selector {
            Selector::L32 => ui.u32_hex_edit(&mut self.rng32.multiplier),
            Selector::L64 => ui.u64_hex_edit(&mut self.rng64.multiplier),
        };
        ui.add_space(4.0);
        ui.subheading("Increment");
        ui.label("For maximum length the increment must be odd.");
        match self.selector {
            Selector::L32 => ui.u32_hex_edit(&mut self.rng32.increment),
            Selector::L64 => ui.u64_hex_edit(&mut self.rng64.increment),
        };

        ui.add_space(8.0);

        if ui.button("step").clicked() {
            match self.selector {
                Selector::L32 => self.rng32.next_u32(),
                Selector::L64 => self.rng64.next_u32(),
            };
        }
        ui.add_space(8.0);
        generate_randoms_box(
            ui,
            match self.selector {
                Selector::L32 => &mut self.rng32,
                Selector::L64 => &mut self.rng64,
            },
            &mut self.n_random,
            &mut self.randoms,
        );
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.selector {
            Selector::L32 => &mut self.rng32,
            Selector::L64 => &mut self.rng64,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        // The OR 1 after the increment and multipler forces odd values
        match self.selector {
            Selector::L32 => {
                self.rng32.state = rng.gen();
                self.rng32.increment = rng.gen::<u32>() | 1;
                self.rng32.multiplier = rng.gen::<u32>() | 1;
                if (self.rng32.multiplier - 1) % 4 != 0 {
                    self.rng32.multiplier = self.rng32.multiplier.wrapping_add(2)
                }
            }
            Selector::L64 => {
                self.rng64.state = rng.gen();
                self.rng64.increment = rng.gen::<u64>() | 1;
                self.rng64.multiplier = rng.gen::<u64>() | 1;
                if (self.rng64.multiplier - 1) % 4 != 0 {
                    self.rng64.multiplier = self.rng64.multiplier.wrapping_add(2)
                }
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
