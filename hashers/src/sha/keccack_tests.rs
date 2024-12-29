#[cfg(test)]
mod sha2_tests {
    use crate::sha::sha3::Keccack;
    use crate::traits::StatefulHasher;

    const EMPTY: &[u8; 0] = &[];
    const ABC: &[u8; 3] = &[0x61, 0x62, 0x63];
    const A_1MIL: &[u8; 1_000_000] = &[0x61; 1_000_000];
    const LETTERS_112: &[u8; 112] = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    const DATA_4: &[u8; 4] = &[0, 1, 2, 3];
    const DATA_200: &[u8; 200] = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
        0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86,
        0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95,
        0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4,
        0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF, 0xB0, 0xB1, 0xB2, 0xB3,
        0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF, 0xC0, 0xC1, 0xC2,
        0xC3, 0xC4, 0xC5, 0xC6, 0xC7,
    ];
    const KEY_32: &[u8; 32] = &[
        0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E,
        0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D,
        0x5E, 0x5F,
    ];

    crate::stateful_hash_tests!(
        empty_sha3_224, Keccack::sha3_224(), EMPTY,
        "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7";
        abc_sha3_224, Keccack::sha3_224(), ABC,
        "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf";
        long_sha3_224, Keccack::sha3_224(), LETTERS_112,
        "543e6868e1666c1a643630df77367ae5a62a85070a51c14cbf665cbc";
        very_long_sha3_224, Keccack::sha3_224(), A_1MIL,
        "d69335b93325192e516a912e6d19a15cb51c6ed5c15243e7a7fd653c";

        empty_sha3_256, Keccack::sha3_256(), EMPTY,
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a";
        abc_sha3_256, Keccack::sha3_256(), ABC,
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532";
        long_sha3_256, Keccack::sha3_256(), LETTERS_112,
        "916f6061fe879741ca6469b43971dfdb28b1a32dc36cb3254e812be27aad1d18";
        very_long_sha3_256, Keccack::sha3_256(), A_1MIL,
        "5c8875ae474a3634ba4fd55ec85bffd661f32aca75c6d699d0cdcb6c115891c1";
        sha3_256_1600_bits, Keccack::sha3_256(), &utils::byte_formatting::ByteFormat::Hex.text_to_bytes("a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3").unwrap(),
        "79f38adec5c20307a98ef76e8324afbfd46cfd81b22e3973c65fa1bd9de31787";
        sha3_256_2008_bits, Keccack::sha3_256(), &utils::byte_formatting::ByteFormat::Hex.text_to_bytes("83af34279ccb5430febec07a81950d30f4b66f484826afee7456f0071a51e1bbc55570b5cc7ec6f9309c17bf5befdd7c6ba6e968cf218a2b34bd5cf927ab846e38a40bbd81759e9e33381016a755f699df35d660007b5eadf292feefb735207ebf70b5bd17834f7bfa0e16cb219ad4af524ab1ea37334aa66435e5d397fc0a065c411ebbce32c240b90476d307ce802ec82c1c49bc1bec48c0675ec2a6c6f3ed3e5b741d13437095707c565e10d8a20b8c20468ff9514fcf31b4249cd82dcee58c0a2af538b291a87e3390d737191a07484a5d3f3fb8c8f15ce056e5e5f8febe5e1fb59d6740980aa06ca8a0c20f5712b4cde5d032e92ab89f0ae1").unwrap(),
        "3298a95cfe59b9d6cab99c36dc1324194c09f97f08944a02d9574bbca3186b41";

        empty_sha3_384, Keccack::sha3_384(), EMPTY,
        "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004";
        long_sha3_384, Keccack::sha3_384(), LETTERS_112,
        "79407d3b5916b59c3e30b09822974791c313fb9ecc849e406f23592d04f625dc8c709b98b43b3852b337216179aa7fc7";
        very_long_sha3_384, Keccack::sha3_384(), A_1MIL,
        "eee9e24d78c1855337983451df97c8ad9eedf256c6334f8e948d252d5e0e76847aa0774ddb90a842190d2c558b4b8340";

        empty_sha3_512, Keccack::sha3_512(), EMPTY,
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26";
        long_sha3_512, Keccack::sha3_512(), LETTERS_112,
        "afebb2ef542e6579c50cad06d2e578f9f8dd6881d7dc824d26360feebf18a4fa73e3261122948efcfd492e74e82e2189ed0fb440d187f382270cb455f21dd185";
        very_long_sha3_512, Keccack::sha3_512(), A_1MIL,
        "3c3a876da14034ab60627c077bb98f7e120a2a5370212dffb3385a18d4f38859ed311d0a9d5141ce9cc5c66ee689b266a8aa18ace8282a0e0db596c90b0a7b87";

        empty_shake128, Keccack::shake_128(200), EMPTY,
        "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef263cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e235b8cc873c23dc62b8d260169afa2f75ab916a58d974918835d25e6a435085b2badfd6dfaac359a5efbb7bcc4b59d538df9a04302e10c8bc1cbf1a0b3a5120ea17cda7cfad765f5623474d368ccca8af0007cd9f5e4c849f167a580b14aabdefaee7eef47cb0fca9767be1fda69419dfb927e9df07348b196691abaeb580b32def58538b8d23f877";
        empty_shake256, Keccack::shake_256(200), EMPTY,
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be141e96616fb13957692cc7edd0b45ae3dc07223c8e92937bef84bc0eab862853349ec75546f58fb7c2775c38462c5010d846c185c15111e595522a6bcd16cf86f3d122109e3b1fdd943b6aec468a2d621a7c06c6a957c62b54dafc3be87567d677231395f6147293b68ceab7a9e0c58d864e8efde4e1b9a46cbe854713672f5caaae314ed9083dab";

        nist_test_32_cshake128, Keccack::cshake_128(32, b"", b"Email Signature"), DATA_4,
        "c1c36925b6409a04f1b504fcbca9d82b4017277cb5ed2b2065fc1d3814d5aaf5";
        nist_test_1600_cshake128, Keccack::cshake_128(32, b"", b"Email Signature"), DATA_200,
        "c5221d50e4f822d96a2e8881a961420f294b7b24fe3d2094baed2c6524cc166b";

        nist_test_32_cshake256, Keccack::cshake_256(64, b"", b"Email Signature"), DATA_4,
        "d008828e2b80ac9d2218ffee1d070c48b8e4c87bff32c9699d5b6896eee0edd164020e2be0560858d9c00c037e34a96937c561a74c412bb4c746469527281c8c";
        nist_test_1600_cshake256, Keccack::cshake_256(64, b"", b"Email Signature"), DATA_200,
        "07dc27b11e51fbac75bc7b3c1d983e8b4b85fb1defaf218912ac86430273091727f42b17ed1df63e8ec118f04b23633c1dfb1574c8fb55cb45da8e25afb092bb";

        nist_test_32_kmac128, Keccack::kmac_128(KEY_32, 32, b""), DATA_4,
        "e5780b0d3ea6f7d3a429c5706aa43a00fadbd7d49628839e3187243f456ee14e";
        nist_test_32_custom_kmac128, Keccack::kmac_128(KEY_32, 32, b"My Tagged Application"), DATA_4,
        "3b1fba963cd8b0b59e8c1a6d71888b7143651af8ba0a7070c0979e2811324aa5";
        nist_test_1600_custom_kmac128, Keccack::kmac_128(KEY_32, 32, b"My Tagged Application"), DATA_200,
        "1f5b4e6cca02209e0dcb5ca635b89a15e271ecc760071dfd805faa38f9729230";

        nist_test_32_custom_kmac256, Keccack::kmac_256(KEY_32, 64, b"My Tagged Application"), DATA_4,
        "20c570c31346f703c9ac36c61c03cb64c3970d0cfc787e9b79599d273a68d2f7f69d4cc3de9d104a351689f27cf6f5951f0103f33f4f24871024d9c27773a8dd";
        nist_test_1600_kmac256, Keccack::kmac_256(KEY_32, 64, b""), DATA_200,
        "75358cf39e41494e949707927cee0af20a3ff553904c86b08f21cc414bcfd691589d27cf5e15369cbbff8b9a4c2eb17800855d0235ff635da82533ec6b759b69";
        nist_test_1600_custom_kmac256, Keccack::kmac_256(KEY_32, 64, b"My Tagged Application"), DATA_200,
        "b58618f71f92e1d56c1b8c55ddd7cd188b97b4ca4d99831eb2699a837da2e4d970fbacfde50033aea585f1a2708510c32d07880801bd182898fe476876fc8965";
    );
}
