use crate::traits::StatefulHasher;

use super::{Sha2_224, Sha2_256, Sha2_384, Sha2_512, Sha2_512_224, Sha2_512_256};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sha2Variant {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
}

impl Sha2Variant {
    pub fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self {
            Sha2Variant::Sha224 => Sha2_224::init().hash(&bytes),
            Sha2Variant::Sha256 => Sha2_256::init().hash(&bytes),
            Sha2Variant::Sha384 => Sha2_384::init().hash(&bytes),
            Sha2Variant::Sha512 => Sha2_512::init().hash(&bytes),
            Sha2Variant::Sha512_224 => Sha2_512_224::init().hash(&bytes),
            Sha2Variant::Sha512_256 => Sha2_512_256::init().hash(&bytes),
        }
    }

    pub fn hasher(&self) -> Box<dyn StatefulHasher> {
        match self {
            Sha2Variant::Sha224 => Box::new(Sha2_224::init()),
            Sha2Variant::Sha256 => Box::new(Sha2_256::init()),
            Sha2Variant::Sha384 => Box::new(Sha2_384::init()),
            Sha2Variant::Sha512 => Box::new(Sha2_512::init()),
            Sha2Variant::Sha512_224 => Box::new(Sha2_512_224::init()),
            Sha2Variant::Sha512_256 => Box::new(Sha2_512_256::init()),
        }
    }
}

crate::stateful_hash_tests!(
    test256, crate::sha::Sha2_256::default(), b"",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    test256_long, crate::sha::Sha2_256::default(), b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";
    test256_very_long, crate::sha::Sha2_256::default(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1";
    test256_one_million, crate::sha::Sha2_256::default(), &[0x61; 1_000_000],
    "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0";

    test224, crate::sha::Sha2_224::default(), b"",
    "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f";
    test224_long, crate::sha::Sha2_224::default(), b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "75388b16512776cc5dba5da1fd890150b0c6455cb4f58b1952522525";
    test224_very_long, crate::sha::Sha2_224::default(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "c97ca9a559850ce97a04a96def6d99a9e0e0e2ab14e6b8df265fc0b3";
    test224_one_million, crate::sha::Sha2_224::default(), &[0x61; 1_000_000],
    "20794655980c91d8bbb4c1ea97618a4bf03f42581948b2ee4ee7ad67";

    test512, crate::sha::Sha2_512::default(), b"",
    "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    test512_long, crate::sha::Sha2_512::default(), b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c33596fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445";
    test512_very_long, crate::sha::Sha2_512::default(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909";
    test512_one_million, crate::sha::Sha2_512::default(), &[0x61; 1_000_000],
    "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b";

    test384, crate::sha::Sha2_384::default(), b"",
    "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b";
    test384_long, crate::sha::Sha2_384::default(), b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "3391fdddfc8dc7393707a65b1b4709397cf8b1d162af05abfe8f450de5f36bc6b0455a8520bc4e6f5fe95b1fe3c8452b";
    test384_very_long, crate::sha::Sha2_384::default(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "09330c33f71147e83d192fc782cd1b4753111b173b3b05d22fa08086e3b0f712fcc7c71a557e2db966c3e9fa91746039";
    test384_one_million, crate::sha::Sha2_384::default(), &[0x61; 1_000_000],
    "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985";

    test512_224, crate::sha::Sha2_512_224::default(), b"",
    "6ed0dd02806fa89e25de060c19d3ac86cabb87d6a0ddd05c333b84f4";
    test512_256, crate::sha::Sha2_512_256::default(), b"",
    "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a";
);
