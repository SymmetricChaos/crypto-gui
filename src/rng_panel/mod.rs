mod a51_controls;
mod a52_controls;
mod alternating_step_controls;
mod blum_blum_shub_controls;
mod chacha_controls;
mod dual_ec_drbg;
mod geffe_controls;
mod halton_controls;
mod hc128_controls;
mod hc256_controls;
mod isaac_controls;
mod jsf_controls;
mod kiss_controls;
mod lcg_controls;
mod lfg_controls;
mod lfsr_controls;
mod mersenne_twister_controls;
mod middle_square_binary_controls;
mod middle_square_controls;
mod naor_reingold_controls;
mod pcg_controls;
mod rabbit_controls;
mod randu_controls;
mod rc4_controls;
mod rule30_controls;
mod salsa20_controls;
mod self_shrinking_generator_controls;
mod shrinking_generator_controls;
mod splitmix_controls;
mod tt800_controls;
mod vmpcr_controls;
mod well_controls;
mod weyl_controls;
mod xorshift64_controls;
mod xoshiro_controls;

use egui::Ui;

use rngs::{
    ids::{rng_categories::RngCategory, RngId},
    ClassicRng,
};

pub trait ClassicRngFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn rng(&mut self) -> &mut dyn ClassicRng;
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
    blumblumshub: blum_blum_shub_controls::BlumBlumShubFrame,
    chacha: chacha_controls::ChaChaFrame,
    dual_ec_drbg: dual_ec_drbg::DualEcFrame,
    geffe: geffe_controls::GeffeFrame,
    halton: halton_controls::HaltonFrame,
    hc128: hc128_controls::Hc128Frame,
    hc256: hc256_controls::Hc256Frame,
    jsf: jsf_controls::JsfFrame,
    kiss: kiss_controls::KissFrame,
    lcg: lcg_controls::LcgFrame,
    lfg: lfg_controls::LfgFrame,
    lfsr: lfsr_controls::LfsrFrame,
    mersenne_twister: mersenne_twister_controls::MTFrame,
    middle_square: middle_square_controls::MiddleSquareFrame,
    middle_square_binary: middle_square_binary_controls::MiddleSquareBinaryFrame,
    naor_reingold: naor_reingold_controls::NaorReingoldFrame,
    pcg: pcg_controls::PcgFrame,
    rabbit: rabbit_controls::RabbitFrame,
    randu: randu_controls::RanduFrame,
    rc4: rc4_controls::Rc4Frame,
    rule30: rule30_controls::Rule30Frame,
    salsa20: salsa20_controls::Salsa20Frame,
    self_shrinking_generator: self_shrinking_generator_controls::SelfShrinkingGeneratorFrame,
    shrinking_generator: shrinking_generator_controls::ShrinkingGeneratorFrame,
    splitmix: splitmix_controls::SplitmixFrame,
    tt800: tt800_controls::Tt800Frame,
    vmpcr: vmpcr_controls::VmpcrFrame,
    well: well_controls::WellFrame,
    weyl: weyl_controls::WeylSequenceFrame,
    xorshift64: xorshift64_controls::Xorshift64Frame,
    xoshiro: xoshiro_controls::XoshiroFrame,
}

impl RngInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_rng: &mut Option<RngId>) {
        combox_box(
            &[
                RngId::AlternatingStep,
                RngId::Jsf,
                RngId::Kiss,
                RngId::Lcg,
                RngId::Lfg,
                RngId::Lfsr,
                RngId::MersenneTwister,
                RngId::MiddleSquare,
                RngId::MiddleSquareBinary,
                RngId::Pcg,
                RngId::Randu,
                RngId::Rc4,
                RngId::Rule30,
                RngId::SelfShrinkingGenerator,
                RngId::ShrinkingGenerator,
                RngId::Splitmix,
                RngId::Tt800,
                RngId::Well,
                RngId::Vmpcr,
                RngId::Xorshift64,
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
                RngId::DualEcDrbg,
                RngId::Geffe,
                RngId::Hc128,
                RngId::Hc256,
                RngId::NaorReingold,
                RngId::Rabbit,
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
            RngId::DualEcDrbg => &mut self.dual_ec_drbg,
            RngId::Geffe => &mut self.geffe,
            RngId::Halton => &mut self.halton,
            RngId::Hc128 => &mut self.hc128,
            RngId::Hc256 => &mut self.hc256,
            RngId::Jsf => &mut self.jsf,
            RngId::Kiss => &mut self.kiss,
            RngId::Lcg => &mut self.lcg,
            RngId::Lfg => &mut self.lfg,
            RngId::Lfsr => &mut self.lfsr,
            RngId::MersenneTwister => &mut self.mersenne_twister,
            RngId::MiddleSquare => &mut self.middle_square,
            RngId::MiddleSquareBinary => &mut self.middle_square_binary,
            RngId::NaorReingold => &mut self.naor_reingold,
            RngId::Pcg => &mut self.pcg,
            RngId::Rabbit => &mut self.rabbit,
            RngId::Randu => &mut self.randu,
            RngId::Rc4 => &mut self.rc4,
            RngId::Rule30 => &mut self.rule30,
            RngId::Salsa20 => &mut self.salsa20,
            RngId::SelfShrinkingGenerator => &mut self.self_shrinking_generator,
            RngId::ShrinkingGenerator => &mut self.shrinking_generator,
            RngId::Splitmix => &mut self.splitmix,
            RngId::Tt800 => &mut self.tt800,
            RngId::Vmpcr => &mut self.vmpcr,
            RngId::Well => &mut self.well,
            RngId::Weyl => &mut self.weyl,
            RngId::Xorshift64 => &mut self.xorshift64,
            RngId::Xoshiro => &mut self.xoshiro,
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
