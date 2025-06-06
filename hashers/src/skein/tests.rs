use super::{skein1024::Skein1024, skein256::Skein256, skein512::Skein512};
use crate::traits::StatefulHasher;

const LONG: &[u8] = b"a very long input intended to force the update method to be invoked at least once for all variants of the Skein hash function family";
const HELLO: &[u8] = b"hello";

crate::stateful_hash_tests!(
    skein256_variants;
    test_256_256_empty, Skein256::init_256(), b"",
    "c8877087da56e072870daa843f176e9453115929094c3a40c463a196c29bf7ba";
    test_256_256_hello, Skein256::init_256(), HELLO,
    "8b467f67dd324c9c9fe9aff562ee0e3746d88abcb2879e4e1b4fbd06a5061f89";
    test_256_128_hello, Skein256::init_128(), HELLO,
    "225ab2deb375c40d320f5ea1379e87e9";
    test_256_256_long, Skein256::init_256(), LONG,
    "a58c74b4608f0d061de7407a8b0362dcfa0161e1d86e37c6c4b608799ca572c1";
    test_256_256_ff, Skein256::init_256(), &[0xff],
    "0b98dcd198ea0e50a7a244c444e25c23da30c10fc9a1f270a6637f1f34e67ed2"
);

crate::stateful_hash_tests!(
    skein512_variants;
    test_512_512_empty, Skein512::init_512(), b"",
    "bc5b4c50925519c290cc634277ae3d6257212395cba733bbad37a4af0fa06af41fca7903d06564fea7a2d3730dbdb80c1f85562dfcc070334ea4d1d9e72cba7a";
    test_512_512_hello, Skein512::init_512(), HELLO,
    "178ba59a793145d36f78ae2742ecbd967825f4c1e228c732340d00f8d08e221714e19cf70be2764aa7bed2277e80328cce01e105c739f96fe3be11f71652545d";
    test_512_512_long, Skein512::init_512(), LONG,
    "46eb60bac1c38da676abf761908e8d90853b86d60e9f5a3e744e606639763392ce90eca08f6142752d870c03fdc8ea8a50d9cf6976588f243c6e231f9bd3eb92";
    test_512_512_ff, Skein512::init_512(), &[0xff],
    "71b7bce6fe6452227b9ced6014249e5bf9a9754c3ad618ccc4e0aae16b316cc8ca698d864307ed3e80b6ef1570812ac5272dc409b5a012df2a579102f340617a"

);

crate::stateful_hash_tests!(
    skein1024_variants;
    test_1024_512_hello, Skein1024::init_512(), HELLO,
    "0279151f5b402203b143071ababa334ecf2dd4e5b621118e24d644a11d1e803614e70c05ad0092a7bfd39663fd0642b69a910b40b37f000a0bf2d1be803b664f";
    test_1024_512_long, Skein1024::init_512(), LONG,
    "a02c38add8144687eebbb563dba51a93d0787e33540aa7a1201005171edcc8c5f287afd89324896d294d92b7242342c9895a3d0e331f944fa3d67c5dab3a674a";
    test_1024_1024_ff, Skein1024::init_1024(), &[0xff],
    "e62c05802ea0152407cdd8787fda9e35703de862a4fbc119cff8590afe79250bccc8b3faf1bd2422ab5c0d263fb2f8afb3f796f048000381531b6f00d85161bc0fff4bef2486b1ebcd3773fabf50ad4ad5639af9040e3f29c6c931301bf79832e9da09857e831e82ef8b4691c235656515d437d2bda33bcec001c67ffde15ba8";
);
