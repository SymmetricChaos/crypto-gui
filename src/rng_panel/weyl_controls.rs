use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::RichText;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::{
    weyl::{WeylSequence, WeylSequence32, WeylSequence64},
    ClassicRng,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    W,
    W32,
    W64,
}

pub struct WeylSequenceFrame {
    rng: WeylSequence,
    rng32: WeylSequence32,
    rng64: WeylSequence64,
    incr_err: bool,
    variant: Variant,
    randoms: String,
    n_random: usize,
}

impl Default for WeylSequenceFrame {
    fn default() -> Self {
        Self {
            rng: WeylSequence::default(),
            rng32: WeylSequence32::default(),
            rng64: WeylSequence64::default(),
            incr_err: false,
            variant: Variant::W,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl WeylSequenceFrame {}

impl ClassicRngFrame for WeylSequenceFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/weyl.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Variant");
        ui.selectable_value(&mut self.variant, Variant::W, "Arbitrary Weyl Sequence");
        ui.selectable_value(&mut self.variant, Variant::W32, "32-Bit Weyl Sequence");
        ui.selectable_value(&mut self.variant, Variant::W64, "64-Bit Weyl Sequence");

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        if self.variant == Variant::W {
            ui.horizontal(|ui| {
                ui.subheading("State");
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    self.rng.state = rng.gen_range(0..(1 << 20));
                }
            });
            ui.u64_drag_value_dec(&mut self.rng.state);

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Modulus");
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    self.rng.state = rng.gen_range(0..(1 << 20));
                }
            });
            ui.u64_drag_value_dec(&mut self.rng.modulus);

            ui.add_space(8.0);
            ui.subheading("Increment");
            if ui.u64_drag_value_dec(&mut self.rng.increment).lost_focus() {
                self.incr_err = self.rng.increment.gcd(&self.rng.modulus) == 1;
            }

            if self.incr_err {
                ui.error_text("");
            } else {
                ui.error_text("Increment must be co-prime to the Modulus.");
            }

            ui.add_space(8.0);
            ui.subheading("Calculation");
            let calc = format!(
                "({} + {}) % {} = {}",
                self.rng.state,
                self.rng.increment,
                self.rng.modulus,
                (self.rng.state + self.rng.increment) % self.rng.modulus
            );
            ui.label(RichText::new(calc).size(16.0));
        } else if self.variant == Variant::W32 {
            ui.label("On 32-bit hardware wrapping addition at the word size will occur automatically, making it very fast to set the modulus to be 4294967296. The only requirement for the increment is that it be odd.");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.subheading("State");
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    self.rng32.state = rng.gen_range(0..u32::MAX);
                }
            });
            ui.u32_drag_value_dec(&mut self.rng32.state);
            ui.add_space(8.0);
            ui.subheading("Increment");
            if ui
                .u32_drag_value_dec(&mut self.rng32.increment)
                .lost_focus()
            {
                self.rng32.increment |= 1
            }
        } else if self.variant == Variant::W64 {
            ui.label("On 64-bit hardware wrapping addition at the word size will occur automatically, making it very fast to set the modulus to be 18446744073709551616. The only requirement for the increment is that it be odd.");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.subheading("State");
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    self.rng64.state = rng.gen_range(0..u64::MAX);
                }
            });
            ui.u64_drag_value_dec(&mut self.rng64.state);
            ui.add_space(8.0);
            ui.subheading("Increment");
            if ui
                .u64_drag_value_dec(&mut self.rng64.increment)
                .lost_focus()
            {
                self.rng64.increment |= 1
            }
        }

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            match self.variant {
                Variant::W => self.rng.next_u32(),
                Variant::W32 => self.rng32.next_u32(),
                Variant::W64 => self.rng64.next_u32(),
            };
        }
        ui.add_space(8.0);
        match self.variant {
            Variant::W => {
                generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms)
            }
            Variant::W32 => {
                generate_randoms_box(ui, &mut self.rng32, &mut self.n_random, &mut self.randoms)
            }
            Variant::W64 => {
                generate_randoms_box(ui, &mut self.rng64, &mut self.n_random, &mut self.randoms)
            }
        }
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.variant {
            Variant::W => &mut self.rng,
            Variant::W32 => &mut self.rng32,
            Variant::W64 => &mut self.rng64,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        if self.variant == Variant::W {
            self.rng.modulus = rng.gen_range(0..(1 << 20));
            self.rng.state = rng.gen_range(0..(self.rng.modulus - 1));
            self.incr_err = true;

            for _ in 0..10_000 {
                let n = rng.gen_range(0..self.rng.modulus);
                if n.gcd(&self.rng.modulus) == 1 {
                    self.rng.increment = n;
                    self.incr_err = false;
                    break;
                }
            }
        } else if self.variant == Variant::W32 {
            self.rng32.state = rng.gen_range(0..u32::MAX);
            self.rng32.increment = rng.gen_range(0..u32::MAX);
            self.rng32.increment |= 1; // force odd
        } else if self.variant == Variant::W64 {
            self.rng64.state = rng.gen_range(0..u64::MAX);
            self.rng64.increment = rng.gen_range(0..u64::MAX);
            self.rng64.increment |= 1; // force odd
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
