use ciphers::{errors::CipherError, ids::CipherId, traits::Cipher};
use egui::Ui;

use self::{
    adfgvx_controls::AdfgvxFrame, affine_controls::AffineFrame, alberti_controls::AlbertiFrame,
    amsco_controls::AmscoFrame, b64_controls::B64Frame, batco_controls::BatcoFrame,
    bazeries_controls::BazeriesFrame, beaufort_controls::BeaufortFrame, bifid_controls::BifidFrame,
    caesar_controls::CaesarFrame, chaocipher_controls::ChaocipherFrame,
    checkerboard_controls::StraddlingCheckerboardFrame, columnar_controls::ColumnarFrame,
    decoder_ring_controls::DecoderRingFrame, dryad_controls::DryadFrame,
    enigma_controls::EnigmaM3Frame, four_square_controls::FourSquareFrame,
    general_sub_controls::GeneralSubstitutionFrame, grille_controls::GrilleFrame,
    hebern_controls::HebernFrame, hutton_controls::HuttonFrame, m209_controls::M209Frame,
    m94_controls::M94Frame, playfair_controls::PlayfairFrame, plugboard_controls::PlugboardFrame,
    polybius_cube_controls::PolybiusCubeFrame, polybius_square_controls::PolybiusSquareFrame,
    porta_controls::PortaFrame, quagmire_controls::QuagmireFrame,
    rail_fence_controls::RailFenceFrame, rs44_controls::Rs44Frame, scytale_controls::ScytaleFrame,
    sigaba_controls::SigabaFrame, slidefair_controls::SlidefairFrame, trifid_controls::TrifidFrame,
    turning_grille_controls::TurningGrilleFrame, two_square_controls::TwoSquareFrame,
    vigenere_controls::VigenereFrame,
};

pub mod _generic_components;
pub mod adfgvx_controls;
pub mod affine_controls;
pub mod alberti_controls;
pub mod amsco_controls;
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
// pub mod purple_controls;

pub trait CipherFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn cipher(&self) -> &dyn Cipher;
    fn randomize(&mut self);
    fn reset(&mut self);
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.cipher().encrypt(text)
    }
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.cipher().decrypt(text)
    }
}

// Quick simple combo box builder
fn combox_box(
    ciphers: &[CipherId],
    identifier: &'static str,
    active_cipher: &mut Option<CipherId>,
    ui: &mut Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in ciphers {
                ui.selectable_value(active_cipher, Some(*id), id.to_string());
            }
        });
    ui.add_space(10.0);
}

#[derive(Default)]
pub struct CipherInterface {
    // Simple Substitution
    affine: AffineFrame,
    caesar: CaesarFrame,
    decoder_ring: DecoderRingFrame,
    gen_sub: GeneralSubstitutionFrame,
    plugboard: PlugboardFrame,

    // Electromechanical
    enigma: EnigmaM3Frame,
    hebern: HebernFrame,
    m209: M209Frame,
    sigaba: SigabaFrame,
    // purple: Purple,

    // Polyalphabetic
    alberti: AlbertiFrame,
    bazeries: BazeriesFrame,
    beaufort: BeaufortFrame,
    chaocipher: ChaocipherFrame,
    hutton: HuttonFrame,
    m94: M94Frame,
    porta: PortaFrame,
    quagmire: QuagmireFrame,
    vigenere: VigenereFrame,

    // Playfair Based
    playfair: PlayfairFrame,
    slidefair: SlidefairFrame,
    two_square: TwoSquareFrame,
    four_square: FourSquareFrame,

    // Transposition
    amsco: AmscoFrame,
    columnar: ColumnarFrame,
    grille: GrilleFrame,
    rail_fence: RailFenceFrame,
    scytale: ScytaleFrame,
    turning_grille: TurningGrilleFrame,

    // Polybius Based
    adfgvx: AdfgvxFrame,
    b64: B64Frame,
    bifid: BifidFrame,
    polybius: PolybiusSquareFrame,
    polybius_cube: PolybiusCubeFrame,
    trifid: TrifidFrame,

    // Tactical
    batco: BatcoFrame,
    checkerboard: StraddlingCheckerboardFrame,
    dryad: DryadFrame,
    rs44: Rs44Frame,
}

impl CipherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_cipher: &mut Option<CipherId>) {
        combox_box(
            &[
                CipherId::Affine,
                CipherId::Caesar,
                CipherId::Decoder,
                CipherId::Plugboard,
                CipherId::Substitution,
            ],
            "Substitution",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Alberti,
                CipherId::Bazeries,
                CipherId::Beaufort,
                CipherId::Chaocipher,
                CipherId::Hutton,
                CipherId::M94,
                CipherId::Porta,
                CipherId::Quagmire,
                CipherId::Vigenere,
            ],
            "Polyalphabetic",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Enigma,
                CipherId::Hebern,
                CipherId::M209,
                CipherId::Sigaba,
                // CipherId::Purple,
            ],
            "Cipher Machine",
            active_cipher,
            ui,
        );

        combox_box(
            &[
                CipherId::Amsco,
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
                CipherId::FourSquare,
                CipherId::Playfair,
                CipherId::Slidefair,
                CipherId::TwoSquare,
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
                CipherId::Adfgvx,
                CipherId::B64,
                CipherId::Bifid,
                CipherId::Checkerboard,
                CipherId::Polybius,
                CipherId::PolybiusCube,
                CipherId::Trifid,
            ],
            "Polybius",
            active_cipher,
            ui,
        );
    }

    pub fn get_active_cipher(&mut self, active_cipher: &CipherId) -> &mut dyn CipherFrame {
        match active_cipher {
            CipherId::Adfgvx => &mut self.adfgvx,
            CipherId::Affine => &mut self.affine,
            CipherId::Alberti => &mut self.alberti,
            CipherId::Amsco => &mut self.amsco,
            CipherId::B64 => &mut self.b64,
            CipherId::Batco => &mut self.batco,
            CipherId::Bazeries => &mut self.bazeries,
            CipherId::Beaufort => &mut self.beaufort,
            CipherId::Bifid => &mut self.bifid,
            CipherId::Caesar => &mut self.caesar,
            CipherId::Chaocipher => &mut self.chaocipher,
            CipherId::Checkerboard => &mut self.checkerboard,
            CipherId::Columnar => &mut self.columnar,
            CipherId::Decoder => &mut self.decoder_ring,
            CipherId::Dryad => &mut self.dryad,
            CipherId::Enigma => &mut self.enigma,
            CipherId::FourSquare => &mut self.four_square,
            CipherId::Grille => &mut self.grille,
            CipherId::Hebern => &mut self.hebern,
            CipherId::Hutton => &mut self.hutton,
            CipherId::M209 => &mut self.m209,
            CipherId::M94 => &mut self.m94,
            CipherId::Playfair => &mut self.playfair,
            CipherId::Plugboard => &mut self.plugboard,
            CipherId::Polybius => &mut self.polybius,
            CipherId::PolybiusCube => &mut self.polybius_cube,
            CipherId::Porta => &mut self.porta,
            // CipherId::Purple => &mut self.purple,
            CipherId::Quagmire => &mut self.quagmire,
            CipherId::RailFence => &mut self.rail_fence,
            CipherId::Rs44 => &mut self.rs44,
            CipherId::Scytale => &mut self.scytale,
            CipherId::Sigaba => &mut self.sigaba,
            CipherId::Slidefair => &mut self.slidefair,
            CipherId::Substitution => &mut self.gen_sub,
            CipherId::Trifid => &mut self.trifid,
            CipherId::TurningGrille => &mut self.turning_grille,
            CipherId::TwoSquare => &mut self.two_square,
            CipherId::Vigenere => &mut self.vigenere,
            _ => todo!("<<<CIPHER NOT FOUND>>>"),
        }
    }
}
