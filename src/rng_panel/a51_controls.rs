use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::a51::A51Rng;
use strum::IntoEnumIterator;

pub struct A51Frame {
    rng: A51Rng,
    randoms: String,
    n_random: usize,
    key: u64,
    burst: String,
}

impl Default for A51Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
            key: 1,
            burst: String::from("A -> B:\nB -> A:"),
        }
    }
}

impl A51Frame {}

impl ClassicRngFrame for A51Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/a51.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.subheading("Key (Taken in Big-endian Order)");
        if ui.u64_hex_edit(&mut self.key).changed() {
            self.rng.key = self.key.to_be_bytes();
            self.rng.ksa()
        }
        ui.add_space(8.0);

        ui.subheading("Frame Number (Limited to 22 Bits)");
        if ui.u32_hex_edit(&mut self.rng.frame_number).changed() {
            self.rng.frame_number &= 0x3fffff; // mask off the high bits
            self.rng.ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Rekeying Rule");
        ui.horizontal(|ui| {
            for rule in rngs::a51::ReKeyRule::iter() {
                ui.selectable_value(&mut self.rng.rekey, rule, rule.to_string());
            }
        });

        ui.subheading("LFSRs");
        ui.monospace(format!("{:019b}", self.rng.lfsrs[0]));
        ui.monospace("^^^  ^             ");
        ui.add_space(4.0);
        ui.monospace(format!("{:022b}", self.rng.lfsrs[1]));
        ui.monospace("^^                    ");
        ui.add_space(4.0);
        ui.monospace(format!("{:023b}", self.rng.lfsrs[2]));
        ui.monospace("^^^            ^       ");
        ui.add_space(16.0);

        if ui.button("Step").clicked() {
            self.rng.next_bit();
        }
        ui.add_space(16.0);

        if ui.button("Burst (228 steps)").clicked() {
            let (a, b) = self.rng.burst_bytes();
            self.burst = format!("A -> B: {:02x?}\nB -> A: {:02x?}", a, b)
        }
        ui.label(&self.burst);
        ui.add_space(16.0);

        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = rng.gen();
        self.rng.key = self.key.to_be_bytes();
        self.rng.frame_number = rng.gen();
        self.rng.frame_number &= 0x3fffff; // mask off the high bits
        self.rng.ksa()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
