use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::middle_square_binary::{MSBSize, MiddleSquareBinary};

pub struct MiddleSquareBinaryFrame {
    rng: MiddleSquareBinary,
    state: u32,
    randoms: String,
    n_random: usize,
}

impl Default for MiddleSquareBinaryFrame {
    fn default() -> Self {
        Self {
            rng: MiddleSquareBinary::default(),
            state: 255,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl ClassicRngFrame for MiddleSquareBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/middle_square_binary.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Size");
        ui.label("Modern desktop and server computer architecture generally has registers of size 32-bits and 64-bits so all of these can be performed exceptionally quickly. The 16-bit and 8-bit versions are mainly of academic interest as they fall into short repeating sequences very quickly.");
        if ui
            .selectable_value(&mut self.rng.width, MSBSize::B32, "32-Bit")
            .clicked()
            || ui
                .selectable_value(&mut self.rng.width, MSBSize::B16, "16-Bit")
                .clicked()
            || ui
                .selectable_value(&mut self.rng.width, MSBSize::B8, "8-Bit")
                .clicked()
        {
            self.rng.state &= self.rng.width.mask();
        };

        ui.horizontal(|ui| {
            ui.subheading("Seed State");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        if ui.u32_hex_edit(&mut self.state).lost_focus() {
            self.rng.state = self.state as u64;
            self.rng.state &= self.rng.width.mask();
        }

        ui.add_space(16.0);

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.state = rng.gen();
        self.rng.state = self.state as u64;
        self.rng.state &= self.rng.width.mask();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
