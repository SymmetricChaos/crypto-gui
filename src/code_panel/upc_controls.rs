use codes::upc::Upc;

pub struct UpcFrame {
    pub code: Upc,
    pub example: String,
}

impl Default for UpcFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            example: String::from("03600029145"),
        }
    }
}

impl CodeFrame for UpcFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // ui.group(|ui| {
        //     ui.label(subheading("Variant"));
        //     ui.horizontal(|ui| {
        //         ui.selectable_value(&mut self.code.variant, , );

        //     });
        // });

        // match self.code.variant {

        // };

        ui.text_edit_singleline(&mut self.example);
        match is_valid_upc_a(&self.example) {
            Ok(_) => {
                ui.horizontal(|ui| {
                    ui.label("Digits:  ");
                    ui.label(self.example.chars().join(" "));
                });
                ui.label("Weights:  1 3 1 3 1 3 1 3 1 3 1");
                ui.horizontal(|ui| {
                    ui.label("Products: ");
                    ui.label(
                        self.example
                            .chars()
                            .filter(|c| *c != '-')
                            .map(|c| c.to_digit(10).unwrap())
                            .zip([1, 3].into_iter().cycle())
                            .map(|(a, b)| (a * b) % 10)
                            .join(" "),
                    );
                });
            }
            Err(e) => {
                ui.label(error_text(&e.inner()));
            }
        }
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
