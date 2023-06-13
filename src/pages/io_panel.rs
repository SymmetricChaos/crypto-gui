use ciphers::ids::CipherId;
use codes::ids::CodeId;
use egui::{Color32, RichText, TextEdit, TextStyle, Ui};

use crate::{
    // attack_panel::{AttackInterface, ViewableAttack},
    cipher_panel::{CipherFrame, CipherInterface},
    code_panel::{CodeFrame, CodeInterface},
    ui_elements::{error_text, text_manip_menu},
    // ids::AttackId,
};

use super::Page;

pub fn encrypt_decrypt(
    ui: &mut Ui,
    cipher: &dyn CipherFrame,
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
    code: &dyn CodeFrame,
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

// pub fn attack(
//     ui: &mut Ui,
//     attack: &dyn ViewableAttack,
//     input: &mut String,
//     output: &mut String,
//     errors: &mut String,
// ) {
//     ui.horizontal(|ui| {
//         if ui
//             .button(RichText::from("DECRYPT").color(Color32::GOLD))
//             .clicked()
//         {
//             errors.clear();
//             match attack.attack_cipher(input) {
//                 Ok(text) => *output = text,
//                 Err(e) => *errors = e.to_string(),
//             }
//         };
//     });
// }

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
        active_cipher: &mut Option<CipherId>,
        active_code: &mut Option<CodeId>,
        // active_attack: &mut AttackId,
        cipher_interface: &mut CipherInterface,
        code_interface: &mut CodeInterface,
        // attack_interface: &mut AttackInterface,
    ) {
        ui.add_space(32.0);
        ui.horizontal(|ui| {
            ui.label("INPUT TEXT");
            text_manip_menu(ui, input);
        });
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.label("OUTPUT TEXT");
            text_manip_menu(ui, output);
        });
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        if active_page == &mut Page::Cipher {
            if let Some(cipher) = active_cipher {
                encrypt_decrypt(
                    ui,
                    cipher_interface.get_active_cipher(cipher),
                    input,
                    output,
                    errors,
                );
            } else {
                ui.label("<<<CIPHER HOMEPAGE>>>");
            }
        }

        if active_page == &mut Page::Code {
            if let Some(code) = active_code {
                encode_decode(
                    ui,
                    code_interface.get_active_code(code),
                    input,
                    output,
                    errors,
                );
            } else {
                ui.label("<<<CODE HOMEPAGE>>>");
            }
        }

        // if active_page == &mut Page::Attack {
        //     attack(
        //         ui,
        //         attack_interface.get_active_attack(active_attack),
        //         input,
        //         output,
        //         errors,
        //     );
        // }

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

        // if active_page == &Page::Cipher {
        //     ui.add_space(16.0);
        //     global_rng_controls(ui);
        // }

        if !errors.is_empty() {
            ui.add_space(24.0);
            ui.label(error_text(errors));
        }
    }
}
