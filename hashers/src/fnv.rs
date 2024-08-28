use crate::traits::ClassicHasher;
use crypto_bigint::{ArrayEncoding, U1024, U256, U512};
use std::fmt::Display;
// use lazy_static::lazy_static;
use num::FromPrimitive;
use strum::EnumIter;
use utils::byte_formatting::ByteFormat;

// lazy_static! {
//     pub static ref P32: BigUint = BigUint::from_str("16777619").unwrap();
//     pub static ref O32: BigUint = BigUint::from_str("2166136261").unwrap();
//     pub static ref P64: BigUint = BigUint::from_str("1099511628211").unwrap();
//     pub static ref O64: BigUint = BigUint::from_str("14695981039346656037").unwrap();
//     pub static ref P128: BigUint = BigUint::from_str("309485009821345068724781371").unwrap();
//     pub static ref O128: BigUint = BigUint::from_str("144066263297769815596495629667062367629").unwrap();
//     pub static ref P256: BigUint = BigUint::from_str("374144419156711147060143317175368453031918731002211").unwrap();
//     pub static ref O256: BigUint = BigUint::from_str("100029257958052580907070968620625704837092796014241193945225284501741471925557").unwrap();
//     pub static ref P512: BigUint = BigUint::from_str("35835915874844867368919076489095108449946327955754392558399825615420669938882575126094039892345713852759").unwrap();
//     pub static ref O512: BigUint = BigUint::from_str("9659303129496669498009435400716310466090418745672637896108374329434462657994582932197716438449813051892206539805784495328239340083876191928701583869517785").unwrap();
//     pub static ref P1024: BigUint = BigUint::from_str("5016456510113118655434598811035278955030765345404790744303017523831112055108147451509157692220295382716162651878526895249385292291816524375083746691371804094271873160484737966720260389217684476157468082573").unwrap();
//     pub static ref O1024: BigUint = BigUint::from_str("14197795064947621068722070641403218320880622795441933960878474914617582723252296732303717722150864096521202355549365628174669108571814760471015076148029755969804077320157692458563003215304957150157403644460363550505412711285966361610267868082893823963790439336411086884584107735010676915").unwrap();
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum FnvSize {
    L32,
    L64,
    L128,
    L256,
    L512,
    L1024,
}

impl Display for FnvSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FnvSize::L32 => write!(f, "32-bit"),
            FnvSize::L64 => write!(f, "64-bit"),
            FnvSize::L128 => write!(f, "128-bit"),
            FnvSize::L256 => write!(f, "256-bit"),
            FnvSize::L512 => write!(f, "512-bit"),
            FnvSize::L1024 => write!(f, "1024-bit"),
        }
    }
}

macro_rules! hash_bytes {
    ($name: ident, $type: ty, $prime: expr, $offset: expr) => {
        pub fn $name(&self, bytes: &[u8]) -> Vec<u8> {
            let mut state = match self.zero_basis {
                true => 0,
                false => $offset,
            };
            for byte in bytes {
                if self.alternate {
                    state ^= <$type>::from_u8(*byte).unwrap();
                    state = state.wrapping_mul($prime)
                } else {
                    state = state.wrapping_mul($prime);
                    state ^= <$type>::from_u8(*byte).unwrap();
                }
            }
            state.to_be_bytes().to_vec()
        }
    };
}

macro_rules! hash_bytes_crypto_bigint {
    ($name: ident, $type: ty, $prime: expr, $offset: expr) => {
        pub fn $name(&self, bytes: &[u8]) -> Vec<u8> {
            let mut state = match self.zero_basis {
                true => <$type>::ZERO,
                false => $offset,
            };
            for byte in bytes {
                if self.alternate {
                    state ^= <$type>::from_u8(*byte);
                    state = state.wrapping_mul(&$prime)
                } else {
                    state = state.wrapping_mul(&$prime);
                    state ^= <$type>::from_u8(*byte);
                }
            }
            state.to_be_byte_array().to_vec()
        }
    };
}

pub struct Fnv {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub size: FnvSize,
    pub alternate: bool,
    pub zero_basis: bool,
}

impl Default for Fnv {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            size: FnvSize::L64,
            alternate: true,
            zero_basis: false,
        }
    }
}

impl Fnv {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn size(mut self, size: FnvSize) -> Self {
        self.size = size;
        self
    }

    pub fn alternate(mut self, alternate: bool) -> Self {
        self.alternate = alternate;
        self
    }

    pub fn zero_basis(mut self, zero_basis: bool) -> Self {
        self.zero_basis = zero_basis;
        self
    }

    hash_bytes!(hash_bytes_32, u32, 16777619_u32, 2166136261_u32);
    hash_bytes!(
        hash_bytes_64,
        u64,
        1099511628211_u64,
        14695981039346656037_u64
    );
    hash_bytes!(
        hash_bytes_128,
        u128,
        309485009821345068724781371_u128,
        144066263297769815596495629667062367629_u128
    );
    hash_bytes_crypto_bigint!(
        hash_bytes_256,
        U256,
        U256::from_be_hex("0000000000000000000001000000000000000000000000000000000000000163"),
        U256::from_be_hex("DD268DBCAAC550362D98C384C4E576CCC8B1536847B6BBB31023B4C8CAEE0535")
    );

    hash_bytes_crypto_bigint!(
        hash_bytes_512,
        U512,
        U512::from_be_hex("00000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000157"),
        U512::from_be_hex("b86db0b1171f4416dca1e50f309990acac87d059c90000000000000000000d21e948f68a34c192f62ea79bc942dbe7ce182036415f56e34bac982aac4afe9fd9")
    );

    hash_bytes_crypto_bigint!(
        hash_bytes_1024,
        U1024,
        U1024::from_be_hex("000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018d"),
        U1024::from_be_hex("0000000000000000005f7a76758ecc4d32e56d5a591028b74b29fc4223fdada16c3bf34eda3674da9a21d9000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004c6d7eb6e73802734510a555f256cc005ae556bde8cc9c6a93b21aff4b16c71ee90b3")
    );
}

impl ClassicHasher for Fnv {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.size {
            FnvSize::L32 => self.hash_bytes_32(bytes),
            FnvSize::L64 => self.hash_bytes_64(bytes),
            FnvSize::L128 => self.hash_bytes_128(bytes),
            FnvSize::L256 => self.hash_bytes_256(bytes),
            FnvSize::L512 => self.hash_bytes_512(bytes),
            FnvSize::L1024 => self.hash_bytes_1024(bytes),
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod fnvhash_tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = Fnv::default();

        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;

        hasher.size = FnvSize::L32;
        assert_eq!("e40c292c", hasher.hash_bytes_from_string("a").unwrap());

        hasher.size = FnvSize::L64;
        assert_eq!(
            "af63dc4c8601ec8c",
            hasher.hash_bytes_from_string("a").unwrap()
        );

        for h in FnvSize::iter() {
            hasher.size = h;
            let v = hasher.hash_bytes_from_string("a").unwrap();
            assert!(v.len() != 0)
        }
    }

    // #[test]
    // fn test_hex() {
    //     println!("{:02x?}", P32.to_bytes_be());
    //     println!("{:02x?}", P64.to_bytes_be());
    //     println!("{:02x?}", P128.to_bytes_be());
    //     println!("{:02x?}", P256.to_bytes_be());
    //     println!("{:02x?}", P512.to_bytes_be());
    //     println!("{:02x?}", P1024.to_bytes_be());

    //     println!("{:02x?}", O32.to_bytes_be());
    //     println!("{:02x?}", O64.to_bytes_be());
    //     println!("{:02x?}", O128.to_bytes_be());
    //     println!("{:02x?}", O256.to_bytes_be());
    //     println!("{:02x?}", O512.to_bytes_be());
    //     println!("{:02x?}", O1024.to_bytes_be());
    // }
}
