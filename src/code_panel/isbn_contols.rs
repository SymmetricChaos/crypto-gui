use super::{View, ViewableCode};
use crate::{
    codes::{ecc::isbn::IsbnVariant, Isbn},
    egui_aux::subheading,
};

impl ViewableCode for Isbn {}

impl View for Isbn {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Variant"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.variant, IsbnVariant::Ten, "ISBN-10");
                ui.selectable_value(&mut self.variant, IsbnVariant::Thirteen, "ISBN-13");
            });
        });

        match self.variant {
            IsbnVariant::Ten => ui.label("ISBN-10 numbers consist of 9 digits and a final check digit which may be X, representing a check value of 10. They are usually formatted as N-NNN-NNNNN-N"),
            IsbnVariant::Thirteen => ui.label("ISBN-13 numbers consist of 12 digits and a final check digit. They are usually formatted as NNN-N-NNN-NNNNN-N. The prefix value 987 is reserved for ISBN-10 numbers being re-coded at ISBN-13 numbers, the final check digit is also recalculated when doing this."),
        };

        //ui.lable("Check Multiple ISBN codes at once.")
        //ui.text_edit_multiline(text)
    }
}
