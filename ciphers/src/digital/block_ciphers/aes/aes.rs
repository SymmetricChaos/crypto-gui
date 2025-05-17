use super::{
    aes_functions::{
        add_round_key, inv_mix_columns, inv_shift_rows, inv_sub_bytes, mix_columns, rot_word,
        shift_rows, sub_bytes, sub_key_slice_to_bytes, transpose_state,
    },
    sbox::sub_word,
};
use crate::{
    aes_methods,
    digital::block_ciphers::block_cipher::{BCMode, BCPadding, BlockCipher},
    impl_cipher_for_block_cipher,
};
use utils::byte_formatting::ByteFormat;

// The three NIST versions of AES vary only in their key schedule so this is all
// wrapped up in macros for conciseness. For the same reason only AES128 is tested
// for encryption below, the rest just check their round keys.
aes_methods!(Aes128, 4, 10);
impl_cipher_for_block_cipher!(Aes128, 16);

aes_methods!(Aes192, 6, 12);
impl_cipher_for_block_cipher!(Aes192, 16);

aes_methods!(Aes256, 8, 14);
impl_cipher_for_block_cipher!(Aes256, 16);

#[cfg(test)]
mod aes128_tests {

    use utils::byte_formatting::make_u32s_be;

    use crate::Cipher;

    use super::*;

    #[test]
    fn test_shift_rows() {
        let mut state = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let output = [0, 1, 2, 3, 5, 6, 7, 4, 10, 11, 8, 9, 15, 12, 13, 14];
        // print_aes_state(&state);
        shift_rows(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, output)
    }

    #[test]
    fn test_mix_cols() {
        let mut state = [
            0xdb, 0x01, 0xf2, 0xd4, 0x13, 0x01, 0xa, 0xd4, 0x53, 0x01, 0x22, 0xd4, 0x45, 0x01,
            0x5c, 0xd5,
        ];
        let original_state = state.clone();
        let output = [
            0x8e, 0x01, 0x9f, 0xd5, 0x4d, 0x01, 0xdc, 0xd5, 0xa1, 0x01, 0x58, 0xd7, 0xbc, 0x01,
            0x9d, 0xd6,
        ];
        // print_aes_state(&state);
        mix_columns(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, output);
        inv_mix_columns(&mut state);
        // print_aes_state(&state);
        assert_eq!(state, original_state);
    }

    #[test]
    fn test_key_schedule_128_1() {
        let mut cipher = Aes128::default();

        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        cipher.ksa_u32([0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c]);
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c],
            [0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605],
            [0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f],
        ];
        let sub_keys = cipher.round_keys;

        let sub_key_word: [[u32; 4]; 3] = [
            make_u32s_be(&sub_keys[0]),
            make_u32s_be(&sub_keys[1]),
            make_u32s_be(&sub_keys[2]),
        ];

        assert_eq!(test_sub_keys[0], sub_key_word[0]);
        assert_eq!(test_sub_keys[1], sub_key_word[1]);
        assert_eq!(test_sub_keys[2], sub_key_word[2]);
    }

    #[test]
    fn test_key_schedule_128_2() {
        let mut cipher = Aes128::default();

        // https://github.com/kaapomoi/key-expander/blob/master/src/lib.rs
        cipher.ksa_u32([0x0, 0x0, 0x0, 0x1]);
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x0, 0x0, 0x0, 0x1],
            [0x62637c63, 0x62637c63, 0x62637c63, 0x62637c62],
            [0x9b73d6c9, 0xf910aaaa, 0x9b73d6c9, 0xf910aaab],
        ];
        let sub_keys = cipher.round_keys;

        let sub_key_word: [[u32; 4]; 3] = [
            make_u32s_be(&sub_keys[0]),
            make_u32s_be(&sub_keys[1]),
            make_u32s_be(&sub_keys[2]),
        ];

        assert_eq!(test_sub_keys[0], sub_key_word[0]);
        assert_eq!(test_sub_keys[1], sub_key_word[1]);
        assert_eq!(test_sub_keys[2], sub_key_word[2]);
    }

    #[test]
    fn test_key_schedule_192() {
        let mut cipher = Aes192::default();

        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        cipher.ksa_u32([
            0x8e73b0f7, 0xda0e6452, 0xc810f32b, 0x809079e5, 0x62f8ead2, 0x522c6b7b,
        ]);
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x8e73b0f7, 0xda0e6452, 0xc810f32b, 0x809079e5],
            [0x62f8ead2, 0x522c6b7b, 0xfe0c91f7, 0x2402f5a5],
            [0xec12068e, 0x6c827f6b, 0x0e7a95b9, 0x5c56fec2],
        ];
        let sub_keys = cipher.round_keys;

        let sub_key_word: [[u32; 4]; 3] = [
            make_u32s_be(&sub_keys[0]),
            make_u32s_be(&sub_keys[1]),
            make_u32s_be(&sub_keys[2]),
        ];

        assert_eq!(test_sub_keys[0], sub_key_word[0]);
        assert_eq!(test_sub_keys[1], sub_key_word[1]);
        assert_eq!(test_sub_keys[2], sub_key_word[2]);
    }

    #[test]
    fn test_key_schedule_256() {
        let mut cipher = Aes256::default();

        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        cipher.ksa_u32([
            0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781, 0x1f352c07, 0x3b6108d7, 0x2d9810a3,
            0x0914dff4,
        ]);
        let test_sub_keys: [[u32; 4]; 3] = [
            [0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781],
            [0x1f352c07, 0x3b6108d7, 0x2d9810a3, 0x0914dff4],
            [0x9ba35411, 0x8e6925af, 0xa51a8b5f, 0x2067fcde],
        ];
        let sub_keys = cipher.round_keys;

        let sub_key_word: [[u32; 4]; 3] = [
            make_u32s_be(&sub_keys[0]),
            make_u32s_be(&sub_keys[1]),
            make_u32s_be(&sub_keys[2]),
        ];

        assert_eq!(test_sub_keys[0], sub_key_word[0]);
        assert_eq!(test_sub_keys[1], sub_key_word[1]);
        assert_eq!(test_sub_keys[2], sub_key_word[2]);
    }

    #[test]
    fn test_encypt_decrypt_ctr() {
        let mut cipher = Aes128::default();
        cipher.mode = BCMode::Ctr;

        cipher.input_format = ByteFormat::Utf8;
        let ptext = "The quick brown fox.";
        let ctext = cipher.encrypt(ptext).unwrap();
        cipher.input_format = ByteFormat::Hex;
        cipher.output_format = ByteFormat::Utf8;
        let decrypt = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, decrypt);
    }

    #[test]
    fn test_encypt_decrypt_ecb() {
        let mut cipher = Aes128::default();
        cipher.mode = BCMode::Ecb;
        cipher.padding = BCPadding::Bit;

        cipher.input_format = ByteFormat::Utf8;
        cipher.output_format = ByteFormat::Hex;
        let ptext = "The quick brown fox.";
        let ctext = cipher.encrypt(ptext).unwrap();
        cipher.input_format = ByteFormat::Hex;
        cipher.output_format = ByteFormat::Utf8;
        let decrypt = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, decrypt);
    }

    #[test]
    fn test_encypt() {
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        let cipher =
            Aes128::default().with_key_u32([0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c]);
        let mut state = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        cipher.encrypt_block(&mut state);
        transpose_state(&mut state);
        assert_eq!(
            [
                0x39, 0x02, 0xdc, 0x19, 0x25, 0xdc, 0x11, 0x6a, 0x84, 0x09, 0x85, 0x0b, 0x1d, 0xfb,
                0x97, 0x32
            ],
            state
        );
    }

    #[test]
    fn test_encypt_key_from_bytes() {
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
        let cipher = Aes128::default().with_key([
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ]);
        let mut state = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        cipher.encrypt_block(&mut state);
        transpose_state(&mut state);
        assert_eq!(
            [
                0x39, 0x02, 0xdc, 0x19, 0x25, 0xdc, 0x11, 0x6a, 0x84, 0x09, 0x85, 0x0b, 0x1d, 0xfb,
                0x97, 0x32
            ],
            state
        );
    }

    #[test]
    fn test_ctr_mode() {
        let mut ptext = ByteFormat::Hex
            .text_to_bytes("6bc1bee22e409f96e93d7e117393172a")
            .unwrap();
        let ctext = ByteFormat::Hex
            .text_to_bytes("874d6191b620e3261bef6864990db6ce")
            .unwrap();
        let mut cipher = Aes128::default();
        cipher.iv = 0xf0f1f2f3f4f5f6f7f8f9fafbfcfdfeff;
        cipher.ksa_u32([0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c]);
        cipher.encrypt_ctr(&mut ptext, cipher.iv.to_be_bytes());
        assert_eq!(ctext, ptext);
    }
}
