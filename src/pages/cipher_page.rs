use eframe::egui::{CentralPanel, Context, Grid, ScrollArea, SidePanel};

use crate::ids::CipherID;

use super::{Page, category_page::CipherCategory};

// #[derive(Default)]
// pub struct CipherControls {
//     cipher_category: CipherCategory,
// }

// impl CipherControls {
//     pub fn view(
//         &mut self,
//         ctx: &Context,
//         cipher_category: &mut CipherCategory,
//         active_cipher: &mut CipherID,
//         active_page: &mut Page,
//     ) {
//         SidePanel::left("cipher_selector_panel")
//             .max_width(300.0)
//             .show(ctx, |ui| {
//                 ui.label("Examples");
//                 for id in cipher_category.ciphers() {
//                     if ui
//                         .selectable_value(active_cipher, *id, id.to_string())
//                         .clicked()
//                     {
//                         *active_page = Page::Cipher(CipherID::Caesar);
//                     };
//                 }
//             });
//         CentralPanel::default().show(ctx, |ui| {
//             Grid::new("cipher_categories").show(ui, |ui| {
//                 ui.selectable_value(cipher_category, CipherCategory::Substituion, "Substitution");
//                 ui.selectable_value(
//                     cipher_category,
//                     CipherCategory::Polyalphabetic,
//                     "Polyalphabetic",
//                 );
//                 ui.selectable_value(
//                     cipher_category,
//                     CipherCategory::RotorMachine,
//                     "Rotor Machine",
//                 );
//                 ui.selectable_value(
//                     cipher_category,
//                     CipherCategory::Transposition,
//                     "Transposition",
//                 );
//                 ui.end_row();
//                 ui.selectable_value(cipher_category, CipherCategory::Playfair, "Playfair");
//                 ui.selectable_value(cipher_category, CipherCategory::Tactical, "Tactical");
//                 ui.selectable_value(cipher_category, CipherCategory::Polybius, "Polybius");
//             });
//             ScrollArea::vertical().show(ui, |ui| ui.label(self.cipher_category.description()));
//         });
//     }
// }
