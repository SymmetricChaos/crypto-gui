pub mod binary_to_text;
pub mod braille;
pub mod commercial;
pub mod ecc;
pub mod mathematical;
pub mod other;
pub mod romaji;
pub mod text_standards;

pub mod compression;
pub mod ids;
pub mod traits;

#[macro_export]
macro_rules! lazy_regex {
    ($($name: ident, $regex: literal);+ $(;)?) => {
        $(
        pub const $name: std::cell::LazyCell<regex::Regex> =
            std::cell::LazyCell::new(|| regex::Regex::new($regex).unwrap());
        )+
    };
}

#[macro_export]
macro_rules! lazy_bimap {
    ($($name: ident : $type: ty = $iter: expr);+ $(;)?) => {
        $(
        pub static $name: std::sync::LazyLock<$type> =
            std::sync::LazyLock::new(|| utils::text_functions::bimap_from_iter($iter));
        )+
    };
}
