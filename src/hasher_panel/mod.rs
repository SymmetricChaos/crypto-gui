use egui::Ui;
use hashers::{
    errors::HasherError,
    ids::{hasher_categories::HasherCategory, HasherId},
};

mod adler32_controls;
mod ascon_controls;
mod blake2_controls;
mod blake3_controls;
mod blake_controls;
mod city_hash_controls;
mod fletcher_controls;
mod fnv_controls;
mod fxhash_controls;
mod ghash_controls;
mod hkdf_controls;
mod hmac_controls;
mod lm_controls;
mod md2_controls;
mod md4_controls;
mod md5_controls;
mod mgf1_controls;
mod murmur3_controls;
mod one_at_a_time_controls;
mod pbkdf1_controls;
mod pbkdf2_controls;
mod pearson_controls;
mod poly1305_controls;
mod radio_gatun;
mod ripemd_controls;
mod scrypt_controls;
mod sha0_controls;
mod sha1_controls;
mod sha2_controls;
mod sha3_controls;
mod shabal_controls;
mod siphash_controls;
mod sm3_controls;
mod snefru_controls;
mod tiger_controls;

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
        egui::ComboBox::from_id_salt(hasher_category.to_string())
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
    adler32: adler32_controls::Adler32Frame,
    ascon: ascon_controls::AsconFrame,
    blake: blake_controls::BlakeFrame,
    blake2: blake2_controls::Blake2Frame,
    // blake3: blake3_controls::Blake3Frame,
    cityhash: city_hash_controls::CityHashFrame,
    fletcher: fletcher_controls::FletcherFrame,
    fnv: fnv_controls::FnvFrame,
    fxhash: fxhash_controls::FxHashFrame,
    ghash: ghash_controls::GhashFrame,
    hkdf: hkdf_controls::HkdfFrame,
    hmac: hmac_controls::HmacFrame,
    lm: lm_controls::LmFrame,
    md2: md2_controls::Md2Frame,
    md4: md4_controls::Md4Frame,
    md5: md5_controls::Md5Frame,
    mgf1: mgf1_controls::Mgf1Frame,
    murmur3: murmur3_controls::Murmur3Frame,
    one_at_a_time: one_at_a_time_controls::OaatFrame,
    pbkdf1: pbkdf1_controls::Pbkdf1Frame,
    pbkdf2: pbkdf2_controls::Pbkdf2Frame,
    pearson: pearson_controls::PearsonFrame,
    poly1305: poly1305_controls::Poly1305Frame,
    radogatun: radio_gatun::RadioGatunFrame,
    ripe_md: ripemd_controls::RipeMdFrame,
    siphash: siphash_controls::SipHashFrame,
    sha0: sha0_controls::Sha0Frame,
    sha1: sha1_controls::Sha1Frame,
    sha2: sha2_controls::Sha2Frame,
    sha3: sha3_controls::Sha3Frame,
    shabal: shabal_controls::ShabalFrame,
    sm3: sm3_controls::Sm3Frame,
    snefru: snefru_controls::SnefruFrame,
    tiger: tiger_controls::TigerFrame,
}

impl HasherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_hasher: &mut Option<HasherId>) {
        combox_box(
            &[
                HasherId::Ascon,
                HasherId::Blake,
                HasherId::Blake2,
                // HasherId::Blake3,
                HasherId::Ghash,
                HasherId::Hkdf,
                HasherId::Hmac,
                HasherId::Md2,
                HasherId::Md4,
                HasherId::Md5,
                HasherId::Mgf1,
                HasherId::Pbkdf1,
                HasherId::Pbkdf2,
                HasherId::Poly1305,
                HasherId::RadioGatun,
                HasherId::RipeMd,
                HasherId::Sha0,
                HasherId::Sha1,
                HasherId::Sha2,
                HasherId::Sha3,
                HasherId::Shabal,
                HasherId::Sm3,
                HasherId::Snefru,
                HasherId::Tiger,
            ],
            active_hasher,
            HasherCategory::Cryptographic,
            ui,
        );

        combox_box(
            &[
                HasherId::Adler32,
                HasherId::CityHash,
                HasherId::Fletcher,
                HasherId::Fnv,
                HasherId::FxHash,
                HasherId::Lm,
                HasherId::MurmurHash3,
                HasherId::OneAtATime,
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
            HasherId::Adler32 => &mut self.adler32,
            HasherId::Ascon => &mut self.ascon,
            HasherId::Blake => &mut self.blake,
            HasherId::Blake2 => &mut self.blake2,
            // HasherId::Blake3 => &mut self.blake3,
            HasherId::CityHash => &mut self.cityhash,
            HasherId::Ghash => &mut self.ghash,
            HasherId::Fletcher => &mut self.fletcher,
            HasherId::Fnv => &mut self.fnv,
            HasherId::FxHash => &mut self.fxhash,
            HasherId::Hkdf => &mut self.hkdf,
            HasherId::Hmac => &mut self.hmac,
            HasherId::Lm => &mut self.lm,
            HasherId::Md2 => &mut self.md2,
            HasherId::Md4 => &mut self.md4,
            HasherId::Md5 => &mut self.md5,
            HasherId::Mgf1 => &mut self.mgf1,
            HasherId::MurmurHash3 => &mut self.murmur3,
            HasherId::OneAtATime => &mut self.one_at_a_time,
            HasherId::Pbkdf1 => &mut self.pbkdf1,
            HasherId::Pbkdf2 => &mut self.pbkdf2,
            HasherId::Pearson => &mut self.pearson,
            HasherId::Poly1305 => &mut self.poly1305,
            HasherId::RadioGatun => &mut self.radogatun,
            HasherId::RipeMd => &mut self.ripe_md,
            HasherId::Sha0 => &mut self.sha0,
            HasherId::Sha1 => &mut self.sha1,
            HasherId::Sha2 => &mut self.sha2,
            HasherId::Sha3 => &mut self.sha3,
            HasherId::Shabal => &mut self.shabal,
            HasherId::SipHash => &mut self.siphash,
            HasherId::Sm3 => &mut self.sm3,
            HasherId::Snefru => &mut self.snefru,
            HasherId::Tiger => &mut self.tiger,
            _ => todo!("<<<NOT IMPLEMENTED>>>"),
        }
    }
}
