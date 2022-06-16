use self::generic_components::encrypt_decrypt;
use crate::{
    cipher_id::CipherID,
    ciphers::{*, polybius::*, tactical::*, polyalphabetic::*, substitution::*, playfair::*, transposition::*}, global_rng::global_rng_controls,
};
use eframe::egui::{self, Color32, RichText, TextEdit, TextStyle};
use rand::prelude::StdRng;

pub mod adfgvx_controls;
pub mod affine_controls;
pub mod alberti_controls;
pub mod b64_controls;
pub mod batco_controls;
pub mod bazeries_controls;
pub mod beaufort_controls;
pub mod bifid_controls;
pub mod caesar_controls;
pub mod chaocipher_controls;
pub mod checkerboard_controls;
pub mod columnar_controls;
pub mod decoder_ring_controls;
pub mod dryad_controls;
pub mod enigma_controls;
pub mod four_square_controls;
pub mod general_sub_controls;
pub mod generic_components;
pub mod grille_controls;
pub mod hutton_controls;
pub mod m209_controls;
pub mod m94_controls;
pub mod playfair_controls;
pub mod polybius_cube_controls;
pub mod polybius_square_controls;
pub mod porta_controls;
pub mod rail_fence_controls;
pub mod scytale_controls;
pub mod sigaba_controls;
pub mod slidefair_controls;
pub mod trifid_controls;
pub mod two_square_controls;
pub mod vigenere_controls;
pub mod quagmire_controls;
pub mod turning_grille_controls;
pub mod plugboard_controls;

pub trait View: Cipher {
    fn ui(&mut self, ui: &mut egui::Ui, rng: &mut StdRng, errors: &mut String);

    // Provides the interface to encrypt and decrypt, should be the same for all ciphers
    // Needs to access the UI, accepts inputs, provide outputs, and report errors
    fn encrypt_decrypt(
        &self,
        ui: &mut egui::Ui,
        input: &mut String,
        output: &mut String,
        errors: &mut String,
    ) {
        ui.horizontal(|ui| {
            if ui
                .button(
                    RichText::from("ENCRYPT")
                        .color(Color32::GOLD)
                        .background_color(Color32::DARK_GRAY),
                )
                .clicked()
            {
                errors.clear();
                match self.encrypt(input) {
                    Ok(text) => *output = text,
                    Err(e) => *errors = e.to_string(),
                }
            };
            if ui
                .button(
                    RichText::from("DECRYPT")
                        .color(Color32::GOLD)
                        .background_color(Color32::DARK_GRAY),
                )
                .clicked()
            {
                errors.clear();
                match self.decrypt(input) {
                    Ok(text) => *output = text,
                    Err(e) => *errors = e.to_string(),
                }
            }
        });
    }
}

fn combox_box(
    ciphers: &[CipherID],
    identifier: &'static str,
    active_cipher: &mut CipherID,
    ui: &mut egui::Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in ciphers {
                ui.selectable_value(active_cipher, *id, format!("{}", id));
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CipherControlPanel {
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    plugboard: Plugboard,

    m209: M209,
    enigma: EnigmaM3,
    sigaba: Sigaba,

    vigenere: Vigenere,
    beaufort: Beaufort,
    alberti: Alberti,
    m94: M94,
    bazeries: Bazeries,
    porta: Porta,
    quagmire: Quagmire,    
    chaocipher: Chaocipher,
    hutton: Hutton,

    playfair: Playfair,
    slidefair: Slidefair,
    two_square: TwoSquare,
    four_square: FourSquare,

    columnar: Columnar,
    grille: Grille,
    rail_fence: RailFence,
    scytale: Scytale,
    turning_grille: TurningGrille,

    polybius: PolybiusSquare,
    polybius_cube: PolybiusCube,
    adfgvx: Adfgvx,
    b64: B64,
    bifid: Bifid,
    trifid: Trifid,
    checkerboard: StraddlingCheckerboard,

    batco: Batco,
    dryad: Dryad,
}

impl CipherControlPanel {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        active_cipher: &mut CipherID,
        rng: &mut StdRng,
        errors: &mut String,
    ) {
        egui::Grid::new("comboboxes").show(ui, |ui| {
            combox_box(
                &[
                    CipherID::Caesar,
                    CipherID::Decoder,
                    CipherID::Affine,
                    CipherID::Substitution,
                    CipherID::Plugboard,
                ],
                "Substitution",
                active_cipher,
                ui,
            );

            combox_box(
                &[
                    CipherID::Vigenere,
                    CipherID::Beaufort,
                    CipherID::M94,
                    CipherID::Alberti,
                    CipherID::Bazeries,
                    CipherID::Porta,
                    CipherID::Quagmire,
                    CipherID::Chaocipher, 
                    CipherID::Hutton
                ],
                "Polyalphabetic",
                active_cipher,
                ui,
            );

            combox_box(
                &[CipherID::M209, CipherID::Enigma, CipherID::Sigaba],
                "Rotor Machine",
                active_cipher,
                ui,
            );

            combox_box(
                &[
                    CipherID::Columnar,
                    CipherID::Grille,
                    CipherID::RailFence,
                    CipherID::Scytale,
                    CipherID::TurningGrille,
                ],
                "Transposition",
                active_cipher,
                ui,
            );

            ui.end_row();

            combox_box(
                &[
                    CipherID::Playfair,
                    CipherID::Slidefair,
                    CipherID::TwoSquare,
                    CipherID::FourSquare,
                ],
                "Playfair",
                active_cipher,
                ui,
            );

            combox_box(
                &[CipherID::Batco, CipherID::Dryad],
                "Tactical",
                active_cipher,
                ui,
            );

            combox_box(
                &[
                    CipherID::Polybius,
                    CipherID::PolybiusCube,
                    CipherID::Adfgvx,
                    CipherID::B64,
                    CipherID::Bifid,
                    CipherID::Trifid,
                    CipherID::Checkerboard,
                ],
                "Polybius",
                active_cipher,
                ui,
            );

        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        let name = RichText::new(String::from(*active_cipher))
            .strong()
            .heading();
        ui.add(egui::Label::new(name));
        ui.label(active_cipher.description());

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match active_cipher {
            CipherID::Caesar => self.caesar.ui(ui, rng, errors),
            CipherID::Affine => self.affine.ui(ui, rng, errors),
            CipherID::Decoder => self.decoder_ring.ui(ui, rng, errors),
            CipherID::Substitution => self.gen_sub.ui(ui, rng, errors),
            CipherID::Polybius => self.polybius.ui(ui, rng, errors),
            CipherID::PolybiusCube => self.polybius_cube.ui(ui, rng, errors),
            CipherID::Vigenere => self.vigenere.ui(ui, rng, errors),
            CipherID::Beaufort => self.beaufort.ui(ui, rng, errors),
            CipherID::M209 => self.m209.ui(ui, rng, errors),
            CipherID::M94 => self.m94.ui(ui, rng, errors),
            CipherID::Alberti => self.alberti.ui(ui, rng, errors),
            CipherID::Playfair => self.playfair.ui(ui, rng, errors),
            CipherID::Columnar => self.columnar.ui(ui, rng, errors),
            CipherID::Adfgvx => self.adfgvx.ui(ui, rng, errors),
            CipherID::B64 => self.b64.ui(ui, rng, errors),
            CipherID::Slidefair => self.slidefair.ui(ui, rng, errors),
            CipherID::Enigma => self.enigma.ui(ui, rng, errors),
            CipherID::Grille => self.grille.ui(ui, rng, errors),
            CipherID::Sigaba => self.sigaba.ui(ui, rng, errors),
            CipherID::Bazeries => self.bazeries.ui(ui, rng, errors),
            CipherID::Chaocipher => self.chaocipher.ui(ui, rng, errors),
            CipherID::Bifid => self.bifid.ui(ui, rng, errors),
            CipherID::Trifid => self.trifid.ui(ui, rng, errors),
            CipherID::RailFence => self.rail_fence.ui(ui, rng, errors),
            CipherID::Scytale => self.scytale.ui(ui, rng, errors),
            CipherID::Batco => self.batco.ui(ui, rng, errors),
            CipherID::Checkerboard => self.checkerboard.ui(ui, rng, errors),
            CipherID::Porta => self.porta.ui(ui, rng, errors),
            CipherID::Dryad => self.dryad.ui(ui, rng, errors),
            CipherID::FourSquare => self.four_square.ui(ui, rng, errors),
            CipherID::TwoSquare => self.two_square.ui(ui, rng, errors),
            CipherID::Hutton => self.hutton.ui(ui, rng, errors),
            CipherID::Quagmire => self.quagmire.ui(ui, rng, errors),
            CipherID::TurningGrille => self.turning_grille.ui(ui, rng, errors),
            CipherID::Plugboard => self.plugboard.ui(ui, rng, errors),
            _ => {
                ui.label("IN PROGRESS");
            }
        }
    }
}

pub struct CipherDisplayPanel {}

impl Default for CipherDisplayPanel {
    fn default() -> Self {
        Self {}
    }
}

impl CipherDisplayPanel {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        input: &mut String,
        output: &mut String,
        errors: &mut String,
        active_cipher: &mut CipherID,
        control_panel: &CipherControlPanel,
    ) {
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        match active_cipher {
            CipherID::Caesar => encrypt_decrypt(ui, &control_panel.caesar, input, output, errors),
            CipherID::Affine => encrypt_decrypt(ui, &control_panel.affine, input, output, errors),
            CipherID::Decoder => {
                encrypt_decrypt(ui, &control_panel.decoder_ring, input, output, errors)
            }
            CipherID::Substitution => {
                encrypt_decrypt(ui, &control_panel.gen_sub, input, output, errors)
            }
            CipherID::Polybius => {
                encrypt_decrypt(ui, &control_panel.polybius, input, output, errors)
            }
            CipherID::PolybiusCube => {
                encrypt_decrypt(ui, &control_panel.polybius_cube, input, output, errors)
            }
            CipherID::Vigenere => {
                encrypt_decrypt(ui, &control_panel.vigenere, input, output, errors)
            }
            CipherID::Beaufort => {
                encrypt_decrypt(ui, &control_panel.beaufort, input, output, errors)
            }
            CipherID::M209 => encrypt_decrypt(ui, &control_panel.m209, input, output, errors),
            CipherID::M94 => encrypt_decrypt(ui, &control_panel.m94, input, output, errors),
            CipherID::Alberti => encrypt_decrypt(ui, &control_panel.alberti, input, output, errors),
            CipherID::Playfair => {
                encrypt_decrypt(ui, &control_panel.playfair, input, output, errors)
            }
            CipherID::Columnar => {
                encrypt_decrypt(ui, &control_panel.columnar, input, output, errors)
            }
            CipherID::Adfgvx => encrypt_decrypt(ui, &control_panel.adfgvx, input, output, errors),
            CipherID::B64 => encrypt_decrypt(ui, &control_panel.b64, input, output, errors),
            CipherID::Slidefair => {
                encrypt_decrypt(ui, &control_panel.slidefair, input, output, errors)
            }
            CipherID::Enigma => encrypt_decrypt(ui, &control_panel.enigma, input, output, errors),
            CipherID::Grille => encrypt_decrypt(ui, &control_panel.grille, input, output, errors),
            CipherID::Sigaba => encrypt_decrypt(ui, &control_panel.sigaba, input, output, errors),
            CipherID::Bazeries => {
                encrypt_decrypt(ui, &control_panel.bazeries, input, output, errors)
            }
            CipherID::Chaocipher => {
                encrypt_decrypt(ui, &control_panel.chaocipher, input, output, errors)
            }
            CipherID::Bifid => encrypt_decrypt(ui, &control_panel.bifid, input, output, errors),
            CipherID::Trifid => encrypt_decrypt(ui, &control_panel.trifid, input, output, errors),
            CipherID::RailFence => {
                encrypt_decrypt(ui, &control_panel.rail_fence, input, output, errors)
            }
            CipherID::Scytale => encrypt_decrypt(ui, &control_panel.scytale, input, output, errors),
            CipherID::Batco => encrypt_decrypt(ui, &control_panel.batco, input, output, errors),
            CipherID::Checkerboard => {
                encrypt_decrypt(ui, &control_panel.checkerboard, input, output, errors)
            }
            CipherID::Porta => encrypt_decrypt(ui, &control_panel.porta, input, output, errors),
            CipherID::Dryad => encrypt_decrypt(ui, &control_panel.dryad, input, output, errors),
            CipherID::FourSquare => {
                encrypt_decrypt(ui, &control_panel.four_square, input, output, errors)
            }
            CipherID::TwoSquare => {
                encrypt_decrypt(ui, &control_panel.two_square, input, output, errors)
            }
            CipherID::Hutton => encrypt_decrypt(ui, &control_panel.hutton, input, output, errors),
            CipherID::Quagmire => encrypt_decrypt(ui, &control_panel.quagmire, input, output, errors),
            CipherID::TurningGrille => encrypt_decrypt(ui, &control_panel.turning_grille, input, output, errors),
            CipherID::Plugboard => encrypt_decrypt(ui, &control_panel.plugboard, input, output, errors),
            _ => *errors = String::from("button must be added to DisplayPanel struct"),
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
