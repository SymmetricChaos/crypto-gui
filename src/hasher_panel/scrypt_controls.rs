use super::HasherFrame;
use hashers::scrypt::Scrypt;

pub struct ScryptFrame {
    hasher: Scrypt,
}

impl ScryptFrame {}

impl HasherFrame for ScryptFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        todo!()
    }

    crate::hash_string! {}
}
