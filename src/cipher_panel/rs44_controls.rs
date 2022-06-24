use crate::{ciphers::tactical::RS44};

use super::{generic_components::*, View, ViewableCipher};
use eframe::{egui::{TextStyle, Ui, Button, DragValue, RichText, Grid}, epaint::Color32};

impl ViewableCipher for RS44 {}

// fn cell_button_sym_char(grille: &crate::grid::Grid<Symbol<char>>, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
//     let cell = grille[(x, y)];
//     let symbol = cell.to_char().to_string();
//     ui.add_enabled(false, Button::new(symbol).frame(false));
// }

fn cell_button_char(grille: &crate::grid::Grid<char>, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille[(x, y)];
    let symbol = cell.to_string();
    ui.add_enabled(false, Button::new(symbol).frame(false));
}

impl View for RS44 {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        if ui.add(
            DragValue::new(&mut self.message_key.0).prefix("x: ").clamp_range(0..=24)
        ).changed() {
            self.set_full_message_key();
        };
        if ui.add(
            DragValue::new(&mut self.message_key.1).prefix("y: ").clamp_range(0..=23)
        ).changed() {
            self.set_full_message_key();
        };
        if self.stencil[self.message_key].is_blocked() {
            ui.label(
                RichText::new("Invalid Start Position")
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        } else {
            ui.label(" ");
        }

        ui.add_space(16.0);

        
        Grid::new("control_rs44_grid").show(ui, |ui| {
            ui.label(" ");
            ui.label(" ");
            for col in 0..25 {
                if col == self.message_key.0 {
                    ui.label("+");
                }
                else {
                    ui.label(" ");
                }
            }
            ui.end_row();
            ui.label(" ");
            ui.label(" ");
            for l in self.xlabels.iter() {
                ui.label(l.to_string());
            }
            ui.end_row();
            ui.label(" ");
            ui.label(" ");
            for n in self.column_nums.iter() {
                ui.label(n.to_string());
            }
            ui.end_row();
            for row in 0..24 {
                if row == self.message_key.1 {
                    ui.label("+");
                } else {
                    ui.label(" ");
                }
                ui.label(self.ylabels[row]);
                for s in self.stencil.get_col(row) {
                    ui.label(s.to_char().to_string());
                } 
                ui.end_row();
            }
        });
        if ui.button("Copy Stencil to Clipboard").clicked() {
            ui.output().copied_text = self.stencil_to_text();
        }

        ui.text_edit_singleline(&mut self.imported_stencil);
        if ui.button("Import Stencil").clicked() {
            match self.text_to_stencil() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }
        
        

        ui.add_space(10.0);
        // The Message Key area. Not needed for encryption.
        ui.collapsing("Message Key", |ui| {

            ui.label("Time of Transmission");
            ui.horizontal(|ui| {
                if ui.add(
                    DragValue::new(&mut self.hours).clamp_range(0..=23)
                ).changed() {
                    self.set_full_message_key();
                };
                ui.label(":");
                if ui.add(
                    DragValue::new(&mut self.minutes).clamp_range(0..=59)
                ).changed() {
                    self.set_full_message_key();
                };
            });

            ui.spacing_mut().item_spacing = (2.0, 2.0).into();
            ui.style_mut().override_text_style = Some(TextStyle::Monospace);
           
            ui.horizontal(|ui| {
                for letter in ["a", "b", "c", "d", "e"] {
                    ui.add_enabled(false, Button::new(letter).frame(false));
                }
            });
            ui.horizontal(|ui| {
                for _ in 0..5 {
                    ui.add_enabled(false, Button::new("-").frame(false));
                }
            });
            for x in 0..self.message_key_maxtrix.num_rows() {
                ui.horizontal(|ui| {
                    for y in 0..self.message_key_maxtrix.num_cols() {
                        cell_button_char(&self.message_key_maxtrix, x, y, ui);
                    }
                });
            }

            ui.add_space(8.0);
            ui.label(RichText::new(format!("Message Key: {}",self.encrypted_message_key)).strong().monospace());
        });

    }
}



#[cfg(test)]
mod rs44_tests {
    use crate::ciphers::Cipher;

    use super::*;
 
    // check configuration for default
    // https://derekbruff.org/blogs/fywscrypto/historical-crypto/rasterschlussel-44-the-stencil-on-steroids/
    const PLAINTEXT: &'static str = "RAINBOWUNICORNHORNSAREIMMENSELYMOREVALUABLETHANTHOSEOFEVENTHELARGESTNARWHALS";
    const CIPHERTEXT: &'static str = "HNANOESONMEGNANAALHRNTRAUHVSCWSTNAOAWVIBHMEFLREMLRNRLTIOEAEEBRSUIYEHREOTOLSEN";
 
    #[test]
    fn encrypt_test() {
        let cipher = RS44::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }
 
    #[test]
    fn decrypt_test() {
        let cipher = RS44::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}