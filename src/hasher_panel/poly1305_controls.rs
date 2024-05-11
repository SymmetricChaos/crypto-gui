use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{errors::HasherError, poly1305::Poly1305, traits::ClassicHasher};
use rand::{thread_rng, RngCore};

pub struct Poly1305Frame {
    hasher: Poly1305,
    key_r_string: String,
    key_s_string: String,
}

impl Default for Poly1305Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            key_r_string: String::new(),
            key_s_string: String::new(),
        }
    }
}

impl Poly1305Frame {
    fn key_r_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_r_string).changed() {
                self.key_r_string = self
                    .key_r_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(32)
                    .collect();

                match self.hasher.key_r_from_string(&self.key_r_string) {
                    Ok(_) => self.key_r_string = format!("{:032x?}", self.hasher.key_r),
                    Err(e) => {
                        ui.error_text(e.to_string());
                    }
                };
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill_bytes(&mut self.hasher.key_r);
                    self.key_r_string = format!("{:032x?}", self.hasher.key_r);
                };
            }
        });
    }

    fn key_s_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_s_string).changed() {
                self.key_s_string = self
                    .key_s_string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(32)
                    .collect();

                match self.hasher.key_s_from_string(&self.key_s_string) {
                    Ok(_) => self.key_s_string = format!("{:032x?}", self.hasher.key_s),
                    Err(e) => {
                        ui.error_text(e.to_string());
                    }
                };
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill_bytes(&mut self.hasher.key_s);
                    self.key_s_string = format!("{:032x?}", self.hasher.key_s);
                };
            }
        });
    }
}

impl HasherFrame for Poly1305Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Key (r)");
        ui.label("The point at which the polynomial is evaluated.");
        self.key_r_control(ui);
        
        ui.add_space(8.0);
        ui.subheading("Key (s)");
        ui.label("A constant that is added after the polynomial is evaluated.");
        self.key_s_control(ui);

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
