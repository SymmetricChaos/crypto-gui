pub mod alphabet;
pub use alphabet::Alphabet;

pub mod vecstring;
pub use vecstring::VecString;

pub mod preset_alphabet;
pub use preset_alphabet::PresetAlphabet;


pub mod text_functions;
pub use text_functions::{
    shuffled_str,
    random_sample_replace,
    random_char_vec,
    validate_alphabet, // this needs to be changed to work with Alphabet
    rank_str,
    keyed_alphabet,
    dedup_alphabet, // should be unnecessary due to Alphabet
    prep_text, // should be made more flexible
};