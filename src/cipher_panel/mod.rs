use std::fmt::Display;

use eframe::egui;

use crate::ciphers::*;

pub mod caesar_controls;
pub mod generic_components;
pub mod affine_controls;
pub mod decoder_ring_controls;
pub mod m209_controls;
pub mod general_sub_controls;
pub mod playfair_controls;
pub mod cyclic_key_controls;
pub mod autokey_controls;
pub mod progressive_key_controls;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String);
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    M209,
    Playfair,
    CyclicKey,
    Autokey,
    ProgressiveKey,
}

impl Default for CipherID {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherID {
    pub fn description(&self) -> &'static str {
        match self {
            CipherID::Caesar => "The Caesar Cipher is perhaps the oldest and simplest of ciphers. A value is chosen that shifts each letter of the alphabet that many positions. For example a shift of 2 turna A in C and Y into A.",
            CipherID::Affine => "The Affine Cipher is a simple extension of the Caesar Cipher that applies an affine transform to the alphabet. Each letter's position has a value added to it and then is multiplied by a certain value. The need for a unique inverse to the multiplication adds some complexity to this cipher.",
            CipherID::Decoder => "A Decoder Ring (as popularized by Little Orphan Annie and Captain Midnight) is a variable on the Caesar cipher. Rather than shift the letters each letter replaced with its numerical value which is then shifted.",
            CipherID::Substitution => "The General Substituion Cipher maps a set of symbols one-to-one onto another arbitary set. This implementation allows only maping the symbols of an alphabet but all simple substitution ciphers are included in principle.",
            CipherID::M209 => "The M209 was an entirely mechanical cipher machine used by the US Military with very complex key settings. The positions of the pins and lugs were set once a day. The exteral positions of the rotors were changed with each message.",
            CipherID::Playfair => "The Playfair cipher swaps letters on a grid to encrypt letters pair by pair.",
            CipherID::CyclicKey => "Cyclic key ciphers repeat their keyword over and over",
            CipherID::Autokey => "Autokey ciphers draw their key from the text itself.",
            CipherID::ProgressiveKey => "Progressive key ciphers repeat their key like a cyclic key cipher but apply a shift at each repetition to stretch it out",
        }
    }
}

impl Display for  CipherID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherID::Caesar => "Caesar",
            CipherID::Affine => "Affine",
            CipherID::Decoder => "Decoder Ring",
            CipherID::Substitution => "General Substittution",
            CipherID::M209 => "M209",
            CipherID::Playfair => "Playfair",
            CipherID::CyclicKey => "Cyclic Key",
            CipherID::Autokey => "Autokey",
            CipherID::ProgressiveKey => "Progressive Key",
        };
        write!(f,"{}",name)
    }
}

fn combox_box(ciphers: &[CipherID], identifier: &'static str, active_cipher: &mut CipherID, ui: &mut egui::Ui) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in ciphers {
                ui.selectable_value(active_cipher, *id, format!("{}",id));
            }
        });
    ui.add_space(10.0);
}

pub struct ControlPanel {
    caesar: Caesar,
    affine: Affine,
    decoder_ring: DecoderRing,
    gen_sub: GeneralSubstitution,
    m209: M209,
    cyclic_key: CyclicKey,
    autokey: Autokey,
    progressive_key: ProgressiveKey,
    playfair: Playfair,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self{ 
            caesar: Caesar::default(),
            affine: Affine::default(),
            decoder_ring: DecoderRing::default(),
            gen_sub: GeneralSubstitution::default(),
            m209: M209::default(),
            cyclic_key: CyclicKey::default(),
            autokey: Autokey::default(),
            playfair: Playfair::default(),
            progressive_key: ProgressiveKey::default(),
        }
    }
}

impl ControlPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, input: &mut String, output: &mut String, active_cipher: &mut CipherID) {
        
        ui.horizontal(|ui| {
            combox_box(
                &[CipherID::Caesar, CipherID::Decoder, CipherID::Affine, CipherID::Substitution],
                "Simple Substitution",
                active_cipher,
                ui
            );
    
            combox_box(
                &[CipherID::CyclicKey, CipherID::Autokey, CipherID::ProgressiveKey],
                "Polyalphabetic",
                active_cipher,
                ui
            );
    
            combox_box(
                &[CipherID::M209],
                "Rotor Machine",
                active_cipher,
                ui
            );
    
            combox_box(
                &[CipherID::Playfair],
                "Other",
                active_cipher,
                ui
            );
        });



        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        match active_cipher {
            CipherID::Caesar => self.caesar.ui(ui, input, output),
            CipherID::Affine => self.affine.ui(ui, input, output),
            CipherID::Decoder => self.decoder_ring.ui(ui, input, output),
            CipherID::Substitution => self.gen_sub.ui(ui, input, output),
            CipherID::M209 => self.m209.ui(ui, input, output),
            CipherID::CyclicKey => self.cyclic_key.ui(ui, input, output),
            CipherID::Autokey => self.autokey.ui(ui, input, output),
            CipherID::ProgressiveKey => self.progressive_key.ui(ui, input, output),
            CipherID::Playfair => self.playfair.ui(ui, input, output),
        }
    }
}
