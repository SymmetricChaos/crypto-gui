use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use crypto_bigint::U256;
use rand::{thread_rng, Rng};
use rngs::{
    dual_ec_drbg::{DualEcDrbgP256, P, P256, Q},
    ClassicRng,
};

pub struct DualEcFrame {
    rng: DualEcDrbgP256,
    seed: U256,
    n_random: usize,
    randoms: String,
}

impl Default for DualEcFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            seed: U256::from_u64(1),
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl DualEcFrame {}

impl ClassicRngFrame for DualEcFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/dual_ec_drbg.rs",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed");
            if ui
                .button("🎲")
                .on_hover_text("random 256-bit state")
                .clicked()
            {
                let mut rng = thread_rng();
                let mut t_state = [0u64; 4];
                rng.fill(&mut t_state);
                self.seed = U256::from_words(t_state);
            }
        });
        ui.label(self.seed.to_string());

        ui.collapsing("Constants", |ui| {
            ui.subheading("Elliptic Curve");
            ui.label("y² = x³ + ax + b (mod m)");
            ui.add_space(4.0);

            ui.subheading("Base Field Size (m)");
            ui.label(P256.m.to_string());
            ui.add_space(4.0);

            ui.subheading("a");
            ui.label(P256.a.to_string());
            ui.subheading("b");
            ui.label(P256.b.to_string());
            ui.add_space(8.0);

            ui.subheading("Point P");
            ui.label(format!("x: {}", P.x.unwrap().to_string()));
            ui.label(format!("y: {}", P.y.unwrap().to_string()));
            ui.add_space(4.0);
            ui.subheading("Point Q");
            ui.label(format!("x: {}", Q.x.unwrap().to_string()));
            ui.label(format!("y: {}", Q.y.unwrap().to_string()));
        });

        ui.subheading("Current State");
        ui.label(self.rng.state.to_string());

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn ClassicRng {
        todo!()
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
