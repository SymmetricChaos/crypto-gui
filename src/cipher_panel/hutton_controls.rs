use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::polyalphabetic::{Hutton, HuttonVersion};
use egui::Ui;
use rand::thread_rng;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
    vecstring::VecString,
};

pub struct HuttonFrame {
    cipher: Hutton,
    alphabet_string: String,
    password_string: String,
    keyword_string: String,
    example: String,
    example_version: HuttonVersion,
    example_password_string: String,
    example_password: Vec<usize>,
    example_password_idx: usize,
    example_keyword: String,
    example_keyed_alphabet: VecString,
    example_output: String,
}

impl Default for HuttonFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            password_string: Default::default(),
            keyword_string: Default::default(),
            example: String::from("EXAMPLE"),
            example_version: HuttonVersion::V1,
            example_password_string: String::from("TEST"),
            example_password: vec![19, 4, 18, 19],
            example_password_idx: 0,
            example_keyword: String::from("KEYWORD"),
            example_keyed_alphabet: VecString::from("KEYWORDABCFGHIJLMNPQSTUVXZ"),
            example_output: String::new(),
        }
    }
}

impl HuttonFrame {
    pub fn step_example(&mut self) {
        if self.example.is_empty() {
            return ();
        }
        let c = self.example.remove(0);
        let p = self.example_password[self.example_password_idx];

        self.example_password_idx = (self.example_password_idx + 1) % self.example_password.len();

        // add the password number to the position of the character in the keyed alphabet
        let mut value = self
            .example_keyed_alphabet
            .chars()
            .position(|x| x == c)
            .unwrap()
            + p;
        // in Version 2 add the plain alphabet position of the first symbol in the keyed alphabet
        if self.cipher.version == HuttonVersion::V2 {
            value += Alphabet::BasicLatin
                .position(self.example_keyed_alphabet.chars().nth(0).unwrap())
                .unwrap();
            value += 1;
        }

        value %= 26;
        self.example_output
            .push(self.example_keyed_alphabet.chars().nth(value).unwrap());

        self.example_keyed_alphabet
            .swap_indicies(self.example_keyed_alphabet.get_pos(c).unwrap(), value);
        self.example_keyword = self.example_keyed_alphabet.to_string();
    }
}

impl CipherFrame for HuttonFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/hutton.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.keyword_string, &self.alphabet_string);
            self.cipher.assign_password(&self.password_string);
            filter_string(&mut self.password_string, &self.alphabet_string);
            filter_string(&mut self.keyword_string, &self.alphabet_string);
        }

        ui.add_space(16.0);
        ui.subheading("Version");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.example_version, HuttonVersion::V1, "V1");
            ui.selectable_value(&mut self.example_version, HuttonVersion::V2, "V2");
        });

        ui.add_space(8.0);
        ui.subheading("Password");
        if ui.control_string(&mut self.password_string).changed() {
            filter_string(&mut self.password_string, &self.alphabet_string);
            self.cipher.assign_password(&self.password_string)
        }

        ui.add_space(8.0);
        ui.subheading("Keyword");
        if ui.control_string(&mut self.keyword_string).changed() {
            filter_string(&mut self.keyword_string, &self.alphabet_string);
            self.cipher
                .assign_key(&self.keyword_string, &self.alphabet_string);
            self.cipher.assign_password(&self.password_string);
        }

        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Example");
            ui.label("Alphabet");
            ui.false_control_string(Alphabet::BasicLatin);

            ui.label("Version");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.cipher.version, HuttonVersion::V1, "V1");
                ui.selectable_value(&mut self.cipher.version, HuttonVersion::V2, "V2");
            });

            ui.label("Password");
            if ui
                .control_string(&mut self.example_password_string)
                .changed()
            {
                filter_string(&mut self.example_password_string, &Alphabet::BasicLatin);
                self.example_password_idx = 0;
                self.example_password = self
                    .example_password_string
                    .chars()
                    .map(|c| Alphabet::BasicLatin.position(c).unwrap() + 1)
                    .collect();
            }
            ui.add_space(8.0);
            ui.label("Keyword");
            if ui.control_string(&mut self.example_keyword).changed() {
                filter_string(&mut self.example_keyword, &Alphabet::BasicLatin);
                self.example_keyed_alphabet =
                    VecString::keyed_alphabet(&self.example_keyword, Alphabet::BasicLatin.into());
            }

            ui.label("Plaintext");
            if ui.control_string(&mut self.example).changed() {
                filter_string(&mut self.example, &Alphabet::BasicLatin)
            }
            if ui.button("Step").clicked() {
                self.step_example()
            }
            ui.label(self.example_keyed_alphabet.to_string());
            ui.add_space(4.0);
            ui.control_string(&mut self.example_output);
        });
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.password_string = shuffled_str(&self.alphabet_string, &mut rng);
        self.cipher.assign_password(&self.password_string);
        self.keyword_string = shuffled_str(&self.alphabet_string, &mut rng);
        self.cipher
            .assign_key(&self.keyword_string, &self.alphabet_string);
    }

    crate::simple_cipher! {}
}
