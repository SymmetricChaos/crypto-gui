use crate::traits::StatefulHasher;

const WORDS: usize = 8;
const ROUNDS: usize = 72;
const SUBKEYS: usize = ROUNDS / 4 + 1;

pub struct Skein512 {
    state: [u64; WORDS],
    key: [u64; WORDS],
    tweak: [u64; 2],
    bytes_taken: u64,
}

impl Default for Skein512 {
    fn default() -> Self {
        Self::init_256()
    }
}

impl Skein512 {
    fn init(iv: [u64; WORDS]) -> Self {
        Self {
            state: iv,
            key: todo!(),
            tweak: todo!(),
            bytes_taken: 0,
        }
    }

    pub fn init_128() -> Self {
        Self::init([
            W(0xA8BC7BF36FBF9F52),
            W(0x1E9872CEBD1AF0AA),
            W(0x309B1790B32190D3),
            W(0xBCFBB8543F94805C),
            W(0x0DA61BCD6E31B11B),
            W(0x1A18EBEAD46A32E3),
            W(0xA2CC5B18CE84AA82),
            W(0x6982AB289D46982D),
        ])
    }

    pub fn init_160() -> Self {
        Self::init([
            W(0x28B81A2AE013BD91),
            W(0xC2F11668B5BDF78F),
            W(0x1760D8F3F6A56F12),
            W(0x4FB747588239904F),
            W(0x21EDE07F7EAF5056),
            W(0xD908922E63ED70B8),
            W(0xB8EC76FFECCB52FA),
            W(0x01A47BB8A3F27A6E),
        ])
    }

    pub fn init_224() -> Self {
        Self::init([
            W(0xCCD0616248677224),
            W(0xCBA65CF3A92339EF),
            W(0x8CCD69D652FF4B64),
            W(0x398AED7B3AB890B4),
            W(0x0F59D1B1457D2BD0),
            W(0x6776FE6575D4EB3D),
            W(0x99FBC70E997413E9),
            W(0x9E2CFCCFE1C41EF7),
        ])
    }

    pub fn init_256() -> Self {
        Self::init([
            W(0xCCD044A12FDB3E13),
            W(0xE83590301A79A9EB),
            W(0x55AEA0614F816E6F),
            W(0x2A2767A4AE9B94DB),
            W(0xEC06025E74DD7683),
            W(0xE7A436CDC4746251),
            W(0xC36FBAF9393AD185),
            W(0x3EEDBA1833EDFC13),
        ])
    }

    pub fn init_384() -> Self {
        Self::init([
            W(0xA3F6C6BF3A75EF5F),
            W(0xB0FEF9CCFD84FAA4),
            W(0x9D77DD663D770CFE),
            W(0xD798CBF3B468FDDA),
            W(0x1BC4A6668A0E4465),
            W(0x7ED7D434E5807407),
            W(0x548FC1ACD4EC44D6),
            W(0x266E17546AA18FF8),
        ])
    }

    pub fn init_512() -> Self {
        Self::init([
            W(0x4903ADFF749C51CE),
            W(0x0D95DE399746DF03),
            W(0x8FD1934127C79BCE),
            W(0x9A255629FF352CB1),
            W(0x5DB62599DF6CA7B0),
            W(0xEABE394CA9D5C3F4),
            W(0x991112C71A75B523),
            W(0xAE18A40B660FCC33),
        ])
    }
}

impl StatefulHasher for Skein512 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_512_256_empty, Skein512::init_256(), b"",
    "39ccc4554a8b31853b9de7a1fe638a24cce6b35a55f2431009e18780335d2621";
    test_512_512_empty, Skein512::init_512(), b"",
    "bc5b4c50925519c290cc634277ae3d6257212395cba733bbad37a4af0fa06af41fca7903d06564fea7a2d3730dbdb80c1f85562dfcc070334ea4d1d9e72cba7a";
);
