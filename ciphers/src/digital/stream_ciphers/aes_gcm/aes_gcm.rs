use super::{
    aes_functions::{
        add_round_key, mix_columns, rot_word, shift_rows, sub_bytes, sub_key_slice_to_bytes,
        transpose_state,
    },
    sbox::sub_word,
};
use utils::byte_formatting::ByteFormat;

crate::aes_gcm_methods!(AesGcm128, 4, 10);

crate::aes_gcm_methods!(AesGcm192, 6, 12);

crate::aes_gcm_methods!(AesGcm256, 8, 14);
