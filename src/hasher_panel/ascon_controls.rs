use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::ascon::AsconHash;

#[derive(Default)]
pub struct AsconFrame {
    hasher: AsconHash,
}

impl HasherFrame for AsconFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(4.0);

        if ui.checkbox(&mut self.hasher.xof, "XOF Mode").changed() {
            if !self.hasher.xof {
                self.hasher.hash_len = self.hasher.hash_len.clamp(16, 32)
            }
        }

        ui.add_space(4.0);

        ui.subheading("Hash Length");
        match self.hasher.xof {
            true => {
                ui.label("Ascon-XOF can return an output of any length but here is limited to 256 bytes (2048 bits).\nIt is domain separated from Ascon-Hash but otherwise works identically.");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=256));
            }
            false => {
                ui.label("Ascon-Hash can return a hash of any length from 16 bytes to 32 bytes (128 bits to 256 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(16..=32));
            }
        }
        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
