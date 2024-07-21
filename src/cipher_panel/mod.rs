use blowfish_controls::BlowfishFrame;
use chacha_ex_controls::ChaChaExNonceFrame;
use ciphers::{
    errors::CipherError,
    ids::{cipher_categories::CipherCategory, CipherId},
    traits::Cipher,
};
use des_controls::DesFrame;
use desx_controls::DesXFrame;
use diffie_hellman_controls::DiffieHellmanFrame;
use egui::Ui;
use idea_controls::IdeaFrame;
use triple_des_controls::TripleDesFrame;
use xor_splitting_controls::XorSecretSplittingFrame;
use xtea_controls::XteaFrame;

use self::{
    adfgvx_controls::AdfgvxFrame, aes_controls::AesFrame, affine_controls::AffineFrame,
    alberti_controls::AlbertiFrame, amsco_controls::AmscoFrame, b64_controls::B64Frame,
    batco_controls::BatcoFrame, bazeries_controls::BazeriesFrame, beaufort_controls::BeaufortFrame,
    bifid_controls::BifidFrame, caesar_controls::CaesarFrame,
    chacha20_poly1305_controls::ChaCha20Poly1305Frame, chacha_controls::ChaChaFrame,
    chaocipher_controls::ChaocipherFrame, checkerboard_controls::StraddlingCheckerboardFrame,
    columnar_controls::ColumnarFrame, decoder_ring_controls::DecoderRingFrame,
    diagonal_columnar_controls::DiagonalColumnarFrame, dryad_controls::DryadFrame,
    enigma_controls::EnigmaM3Frame, four_square_controls::FourSquareFrame,
    general_sub_controls::GeneralSubstitutionFrame, grille_controls::GrilleFrame,
    hebern_controls::HebernFrame, hutton_controls::HuttonFrame, m209_controls::M209Frame,
    m94_controls::M94Frame, nihilist_controls::NihilistFrame, playfair_controls::PlayfairFrame,
    plugboard_controls::PlugboardFrame, polybius_cube_controls::PolybiusCubeFrame,
    polybius_square_controls::PolybiusSquareFrame, porta_controls::PortaFrame,
    purple_controls::PurpleFrame, quagmire_controls::QuagmireFrame,
    rail_fence_controls::RailFenceFrame, rc4_controls::Rc4Frame, rc5_controls::Rc5Frame,
    rs44_controls::Rs44Frame, rsa_controls::RsaFrame, salsa20_controls::Salsa20Frame,
    scytale_controls::ScytaleFrame, seriated_playfair_controls::SeriatedPlayfairFrame,
    shamir_controls::ShamirSecretSharingFrame, sigaba_controls::SigabaFrame,
    slidefair_controls::SlidefairFrame, tea_controls::TeaFrame, trifid_controls::TrifidFrame,
    turning_grille_controls::TurningGrilleFrame, two_square_controls::TwoSquareFrame,
    vic_controls::VicFrame, vigenere_controls::VigenereFrame,
};

mod adfgvx_controls;
mod aes_controls;
mod affine_controls;
mod alberti_controls;
mod amsco_controls;
mod b64_controls;
mod batco_controls;
mod bazeries_controls;
mod beaufort_controls;
mod bifid_controls;
mod blowfish_controls;
mod caesar_controls;
mod chacha20_poly1305_controls;
mod chacha_controls;
mod chacha_ex_controls;
mod chaocipher_controls;
mod checkerboard_controls;
mod columnar_controls;
mod decoder_ring_controls;
mod des_controls;
mod desx_controls;
mod diagonal_columnar_controls;
mod diffie_hellman_controls;
mod dryad_controls;
mod elgamal_controls;
mod enigma_controls;
mod four_square_controls;
mod general_sub_controls;
mod grille_controls;
mod hebern_controls;
mod hutton_controls;
mod idea_controls;
mod m209_controls;
mod m94_controls;
mod nihilist_controls;
mod playfair_controls;
mod plugboard_controls;
mod polybius_cube_controls;
mod polybius_square_controls;
mod porta_controls;
mod purple_controls;
mod quagmire_controls;
mod rail_fence_controls;
mod rc4_controls;
mod rc5_controls;
mod rs44_controls;
mod rsa_controls;
mod salsa20_controls;
mod scytale_controls;
mod seriated_playfair_controls;
mod shamir_controls;
mod sigaba_controls;
mod slidefair_controls;
mod tea_controls;
mod trifid_controls;
mod triple_des_controls;
mod turning_grille_controls;
mod two_square_controls;
mod vic_controls;
mod vigenere_controls;
mod xor_splitting_controls;
mod xtea_controls;

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
    code: &[CipherId],
    active_cipher: &mut Option<CipherId>,
    cipher_category: CipherCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(cipher_category.to_string())
            .selected_text(cipher_category.to_string())
            .show_ui(ui, |ui| {
                for id in code {
                    ui.selectable_value(active_cipher, Some(*id), id.to_string());
                }
            });
        ui.menu_button("+", |ui| {
            ui.label(cipher_category.description());
        })
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
    purple: PurpleFrame,

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
    seriated: SeriatedPlayfairFrame,
    slidefair: SlidefairFrame,
    two_square: TwoSquareFrame,
    four_square: FourSquareFrame,

    // Transposition
    amsco: AmscoFrame,
    columnar: ColumnarFrame,
    diagonal_columnar: DiagonalColumnarFrame,
    grille: GrilleFrame,
    rail_fence: RailFenceFrame,
    scytale: ScytaleFrame,
    turning_grille: TurningGrilleFrame,

    // Polybius Based
    adfgvx: AdfgvxFrame,
    b64: B64Frame,
    bifid: BifidFrame,
    nihilist: NihilistFrame,
    polybius: PolybiusSquareFrame,
    polybius_cube: PolybiusCubeFrame,
    trifid: TrifidFrame,

    // Tactical
    batco: BatcoFrame,
    checkerboard: StraddlingCheckerboardFrame,
    dryad: DryadFrame,
    rs44: Rs44Frame,

    // Digital
    aes: AesFrame,
    blowfish: BlowfishFrame,
    chacha: ChaChaFrame,
    chachaexnonce: ChaChaExNonceFrame,
    chacha20poly1305: ChaCha20Poly1305Frame,
    des: DesFrame,
    desx: DesXFrame,
    diffie_hellman: DiffieHellmanFrame,
    idea: IdeaFrame,
    rc4: Rc4Frame,
    rc5: Rc5Frame,
    rsa: RsaFrame,
    tea: TeaFrame,
    triple_des: TripleDesFrame,
    salsa20: Salsa20Frame,
    xtea: XteaFrame,

    // Other
    shamir: ShamirSecretSharingFrame,
    vic: VicFrame,
    xor_splitting: XorSecretSplittingFrame,
}

impl CipherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_cipher: &mut Option<CipherId>) {
        combox_box(
            &[
                CipherId::Enigma,
                CipherId::Hebern,
                CipherId::M209,
                CipherId::Purple,
                CipherId::Sigaba,
            ],
            active_cipher,
            CipherCategory::Electromechanical,
            ui,
        );

        combox_box(
            &[
                CipherId::FourSquare,
                CipherId::Playfair,
                CipherId::SeriatedPlayfair,
                CipherId::Slidefair,
                CipherId::TwoSquare,
            ],
            active_cipher,
            CipherCategory::Playfair,
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
            active_cipher,
            CipherCategory::Polyalphabetic,
            ui,
        );

        combox_box(
            &[
                CipherId::Adfgvx,
                CipherId::B64,
                CipherId::Bifid,
                CipherId::Checkerboard,
                CipherId::Nihilist,
                CipherId::Polybius,
                CipherId::PolybiusCube,
                CipherId::Trifid,
            ],
            active_cipher,
            CipherCategory::Polybius,
            ui,
        );

        combox_box(
            &[
                CipherId::Affine,
                CipherId::Caesar,
                CipherId::Decoder,
                CipherId::Plugboard,
                CipherId::Substitution,
            ],
            active_cipher,
            CipherCategory::Substituion,
            ui,
        );

        combox_box(
            &[CipherId::Batco, CipherId::Dryad, CipherId::Rs44],
            active_cipher,
            CipherCategory::Tactical,
            ui,
        );

        combox_box(
            &[
                CipherId::Amsco,
                CipherId::Columnar,
                CipherId::DiagonalColumnar,
                CipherId::Grille,
                CipherId::RailFence,
                CipherId::Scytale,
                CipherId::TurningGrille,
            ],
            active_cipher,
            CipherCategory::Transposition,
            ui,
        );

        combox_box(
            &[
                CipherId::Aes,
                CipherId::Blowfish,
                CipherId::ChaCha,
                CipherId::ChaChaExtendedNonce,
                CipherId::ChaCha20Poly1305,
                CipherId::Des,
                CipherId::DesX,
                CipherId::DiffieHellman,
                CipherId::Idea,
                CipherId::Rc4,
                CipherId::Rc5,
                CipherId::Rsa,
                CipherId::Salsa20,
                CipherId::Tea,
                CipherId::TripleDes,
                CipherId::Xtea,
            ],
            active_cipher,
            CipherCategory::Digital,
            ui,
        );

        combox_box(
            &[CipherId::Shamir, CipherId::XorSplitting],
            active_cipher,
            CipherCategory::Sharing,
            ui,
        );

        combox_box(
            &[
                CipherId::Adfgvx,
                CipherId::B64,
                CipherId::Bifid,
                CipherId::Trifid,
                CipherId::Vic,
            ],
            active_cipher,
            CipherCategory::Composite,
            ui,
        );
    }

    pub fn get_active_cipher(&mut self, active_cipher: &CipherId) -> &mut dyn CipherFrame {
        match active_cipher {
            CipherId::Aes => &mut self.aes,
            CipherId::Adfgvx => &mut self.adfgvx,
            CipherId::Affine => &mut self.affine,
            CipherId::Alberti => &mut self.alberti,
            CipherId::Amsco => &mut self.amsco,
            CipherId::B64 => &mut self.b64,
            CipherId::Batco => &mut self.batco,
            CipherId::Bazeries => &mut self.bazeries,
            CipherId::Beaufort => &mut self.beaufort,
            CipherId::Bifid => &mut self.bifid,
            CipherId::Blowfish => &mut self.blowfish,
            CipherId::Caesar => &mut self.caesar,
            CipherId::ChaCha => &mut self.chacha,
            CipherId::ChaChaExtendedNonce => &mut self.chachaexnonce,
            CipherId::ChaCha20Poly1305 => &mut self.chacha20poly1305,
            CipherId::Chaocipher => &mut self.chaocipher,
            CipherId::Checkerboard => &mut self.checkerboard,
            CipherId::Columnar => &mut self.columnar,
            CipherId::Decoder => &mut self.decoder_ring,
            CipherId::Des => &mut self.des,
            CipherId::DesX => &mut self.desx,
            CipherId::DiagonalColumnar => &mut self.diagonal_columnar,
            CipherId::DiffieHellman => &mut self.diffie_hellman,
            CipherId::Dryad => &mut self.dryad,
            CipherId::Enigma => &mut self.enigma,
            CipherId::FourSquare => &mut self.four_square,
            CipherId::Grille => &mut self.grille,
            CipherId::Hebern => &mut self.hebern,
            CipherId::Hutton => &mut self.hutton,
            CipherId::Idea => &mut self.idea,
            CipherId::M209 => &mut self.m209,
            CipherId::M94 => &mut self.m94,
            CipherId::Nihilist => &mut self.nihilist,
            CipherId::Playfair => &mut self.playfair,
            CipherId::Plugboard => &mut self.plugboard,
            CipherId::Polybius => &mut self.polybius,
            CipherId::PolybiusCube => &mut self.polybius_cube,
            CipherId::Porta => &mut self.porta,
            CipherId::Purple => &mut self.purple,
            CipherId::Quagmire => &mut self.quagmire,
            CipherId::RailFence => &mut self.rail_fence,
            CipherId::Rc4 => &mut self.rc4,
            CipherId::Rc5 => &mut self.rc5,
            CipherId::Rsa => &mut self.rsa,
            CipherId::Rs44 => &mut self.rs44,
            CipherId::Salsa20 => &mut self.salsa20,
            CipherId::Scytale => &mut self.scytale,
            CipherId::SeriatedPlayfair => &mut self.seriated,
            CipherId::Shamir => &mut self.shamir,
            CipherId::Sigaba => &mut self.sigaba,
            CipherId::Slidefair => &mut self.slidefair,
            CipherId::Substitution => &mut self.gen_sub,
            CipherId::Tea => &mut self.tea,
            CipherId::Trifid => &mut self.trifid,
            CipherId::TripleDes => &mut self.triple_des,
            CipherId::TurningGrille => &mut self.turning_grille,
            CipherId::TwoSquare => &mut self.two_square,
            CipherId::Vic => &mut self.vic,
            CipherId::Vigenere => &mut self.vigenere,
            CipherId::XorSplitting => &mut self.xor_splitting,
            CipherId::Xtea => &mut self.xtea,
            _ => todo!("<<<CIPHER NOT FOUND>>>"),
        }
    }
}
