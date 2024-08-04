use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::middle_square_binary::{MSBSize, MiddleSquareBinary};

pub struct MiddleSquareBinaryFrame {
    rng: MiddleSquareBinary,
    randoms: String,
    n_random: usize,
}

impl Default for MiddleSquareBinaryFrame {
    fn default() -> Self {
        Self {
            rng: MiddleSquareBinary::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl ClassicRngFrame for MiddleSquareBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Size");
        ui.label("Modern desktop and server computer architecture generally has registers of size 32-bits and 64-bits so all of these can be performed exceptionally quickly. The 16-bit and 8-bit versions are mainly of academic interest as they fall into short repeating sequences very quickly.");
        // ui.selectable_value(&mut self.rng.width, MSBSize::B64, "64-Bit");
        if ui
            .selectable_value(&mut self.rng.width, MSBSize::B32, "32-Bit")
            .changed()
        {
            self.rng.state &= self.rng.width.mask();
        };
        if ui
            .selectable_value(&mut self.rng.width, MSBSize::B16, "16-Bit")
            .changed()
        {
            self.rng.state &= self.rng.width.mask();
        };
        if ui
            .selectable_value(&mut self.rng.width, MSBSize::B8, "8-Bit")
            .changed()
        {
            self.rng.state &= self.rng.width.mask();
        };

        ui.subheading("State");
        match self.rng.width {
            // MSBSize::B64 => todo!(),
            MSBSize::B32 => {
                if ui
                    .add(
                        DragValue::new(&mut self.rng.state)
                            .range(0..=u32::MAX)
                            .hexadecimal(8, false, false),
                    )
                    .changed()
                {
                    self.rng.state &= self.rng.width.mask();
                }
            }
            MSBSize::B16 => {
                if ui
                    .add(
                        DragValue::new(&mut self.rng.state)
                            .range(0..=u16::MAX)
                            .hexadecimal(4, false, false),
                    )
                    .changed()
                {
                    self.rng.state &= self.rng.width.mask();
                }
            }
            MSBSize::B8 => {
                if ui
                    .add(
                        DragValue::new(&mut self.rng.state)
                            .range(0..=u8::MAX)
                            .hexadecimal(2, false, false),
                    )
                    .changed()
                {
                    self.rng.state &= self.rng.width.mask();
                }
            }
        };

        ui.add_space(16.0);

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen::<u64>() & self.rng.width.mask();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
