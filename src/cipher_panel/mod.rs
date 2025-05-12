use ciphers::{
    errors::CipherError,
    ids::{cipher_categories::CipherCategory, CipherId},
};
use egui::Ui;

mod a51_controls;
mod a52_controls;
mod adfgvx_controls;
mod aes_controls;
mod aes_gcm_controls;
mod affine_controls;
mod alberti_controls;
mod amsco_controls;
mod aria_controls;
mod ascon128_controls;
mod ascon80pq_controls;
mod b64_controls;
mod batco_controls;
mod bazeries_controls;
mod beaufort_controls;
mod bifid_controls;
mod blowfish_controls;
mod caesar_controls;
mod camellia_controls;
mod chacha20_poly1305_controls;
mod chacha_controls;
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
mod fealnx_control;
mod four_square_controls;
mod general_sub_controls;
mod gost_controls;
mod grille_controls;
mod hebern_controls;
mod hutton_controls;
mod idea_controls;
mod lea_controls;
mod m209_controls;
mod m94_controls;
mod misty1_controls;
mod nihilist_controls;
mod playfair_controls;
mod plugboard_controls;
mod polybius_cube_controls;
mod polybius_square_controls;
mod porta_controls;
mod purple_controls;
mod quagmire_controls;
mod rail_fence_controls;
mod rc2_controls;
mod rc4_controls;
mod rc5_controls;
mod rs44_controls;
mod rsa_controls;
mod salsa20_controls;
mod scytale_controls;
mod seed_controls;
mod seriated_playfair_controls;
mod serpent_controls;
mod shamir_controls;
mod sigaba_controls;
mod simon_controls;
mod slidefair_controls;
mod sm4_controls;
mod snow3g_controls;
mod speck_controls;
mod tea_controls;
mod threefish_controls;
mod trifid_controls;
mod triple_des_controls;
mod turning_grille_controls;
mod two_square_controls;
mod twofish_controls;
mod vic_controls;
mod vigenere_controls;
mod xchacha_controls;
mod xor_splitting_controls;
mod xtea_controls;

#[macro_export]
macro_rules! simple_cipher {
    () => {
        fn reset(&mut self) {
            *self = Self::default()
        }

        fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
            ciphers::Cipher::encrypt(&self.cipher, text)
        }

        fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
            ciphers::Cipher::decrypt(&self.cipher, text)
        }
    };
}

#[macro_export]
macro_rules! simple_block_cipher {
    ($blocksize: literal) => {
        fn reset(&mut self) {
            *self = Self::default()
        }

        fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
            use ciphers::digital::block_ciphers::block_cipher::BlockCipher;
            // Interpret the input
            let mut bytes = self
                .input_format
                .text_to_bytes(text)
                .map_err(|e| ciphers::errors::CipherError::Input(e.to_string()))?;

            // Provide the necessary kind and amount of padding
            if self.cipher.mode.padded() {
                self.cipher.padding.add_padding(&mut bytes, $blocksize)?;
            }

            // Select the correct mode. Since block ciphers all implement the BlockCipher
            // trait these are available for free. The fully qualified names for each of
            // the encrypt and decrypt functions are too messy and avoiding them is a pain
            // so when this macro is called the file must have the BlockCipher trait
            // imported.
            match self.cipher.mode {
                ciphers::digital::block_ciphers::block_cipher::BCMode::Ecb => {
                    self.cipher.encrypt_ecb(&mut bytes)
                }
                ciphers::digital::block_ciphers::block_cipher::BCMode::Ctr => self
                    .cipher
                    .encrypt_ctr(&mut bytes, self.cipher.iv.to_be_bytes()),
                ciphers::digital::block_ciphers::block_cipher::BCMode::Cbc => self
                    .cipher
                    .encrypt_cbc(&mut bytes, self.cipher.iv.to_be_bytes()),
                ciphers::digital::block_ciphers::block_cipher::BCMode::Pcbc => self
                    .cipher
                    .encrypt_pcbc(&mut bytes, self.cipher.iv.to_be_bytes()),
                ciphers::digital::block_ciphers::block_cipher::BCMode::Ofb => self
                    .cipher
                    .encrypt_ofb(&mut bytes, self.cipher.iv.to_be_bytes()),
                ciphers::digital::block_ciphers::block_cipher::BCMode::Cfb => self
                    .cipher
                    .encrypt_cfb(&mut bytes, self.cipher.iv.to_be_bytes()),
            };

            Ok(self.output_format.byte_slice_to_text(&bytes))
        }

        fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
            use crate::digital::block_ciphers::block_cipher::BlockCipher;
            // Interpret the input
            let mut bytes = self
                .input_format
                .text_to_bytes(text)
                .map_err(|e| crate::errors::CipherError::Input(e.to_string()))?;

            // If padding is needed return an error if the input for decryption is the wrong size
            if self.mode.padded() {
                if bytes.len() % $blocksize != 0 {
                    return Err(crate::errors::CipherError::General(format!(
                        "decryption requires blocks of exactly {} bytes",
                        $blocksize
                    )));
                }
            }

            // Select the correct mode. Since block ciphers all implement the BlockCipher
            // trait these are available for free. The fully qualified names for each of
            // the encrypt and decrypt functions are too messy and avoiding them is a pain
            // so when this macro is called the file must have the BlockCipher trait
            // imported.
            match self.mode {
                crate::digital::block_ciphers::block_cipher::BCMode::Ecb => {
                    self.decrypt_ecb(&mut bytes)
                }
                crate::digital::block_ciphers::block_cipher::BCMode::Ctr => {
                    self.decrypt_ctr(&mut bytes, self.iv.to_be_bytes())
                }
                crate::digital::block_ciphers::block_cipher::BCMode::Cbc => {
                    self.decrypt_cbc(&mut bytes, self.iv.to_be_bytes())
                }
                crate::digital::block_ciphers::block_cipher::BCMode::Pcbc => {
                    self.decrypt_pcbc(&mut bytes, self.iv.to_be_bytes())
                }
                crate::digital::block_ciphers::block_cipher::BCMode::Ofb => {
                    self.decrypt_ofb(&mut bytes, self.iv.to_be_bytes())
                }
                crate::digital::block_ciphers::block_cipher::BCMode::Cfb => {
                    self.decrypt_cfb(&mut bytes, self.iv.to_be_bytes())
                }
            };

            // Remove the appropriate kind and amount of padding
            if self.mode.padded() {
                self.padding.strip_padding(&mut bytes, $blocksize)?;
            }

            Ok(self.output_format.byte_slice_to_text(&bytes))
        }
    };
}

pub trait CipherFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn randomize(&mut self);
    fn reset(&mut self);
    fn encrypt_string(&self, text: &str) -> Result<String, CipherError>;
    fn decrypt_string(&self, text: &str) -> Result<String, CipherError>;
}

// Quick simple combo box builder
fn combox_box(
    code: &[CipherId],
    active_cipher: &mut Option<CipherId>,
    cipher_category: CipherCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt(cipher_category.to_string())
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
    affine: affine_controls::AffineFrame,
    caesar: caesar_controls::CaesarFrame,
    decoder_ring: decoder_ring_controls::DecoderRingFrame,
    gen_sub: general_sub_controls::GeneralSubstitutionFrame,
    plugboard: plugboard_controls::PlugboardFrame,

    // Electromechanical
    enigma: enigma_controls::EnigmaM3Frame,
    hebern: hebern_controls::HebernFrame,
    m209: m209_controls::M209Frame,
    sigaba: sigaba_controls::SigabaFrame,
    purple: purple_controls::PurpleFrame,

    // Polyalphabetic
    alberti: alberti_controls::AlbertiFrame,
    bazeries: bazeries_controls::BazeriesFrame,
    beaufort: beaufort_controls::BeaufortFrame,
    chaocipher: chaocipher_controls::ChaocipherFrame,
    hutton: hutton_controls::HuttonFrame,
    m94: m94_controls::M94Frame,
    porta: porta_controls::PortaFrame,
    quagmire: quagmire_controls::QuagmireFrame,
    vigenere: vigenere_controls::VigenereFrame,

    // Playfair Based
    playfair: playfair_controls::PlayfairFrame,
    seriated: seriated_playfair_controls::SeriatedPlayfairFrame,
    slidefair: slidefair_controls::SlidefairFrame,
    two_square: two_square_controls::TwoSquareFrame,
    four_square: four_square_controls::FourSquareFrame,

    // Transposition
    amsco: amsco_controls::AmscoFrame,
    columnar: columnar_controls::ColumnarFrame,
    diagonal_columnar: diagonal_columnar_controls::DiagonalColumnarFrame,
    grille: grille_controls::GrilleFrame,
    rail_fence: rail_fence_controls::RailFenceFrame,
    scytale: scytale_controls::ScytaleFrame,
    turning_grille: turning_grille_controls::TurningGrilleFrame,

    // Polybius Based
    adfgvx: adfgvx_controls::AdfgvxFrame,
    b64: b64_controls::B64Frame,
    bifid: bifid_controls::BifidFrame,
    nihilist: nihilist_controls::NihilistFrame,
    polybius: polybius_square_controls::PolybiusSquareFrame,
    polybius_cube: polybius_cube_controls::PolybiusCubeFrame,
    trifid: trifid_controls::TrifidFrame,

    // Tactical
    batco: batco_controls::BatcoFrame,
    checkerboard: checkerboard_controls::StraddlingCheckerboardFrame,
    dryad: dryad_controls::DryadFrame,
    rs44: rs44_controls::Rs44Frame,

    // Block
    aes: aes_controls::AesFrame,
    aria: aria_controls::AriaFrame,
    ascon128: ascon128_controls::Ascon128Frame,
    ascon80pq: ascon80pq_controls::Ascon80pqFrame,
    blowfish: blowfish_controls::BlowfishFrame,
    des: des_controls::DesFrame,
    desx: desx_controls::DesXFrame,
    fealnx: fealnx_control::FealNxFrame,
    gost: gost_controls::GostFrame,
    idea: idea_controls::IdeaFrame,
    lea: lea_controls::LeaFrame,
    misty1: misty1_controls::Misty1Frame,
    rc2: rc2_controls::Rc2Frame,
    rc5: rc5_controls::Rc5Frame,
    seed: seed_controls::SeedFrame,
    serpent: serpent_controls::SerpentFrame,
    simon: simon_controls::SimonFrame,
    sm4: sm4_controls::Sm4Frame,
    speck: speck_controls::SpeckFrame,
    tea: tea_controls::TeaFrame,
    threefish: threefish_controls::ThreefishFrame,
    triple_des: triple_des_controls::TripleDesFrame,
    twofish: twofish_controls::TwofishFrame,
    xtea: xtea_controls::XteaFrame,

    // Stream
    a51: a51_controls::A51Frame,
    a52: a52_controls::A52Frame,
    aes_gcm: aes_gcm_controls::AesGcmFrame,
    chacha: chacha_controls::ChaChaFrame,
    chacha20poly1305: chacha20_poly1305_controls::ChaCha20Poly1305Frame,
    rc4: rc4_controls::Rc4Frame,
    salsa20: salsa20_controls::Salsa20Frame,
    snow3g: snow3g_controls::Snow3GFrame,
    xchacha: xchacha_controls::XChaChaFrame,

    // Public Key
    diffie_hellman: diffie_hellman_controls::DiffieHellmanFrame,
    rsa: rsa_controls::RsaFrame,

    // Composite
    vic: vic_controls::VicFrame,

    // Secret ShARING
    shamir: shamir_controls::ShamirSecretSharingFrame,
    xor_splitting: xor_splitting_controls::XorSecretSplittingFrame,
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

        combox_box(
            &[
                CipherId::Aes,
                CipherId::Aria,
                CipherId::Ascon128,
                CipherId::Ascon80pq,
                CipherId::Blowfish,
                CipherId::Des,
                CipherId::DesX,
                CipherId::FealNx,
                CipherId::Gost,
                CipherId::Idea,
                CipherId::Lea,
                CipherId::Misty1,
                CipherId::Rc2,
                CipherId::Rc5,
                CipherId::Seed,
                CipherId::Serpent,
                CipherId::Simon,
                CipherId::Sm4,
                CipherId::Speck,
                CipherId::Tea,
                CipherId::Threefish,
                CipherId::TripleDes,
                CipherId::Twofish,
                CipherId::Xtea,
            ],
            active_cipher,
            CipherCategory::DigitalBlock,
            ui,
        );

        combox_box(
            &[
                CipherId::A51,
                CipherId::A52,
                CipherId::AesGcm,
                CipherId::ChaCha,
                CipherId::ChaCha20Poly1305,
                CipherId::Rc4,
                CipherId::Salsa20,
                CipherId::Snow3G,
                CipherId::XChaCha,
            ],
            active_cipher,
            CipherCategory::DigitalStream,
            ui,
        );

        combox_box(
            &[CipherId::Shamir, CipherId::XorSplitting],
            active_cipher,
            CipherCategory::Sharing,
            ui,
        );

        combox_box(
            &[CipherId::Rsa, CipherId::DiffieHellman],
            active_cipher,
            CipherCategory::PublicKey,
            ui,
        );
    }

    pub fn get_active_cipher(&mut self, active_cipher: &CipherId) -> &mut dyn CipherFrame {
        match active_cipher {
            CipherId::A51 => &mut self.a51,
            CipherId::A52 => &mut self.a52,
            CipherId::Aria => &mut self.aria,
            CipherId::Aes => &mut self.aes,
            CipherId::AesGcm => &mut self.aes_gcm,
            CipherId::Adfgvx => &mut self.adfgvx,
            CipherId::Affine => &mut self.affine,
            CipherId::Alberti => &mut self.alberti,
            CipherId::Amsco => &mut self.amsco,
            CipherId::Ascon128 => &mut self.ascon128,
            CipherId::Ascon80pq => &mut self.ascon80pq,
            CipherId::B64 => &mut self.b64,
            CipherId::Batco => &mut self.batco,
            CipherId::Bazeries => &mut self.bazeries,
            CipherId::Beaufort => &mut self.beaufort,
            CipherId::Bifid => &mut self.bifid,
            CipherId::Blowfish => &mut self.blowfish,
            CipherId::Caesar => &mut self.caesar,
            CipherId::ChaCha => &mut self.chacha,
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
            CipherId::FealNx => &mut self.fealnx,
            CipherId::FourSquare => &mut self.four_square,
            CipherId::Gost => &mut self.gost,
            CipherId::Grille => &mut self.grille,
            CipherId::Hebern => &mut self.hebern,
            CipherId::Hutton => &mut self.hutton,
            CipherId::Idea => &mut self.idea,
            CipherId::Lea => &mut self.lea,
            CipherId::M209 => &mut self.m209,
            CipherId::M94 => &mut self.m94,
            CipherId::Misty1 => &mut self.misty1,
            CipherId::Nihilist => &mut self.nihilist,
            CipherId::Playfair => &mut self.playfair,
            CipherId::Plugboard => &mut self.plugboard,
            CipherId::Polybius => &mut self.polybius,
            CipherId::PolybiusCube => &mut self.polybius_cube,
            CipherId::Porta => &mut self.porta,
            CipherId::Purple => &mut self.purple,
            CipherId::Quagmire => &mut self.quagmire,
            CipherId::RailFence => &mut self.rail_fence,
            CipherId::Rc2 => &mut self.rc2,
            CipherId::Rc4 => &mut self.rc4,
            CipherId::Rc5 => &mut self.rc5,
            CipherId::Rsa => &mut self.rsa,
            CipherId::Rs44 => &mut self.rs44,
            CipherId::Salsa20 => &mut self.salsa20,
            CipherId::Scytale => &mut self.scytale,
            CipherId::Seed => &mut self.seed,
            CipherId::SeriatedPlayfair => &mut self.seriated,
            CipherId::Serpent => &mut self.serpent,
            CipherId::Shamir => &mut self.shamir,
            CipherId::Sigaba => &mut self.sigaba,
            CipherId::Simon => &mut self.simon,
            CipherId::Slidefair => &mut self.slidefair,
            CipherId::Sm4 => &mut self.sm4,
            CipherId::Snow3G => &mut self.snow3g,
            CipherId::Speck => &mut self.speck,
            CipherId::Substitution => &mut self.gen_sub,
            CipherId::Tea => &mut self.tea,
            CipherId::Threefish => &mut self.threefish,
            CipherId::Trifid => &mut self.trifid,
            CipherId::TripleDes => &mut self.triple_des,
            CipherId::TurningGrille => &mut self.turning_grille,
            CipherId::Twofish => &mut self.twofish,
            CipherId::TwoSquare => &mut self.two_square,
            CipherId::Vic => &mut self.vic,
            CipherId::Vigenere => &mut self.vigenere,
            CipherId::XorSplitting => &mut self.xor_splitting,
            CipherId::XChaCha => &mut self.xchacha,
            CipherId::Xtea => &mut self.xtea,
            _ => todo!("<<<CIPHER NOT FOUND>>>"),
        }
    }
}
