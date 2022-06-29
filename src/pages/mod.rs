pub mod text_prep_page;
pub use text_prep_page::TextPrepPage;

pub mod category_page;
pub use category_page::CipherCategory;


#[derive(Debug, PartialEq, Eq)]
pub enum Page {
    About,
    Cipher,
    Code,
    CipherCategory,
    TextPrep,
}