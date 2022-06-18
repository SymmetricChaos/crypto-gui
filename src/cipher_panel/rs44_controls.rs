use crate::ciphers::tactical::RS44;

use super::{generic_components::*, View, ViewableCipher};
use eframe::{egui::{TextStyle, Ui, Button, DragValue, RichText}, epaint::Color32};

impl ViewableCipher for RS44 {}

fn cell_button(grille: &mut RS44, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.stencil[(x, y)];
    let symbol = cell.to_char().to_string();
    ui.add_enabled(false, Button::new(symbol).frame(false));
}

impl View for RS44 {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label(format!("xlabels {:?}",self.xlabels));
        ui.label(format!("column numbers {:?}",self.column_nums));
        
        ui.label(format!("ylabels {:?}",self.ylabels));

        ui.add(
            DragValue::new(&mut self.message_key.0).prefix("x: ").clamp_range(0..=24)
        );
        ui.add(
            DragValue::new(&mut self.message_key.1).prefix("y: ").clamp_range(0..=23)
        );
        if self.stencil[self.message_key].is_blocked() {
            ui.label(
                RichText::new("Invalid Start Position")
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        }


        ui.collapsing("Message Key", |ui| {

            ui.label("Time of Transmission");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.hours).clamp_range(0..=23)
                );
                ui.label(":");
                ui.add(
                    DragValue::new(&mut self.minutes).clamp_range(0..=59)
                );

            });
        });




        ui.spacing_mut().item_spacing = (2.0, 2.0).into();
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        for x in 0..self.stencil.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.stencil.num_cols() {
                    cell_button(self, x, y, ui);
                }
            });
        }

    }
}
