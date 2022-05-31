pub mod vecstring;
pub use vecstring::VecString;

// pub mod vecstring_ex;
// pub use vecstring_ex::VecStringExtended;

pub mod preset_alphabet;
pub use preset_alphabet::PresetAlphabet;

pub mod text_functions;
pub use text_functions::{
    dedup_alphabet,
    keyed_alphabet,
    prep_text, // should be made more flexible
    random_char_vec,
    random_sample_replace,
    rank_str,
    shuffled_str,
    validate_alphabet,
};
