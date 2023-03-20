pub mod code_category_page;
pub mod text_prep_page;
pub use text_prep_page::TextPrepPage;

pub mod io_panel;

pub mod cipher_category_page;

// pub mod rng_page;
// pub use rng_page::RngInfoPage;

#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    About,
    Cipher,
    Code,
    Rng,
    CipherCategory,
    TextPrep,
}
