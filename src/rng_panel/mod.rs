pub mod halton_controls;
pub mod lcg_controls;
pub mod lfg_controls;
pub mod lfsr_controls;
pub mod middle_square_controls;
pub mod pcg_controls;
pub mod rc4_controls;
pub mod weyl_controls;

use egui::Ui;
use rngs::{
    ids::{rng_categories::RngCategory, RngId},
    ClassicRng,
};

use self::{
    halton_controls::HaltonFrame, lcg_controls::LcgFrame, lfsr_controls::LfsrFrame,
    middle_square_controls::MiddleSquareFrame, pcg_controls::PcgFrame, rc4_controls::Rc4Frame,
    weyl_controls::WeylSequenceFrame,
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
    code_category: RngCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(code_category.to_string())
            .selected_text(code_category.to_string())
            .show_ui(ui, |ui| {
                for id in rng {
                    ui.selectable_value(active_rng, Some(*id), id.to_string());
                }
            });
        ui.label("+").on_hover_text(code_category.description());
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct RngInterface {
    halton: HaltonFrame,
    lcg: LcgFrame,
    lfsr: LfsrFrame,
    middle_square: MiddleSquareFrame,
    pcg: PcgFrame,
    rc4: Rc4Frame,
    weyl: WeylSequenceFrame,
}

impl RngInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_rng: &mut Option<RngId>) {
        combox_box(
            &[
                RngId::Lcg,
                RngId::Lfsr,
                RngId::MiddleSquare,
                RngId::Pcg,
                RngId::Rc4,
            ],
            active_rng,
            RngCategory::Pseudorandom,
            ui,
        );

        combox_box(
            &[RngId::Halton, RngId::Weyl],
            active_rng,
            RngCategory::Quasirandom,
            ui,
        );

        combox_box(&[], active_rng, RngCategory::Truerandom, ui);
    }

    pub fn get_active_rng(&mut self, active_rng: &RngId) -> &mut dyn ClassicRngFrame {
        match active_rng {
            RngId::Halton => &mut self.halton,
            RngId::Lcg => &mut self.lcg,
            // RngId::Lfg => &mut self.lfg,
            RngId::Lfsr => &mut self.lfsr,
            RngId::MiddleSquare => &mut self.middle_square,
            RngId::Pcg => &mut self.pcg,
            RngId::Rc4 => &mut self.rc4,
            RngId::Weyl => &mut self.weyl,
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
