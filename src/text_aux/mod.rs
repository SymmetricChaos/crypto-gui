pub mod alphabet;
pub use alphabet::Alphabet;

pub mod vecstring;
pub use vecstring::VecString;

pub mod preset_alphabet;
pub use preset_alphabet::PresetAlphabet;

pub mod text_functions;
pub use text_functions::{
    dedup_alphabet, // should be unnecessary due to Alphabet
    keyed_alphabet,
    prep_text, // should be made more flexible
    random_char_vec,
    random_sample_replace,
    rank_str,
    shuffled_str,
    validate_alphabet, // this needs to be changed to work with Alphabet
};
