pub mod text_prep_page;
pub use text_prep_page::TextPrepPage;

pub mod io_panel;

// pub mod cipher_category_page;

#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    About,
    Cipher,
    Code,
    TextPrep,
}
