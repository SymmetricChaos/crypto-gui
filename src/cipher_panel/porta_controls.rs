use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polyalphabetic::{porta::PORTA_TABLEAUX, Porta},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_string_sample_replace},
};

#[derive(Default)]
pub struct PortaFrame {
    cipher: Porta,
    key_string: String,
}

impl CipherFrame for PortaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/porta.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            filter_string(&mut self.key_string, &Alphabet::BasicLatin);
            match self.cipher.assign_key(&self.key_string) {
                Ok(_) => (),
                Err(e) => {
                    ui.error_text(e.inner());
                }
            }
        }
        ui.add_space(16.0);

        ui.subheading("Tableaux");
        egui::Grid::new("porta_tableaux")
            .num_columns(26)
            .min_col_width(1.0)
            .max_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for row in PORTA_TABLEAUX.iter() {
                    for c in row.chars() {
                        ui.mono(c);
                    }
                    ui.end_row();
                }
            });
        ui.add_space(16.0);

        // This is possible but not yet implemented
        // ui.label("Mode");
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut self.mode, CylicKey, "Cyclic");
        //     ui.selectable_value(&mut self.mode, Autokey, "Autokey");
        //     ui.selectable_value(&mut self.mode, ProgKey, "Progressive");
        // });

        // if self.mode == ProgKey {
        //     ui.add_space(16.0);
        //     ui.label("Step size");
        //     let alpha_range = 0..=(self.alphabet_len() - 1);
        //     ui.add(Slider::new(&mut self.prog_shift, alpha_range));
        //     ui.add_space(16.0);
        // }

        // match self.multikey {
        //     true => {
        //         ui.horizontal(|ui| {
        //             ui.label("Keywords");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.keywords[0]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.keywords[1]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.keywords[2]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.keywords[3]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.keywords[4]).font(TextStyle::Monospace));
        //     }
        //     false => {
        //         ui.horizontal(|ui| {
        //             ui.label("Keyword ");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.keywords[0]).font(TextStyle::Monospace));
        //     }
        // }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);
        self.key_string =
            random_string_sample_replace(Alphabet::BasicLatin.into(), n_chars, &mut rng);
        self.cipher
            .assign_key(&self.key_string)
            .expect("randomizer picked invalid characters")
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
