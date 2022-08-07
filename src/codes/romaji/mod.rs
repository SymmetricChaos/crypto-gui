// The methods used here are all copied: https://github.com/PSeitz/wana_kana_rust
// Just wanted to be able to cover additional romanizations

pub mod kunrei;
pub use kunrei::KUNREI_SHIKI;

pub mod hepbern;
pub use hepbern::HEPBERN_SHIKI;

pub mod nihon;
pub use nihon::NIHON_SHIKI;

pub mod romaji_to_kana;
pub use romaji_to_kana::ROMAJI_TO_KANA;

pub mod romaji;
