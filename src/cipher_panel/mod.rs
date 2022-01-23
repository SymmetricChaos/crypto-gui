use eframe::egui::{self, TextStyle};


use crate::ciphers::*;

pub mod caesar_controls;
pub mod generic_components;
pub mod affine_controls;
pub mod decoder_ring_controls;
pub mod m209_controls;
pub mod general_sub_controls;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String);
}

#[derive(PartialEq)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    M209,
}

impl Default for CipherID {
    fn default() -> Self {
        Self::Caesar
    }
}

pub struct DisplayPanel {
    description: &'static str,
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self { description: "" }
    }
}

impl View for DisplayPanel {
    fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String) {
        ui.label(format!{"Description:\n{}",self.description});

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui.label("INPUT TEXT");
        ui.add(egui::TextEdit::multiline(input).text_style(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(egui::TextEdit::multiline(output).text_style(TextStyle::Monospace));
    }
}

pub struct ControlPanel {
    active_cipher: CipherID,
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    m209: M209,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self{ 
            active_cipher: CipherID::Caesar,
            caesar: Caesar::default(),
            affine: Affine::default(),
            decoder_ring: DecoderRing::default(),
            gen_sub: GeneralSubstitution::default(),
            m209: M209::default(),
        }
    }
}

impl View for ControlPanel {
    fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_cipher, CipherID::Caesar, "Caesar");
            ui.selectable_value(&mut self.active_cipher, CipherID::Affine, "Affine");
            ui.selectable_value(&mut self.active_cipher, CipherID::Decoder, "Decoder Ring");
            ui.selectable_value(&mut self.active_cipher, CipherID::Substitution, "General Substitution");
            ui.selectable_value(&mut self.active_cipher, CipherID::M209, "M209");
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match self.active_cipher {
            CipherID::Caesar => self.caesar.ui(ui, input, output),
            CipherID::Affine => self.affine.ui(ui, input, output),
            CipherID::Decoder => self.decoder_ring.ui(ui, input, output),
            CipherID::Substitution => self.gen_sub.ui(ui, input, output),
            CipherID::M209 => self.m209.ui(ui, input, output),
        }
    }
}
