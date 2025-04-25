pub mod round_functions;
use crypto_bigint::{U1024, U256, U512};

const C240: u64 = 0x1BD11BDAA9FC1A22;

macro_rules! threefish {
    ($name: ident, $block_words: literal, $block_bytes: literal, $rounds: literal, $iv: ty, $round_func: expr, $round_func_inv: expr) => {
        pub struct $name {
            pub input_format: utils::byte_formatting::ByteFormat,
            pub output_format: utils::byte_formatting::ByteFormat,
            pub iv: $iv,
            pub subkeys: [[u64; $block_words]; ($rounds / 4 + 1)],
            pub mode: crate::digital::block_ciphers::block_cipher::BCMode,
            pub padding: crate::digital::block_ciphers::block_cipher::BCPadding,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    input_format: utils::byte_formatting::ByteFormat::Hex,
                    output_format: utils::byte_formatting::ByteFormat::Hex,
                    iv: <$iv>::ZERO,
                    subkeys: [[0; $block_words]; ($rounds / 4 + 1)],
                    mode: Default::default(),
                    padding: Default::default(),
                }
            }
        }

        impl $name {
            pub fn create_subkeys(
                key: &[u8; $block_bytes],
                tweak: &[u8; 16],
            ) -> [[u64; $block_words]; ($rounds / 4 + 1)] {
                let mut ex_tweak = [0_u64; 3];
                utils::byte_formatting::fill_u64s_le(&mut ex_tweak[0..2], tweak);
                ex_tweak[2] = ex_tweak[0] ^ ex_tweak[1];
                let mut ex_key = [0_u64; $block_words + 1];
                utils::byte_formatting::fill_u64s_le(&mut ex_key[0..$block_words], key);
                ex_key[$block_words] = ex_key.iter().fold(C240, core::ops::BitXor::bitxor);

                let mut subkeys = [[0u64; $block_words]; ($rounds / 4 + 1)];

                // The inner loop allows this to be reused for other key sizes
                for k in 0..($rounds / 4 + 1) {
                    for i in 0..$block_words {
                        subkeys[k][i] = ex_key[(k + i) % ($block_words + 1)];
                        if i == $block_words - 3 {
                            subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[k % 3]);
                        } else if i == $block_words - 2 {
                            subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[(k + 1) % 3]);
                        } else if i == $block_words - 1 {
                            subkeys[k][i] = subkeys[k][i].wrapping_add(k as u64);
                        }
                    }
                }
                subkeys
            }

            pub fn with_key_and_tweak(key: &[u8; $block_bytes], tweak: &[u8; 16]) -> Self {
                Self {
                    input_format: utils::byte_formatting::ByteFormat::Hex,
                    output_format: utils::byte_formatting::ByteFormat::Hex,
                    iv: <$iv>::ZERO,
                    subkeys: Self::create_subkeys(key, tweak),
                    mode: Default::default(),
                    padding: Default::default(),
                }
            }
        }

        crate::block_cipher_builders!($name, $iv);

        impl crate::digital::block_ciphers::block_cipher::BlockCipher<$block_bytes> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                let mut block: [u64; $block_words] = utils::byte_formatting::make_u64s_le(bytes);

                for r in 0..($rounds / 8) {
                    $round_func(&mut block, &self.subkeys[(2 * r)..][..2]);
                }

                for i in 0..$block_words {
                    block[i] = block[i].wrapping_add(self.subkeys[($rounds / 4 + 1) - 1][i])
                }

                utils::byte_formatting::u64s_to_bytes_le(bytes, &block.map(|n| n));
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut block: [u64; $block_words] = utils::byte_formatting::make_u64s_le(bytes);

                for i in 0..$block_words {
                    block[i] = block[i].wrapping_sub(self.subkeys[($rounds / 4 + 1) - 1][i])
                }

                for r in (0..($rounds / 8)).rev() {
                    $round_func_inv(&mut block, &self.subkeys[(2 * r)..][..2]);
                }

                utils::byte_formatting::u64s_to_bytes_le(bytes, &block.map(|n| n));
            }
        }

        crate::impl_cipher_for_block_cipher!($name, $block_bytes);
    };
}

use round_functions::{
    octo_round_1024, octo_round_1024_inv, octo_round_256, octo_round_256_inv, octo_round_512,
    octo_round_512_inv,
};

threefish!(
    Threefish256,
    4,
    32,
    72,
    U256,
    octo_round_256,
    octo_round_256_inv
);
threefish!(
    Threefish512,
    8,
    64,
    72,
    U512,
    octo_round_512,
    octo_round_512_inv
);
threefish!(
    Threefish1024,
    16,
    128,
    80,
    U1024,
    octo_round_1024,
    octo_round_1024_inv
);

crate::test_block_cipher!(
    test_256, Threefish256::with_key_and_tweak(
        &hex_literal::hex!("1011121314151617 18191A1B1C1D1E1F 2021222324252627 28292A2B2C2D2E2F"),
        &hex_literal::hex!("0001020304050607 08090A0B0C0D0E0F")
    ),
    hex_literal::hex!("FFFEFDFCFBFAF9F8 F7F6F5F4F3F2F1F0 EFEEEDECEBEAE9E8 E7E6E5E4E3E2E1E0"),
    hex_literal::hex!("E0D091FF0EEA8FDF C98192E62ED80AD5 9D865D08588DF476 657056B5955E97DF");

    test_512, Threefish512::with_key_and_tweak(
        &hex_literal::hex!("1011121314151617 18191A1B1C1D1E1F 2021222324252627 28292A2B2C2D2E2F 3031323334353637 38393A3B3C3D3E3F 4041424344454647 48494A4B4C4D4E4F"),
        &hex_literal::hex!("0001020304050607 08090A0B0C0D0E0F")
    ),
    hex_literal::hex!("FFFEFDFCFBFAF9F8 F7F6F5F4F3F2F1F0 EFEEEDECEBEAE9E8 E7E6E5E4E3E2E1E0 DFDEDDDCDBDAD9D8 D7D6D5D4D3D2D1D0 CFCECDCCCBCAC9C8 C7C6C5C4C3C2C1C0"),
    hex_literal::hex!("E304439626D45A2C B401CAD8D636249A 6338330EB06D45DD 8B36B90E97254779 272A0A8D99463504 784420EA18C9A725 AF11DFFEA1016234 8927673D5C1CAF3D");

    test_1024, Threefish1024::with_key_and_tweak(
        &hex_literal::hex!("1011121314151617 18191A1B1C1D1E1F 2021222324252627 28292A2B2C2D2E2F 3031323334353637 38393A3B3C3D3E3F 4041424344454647 48494A4B4C4D4E4F 5051525354555657 58595A5B5C5D5E5F 6061626364656667 68696A6B6C6D6E6F 7071727374757677 78797A7B7C7D7E7F 8081828384858687 88898A8B8C8D8E8F"),
        &hex_literal::hex!("0001020304050607 08090A0B0C0D0E0F")
    ),
    hex_literal::hex!("FFFEFDFCFBFAF9F8 F7F6F5F4F3F2F1F0 EFEEEDECEBEAE9E8 E7E6E5E4E3E2E1E0 DFDEDDDCDBDAD9D8 D7D6D5D4D3D2D1D0 CFCECDCCCBCAC9C8 C7C6C5C4C3C2C1C0 BFBEBDBCBBBAB9B8 B7B6B5B4B3B2B1B0 AFAEADACABAAA9A8 A7A6A5A4A3A2A1A0 9F9E9D9C9B9A9998 9796959493929190 8F8E8D8C8B8A8988 8786858483828180"),
    hex_literal::hex!(" A6654DDBD73CC3B0 5DD777105AA849BC E49372EAAFFC5568 D254771BAB85531C 94F780E7FFAAE430 D5D8AF8C70EEBBE1 760F3B42B737A89C B363490D670314BD 8AA41EE63C2E1F45 FBD477922F8360B3 88D6125EA6C7AF0A D7056D01796E90C8 3313F4150A5716B3 0ED5F569288AE974 CE2B4347926FCE57 DE44512177DD7CDE");
);
