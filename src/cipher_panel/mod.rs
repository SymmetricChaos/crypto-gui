use crate::cipher_id::CipherID;
use eframe::egui::{self, TextEdit, TextStyle, RichText, Color32};

use crate::ciphers::*;

pub mod caesar_controls;
pub mod generic_components;
pub mod affine_controls;
pub mod decoder_ring_controls;
pub mod m209_controls;
pub mod general_sub_controls;
pub mod playfair_controls;
pub mod cyclic_key_controls;
pub mod autokey_controls;
pub mod progressive_key_controls;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String, errors: &mut String);
}

fn combox_box(ciphers: &[CipherID], identifier: &'static str, active_cipher: &mut CipherID, ui: &mut egui::Ui) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in ciphers {
                ui.selectable_value(active_cipher, *id, format!("{}",id));
            }
        });
    ui.add_space(10.0);
}

pub struct ControlPanel {
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    m209: M209,
    cyclic_key: CyclicKey,
    autokey: Autokey,
    progressive_key: ProgressiveKey,
    playfair: Playfair,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self{ 
            caesar: Caesar::default(),
            affine: Affine::default(),
            decoder_ring: DecoderRing::default(),
            gen_sub: GeneralSubstitution::default(),
            m209: M209::default(),
            cyclic_key: CyclicKey::default(),
            autokey: Autokey::default(),
            playfair: Playfair::default(),
            progressive_key: ProgressiveKey::default(),
        }
    }
}

impl ControlPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String, errors: &mut String, active_cipher: &mut CipherID) {
        
        ui.horizontal(|ui| {
            combox_box(
                &[CipherID::Caesar, CipherID::Decoder, CipherID::Affine, CipherID::Substitution],
                "Simple Substitution",
                active_cipher, ui
            );
    
            combox_box(
                &[CipherID::CyclicKey, CipherID::Autokey, CipherID::ProgressiveKey],
                "Polyalphabetic",
                active_cipher, ui
            );
    
            combox_box(
                &[CipherID::M209],
                "Rotor Machine",
                active_cipher, ui
            );
    
            combox_box(
                &[CipherID::Playfair],
                "Other",
                active_cipher, ui
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui.label(format!{"Description:\n{}",active_cipher.description()});

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match active_cipher {
            CipherID::Caesar => self.caesar.ui(ui, input, output, errors),
            CipherID::Affine => self.affine.ui(ui, input, output, errors),
            CipherID::Decoder => self.decoder_ring.ui(ui, input, output, errors),
            CipherID::Substitution => self.gen_sub.ui(ui, input, output, errors),
            CipherID::M209 => self.m209.ui(ui, input, output, errors),
            CipherID::CyclicKey => self.cyclic_key.ui(ui, input, output, errors),
            CipherID::Autokey => self.autokey.ui(ui, input, output, errors),
            CipherID::ProgressiveKey => self.progressive_key.ui(ui, input, output, errors),
            CipherID::Playfair => self.playfair.ui(ui, input, output, errors),
        }
    }
}



pub struct DisplayPanel {
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self{ }
    }
}

impl DisplayPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
       
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).text_style(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).text_style(TextStyle::Monospace));
       
        // ui.horizontal(|ui| {
        //     if ui.button("UPPERCASE").clicked() {
        //         input = &mut input.to_uppercase();
        //         output = &mut output.to_uppercase();
        //     }
        //     if ui.button("lowercase").clicked() {
        //         input = &mut input.to_lowercase();
        //         output = &mut output.to_lowercase();
        //     }
        // });
       
        // if ui.button("strip whitespace").clicked() {
        //     input = &mut input.split_whitespace().collect();
        //     output = &mut output.split_whitespace().collect();
        // }

        if ui.button("clear").clicked() {
            input.clear();
            output.clear();
            errors.clear();
        }

        if ui.button("swap input/output").clicked() {
            std::mem::swap(input, output)
        }
       
        if !errors.is_empty() {
            ui.add_space(24.0);
            ui.label(RichText::new(errors.clone())
                .color(Color32::RED)
                .background_color(Color32::BLACK)
                .monospace()
            );
        }

    }
}