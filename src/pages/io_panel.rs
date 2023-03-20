use egui::{Color32, RichText, TextEdit, TextStyle, Ui};

use crate::{
    cipher_panel::{CipherInterface, ViewableCipher},
    code_panel::{CodeInterface, ViewableCode},
    global_rng::global_rng_controls,
    ids::{CipherID, CodeID},
};

use super::Page;

pub fn encrypt_decrypt(
    ui: &mut Ui,
    cipher: &dyn ViewableCipher,
    input: &mut String,
    output: &mut String,
    errors: &mut String,
) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::from("ENCRYPT").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match cipher.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui
            .button(RichText::from("DECRYPT").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match cipher.decrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}

pub fn encode_decode(
    ui: &mut Ui,
    code: &dyn ViewableCode,
    input: &mut String,
    output: &mut String,
    errors: &mut String,
) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::from("ENCODE").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match code.encode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui
            .button(RichText::from("DECODE").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match code.decode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}

#[derive(Default)]
pub struct IOPanel {}

impl IOPanel {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        input: &mut String,
        output: &mut String,
        errors: &mut String,
        active_page: &mut Page,
        active_cipher: &mut CipherID,
        active_code: &mut CodeID,
        cipher_interface: &mut CipherInterface,
        code_interface: &mut CodeInterface,
    ) {
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        if active_page == &mut Page::Cipher {
            encrypt_decrypt(
                ui,
                cipher_interface.get_active_cipher(active_cipher),
                input,
                output,
                errors,
            );
        }

        if active_page == &mut Page::Code {
            encode_decode(
                ui,
                code_interface.get_active_code(active_code),
                input,
                output,
                errors,
            );
        }

        ui.add_space(10.0);
        if ui.button("clear").clicked() {
            input.clear();
            output.clear();
            errors.clear();
        }

        ui.add_space(10.0);
        if ui.button("swap input/output").clicked() {
            std::mem::swap(input, output)
        }

        ui.add_space(16.0);
        global_rng_controls(ui);

        if !errors.is_empty() {
            ui.add_space(24.0);
            ui.label(
                RichText::new(errors.clone())
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        }
    }
}
