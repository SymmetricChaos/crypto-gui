use self::generic_components::encrypt_decrypt;
use crate::{
    cipher_id::CipherID,
    ciphers::{*, polybius::*, tactical::*, polyalphabetic::*, substitution::*, playfair::*, transposition::*}, global_rng::global_rng_controls,
};
use eframe::egui::{self, Color32, RichText, TextEdit, TextStyle, Ui};

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

pub trait ViewableCipher: View + Cipher {}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}

// Quick simple combo box builder
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

        let c = self.get_active_cipher(active_cipher);
        c.ui(ui, errors);
    }
    
    
    pub fn get_active_cipher(&mut self, active_cipher: &mut CipherID) -> &mut dyn ViewableCipher {
        match active_cipher {
            CipherID::Caesar => &mut self.caesar,
            CipherID::Affine => &mut self.affine,
            CipherID::Decoder => &mut self.decoder_ring,
            CipherID::Substitution => &mut self.gen_sub,
            CipherID::Polybius => &mut self.polybius,
            CipherID::PolybiusCube => &mut self.polybius_cube,
            CipherID::Vigenere => &mut self.vigenere,
            CipherID::Beaufort => &mut self.beaufort,
            CipherID::M209 => &mut self.m209,
            CipherID::M94 => &mut self.m94,
            CipherID::Alberti => &mut self.alberti,
            CipherID::Playfair => &mut self.playfair,
            CipherID::Columnar => &mut self.columnar,
            CipherID::Adfgvx => &mut self.adfgvx,
            CipherID::B64 => &mut self.b64,
            CipherID::Slidefair => &mut self.slidefair,
            CipherID::Enigma => &mut self.enigma,
            CipherID::Grille => &mut self.grille,
            CipherID::Sigaba => &mut self.sigaba,
            CipherID::Bazeries => &mut self.bazeries,
            CipherID::Chaocipher => &mut self.chaocipher,
            CipherID::Bifid => &mut self.bifid,
            CipherID::Trifid => &mut self.trifid,
            CipherID::RailFence => &mut self.rail_fence,
            CipherID::Scytale => &mut self.scytale,
            CipherID::Batco => &mut self.batco,
            CipherID::Checkerboard => &mut self.checkerboard,
            CipherID::Porta => &mut self.porta,
            CipherID::Dryad => &mut self.dryad,
            CipherID::FourSquare => &mut self.four_square,
            CipherID::TwoSquare => &mut self.two_square,
            CipherID::Hutton => &mut self.hutton,
            CipherID::Quagmire => &mut self.quagmire,
            CipherID::TurningGrille => &mut self.turning_grille,
            CipherID::Plugboard => &mut self.plugboard,
            _ => todo!(),
        }
    }
}

#[derive(Default)]
pub struct CipherDisplayPanel {}


impl CipherDisplayPanel {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        input: &mut String,
        output: &mut String,
        errors: &mut String,
        active_cipher: &mut CipherID,
        control_panel: &mut CipherControlPanel,
    ) {
        ui.add_space(32.0);
        ui.label("INPUT TEXT");
        ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
        ui.add_space(16.0);
        ui.label("OUTPUT TEXT");
        ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

        encrypt_decrypt(ui, control_panel.get_active_cipher(active_cipher), input, output, errors);

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