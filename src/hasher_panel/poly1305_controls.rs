use super::{byte_formatting_io, HasherFrame};
use crate::ui_elements::UiElements;
use hashers::{errors::HasherError, poly1305::Poly1305, traits::ClassicHasher};
use rand::{thread_rng, RngCore};

pub struct Poly1305Frame {
    hasher: Poly1305,
    key_string: String,
}

impl Default for Poly1305Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            key_string: String::new(),
        }
    }
}

impl Poly1305Frame {
    fn key_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).changed() {
                self.key_string = self
                    .key_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(32)
                    .collect();

                match self.hasher.key_r_from_string_lossy(&self.key_string) {
                    Ok(_) => self.key_string = format!("{:032x?}", self.hasher.key_r),
                    Err(e) => {
                        ui.error_text(e.to_string());
                    }
                };
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill_bytes(&mut self.hasher.key_r);
                    self.key_string = format!("{:032x?}", self.hasher.key_r);
                };
            }
        });
    }
}

impl HasherFrame for Poly1305Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Key (r)");
        ui.label("The point at which the polynomial is evaluated.");
        self.key_control(ui);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
