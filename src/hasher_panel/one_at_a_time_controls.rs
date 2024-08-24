use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::one_at_a_time::OneAtATime;

pub struct OaatFrame {
    hasher: OneAtATime,
}

impl Default for OaatFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl HasherFrame for OaatFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
    }
    crate::hash_string! {}
}
