pub mod text_prep_page;
pub use text_prep_page::TextPrepPage;

pub mod category_page;
pub use category_page::CipherCategoryPage;


#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    About,
    Cipher,
    Code,
    CipherCategory,
    TextPrep,
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