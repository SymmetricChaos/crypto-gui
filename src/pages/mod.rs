pub mod text_prep_page;
pub use text_prep_page::TextPrepPage;

pub mod category_page;
pub use category_page::CipherCategoryPage;

pub mod rng_pages;
pub use rng_pages::RngInfoPage;

#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    About,
    Cipher,
    Code,
    CipherCategory,
    TextPrep,
    Rng,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CipherCategory {
    Substituion,
    Polyalphabetic,
    RotorMachine,
    Transposition,
    Playfair,
    Tactical,
    Polybius,
}

impl Default for CipherCategory {
    fn default() -> Self {
        Self::Substituion
    }
}
