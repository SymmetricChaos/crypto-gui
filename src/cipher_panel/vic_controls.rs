use crate::ui_elements::UiElements;
use ciphers::{polybius::StraddlingCheckerboard, vic::Vic, Cipher};
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};
use utils::{preset_alphabet::Alphabet, text_functions::random_string_sample_replace};

use super::CipherFrame;

pub struct VicFrame {
    cipher: Vic,
    checkerboard: StraddlingCheckerboard,
}

impl Default for VicFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            checkerboard: StraddlingCheckerboard::default(),
        }
    }
}

impl CipherFrame for VicFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);

        ui.subheading("Date");
        ui.label("The date that the message is sent. Leading zeroes should not be used.");
        ui.control_string(&mut self.cipher.date);
        if self.cipher.date.chars().count() < 6 {
            ui.error_text("date must have at least six digits");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);

        ui.subheading("Key Group");
        ui.label("A unique key group is chosen randomly for each message.");
        ui.control_string(&mut self.cipher.key_group);
        if self.cipher.key_group.chars().count() != 5 {
            ui.error_text("key group must have exactly five digits");
        } else {
            ui.error_text("");
        }

        ui.collapsing("Key Group Position", |ui| {
            ui.label("The unique key group needs to be transmitted to the reciever. The message is divided into groups of five digits and and key group inserted at the given position, the sixth digit of the date. This tool does not insert the keygroup on encryption or extract it during decryption.");
            match self
                .cipher
                .date
                .chars()
                .filter(|c| c.is_ascii_digit())
                .nth(5)
            {
                Some(c) => ui.mono(c),
                None => ui.error_text("date does not have a sixth digit"),
            };
        });

        ui.add_space(8.0);

        ui.subheading("Phrase");
        ui.label("Each spy is given their own phrase to memorize.");
        if ui.control_string(&mut self.cipher.phrase).changed() {
            self.cipher.phrase = self
                .cipher
                .phrase
                .chars()
                .filter(|c| Alphabet::BasicLatin.contains(c))
                .collect();
        }
        if self.cipher.phrase.chars().count() < 20 {
            ui.error_text("phrase must have at least twenty letters");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);

        ui.subheading("Personal Number");
        ui.label("A number is assigned to each spy.");
        ui.add(DragValue::new(&mut self.cipher.pin).clamp_range(1..=20));

        ui.add_space(16.0);

        ui.subheading("Key Derivation");
        ui.collapsing("Sequencing", |ui| ui.label("Ten symbols are replaced with digits. First the symbol with the lowest value is assigned '1', then the next lowest values is assigned '2', and so on ending with '0'. Ties are resolved left to right."));
        ui.collapsing("Addition/Subtraction", |ui| {ui.label("Addition and subtraction are performed modulo 10 so that zero follows nine when adding and nine follows zero when subtracting.")});
        ui.collapsing("Chain Addition", |ui| {
            ui.label("Sequentially add pairs of digits together. So when extending 934 by three digits the result is 276 because: 9 + 3 = 2, 3 + 4 = 7, 4 + 2 = 6.")
        });
        match self.cipher.key_derivation_string() {
            Ok(text) => {
                self.checkerboard
                    .assign_top_row(&self.cipher.key_derivation().unwrap().2);
                ui.mono(text)
            }
            Err(e) => ui.error_text(e),
        };
        ui.add_space(16.0);
        ui.label("First the S key is used to set the top row of a Straddling Checkerboard, shown below, and the text encrypted that way. Afterward the Q key is used for Columnar Transposition then the R key for Diagonal Columnar Transposition.");
        ui.add_space(16.0);

        ui.subheading("Straddling Checkerboard");
        ui.mono(self.checkerboard.cipher_page());

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key_group = random_string_sample_replace("0123456789", 5, &mut rng);
        self.cipher.date = {
            let day = rng.gen_range(1..=31);
            let month = rng.gen_range(1..=12);
            let year = rng.gen_range(1922..=1991);
            format!("{day}/{month}/{year}")
        };
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
