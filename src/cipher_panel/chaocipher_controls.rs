use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset};
use ciphers::{
    polyalphabetic::{
        chaocipher::{left_permute, right_permute},
        Chaocipher,
    },
    Cipher,
};
use egui::Ui;
use rand::thread_rng;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
    vecstring::VecString,
};

pub struct ChaocipherFrame {
    cipher: Chaocipher,
    left_string: String,
    right_string: String,
    example: String,
    example_left: VecString,
    example_right: VecString,
    // example_left_string: String,
    // example_right_string: String,
    example_outout: String,
}

impl Default for ChaocipherFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            left_string: String::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right_string: String::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
            example: String::from("EXAMPLE"),
            example_left: VecString::from(Alphabet::BasicLatin),
            example_right: VecString::from(Alphabet::BasicLatin),
            // example_left_string: String::from(Alphabet::BasicLatin),
            // example_right_string: String::from(Alphabet::BasicLatin),
            example_outout: String::new(),
        }
    }
}

impl CipherFrame for ChaocipherFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        if control_string(ui, &mut self.left_string).changed() {
            self.cipher.assign_left(&self.left_string)
        }

        if control_string(ui, &mut self.right_string).changed() {
            self.cipher.assign_right(&self.right_string)
        }
        ui.add_space(16.0);

        ui.collapsing("Step-by-Step Example", |ui| {
            ui.label("Plaintext");
            if control_string(ui, &mut self.example).changed() {
                filter_string(&mut self.example, Alphabet::BasicLatin.into())
            }

            if ui.button("Step").clicked() {
                if !self.example.is_empty() {
                    let c = self.example.remove(0);
                    let n = self.example_right.get_pos(c).unwrap();
                    self.example_outout
                        .push(*self.example_left.get_char(n).unwrap());
                    left_permute(&mut self.example_left, n);
                    right_permute(&mut self.example_right, n);
                }
            }

            ui.label(self.example_left.to_string());
            ui.label(self.example_right.to_string());
            ui.add_space(4.0);
            ui.label(&self.example_outout);
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.left_string = shuffled_str(Alphabet::BasicLatin.into(), &mut rng);
        self.cipher.assign_left(&self.left_string);

        self.right_string = shuffled_str(Alphabet::BasicLatin.into(), &mut rng);
        self.cipher.assign_right(&self.right_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
