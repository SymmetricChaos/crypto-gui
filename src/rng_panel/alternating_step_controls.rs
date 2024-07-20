use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::alternating_step::AlternatingStep;
use utils::bits::Bit;

pub struct AlternatingStepFrame {
    rng: AlternatingStep,
    vector_lengths: [usize; 3],
    randoms: String,
    n_random: usize,
}

impl Default for AlternatingStepFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_lengths: [16, 16, 16],
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl AlternatingStepFrame {}

impl ClassicRngFrame for AlternatingStepFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.rng.big_endian, true, "Big Endian");
            ui.selectable_value(&mut self.rng.big_endian, false, "Little Endian");
        });
        ui.add_space(16.0);

        for i in 0..3 {
            let lfsr = &mut self.rng.lfsrs[i];

            ui.subheading("Number of Bits");
            if ui
                .add(DragValue::new(&mut self.vector_lengths[i]).clamp_range(4..=32))
                .changed()
            {
                lfsr.bits.truncate(self.vector_lengths[i]);
                while lfsr.bits.len() < self.vector_lengths[i] {
                    lfsr.bits.push(utils::bits::Bit::Zero)
                }
                lfsr.taps.truncate(self.vector_lengths[i]);
                while lfsr.taps.len() < self.vector_lengths[i] {
                    lfsr.taps.push(false)
                }
            };
            ui.add_space(4.0);

            ui.subheading(format!("Internal State for LFSR {}", i));
            ui.add_space(8.0);
            egui::Grid::new(format!("lfsr_state{}", i))
                .num_columns(self.vector_lengths[i])
                .max_col_width(5.0)
                .min_col_width(5.0)
                .show(ui, |ui| {
                    for b in lfsr.bits.iter_mut() {
                        let x = RichText::from(b.to_string()).monospace().size(12.0);
                        if ui.button(x).clicked() {
                            b.flip()
                        }
                    }
                    ui.end_row();
                    for t in lfsr.taps.iter_mut() {
                        match t {
                            true => {
                                if ui
                                    .button(RichText::from("^").monospace().size(12.0))
                                    .clicked()
                                {
                                    *t = false
                                }
                            }
                            false => {
                                if ui
                                    .button(RichText::from("_").monospace().size(12.0))
                                    .clicked()
                                {
                                    *t = true
                                }
                            }
                        }
                    }
                });
            ui.subheading(format!("Next Bit: {}", self.rng.peek_next_bit()));
            ui.add_space(8.0);
        }

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for lfsr in self.rng.rngs.iter_mut() {
            for b in lfsr.bits.iter_mut() {
                *b = Bit::from(rng.gen_bool(0.5));
            }
            for t in lfsr.taps.iter_mut() {
                *t = rng.gen_bool(0.15);
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
