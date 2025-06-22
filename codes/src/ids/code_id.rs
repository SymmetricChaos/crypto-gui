use json::{iterators::Members, JsonValue};
use std::{fmt::Display, sync::LazyLock};

macro_rules! code_ids_and_names {
    ($( $id: ident, $name: expr);+ $(;)?) => {

        #[derive(PartialEq, Eq, Debug, Clone, Copy)]
        pub enum CodeId {
            $(
                $id,
            )+
        }

        impl Display for CodeId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(
                        CodeId::$id => $name,
                    )+
                };
                write!(f, "{}", name)
            }
        }

    }
}

code_ids_and_names!(
    Ascii, "ASCII";
    Ascii85, "Ascii85";
    Bacon, "Bacon";
    BalancedTernary, "Balanced Ternary";
    Barbier, "Barbier";
    BaseN, "Base-N";
    BaseNegativeTwo, "Base Negative 2";
    BaseNBijective, "Bijective Base-N";
    BaseX, "BaseX";
    Hexadecimal, "Hexadecimal";
    Base32, "Base32";
    Base64, "Base64";
    Baudot, "Baudot";
    BinaryCodedDecimal, "Binary-Coded Decimal";
    BiquinaryDecimal, "Biquinary Coded Decimal";
    Braille, "Simplified Braille";
    BrailleEncoding, "Braille Encodings";
    ByteAsNum, "Bytes as Numbers";
    Bytewords, "Bytewords";
    Ccsid, "CCSID";
    CcsidBinary, "CCSID";
    Combinadic, "Combinadic";
    CyclicRedundancyCheck, "Cyclic Redundancy Check";
    Damm, "Damm";
    Elias, "Elias";
    ExpGolomb, "Exponential Golomb";
    Factoradic, "Factoradic";
    Fibonacci, "Fibonacci";
    FixedWidth, "Fixed-Width";
    FlagSemaphore, "Flag Semaphore";
    Godel, "Gödel";
    Golomb, "Golomb";
    Gray, "Gray Code";
    Hamming, "Hamming Code";
    IcsFlags, "ICS Flags";
    IntelHex, "IntelHex";
    Isbn, "ISBN";
    Itf, "ITF";
    Leb128, "LEB128";
    Levenshtein, "Levenshtein";
    Linotype, "Linotype";
    Luhn, "Luhn's Algorithm";
    MofN, "M-of-N";
    Morse, "Morse";
    Needle, "Needle";
    NegativeBase, "Negative Base";
    ParityBit, "Parity Bit";
    Pgp, "PGP Words";
    Primorial, "Primorial";
    Punycode, "Punycode";
    Repetition, "Repetition";
    Romaji, "Romaji";
    RomanNumeral, "Roman Numeral";
    RunLengthEncoding, "Run Length Encoding";
    RunLengthEncodingBytes, "Run Length Encoding Bytes";
    Skey, "S/KEY";
    SpellingAlphabet, "Spelling Alphabet";
    Tap, "Tap";
    TruncatedBinary, "Truncated Binary";
    TwosComplement, "Two's Complement";
    Ueb, "Unified English Braille";
    Unary, "Unary";
    UnarySymmetric, "Symmetric Unary";
    Unicode, "Unicode";
    Upc, "UPC";
    Verhoeff, "Verhoeff";
    Wabun, "Wabun";
);

impl Default for CodeId {
    fn default() -> Self {
        Self::Ascii
    }
}

impl CodeId {
    pub fn description(&self) -> &JsonValue {
        &CODE_INFORMATION[self.to_string()]["Description"]
    }

    pub fn authors(&self) -> &JsonValue {
        &CODE_INFORMATION[self.to_string()]["Authors"]
    }

    pub fn publication_date(&self) -> &JsonValue {
        &CODE_INFORMATION[self.to_string()]["Publication"]
    }

    pub fn traits(&self) -> Members {
        CODE_INFORMATION[self.to_string()]["Traits"].members()
    }
}

impl From<CodeId> for String {
    fn from(id: CodeId) -> Self {
        id.to_string()
    }
}

pub static CODE_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("code_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse code_descriptions")
});
