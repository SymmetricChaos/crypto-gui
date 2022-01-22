use eframe::{egui::{CtxRef, self, TopBottomPanel, SidePanel, CentralPanel}, epi};

use super::View;

pub struct ClassicCrypto {
    selector: super::SelectorPanel,
    display: super::DisplayPanel,
    control: super::ControlPanel,

}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            selector: super::SelectorPanel::default(),
            display: super::DisplayPanel::default(),
            control: super::ControlPanel::default(),
        }
    }
}


impl epi::App for ClassicCrypto {
    fn name(&self) -> &str {
        "Classical Cryptography"
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    

    fn update(&mut self, ctx: &CtxRef, frame: &epi::Frame) {
        TopBottomPanel::top("selector_panel").show(ctx, |ui| {
            self.selector.ui(ui)
        });
        SidePanel::right("display_panel").show(ctx, |ui| {
            self.display.ui(ui)
        });
        CentralPanel::default().show(ctx, |ui| {
            self.control.ui(ui)
        });
    }
}