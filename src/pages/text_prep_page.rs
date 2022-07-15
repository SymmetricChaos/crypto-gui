use eframe::egui::{CentralPanel, Context, SidePanel, TextEdit};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub struct TextPrepPage {
    pub text: String,
    pub num_bytes: usize,
    pub num_chars: usize,
    pub num_graphemes: usize,
    pub replace_from: String,
    pub replace_to: String,
    pub remove: String,
    pub keep: String,
}

impl Default for TextPrepPage {
    fn default() -> Self {
        Self {
            text: String::from("ô"),
            num_bytes: 3,
            num_chars: 2,
            num_graphemes: 1,
            replace_from: Default::default(),
            replace_to: Default::default(),
            remove: Default::default(),
            keep: Default::default(),
        }
    }
}

impl TextPrepPage {
    // Information counters
    fn counts(&mut self) {
        self.num_bytes = self.text.len();
        self.num_chars = self.text.chars().count();
        self.num_graphemes = self.text.graphemes(true).count();
    }

    // Buttons
    fn clear(&mut self) {
        self.text.clear();
        self.counts();
    }

    fn remove_whitespace(&mut self) {
        self.text = self.text.split_whitespace().collect();
        self.counts();
    }

    fn uppercase(&mut self) {
        self.text = self.text.to_uppercase();
        self.counts();
    }

    fn lowercase(&mut self) {
        self.text = self.text.to_lowercase();
        self.counts();
    }

    fn replace(&mut self) {
        self.text = self.text.replace(&self.replace_from, &self.replace_to);
        self.counts();
    }

    fn normalize(&mut self) {
        self.text = self.text.nfc().collect();
        self.counts();
    }

    fn remove_characters(&mut self) {
        self.text = self
            .text
            .chars()
            .filter(|c| !self.remove.contains(*c))
            .collect();
        self.counts();
    }

    fn keep_characters(&mut self) {
        self.text = self
            .text
            .chars()
            .filter(|c| self.remove.contains(*c))
            .collect();
        self.counts();
    }

    pub fn view(&mut self, ctx: &Context) {
        SidePanel::left("text_prep_page_buttons")
            .max_width(300.0)
            .show(ctx, |ui| {
                if ui.button("Clear").clicked() {
                    self.clear()
                }

                ui.add_space(10.0);

                if ui.button("Remove Whitespace").clicked() {
                    self.remove_whitespace();
                }

                ui.add_space(10.0);

                if ui.button("UPPERCASE").clicked() {
                    self.uppercase();
                }

                ui.add_space(10.0);

                if ui.button("lowercase").clicked() {
                    self.lowercase();
                }

                ui.add_space(10.0);

                ui.collapsing("Advanced", |ui| {
                    let normalize = ui
                        .button("Normalize")
                        .on_hover_text_at_pointer("Normalize Unicode Representation");
                    if normalize.clicked() {
                        self.normalize();
                    }

                    ui.add_space(10.0);

                    if ui.button("Replace").clicked() {
                        self.replace();
                    }
                    ui.label("from:");
                    ui.text_edit_singleline(&mut self.replace_from);
                    ui.label("  to:");
                    ui.text_edit_singleline(&mut self.replace_to);

                    ui.add_space(10.0);

                    if ui.button("Remove All").clicked() {
                        self.remove_characters();
                    }
                    ui.label("characters:");
                    ui.text_edit_singleline(&mut self.remove);

                    ui.add_space(10.0);

                    if ui.button("Retain Only").clicked() {
                        self.keep_characters();
                    }
                    ui.label("characters:");
                    ui.text_edit_singleline(&mut self.keep);
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            let main_text = TextEdit::multiline(&mut self.text);
            if ui.add(main_text).changed() {
                self.counts();
            };
            ui.label(format!("Bytes:      {}", self.num_bytes))
                .on_hover_text_at_pointer(
                    "Number of bytes used to represent the text as a UTF-8 string",
                );
            ui.label(format!("Characters: {}", self.num_chars))
                .on_hover_text_at_pointer("Number of Unicode codepoints in the string");
            ui.label(format!("Graphemes:  {}", self.num_graphemes))
                .on_hover_text_at_pointer("Number of Unicode graphemes in the string");
        });
    }
}
