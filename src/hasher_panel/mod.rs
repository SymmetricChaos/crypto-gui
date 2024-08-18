use egui::Ui;
use hashers::{
    errors::HasherError,
    ids::{hasher_categories::HasherCategory, HasherId},
};

mod blake2_controls;
mod blake3_controls;
mod blake_controls;
mod fnv_controls;
mod hmac_controls;
mod lm_controls;
mod md2_controls;
mod md4_controls;
mod md5_controls;
mod mgf1_controls;
mod pbkdf2_controls;
mod pearson_controls;
mod poly1305_controls;
mod radio_gatun;
mod sha0_controls;
mod sha1_controls;
mod sha2_controls;
mod sha3_controls;
mod siphash_controls;

pub trait HasherFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn hash_string(&self, text: &str) -> Result<String, HasherError>;
}

#[macro_export]
macro_rules! hash_string {
    () => {
        fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
            hashers::traits::ClassicHasher::hash_bytes_from_string(&self.hasher, text)
        }
    };
}

// Quick simple combo box builder
fn combox_box(
    hasher: &[HasherId],
    active_hasher: &mut Option<HasherId>,
    hasher_category: HasherCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(hasher_category.to_string())
            .selected_text(hasher_category.to_string())
            .show_ui(ui, |ui| {
                for id in hasher {
                    ui.selectable_value(active_hasher, Some(*id), id.to_string());
                }
            });
        ui.menu_button("+", |ui| ui.label(hasher_category.description()))
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct HasherInterface {
    blake: blake_controls::BlakeFrame,
    blake2: blake2_controls::Blake2Frame,
    blake3: blake3_controls::Blake3Frame,
    fnv: fnv_controls::FnvFrame,
    hmac: hmac_controls::HmacFrame,
    lm: lm_controls::LmFrame,
    md2: md2_controls::Md2Frame,
    md4: md4_controls::Md4Frame,
    md5: md5_controls::Md5Frame,
    mgf1: mgf1_controls::Mgf1Frame,
    pbkdf2: pbkdf2_controls::Pbkdf2Frame,
    pearson: pearson_controls::PearsonFrame,
    poly1305: poly1305_controls::Poly1305Frame,
    siphash: siphash_controls::SipHashFrame,
    sha0: sha0_controls::Sha0Frame,
    sha1: sha1_controls::Sha1Frame,
    sha2: sha2_controls::Sha2Frame,
    sha3: sha3_controls::Sha3Frame,
}

impl HasherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_hasher: &mut Option<HasherId>) {
        combox_box(
            &[
                HasherId::Blake,
                HasherId::Blake2,
                HasherId::Blake3,
                HasherId::Hmac,
                HasherId::Md2,
                HasherId::Md4,
                HasherId::Md5,
                HasherId::Mgf1,
                HasherId::Pbkdf2,
                HasherId::Poly1305,
                HasherId::Sha0,
                HasherId::Sha1,
                HasherId::Sha2,
                HasherId::Sha3,
            ],
            active_hasher,
            HasherCategory::Cryptographic,
            ui,
        );

        combox_box(
            &[
                HasherId::Fnv,
                HasherId::Lm,
                HasherId::Pearson,
                HasherId::SipHash,
            ],
            active_hasher,
            HasherCategory::NonCryptographic,
            ui,
        );
    }

    pub fn get_active_hasher(&mut self, active_hasher: &HasherId) -> &mut dyn HasherFrame {
        match active_hasher {
            HasherId::Argon2 => todo!(),
            HasherId::Blake => &mut self.blake,
            HasherId::Blake2 => &mut self.blake2,
            HasherId::Blake3 => &mut self.blake3,
            HasherId::Fnv => &mut self.fnv,
            HasherId::Hmac => &mut self.hmac,
            HasherId::Lm => &mut self.lm,
            HasherId::Md2 => &mut self.md2,
            HasherId::Md4 => &mut self.md4,
            HasherId::Md5 => &mut self.md5,
            HasherId::Md6 => todo!(),
            HasherId::Mgf1 => &mut self.mgf1,
            HasherId::Sha0 => &mut self.sha0,
            HasherId::Sha1 => &mut self.sha1,
            HasherId::Sha2 => &mut self.sha2,
            HasherId::Sha3 => &mut self.sha3,
            HasherId::Pbkdf2 => &mut self.pbkdf2,
            HasherId::Pearson => &mut self.pearson,
            HasherId::Poly1305 => &mut self.poly1305,
            HasherId::SipHash => &mut self.siphash,
            _ => todo!("<<<NOT IMPLEMENTED>>>"),
        }
    }
}
