use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::DragValue;
use itertools::Itertools;
use num_prime::RandPrime;
use rand::{thread_rng, Rng};
use rngs::naor_reingold::NaorReingold;
use utils::math_functions::{mod_pow_64, prime_factors};

pub struct NaorReingoldFrame {
    rng: NaorReingold,
    n_random: usize,
    randoms: String,
    p: u64,
    q: u64,
    generator: u64,
    generator_err: bool,
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
            generator: 27,
            generator_err: false,
            arr: vec![7, 6, 5, 4, 3, 2],
            arr_string: String::from("7, 6, 5, 4, 3, 2"),
            arr_error: false,
            ctr: 1,
        }
    }
}

impl NaorReingoldFrame {
    fn set_rng_verbose(&mut self, errors: &mut String) {
        match NaorReingold::init_verbose(self.p, self.q, self.generator, self.arr.clone(), self.ctr)
        {
            Ok(rng) => {
                errors.clear();
                self.rng = rng;
            }
            Err(errs) => {
                errors.clear();
                for e in errs {
                    errors.push_str(e);
                    errors.push('\n');
                }
            }
        }
    }
}

impl ClassicRngFrame for NaorReingoldFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        if ui
            .button("Random Function")
            .on_hover_text("choose values for a random PRNG")
            .clicked()
        {
            self.randomize();
        }

        ui.horizontal(|ui| {
            ui.subheading("p (prime)");
            if ui
                .button("🎲")
                .on_hover_text("random 16-bit prime")
                .clicked()
            {
                let mut rng = thread_rng();
                self.p = rng.gen_prime(16, None);
                self.set_rng_verbose(errors);
            }
        });
        if ui
            .add(DragValue::new(&mut self.p).range(3..=(u32::MAX as usize)))
            .lost_focus()
        {
            self.set_rng_verbose(errors);
        }
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
                self.q = f[rng.gen_range(1..f.len())];
                self.set_rng_verbose(errors);
            }
        });
        if ui
            .add(DragValue::new(&mut self.q).range(3..=((self.p - 1) as usize)))
            .lost_focus()
        {
            self.set_rng_verbose(errors);
        }
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("g (Generator)");
            if ui
                .button("🎲")
                .on_hover_text("random generating element")
                .clicked()
            {
                self.generator = 0;
                self.generator_err = true;
                let mut rng = thread_rng();
                for _ in 0..10_000 {
                    let i = rng.gen_range(2..self.p);
                    if mod_pow_64(i, self.q, self.p) == 1 {
                        self.generator = i;
                        self.generator_err = false;
                        break;
                    }
                }
                self.set_rng_verbose(errors);
            }
        });
        if ui.add(DragValue::new(&mut self.generator)).lost_focus() {
            self.set_rng_verbose(errors);
        }
        if self.generator_err {
            ui.error_text("unable to find a generator, try again");
        }
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Counter");
            if ui.add(DragValue::new(&mut self.ctr)).lost_focus() {
                self.set_rng_verbose(errors);
            }
            ui.subheading(format!("({})", self.rng.ctr));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Array");
            if ui.button("🎲").on_hover_text("random array").clicked() {
                let mut rng = thread_rng();
                self.arr.clear();
                self.arr_error = false;
                for _ in 0..rng.gen_range(10..=20) {
                    self.arr.push(rng.gen_range(1..self.q));
                }
                self.arr_string = self.arr.iter().map(|n| n.to_string()).join(", ");
                self.set_rng_verbose(errors);
            }
        });
        if ui.text_edit_multiline(&mut self.arr_string).lost_focus() {
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
            if !self.arr_error {
                self.set_rng_verbose(errors);
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
        self.p = rng.gen_prime(16, None);

        let f = prime_factors(self.p - 1);
        self.q = f[rng.gen_range(1..f.len())];

        for i in 2..self.p {
            if mod_pow_64(i, self.q, self.p) == 1 {
                self.generator = i;
                break;
            }
        }

        self.arr.clear();
        self.arr_error = false;
        for _ in 0..rng.gen_range(10..=20) {
            self.arr.push(rng.gen_range(1..self.q));
        }
        self.arr_string = self.arr.iter().map(|n| n.to_string()).join(", ");

        self.ctr = 1;

        self.rng =
            NaorReingold::init(self.p, self.q, self.generator, self.arr.clone(), self.ctr).unwrap();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
