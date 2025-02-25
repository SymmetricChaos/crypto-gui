use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::DragValue;
use num_prime::RandPrime;
use rand::{thread_rng, Rng};
use rngs::naor_reingold::NaorReingold;
use utils::math_functions::prime_factors;

pub struct NaorReingoldFrame {
    rng: NaorReingold,
    n_random: usize,
    randoms: String,
    p: u64,
    q: u64,
    g: u64,
    arr: Vec<u64>,
    arr_string: String,
    arr_error: bool,
    ctr: u64,
}

impl Default for NaorReingoldFrame {
    fn default() -> Self {
        Self {
            rng: NaorReingold::default(),
            n_random: 5,
            randoms: String::new(),
            p: 1223,
            q: 47,
            g: 27,
            arr: vec![7, 6, 5, 4, 3, 2],
            arr_string: String::from("7, 6, 5, 4, 3, 2"),
            arr_error: false,
            ctr: 1,
        }
    }
}

impl ClassicRngFrame for NaorReingoldFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.subheading("p (prime)");
            if ui.button("🎲").on_hover_text("random prime").clicked() {
                let mut rng = thread_rng();
                self.p = rng.gen_prime(32, None);
            }
        });
        ui.add(DragValue::new(&mut self.p).range(3..=(u32::MAX as usize)));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("q (prime)");
            if ui
                .button("🎲")
                .on_hover_text("random prime that divides p-1")
                .clicked()
            {
                let mut rng = thread_rng();
                let f = prime_factors(self.p - 1);
                self.q = f[rng.gen_range(0..f.len())]
            }
        });
        ui.add(DragValue::new(&mut self.q).range(3..=((self.p - 1) as usize)));
        ui.add_space(8.0);

        ui.subheading("g (Generator)");
        ui.add(DragValue::new(&mut self.g));
        ui.add_space(8.0);

        ui.subheading("Counter");
        ui.add(DragValue::new(&mut self.ctr));
        ui.add_space(8.0);

        ui.subheading("Array");
        if ui.text_edit_singleline(&mut self.arr_string).lost_focus() {
            self.arr.clear();
            self.arr_error = false;
            for s in self.arr_string.split(",").map(|s| s.trim()) {
                if let Ok(n) = u64::from_str_radix(s, 10) {
                    self.arr.push(n);
                } else {
                    self.arr.clear();
                    self.arr_error = true;
                    break;
                }
            }
        };
        if self.arr_error {
            ui.error_text("unable to parse array");
        }
        ui.add_space(8.0);

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
