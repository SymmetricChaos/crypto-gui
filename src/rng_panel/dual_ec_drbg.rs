use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use crypto_bigint::U256;
use hashers::{sha::Sha256, traits::StatefulHasher};
use rand::{thread_rng, Rng};
use rngs::{
    dual_ec_drbg::{DualEcDrbgP256, P, P256, Q},
    ClassicRng,
};

pub struct DualEcFrame {
    rng: DualEcDrbgP256,
    entropy: U256,
    personalization: String,
    nonce: u128,
    n_random: usize,
    randoms: String,
}

impl Default for DualEcFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            entropy: U256::from_u64(1),
            personalization: String::new(),
            nonce: 0,
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl DualEcFrame {
    pub fn instantiate(&mut self) {
        let mut hasher = Sha256::init();
        hasher.update(&[1_u8]);
        hasher.update(&256_u32.to_be_bytes());
        hasher.update(&self.entropy.to_be_bytes());
        hasher.update(&self.nonce.to_be_bytes());
        hasher.update(self.personalization.as_bytes());
        let bytes: Vec<u8> = hasher.finalize();
        self.rng = DualEcDrbgP256 {
            state: U256::from_be_slice(&bytes),
            buffer: Vec::with_capacity(30),
            ctr: 0,
        }
    }
}

impl ClassicRngFrame for DualEcFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/dual_ec_drbg.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Instantiation");
        ui.label("NIST instantiation procedures are flexible. This implementation uses SHA-256 to produce the state from the input below, where || is concatenation.\n0x01 || 0x00000001 || Entropy || Nonce || PersonalizationString");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Entropy");
            if ui.button("🎲").on_hover_text("256 random bits").clicked() {
                let mut rng = thread_rng();
                #[cfg(target_pointer_width = "64")]
                let mut t_state = [0u64; 4];
                #[cfg(target_pointer_width = "32")]
                let mut t_state = [0u32; 8];
                rng.fill(&mut t_state);
                self.entropy = U256::from_words(t_state).rem(&P256.m);
                self.instantiate();
            }
        });
        ui.label("Random data from an entropy source. Here always 256 bits.");
        for limb in self.entropy.as_words_mut() {
            #[cfg(target_pointer_width = "64")]
            ui.u64_hex_edit(limb);
            #[cfg(target_pointer_width = "32")]
            ui.u32_hex_edit(limb);
        }

        ui.subheading("Nonce");
        ui.label("Unique value for each instantiation. Here always 128 bits.");
        if ui.u128_hex_edit(&mut self.nonce).lost_focus() {
            self.instantiate();
        }
        ui.add_space(8.0);

        ui.subheading("Personalization String");
        ui.label("Static globally unique value. Arbitrary length.");
        if ui.control_string(&mut self.personalization).lost_focus() {
            self.instantiate();
        }
        ui.add_space(8.0);

        ui.subheading("Current State");
        for limb in self.rng.state.as_words_mut() {
            #[cfg(target_pointer_width = "64")]
            ui.u64_hex_edit(limb);
            #[cfg(target_pointer_width = "32")]
            ui.u32_hex_edit(limb);
        }

        ui.collapsing("Constants", |ui| {
            ui.subheading("Elliptic Curve");
            ui.label("y² = x³ + ax + b (mod m)");
            ui.add_space(8.0);

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
        ui.add_space(8.0);

        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
