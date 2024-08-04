use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::a51::A51Rng;

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
        ui.subheading("Key (Taken in Big-endian Order)");
        if ui.u64_drag_value_hex(&mut self.key).changed() {
            self.rng.key = self.key.to_be_bytes();
            self.rng.ksa()
        }
        ui.add_space(8.0);

        ui.subheading("Frame Number (Limited to 22 Bits)");
        if ui.u32_drag_value_hex(&mut self.rng.frame_number).changed() {
            self.rng.frame_number &= 0x3fffff; // mask off the high bits
            self.rng.ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Rekeying Rule");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.rng.rekey, rngs::a51::ReKeyRule::K114, "114-bits");
            ui.selectable_value(&mut self.rng.rekey, rngs::a51::ReKeyRule::K228, "228-bits");
            ui.selectable_value(&mut self.rng.rekey, rngs::a51::ReKeyRule::KNever, "Never")
        });

        ui.subheading("LFSRs");
        ui.monospace(format!("{:019b}", self.rng.lfsrs[0].register));
        ui.monospace("^^^  ^             ");
        ui.add_space(4.0);
        ui.monospace(format!("{:022b}", self.rng.lfsrs[1].register));
        ui.monospace("^^                    ");
        ui.add_space(4.0);
        ui.monospace(format!("{:023b}", self.rng.lfsrs[2].register));
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

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
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
