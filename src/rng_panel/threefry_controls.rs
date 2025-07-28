use egui::DragValue;
use rngs::threefry::threefry::{Threefry2_32, Threefry2_64, Threefry4_32, Threefry4_64};

use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    T2_32,
    T2_64,
    T4_32,
    T4_64,
}

pub struct ThreefryFrame {
    rng2_32: Threefry2_32,
    rng4_32: Threefry4_32,
    rng2_64: Threefry2_64,
    rng4_64: Threefry4_64,
    variant: Variant,
    randoms: String,
    n_random: usize,
    rounds: usize,
}

impl Default for ThreefryFrame {
    fn default() -> Self {
        Self {
            rng2_32: Default::default(),
            rng2_64: Default::default(),
            rng4_32: Default::default(),
            rng4_64: Default::default(),
            variant: Variant::T2_32,
            randoms: String::new(),
            n_random: 5,
            rounds: 20,
        }
    }
}

impl ClassicRngFrame for ThreefryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/threefry",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::T2_32, "Threefry-2×32");
            ui.selectable_value(&mut self.variant, Variant::T2_64, "Threefry-2×64");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::T4_32, "Threefry-4×32");
            ui.selectable_value(&mut self.variant, Variant::T4_64, "Threefry-4×64");
        });
        ui.add_space(8.0);

        if ui
            .add(DragValue::new(&mut self.rounds).range(1..=16))
            .changed()
        {
            self.rng2_32.rounds = self.rounds;
            self.rng2_64.rounds = self.rounds;
            self.rng4_32.rounds = self.rounds;
            self.rng4_64.rounds = self.rounds;
        }

        match self.variant {
            Variant::T2_32 => {
                ui.subheading("Key");
                ui.u32_hex_edit(&mut self.rng2_32.key[0]);
                ui.u32_hex_edit(&mut self.rng2_32.key[1]);
                ui.add_space(8.0);

                ui.subheading("Counter");
                ui.u32_hex_edit(&mut self.rng2_32.ctr[0]);
                ui.u32_hex_edit(&mut self.rng2_32.ctr[1]);
                ui.add_space(8.0);
            }
            Variant::T2_64 => {
                ui.subheading("Key");
                ui.u64_hex_edit(&mut self.rng2_64.key[0]);
                ui.u64_hex_edit(&mut self.rng2_64.key[1]);
                ui.add_space(8.0);

                ui.subheading("Counter");
                ui.u64_hex_edit(&mut self.rng2_64.ctr[0]);
                ui.u64_hex_edit(&mut self.rng2_64.ctr[1]);
                ui.add_space(8.0);
            }
            Variant::T4_32 => {
                ui.subheading("Key");
                ui.u32_hex_edit(&mut self.rng4_32.key[0]);
                ui.u32_hex_edit(&mut self.rng4_32.key[1]);
                ui.add_space(8.0);

                ui.subheading("Counter");
                ui.u32_hex_edit(&mut self.rng4_32.ctr[0]);
                ui.u32_hex_edit(&mut self.rng4_32.ctr[1]);
                ui.u32_hex_edit(&mut self.rng4_32.ctr[2]);
                ui.u32_hex_edit(&mut self.rng4_32.ctr[3]);
                ui.add_space(8.0);
            }
            Variant::T4_64 => {
                ui.subheading("Key");
                ui.u64_hex_edit(&mut self.rng4_64.key[0]);
                ui.u64_hex_edit(&mut self.rng4_64.key[1]);
                ui.add_space(8.0);

                ui.subheading("Counter");
                ui.u64_hex_edit(&mut self.rng4_64.ctr[0]);
                ui.u64_hex_edit(&mut self.rng4_64.ctr[1]);
                ui.u64_hex_edit(&mut self.rng4_64.ctr[2]);
                ui.u64_hex_edit(&mut self.rng4_64.ctr[3]);
                ui.add_space(8.0);
            }
        }

        generate_randoms_box(
            ui,
            match self.variant {
                Variant::T2_32 => &mut self.rng2_32,
                Variant::T2_64 => &mut self.rng2_64,
                Variant::T4_32 => &mut self.rng4_32,
                Variant::T4_64 => &mut self.rng4_64,
            },
            &mut self.n_random,
            &mut self.randoms,
        );
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.variant {
            Variant::T2_32 => &mut self.rng2_32,
            Variant::T2_64 => &mut self.rng2_64,
            Variant::T4_32 => &mut self.rng4_32,
            Variant::T4_64 => &mut self.rng4_64,
        }
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
