use crate::cipher_id::CipherID;
use eframe::egui::{self, TextEdit, TextStyle, RichText, Color32};

use crate::ciphers::*;

use self::generic_components::encrypt_decrypt;

pub mod caesar_controls;
pub mod generic_components;
pub mod affine_controls;
pub mod decoder_ring_controls;
pub mod m209_controls;
pub mod general_sub_controls;
pub mod playfair_controls;
pub mod vigenere_controls;
pub mod beaufort_controls;
pub mod alberti_controls;
pub mod m94_controls;
pub mod polybius_controls;
pub mod columnar_controls;
pub mod adfgvx_controls;
pub mod b64_controls;
pub mod slidefair_controls;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
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

#[derive(Default)]
pub struct ControlPanel {
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    polybius: Polybius,

    m209: M209,

    vigenere: Vigenere,
    beaufort: Beaufort,
    alberti: Alberti,
    m94: M94,

    playfair: Playfair,
    slidefair: Slidefair,

    columnar: Columnar,

    adfgvx: ADFGVX,
    b64: B64,
}


impl ControlPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, active_cipher: &mut CipherID) {
        
        egui::Grid::new("comboboxes").show(ui, |ui| {
            combox_box(
                &[CipherID::Caesar, CipherID::Decoder, CipherID::Affine, CipherID::Substitution, CipherID::Polybius],
                "Simple Substitution",
                active_cipher, ui
            );
    
            combox_box(
                &[CipherID::Vigenere, CipherID::Beaufort, CipherID::M94, CipherID::Alberti],
                "Polyalphabetic",
                active_cipher, ui
            );
    
            combox_box(
                &[CipherID::M209], //CipherID::Enigma, CipherID::SIGABA],
                "Rotor Machine",
                active_cipher, ui
            );

            combox_box(
                &[CipherID::Columnar],
                "Transposition",
                active_cipher, ui
            );

            ui.end_row();

            combox_box(
                &[CipherID::Playfair, CipherID::Slidefair],
                "Playfair",
                active_cipher, ui
            );

            combox_box(
                &[CipherID::ADFGVX, CipherID::B64],
                "Composite",
                active_cipher, ui
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        let name = RichText::new(String::from(*active_cipher))
            .strong()
            .heading();
        ui.add(egui::Label::new(name));
        ui.label(active_cipher.description());

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match active_cipher {
            CipherID::Caesar => self.caesar.ui(ui),
            CipherID::Affine => self.affine.ui(ui),
            CipherID::Decoder => self.decoder_ring.ui(ui),
            CipherID::Substitution => self.gen_sub.ui(ui),
            CipherID::Polybius => self.polybius.ui(ui),
            CipherID::Vigenere => self.vigenere.ui(ui),
            CipherID::Beaufort => self.beaufort.ui(ui),
            CipherID::M209 => self.m209.ui(ui),
            CipherID::M94 => self.m94.ui(ui),
            CipherID::Alberti => self.alberti.ui(ui),
            CipherID::Playfair => self.playfair.ui(ui),
            CipherID::Columnar => self.columnar.ui(ui),
            CipherID::ADFGVX => self.adfgvx.ui(ui),
            CipherID::B64 => self.b64.ui(ui),
            CipherID::Slidefair => self.slidefair.ui(ui),
            _ => { ui.label("IN PROGRESS"); },
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
    pub fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String, errors: &mut String, active_cipher: &mut CipherID, control_panel: &ControlPanel,) {
       
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        match active_cipher {
            CipherID::Caesar => encrypt_decrypt(ui, &control_panel.caesar, input, output, errors),
            CipherID::Affine => encrypt_decrypt(ui, &control_panel.affine, input, output, errors),
            CipherID::Decoder => encrypt_decrypt(ui, &control_panel.decoder_ring, input, output, errors),
            CipherID::Substitution => encrypt_decrypt(ui, &control_panel.gen_sub, input, output, errors),
            CipherID::Polybius => encrypt_decrypt(ui, &control_panel.polybius, input, output, errors),
            CipherID::Vigenere => encrypt_decrypt(ui, &control_panel.vigenere, input, output, errors),
            CipherID::Beaufort => encrypt_decrypt(ui, &control_panel.beaufort, input, output, errors),
            CipherID::M209 => encrypt_decrypt(ui, &control_panel.m209, input, output, errors),
            CipherID::M94 => encrypt_decrypt(ui, &control_panel.m94, input, output, errors),
            CipherID::Alberti => encrypt_decrypt(ui, &control_panel.alberti, input, output, errors),
            CipherID::Playfair => encrypt_decrypt(ui, &control_panel.playfair, input, output, errors),
            CipherID::Columnar => encrypt_decrypt(ui, &control_panel.columnar, input, output, errors),
            CipherID::ADFGVX => encrypt_decrypt(ui, &control_panel.adfgvx, input, output, errors),
            CipherID::B64 => encrypt_decrypt(ui, &control_panel.b64, input, output, errors),
            CipherID::Slidefair => encrypt_decrypt(ui, &control_panel.slidefair, input, output, errors),
            _ => { *errors = String::from("button not linked properly") }
        }

        ui.add_space(10.0);
        if ui.button("clear").clicked() {
            input.clear();
            output.clear();
            errors.clear();
        }

        ui.add_space(10.0);
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