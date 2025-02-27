mod a51_controls;
mod a52_controls;
mod alternating_step_controls;
mod blumblumshub_controls;
mod chacha_controls;
mod geffe_controls;
mod halton_controls;
mod isaac_controls;
mod jsf_controls;
mod lcg_controls;
mod lfg_controls;
mod lfsr_controls;
mod mersenne_twister_controls;
mod middle_square_binary_controls;
mod middle_square_controls;
mod naor_reingold_controls;
mod pcg_controls;
mod rc4_controls;
mod salsa20_controls;
mod self_shrinking_generator_controls;
mod shrinking_generator_controls;
mod splitmix_controls;
mod vmpcr_controls;
mod weyl_controls;
mod xorshift_controls;
mod xoshiro_controls;

use egui::Ui;

use rngs::{
    ids::{rng_categories::RngCategory, RngId},
    ClassicRng,
};

pub trait ClassicRngFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn rng(&self) -> &dyn ClassicRng;
    fn randomize(&mut self);
    fn reset(&mut self);
}

// Quick simple combo box builder
fn combox_box(
    rng: &[RngId],
    active_rng: &mut Option<RngId>,
    rng_category: RngCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt(rng_category.to_string())
            .selected_text(rng_category.to_string())
            .show_ui(ui, |ui| {
                for id in rng {
                    ui.selectable_value(active_rng, Some(*id), id.to_string());
                }
            });
        ui.menu_button("+", |ui| ui.label(rng_category.description()))
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct RngInterface {
    a51: a51_controls::A51Frame,
    a52: a52_controls::A52Frame,
    alternating_step: alternating_step_controls::AlternatingStepFrame,
    blumblumshub: blumblumshub_controls::BlumBlumShubFrame,
    chacha: chacha_controls::ChaChaFrame,
    geffe: geffe_controls::GeffeFrame,
    halton: halton_controls::HaltonFrame,
    jsf: jsf_controls::JsfFrame,
    lcg: lcg_controls::LcgFrame,
    lfg: lfg_controls::LfgFrame,
    lfsr: lfsr_controls::LfsrFrame,
    mersenne_twister: mersenne_twister_controls::MTFrame,
    middle_square: middle_square_controls::MiddleSquareFrame,
    middle_square_binary: middle_square_binary_controls::MiddleSquareBinaryFrame,
    naor_reingold: naor_reingold_controls::NaorReingoldFrame,
    pcg: pcg_controls::PcgFrame,
    rc4: rc4_controls::Rc4Frame,
    salsa20: salsa20_controls::Salsa20Frame,
    self_shrinking_generator: self_shrinking_generator_controls::SelfShrinkingGeneratorFrame,
    shrinking_generator: shrinking_generator_controls::ShrinkingGeneratorFrame,
    splitmix: splitmix_controls::SplitmixFrame,
    vmpcr: vmpcr_controls::VmpcrFrame,
    weyl: weyl_controls::WeylSequenceFrame,
    xorshift: xorshift_controls::XorshiftFrame,
    xoshiro: xoshiro_controls::XoshiroFrame,
}

impl RngInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_rng: &mut Option<RngId>) {
        combox_box(
            &[
                RngId::AlternatingStep,
                RngId::Jsf,
                RngId::Lcg,
                RngId::Lfg,
                RngId::Lfsr,
                RngId::MersenneTwister,
                RngId::MiddleSquare,
                RngId::MiddleSquareBinary,
                RngId::Rc4,
                RngId::SelfShrinkingGenerator,
                RngId::ShrinkingGenerator,
                RngId::Splitmix,
                RngId::Vmpcr,
                RngId::Xorshift,
                RngId::Xoshiro,
            ],
            active_rng,
            RngCategory::PRNG,
            ui,
        );

        combox_box(
            &[RngId::Halton, RngId::Weyl],
            active_rng,
            RngCategory::QRNG,
            ui,
        );

        combox_box(
            &[
                RngId::A51,
                RngId::A52,
                RngId::BlumBlumShub,
                RngId::ChaCha,
                RngId::Geffe,
                RngId::NaorReingold,
                RngId::Pcg,
                RngId::Salsa20,
            ],
            active_rng,
            RngCategory::CSPRNG,
            ui,
        );

        combox_box(&[], active_rng, RngCategory::TRNG, ui);
    }

    pub fn get_active_rng(&mut self, active_rng: &RngId) -> &mut dyn ClassicRngFrame {
        match active_rng {
            RngId::A51 => &mut self.a51,
            RngId::A52 => &mut self.a52,
            RngId::AlternatingStep => &mut self.alternating_step,
            RngId::BlumBlumShub => &mut self.blumblumshub,
            RngId::ChaCha => &mut self.chacha,
            RngId::Geffe => &mut self.geffe,
            RngId::Halton => &mut self.halton,
            RngId::Jsf => &mut self.jsf,
            RngId::Lcg => &mut self.lcg,
            RngId::Lfg => &mut self.lfg,
            RngId::Lfsr => &mut self.lfsr,
            RngId::MersenneTwister => &mut self.mersenne_twister,
            RngId::MiddleSquare => &mut self.middle_square,
            RngId::MiddleSquareBinary => &mut self.middle_square_binary,
            RngId::NaorReingold => &mut self.naor_reingold,
            RngId::Pcg => &mut self.pcg,
            RngId::Rc4 => &mut self.rc4,
            RngId::Salsa20 => &mut self.salsa20,
            RngId::SelfShrinkingGenerator => &mut self.self_shrinking_generator,
            RngId::ShrinkingGenerator => &mut self.shrinking_generator,
            RngId::Splitmix => &mut self.splitmix,
            RngId::Vmpcr => &mut self.vmpcr,
            RngId::Weyl => &mut self.weyl,
            RngId::Xorshift => &mut self.xorshift,
            RngId::Xoshiro => &mut self.xoshiro,
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
