use eframe::egui::{self, TextStyle};

use self::caesar_controls::CaesarControls;

pub mod app;
pub mod caesar_controls;
pub mod generic_components;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
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
    input: String,
    output: String,
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self { description: "", input: String::default(), output: String::default() }
    }
}

impl View for DisplayPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!{"Description:\n{}",self.description});

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui.label("INPUT TEXT");
        ui.add(egui::TextEdit::multiline(&mut self.input).text_style(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEST");
        ui.add(egui::TextEdit::multiline(&mut self.output).text_style(TextStyle::Monospace));
    }
}

pub struct ControlPanel {
    active_cipher: CipherID,
    caesar: CaesarControls,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self{ 
            active_cipher: CipherID::Caesar,
            caesar: CaesarControls::default(),
        }
    }
}

impl View for ControlPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_cipher, CipherID::Caesar, "Caesar");
            ui.selectable_value(&mut self.active_cipher, CipherID::Affine, "Affine");
            ui.selectable_value(&mut self.active_cipher, CipherID::Decoder, "Decoder");
            ui.selectable_value(&mut self.active_cipher, CipherID::Substitution, "Substitution");
            ui.selectable_value(&mut self.active_cipher, CipherID::M209, "M209");
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match self.active_cipher {
            CipherID::Caesar => self.caesar.ui(ui),
            CipherID::Affine => todo!("cipher controls not implemented"),
            CipherID::Decoder => todo!("cipher controls not implemented"),
            CipherID::Substitution => todo!("cipher controls not implemented"),
            CipherID::M209 => todo!("cipher controls not implemented"),
        }
    }
}
