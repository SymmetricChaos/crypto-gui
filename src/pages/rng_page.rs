use eframe::egui::{CentralPanel, Context, ScrollArea, SidePanel};

use crate::ids::RngID;

use super::Page;

#[derive(Default)]
pub struct RngInfoPage {}

impl RngInfoPage {
    pub fn view(&mut self, ctx: &Context, active_rng: &mut RngID, active_page: &mut Page) {
        SidePanel::left("rng_selector_panel")
            .max_width(300.0)
            .show(ctx, |ui| {
                ui.label("Examples");
                for id in [RngID::Lfsr] {
                    if ui
                        .selectable_value(active_rng, id, id.to_string())
                        .clicked()
                    {
                        *active_page = Page::Rng(None);
                    };
                }
            });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| ui.label("RNGs are important in cryptography."));
        });
    }
}
