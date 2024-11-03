use super::HasherFrame;
use hashers::fletcher::{Fletcher, FletcherhWidth};

pub struct FletcherFrame {
    hasher: Fletcher,
}

impl Default for FletcherFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
        }
    }
}

impl FletcherFrame {}

impl HasherFrame for FletcherFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/fletcher.rs",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.hasher.width, FletcherhWidth::W16, "Fletcher-16");
        ui.selectable_value(&mut self.hasher.width, FletcherhWidth::W32, "Fletcher-32");
        ui.selectable_value(&mut self.hasher.width, FletcherhWidth::W64, "Fletcher-64");

        ui.add_space(16.0);

        ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
