use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::RichText;
use rand::{thread_rng, Rng};
use rngs::rule30::{Rule30, WolframCode};
use strum::IntoEnumIterator;

pub struct Rule30Frame {
    rng: Rule30,
    n_random: usize,
    randoms: String,
    seed: u64,
    rule: WolframCode,
    tap: usize,
}

impl Default for Rule30Frame {
    fn default() -> Self {
        Self {
            rng: Rule30::default(),
            n_random: 1,
            randoms: String::new(),
            seed: 12345,
            rule: WolframCode::R30,
            tap: 127,
        }
    }
}

impl Rule30Frame {}

impl ClassicRngFrame for Rule30Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/rule30.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Seed Value");
        if ui.u64_hex_edit(&mut self.seed).lost_focus() {
            self.rng = Rule30::init(self.seed, self.rule, self.tap)
        }
        ui.add_space(8.0);

        ui.subheading("Tap Row");
        if ui
            .add(egui::DragValue::new(&mut self.tap).range(0..=127))
            .lost_focus()
        {
            self.rng = Rule30::init(self.seed, self.rule, self.tap)
        };
        ui.add_space(8.0);

        ui.subheading("Wolfram Code");
        ui.horizontal(|ui| {
            for variant in WolframCode::iter() {
                if ui
                    .selectable_value(&mut self.rule, variant, variant.to_string())
                    .clicked()
                {
                    self.rng = Rule30::init(self.seed, self.rule, self.tap)
                }
            }
        });
        ui.add_space(8.0);

        ui.subheading("Current State (128-bits)");
        ui.label(
            // RichText::new(self.rng.print_state('▁', '█')) // looks better, IMO but not as compatible
            RichText::new(self.rng.print_state('_', '#'))
                .size(10.0)
                .monospace(),
        );

        ui.add_space(8.0);
        if ui.button("Step").clicked() {
            self.rng.step();
        }

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        self.seed = thread_rng().gen();
        self.rng = Rule30::init(self.seed, self.rule, self.tap)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
