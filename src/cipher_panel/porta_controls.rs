use super::CipherFrame;
use crate::ui_elements::{control_string, error_text, mono, randomize_reset};
use ciphers::{
    polyalphabetic::{porta::PORTA_TABLEAUX, Porta},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{functions::random_sample_replace, preset_alphabet::Alphabet};

#[derive(Default)]
pub struct PortaFrame {
    cipher: Porta,
    key_string: String,
}

impl CipherFrame for PortaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.key_string).changed() {
            match self.cipher.assign_key(&self.key_string) {
                Ok(_) => (),
                Err(e) => {
                    ui.label(error_text(e.inner()));
                }
            }
        }
        ui.add_space(16.0);

        ui.label("Tableaux");
        egui::Grid::new("porta_tableaux")
            .num_columns(26)
            .min_col_width(1.0)
            .max_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for row in PORTA_TABLEAUX.iter() {
                    for c in row.chars() {
                        ui.label(mono(c));
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
        //             ui.label("Key Words");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[1]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[2]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[3]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[4]).font(TextStyle::Monospace));
        //     }
        //     false => {
        //         ui.horizontal(|ui| {
        //             ui.label("Key Word ");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
        //     }
        // }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);
        self.key_string = random_sample_replace(Alphabet::BasicLatin.into(), n_chars, &mut rng);
        self.cipher
            .assign_key(&self.key_string)
            .expect("randomizer picked invalid characters")
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
