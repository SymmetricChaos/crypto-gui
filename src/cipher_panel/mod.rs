use crate::{
    ciphers::{
        hebern::Hebern, playfair::*, polyalphabetic::*, polybius::*, substitution::*, tactical::*,
        transposition::*, *,
    },
    ids::CipherId,
};
use eframe::egui::{self, Ui};

pub mod _generic_components;
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
pub mod grille_controls;
pub mod hebern_controls;
pub mod hutton_controls;
pub mod m209_controls;
pub mod m94_controls;
pub mod playfair_controls;
pub mod plugboard_controls;
pub mod polybius_cube_controls;
pub mod polybius_square_controls;
pub mod porta_controls;
pub mod purple_controls;
pub mod quagmire_controls;
pub mod rail_fence_controls;
pub mod rs44_controls;
pub mod scytale_controls;
pub mod sigaba_controls;
pub mod slidefair_controls;
pub mod trifid_controls;
pub mod turning_grille_controls;
pub mod two_square_controls;
pub mod vigenere_controls;

pub trait ViewableCipher: View + Cipher {}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}

// Quick simple combo box builder
fn combox_box(
    ciphers: &[CipherId],
    identifier: &'static str,
    active_cipher: &mut CipherId,
    ui: &mut Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in ciphers {
                ui.selectable_value(active_cipher, *id, id.to_string());
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CipherInterface {
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    plugboard: Plugboard,

    m209: M209,
    enigma: EnigmaM3,
    sigaba: Sigaba,
    hebern: Hebern,
    // purple: Purple,
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
    rs44: RS44,
}

impl CipherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_cipher: &mut CipherId) {
        combox_box(
            &[
                CipherId::Caesar,
                CipherId::Decoder,
                CipherId::Affine,
                CipherId::Substitution,
                CipherId::Plugboard,
            ],
            "Substitution",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Vigenere,
                CipherId::Beaufort,
                CipherId::M94,
                CipherId::Alberti,
                CipherId::Bazeries,
                CipherId::Porta,
                CipherId::Quagmire,
                CipherId::Chaocipher,
                CipherId::Hutton,
            ],
            "Polyalphabetic",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::M209,
                CipherId::Enigma,
                CipherId::Sigaba,
                CipherId::Hebern,
                // CipherId::Purple,
            ],
            "Cipher Machine",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Columnar,
                CipherId::Grille,
                CipherId::RailFence,
                CipherId::Scytale,
                CipherId::TurningGrille,
            ],
            "Transposition",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Playfair,
                CipherId::Slidefair,
                CipherId::TwoSquare,
                CipherId::FourSquare,
            ],
            "Playfair",
            active_cipher,
            ui,
        );

        combox_box(
            &[CipherId::Batco, CipherId::Dryad, CipherId::Rs44],
            "Tactical",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Polybius,
                CipherId::PolybiusCube,
                CipherId::Adfgvx,
                CipherId::B64,
                CipherId::Bifid,
                CipherId::Trifid,
                CipherId::Checkerboard,
            ],
            "Polybius",
            active_cipher,
            ui,
        );
    }

    pub fn get_active_cipher(&mut self, active_cipher: &CipherId) -> &mut dyn ViewableCipher {
        match active_cipher {
            CipherId::Caesar => &mut self.caesar,
            CipherId::Affine => &mut self.affine,
            CipherId::Decoder => &mut self.decoder_ring,
            CipherId::Substitution => &mut self.gen_sub,
            CipherId::Polybius => &mut self.polybius,
            CipherId::PolybiusCube => &mut self.polybius_cube,
            CipherId::Vigenere => &mut self.vigenere,
            CipherId::Beaufort => &mut self.beaufort,
            CipherId::M209 => &mut self.m209,
            CipherId::M94 => &mut self.m94,
            CipherId::Alberti => &mut self.alberti,
            CipherId::Playfair => &mut self.playfair,
            CipherId::Columnar => &mut self.columnar,
            CipherId::Adfgvx => &mut self.adfgvx,
            CipherId::B64 => &mut self.b64,
            CipherId::Slidefair => &mut self.slidefair,
            CipherId::Enigma => &mut self.enigma,
            CipherId::Grille => &mut self.grille,
            CipherId::Sigaba => &mut self.sigaba,
            CipherId::Bazeries => &mut self.bazeries,
            CipherId::Chaocipher => &mut self.chaocipher,
            CipherId::Bifid => &mut self.bifid,
            CipherId::Trifid => &mut self.trifid,
            CipherId::RailFence => &mut self.rail_fence,
            CipherId::Scytale => &mut self.scytale,
            CipherId::Batco => &mut self.batco,
            CipherId::Checkerboard => &mut self.checkerboard,
            CipherId::Porta => &mut self.porta,
            CipherId::Dryad => &mut self.dryad,
            CipherId::FourSquare => &mut self.four_square,
            CipherId::TwoSquare => &mut self.two_square,
            CipherId::Hutton => &mut self.hutton,
            CipherId::Quagmire => &mut self.quagmire,
            CipherId::TurningGrille => &mut self.turning_grille,
            CipherId::Plugboard => &mut self.plugboard,
            CipherId::Rs44 => &mut self.rs44,
            CipherId::Hebern => &mut self.hebern,
            // CipherId::Purple => &mut self.purple,
            _ => todo!(),
        }
    }
}

// #[derive(Default)]
// pub struct CipherIO {}

// impl CipherIO {
//     pub fn ui(
//         &mut self,
//         ui: &mut egui::Ui,
//         input: &mut String,
//         output: &mut String,
//         errors: &mut String,
//         active_cipher: &mut CipherID,
//         control_panel: &mut CipherInterface,
//     ) {
//         ui.add_space(32.0);
//         ui.label("INPUT TEXT");
//         ui.add(TextEdit::multiline(input).font(TextStyle::Monospace));
//         ui.add_space(16.0);
//         ui.label("OUTPUT TEXT");
//         ui.add(TextEdit::multiline(output).font(TextStyle::Monospace));

//         encrypt_decrypt(
//             ui,
//             control_panel.get_active_cipher(active_cipher),
//             input,
//             output,
//             errors,
//         );

//         ui.add_space(10.0);
//         if ui.button("clear").clicked() {
//             input.clear();
//             output.clear();
//             errors.clear();
//         }

//         ui.add_space(10.0);
//         if ui.button("swap input/output").clicked() {
//             std::mem::swap(input, output)
//         }

//         ui.add_space(16.0);
//         global_rng_controls(ui);

//         if !errors.is_empty() {
//             ui.add_space(24.0);
//             ui.label(
//                 RichText::new(errors.clone())
//                     .color(Color32::RED)
//                     .background_color(Color32::BLACK)
//                     .monospace(),
//             );
//         }
//     }
// }
