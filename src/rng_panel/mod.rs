mod blumblumshub_controls;
mod chacha_controls;
mod geffe_controls;
mod halton_controls;
mod jsf_controls;
mod lcg_controls;
mod lfg_controls;
mod lfsr_controls;
mod mersenne_twister_controls;
mod middle_square_binary_controls;
mod middle_square_controls;
mod pcg_controls;
mod rc4_controls;
mod splitmix_controls;
mod vmpcr_controls;
mod weyl_controls;
mod xorshift_controls;
mod xoshiro_controls;
use chacha_controls::ChaChaFrame;
use geffe_controls::GeffeFrame;
use middle_square_binary_controls::MiddleSquareBinaryFrame;

use egui::Ui;

use rngs::{
    ids::{rng_categories::RngCategory, RngId},
    ClassicRng,
};

use self::{
    blumblumshub_controls::BlumBlumShubFrame, halton_controls::HaltonFrame, jsf_controls::JsfFrame,
    lcg_controls::LcgFrame, lfsr_controls::LfsrFrame, mersenne_twister_controls::MTFrame,
    middle_square_controls::MiddleSquareFrame, pcg_controls::PcgFrame, rc4_controls::Rc4Frame,
    splitmix_controls::SplitmixFrame, vmpcr_controls::VmpcrFrame, weyl_controls::WeylSequenceFrame,
    xorshift_controls::XorshiftFrame, xoshiro_controls::XoshiroFrame,
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
        egui::ComboBox::from_id_source(rng_category.to_string())
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
    blumblumshub: BlumBlumShubFrame,
    chacha: ChaChaFrame,
    geffe: GeffeFrame,
    halton: HaltonFrame,
    jsf: JsfFrame,
    lcg: LcgFrame,
    lfsr: LfsrFrame,
    mersenne_twister: MTFrame,
    middle_square: MiddleSquareFrame,
    middle_square_binary: MiddleSquareBinaryFrame,
    pcg: PcgFrame,
    rc4: Rc4Frame,
    splitmix: SplitmixFrame,
    vmpcr: VmpcrFrame,
    weyl: WeylSequenceFrame,
    xorshift: XorshiftFrame,
    xoshiro: XoshiroFrame,
}

impl ClassicRngFrame for RngInterface {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        <JsfFrame as ClassicRngFrame>::ui(&mut self.jsf, ui, _errors)
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        <JsfFrame as ClassicRngFrame>::rng(&self.jsf)
    }

    fn randomize(&mut self) {
        <JsfFrame as ClassicRngFrame>::randomize(&mut self.jsf)
    }

    fn reset(&mut self) {
        <JsfFrame as ClassicRngFrame>::reset(&mut self.jsf)
    }
}

impl RngInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_rng: &mut Option<RngId>) {
        combox_box(
            &[
                RngId::BlumBlumShub,
                RngId::Jsf,
                RngId::Lcg,
                RngId::Lfsr,
                RngId::MersenneTwister,
                RngId::MiddleSquare,
                RngId::MiddleSquareBinary,
                RngId::Rc4,
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
            &[RngId::BlumBlumShub, RngId::ChaCha, RngId::Geffe, RngId::Pcg],
            active_rng,
            RngCategory::CSPRNG,
            ui,
        );

        combox_box(&[], active_rng, RngCategory::TRNG, ui);
    }

    pub fn get_active_rng(&mut self, active_rng: &RngId) -> &mut dyn ClassicRngFrame {
        match active_rng {
            RngId::BlumBlumShub => &mut self.blumblumshub,
            RngId::ChaCha => &mut self.chacha,
            RngId::Geffe => &mut self.geffe,
            RngId::Halton => &mut self.halton,
            RngId::Jsf => &mut self.jsf,
            RngId::Lcg => &mut self.lcg,
            // RngId::Lfg => &mut self.lfg,
            RngId::Lfsr => &mut self.lfsr,
            RngId::MersenneTwister => &mut self.mersenne_twister,
            RngId::MiddleSquare => &mut self.middle_square,
            RngId::MiddleSquareBinary => &mut self.middle_square_binary,
            RngId::Pcg => &mut self.pcg,
            RngId::Rc4 => &mut self.rc4,
            RngId::Splitmix => &mut self.splitmix,
            RngId::Vmpcr => &mut self.vmpcr,
            RngId::Weyl => &mut self.weyl,
            RngId::Xorshift => &mut self.xorshift,
            RngId::Xoshiro => &mut self.xoshiro,
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
