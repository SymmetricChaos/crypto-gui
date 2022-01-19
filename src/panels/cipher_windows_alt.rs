

use eframe::egui::{self, CtxRef};

use super::{caesar_widget::CaesarWidget, affine_widget::AffineWidget};

/// Something to view in the cipher windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

#[derive(PartialEq)]
pub enum CipherName {
    Caesar,
    Affine,
}

impl Default for CipherName {
    fn default() -> Self {
        Self::Caesar
    }
}

#[derive(Default)]
pub struct Ciphers {
    caesar: CaesarWidget,
    affine: AffineWidget,
    open_panel: CipherName,
}

impl Ciphers {
    pub fn ui(&mut self, ctx: &CtxRef) {

        egui::TopBottomPanel::top("selector_panel").show(ctx, |ui|{
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.open_panel, CipherName::Caesar, "Caesar");
                ui.selectable_value(&mut self.open_panel, CipherName::Affine, "Affine");
            });
            ui.separator();

            match self.open_panel {
                CipherName::Caesar => {
                    ui.add(&mut self.caesar);
                }
                CipherName::Affine => {
                    ui.add(&mut self.affine);
                }
            }
        });

    }
}