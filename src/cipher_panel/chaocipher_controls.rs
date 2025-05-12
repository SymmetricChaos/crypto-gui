use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::polyalphabetic::{
    chaocipher::{left_permute, right_permute},
    Chaocipher,
};
use egui::Ui;
use rand::thread_rng;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, filter_unique_string, shuffled_str},
    vecstring::VecString,
};

pub struct ChaocipherFrame {
    cipher: Chaocipher,
    left_string: String,
    right_string: String,
    example: String,
    example_left: VecString,
    example_right: VecString,
    example_output: String,
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
            example_output: String::new(),
        }
    }
}

impl CipherFrame for ChaocipherFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/chaocipher.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Left Alphabet");
        if ui.control_string(&mut self.left_string).changed() {
            filter_unique_string(&mut self.left_string, &Alphabet::BasicLatin);
            self.cipher.assign_left(&self.left_string)
        }

        ui.subheading("Right Alphabet");
        if ui.control_string(&mut self.right_string).changed() {
            filter_unique_string(&mut self.right_string, &Alphabet::BasicLatin);
            self.cipher.assign_right(&self.right_string)
        }
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Step-by-Step Example");
            ui.add_space(4.0);
            ui.label("Plaintext");
            if ui.control_string(&mut self.example).changed() {
                filter_string(&mut self.example, &Alphabet::BasicLatin)
            }

            ui.label("Encrypt one letter from the plaintext and see how the alphabets change.");
            if ui.button("Step").clicked() {
                if !self.example.is_empty() {
                    let c = self.example.remove(0);
                    let n = self.example_right.get_pos(c).unwrap();
                    self.example_output
                        .push(*self.example_left.get_char(n).unwrap());
                    left_permute(&mut self.example_left, n);
                    right_permute(&mut self.example_right, n);
                }
            }
            ui.add_space(4.0);
            ui.mono(format!("Left:  {}", self.example_left.to_string()));
            ui.mono(format!("Right: {}", self.example_left.to_string()));
            ui.add_space(4.0);
            ui.mono(&self.example_output);
        });
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.left_string = shuffled_str(Alphabet::BasicLatin.into(), &mut rng);
        self.cipher.assign_left(&self.left_string);

        self.right_string = shuffled_str(Alphabet::BasicLatin.into(), &mut rng);
        self.cipher.assign_right(&self.right_string);
    }

    crate::simple_cipher! {}
}
