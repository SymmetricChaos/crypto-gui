use eframe::egui::{self, TextStyle};

pub mod app;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct ControlPanel;

impl Default for ControlPanel {
    fn default() -> Self {
        Self {  }
    }
}

impl View for ControlPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("controls vary with cipher");
    }
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

pub struct SelectorPanel {
    active_panel: CipherID
}

impl Default for SelectorPanel {
    fn default() -> Self {
        Self { active_panel: CipherID::default() }
    }
}

impl View for SelectorPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_panel, CipherID::Caesar, "Caesar");
            ui.selectable_value(&mut self.active_panel, CipherID::Affine, "Affine");
            ui.selectable_value(&mut self.active_panel, CipherID::Decoder, "Decoder");
            ui.selectable_value(&mut self.active_panel, CipherID::Substitution, "Substitution");
            ui.selectable_value(&mut self.active_panel, CipherID::M209, "M209");
        });
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

pub struct CipherPanel {
    controls: ControlPanel,
    display: DisplayPanel
}