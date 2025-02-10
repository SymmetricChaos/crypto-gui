use super::BinaryToText;
use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::{
    byte_formatting::ByteFormat,
    text_functions::{closest_match, string_pairs},
};

const BYTEWORD_WORDS: [&'static str; 256] = [
    "able", "acid", "also", "apex", "aqua", "arch", "atom", "aunt", "away", "axis", "back", "bald",
    "barn", "belt", "beta", "bias", "blue", "body", "brag", "brew", "bulb", "buzz", "calm", "cash",
    "cats", "chef", "city", "claw", "code", "cola", "cook", "cost", "crux", "curl", "cusp", "cyan",
    "dark", "data", "days", "deli", "dice", "diet", "door", "down", "draw", "drop", "drum", "dull",
    "duty", "each", "easy", "echo", "edge", "epic", "even", "exam", "exit", "eyes", "fact", "fair",
    "fern", "figs", "film", "fish", "fizz", "flap", "flew", "flux", "foxy", "free", "frog", "fuel",
    "fund", "gala", "game", "gear", "gems", "gift", "girl", "glow", "good", "gray", "grim", "guru",
    "gush", "gyro", "half", "hang", "hard", "hawk", "heat", "help", "high", "hill", "holy", "hope",
    "horn", "huts", "iced", "idea", "idle", "inch", "inky", "into", "iris", "iron", "item", "jade",
    "jazz", "join", "jolt", "jowl", "judo", "jugs", "jump", "junk", "jury", "keep", "keno", "kept",
    "keys", "kick", "kiln", "king", "kite", "kiwi", "knob", "lamb", "lava", "lazy", "leaf", "legs",
    "liar", "limp", "lion", "list", "logo", "loud", "love", "luau", "luck", "lung", "main", "many",
    "math", "maze", "memo", "menu", "meow", "mild", "mint", "miss", "monk", "nail", "navy", "need",
    "news", "next", "noon", "note", "numb", "obey", "oboe", "omit", "onyx", "open", "oval", "owls",
    "paid", "part", "peck", "play", "plus", "poem", "pool", "pose", "puff", "puma", "purr", "quad",
    "quiz", "race", "ramp", "real", "redo", "rich", "road", "rock", "roof", "ruby", "ruin", "runs",
    "rust", "safe", "saga", "scar", "sets", "silk", "skew", "slot", "soap", "solo", "song", "stub",
    "surf", "swan", "taco", "task", "taxi", "tent", "tied", "time", "tiny", "toil", "tomb", "toys",
    "trip", "tuna", "twin", "ugly", "undo", "unit", "urge", "user", "vast", "very", "veto", "vial",
    "vibe", "view", "visa", "void", "vows", "wall", "wand", "warm", "wasp", "wave", "waxy", "webs",
    "what", "when", "whiz", "wolf", "work", "yank", "yawn", "yell", "yoga", "yurt", "zaps", "zero",
    "zest", "zinc", "zone", "zoom",
];

const BYTEWORD_MINWORDS: [&'static str; 256] = [
    "ae", "ad", "ao", "ax", "aa", "ah", "am", "at", "ay", "as", "bk", "bd", "bn", "bt", "ba", "bs",
    "be", "by", "bg", "bw", "bb", "bz", "cm", "ch", "cs", "cf", "cy", "cw", "ce", "ca", "ck", "ct",
    "cx", "cl", "cp", "cn", "dk", "da", "ds", "di", "de", "dt", "dr", "dn", "dw", "dp", "dm", "dl",
    "dy", "eh", "ey", "eo", "ee", "ec", "en", "em", "et", "es", "ft", "fr", "fn", "fs", "fm", "fh",
    "fz", "fp", "fw", "fx", "fy", "fe", "fg", "fl", "fd", "ga", "ge", "gr", "gs", "gt", "gl", "gw",
    "gd", "gy", "gm", "gu", "gh", "go", "hf", "hg", "hd", "hk", "ht", "hp", "hh", "hl", "hy", "he",
    "hn", "hs", "id", "ia", "ie", "ih", "iy", "io", "is", "in", "im", "je", "jz", "jn", "jt", "jl",
    "jo", "js", "jp", "jk", "jy", "kp", "ko", "kt", "ks", "kk", "kn", "kg", "ke", "ki", "kb", "lb",
    "la", "ly", "lf", "ls", "lr", "lp", "ln", "lt", "lo", "ld", "le", "lu", "lk", "lg", "mn", "my",
    "mh", "me", "mo", "mu", "mw", "md", "mt", "ms", "mk", "nl", "ny", "nd", "ns", "nt", "nn", "ne",
    "nb", "oy", "oe", "ot", "ox", "on", "ol", "os", "pd", "pt", "pk", "py", "ps", "pm", "pl", "pe",
    "pf", "pa", "pr", "qd", "qz", "re", "rp", "rl", "ro", "rh", "rd", "rk", "rf", "ry", "rn", "rs",
    "rt", "se", "sa", "sr", "ss", "sk", "sw", "st", "sp", "so", "sg", "sb", "sf", "sn", "to", "tk",
    "ti", "tt", "td", "te", "ty", "tl", "tb", "ts", "tp", "ta", "tn", "uy", "uo", "ut", "ue", "ur",
    "vt", "vy", "vo", "vl", "ve", "vw", "va", "vd", "vs", "wl", "wd", "wm", "wp", "we", "wy", "ws",
    "wt", "wn", "wz", "wf", "wk", "yk", "yn", "yl", "ya", "yt", "zs", "zo", "zt", "zc", "ze", "zm",
];

fn bytes_to_words(bytes: &[u8]) -> Vec<&'static str> {
    let mut out = Vec::with_capacity(bytes.len());
    for byte in bytes {
        out.push(byte_to_word(byte))
    }
    out
}

fn byte_to_word(byte: &u8) -> &'static str {
    BYTEWORD_WORDS[*byte as usize]
}

// Find the closest matching word
fn word_to_byte(word: &str) -> Result<u8, CodeError> {
    let (idx, dist) = closest_match(word, &BYTEWORD_WORDS);
    if dist >= 2 {
        Err(CodeError::Input(format!("invalid word `{}` found", word)))
    } else {
        Ok(idx as u8)
    }
}

fn words_to_bytes(words: &[&str]) -> Result<Vec<u8>, CodeError> {
    let mut out = Vec::with_capacity(words.len());
    for word in words {
        out.push(word_to_byte(word)?);
    }
    Ok(out)
}

fn minword_to_byte(word: &str) -> Result<u8, CodeError> {
    BYTEWORD_MINWORDS
        .iter()
        .position(|p| p == &word)
        .ok_or_else(|| CodeError::Input(format!("invalid word `{}` found", word)))
        .map(|n| n as u8)
}

fn minwords_to_bytes(words: &[&str]) -> Result<Vec<u8>, CodeError> {
    let mut out = Vec::with_capacity(words.len());
    for word in words {
        out.push(minword_to_byte(word)?);
    }
    Ok(out)
}

fn byte_to_minword(byte: &u8) -> &'static str {
    BYTEWORD_MINWORDS[*byte as usize]
}

fn bytes_to_minwords(bytes: &[u8]) -> Vec<&'static str> {
    let mut out = Vec::with_capacity(bytes.len());
    for byte in bytes {
        out.push(byte_to_minword(byte))
    }
    out
}

#[derive(Debug, PartialEq, Eq)]
pub enum Separator {
    Space,
    Dash,
}

impl Separator {
    pub fn str(&self) -> &'static str {
        match self {
            Self::Space => " ",
            Self::Dash => "-",
        }
    }
}

pub struct Bytewords {
    pub mode: ByteFormat,
    pub sep: Separator,
    pub minwords: bool,
}

impl Default for Bytewords {
    fn default() -> Self {
        Self {
            mode: ByteFormat::Hex,
            sep: Separator::Space,
            minwords: false,
        }
    }
}

impl Bytewords {
    pub fn chars_codes(&self) -> impl Iterator<Item = (String, String)> + '_ {
        let words = match self.minwords {
            true => &BYTEWORD_MINWORDS,
            false => &BYTEWORD_WORDS,
        };
        (0..256).map(|n| (format!("{n:02x}"), format!("{}", words[n])))
    }
}

impl BinaryToText for Bytewords {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        match self.minwords {
            true => Ok(bytes_to_minwords(bytes).join("")),
            false => Ok(bytes_to_words(bytes).join(self.sep.str())),
        }
    }
}

impl Code for Bytewords {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
            ByteFormat::Binary => self.encode_bits(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if self.minwords {
            let words = string_pairs(text);
            let bytes = minwords_to_bytes(&words)?;
            Ok(self.mode.byte_slice_to_text(bytes))
        } else {
            let words = text.split(self.sep.str()).collect_vec();
            let bytes = words_to_bytes(&words)?;
            Ok(self.mode.byte_slice_to_text(&bytes))
        }
    }
}

#[cfg(test)]
mod byteword_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut code = Bytewords::default();
        code.mode = ByteFormat::Hex;
        let bytes = "d99d6ca20150c7098580125e2ab0981253468b2dbc5202c11947dac904f40b";
        assert_eq!(
            "tuna next jazz oboe acid good slot axis limp lava brag holy door puff monk brag guru frog luau drop roof grim also safe chef fuel twin solo aqua work bald",
            code.encode(bytes).unwrap(),
        );
    }

    #[test]
    fn test_decode() {
        let mut code = Bytewords::default();
        code.mode = ByteFormat::Hex;
        let words = "tuna next jazz oboe acid good slot axis limp lava brag holy door puff monk brag guru frog luau drop roof grim also safe chef fuel twin solo aqua work bald";
        assert_eq!(
            code.decode(words).unwrap(),
            "d99d6ca20150c7098580125e2ab0981253468b2dbc5202c11947dac904f40b"
        );
    }

    #[test]
    fn test_decode_errs() {
        let mut code = Bytewords::default();
        code.mode = ByteFormat::Hex;
        // Simple errors can be corrected
        let words = "tun! ne私xt jazd";
        assert_eq!(code.decode(words).unwrap(), "d99d6c");

        // Others cannot, words with a distance of more than 1 from a valid word are rejected
        let words = "tuna next jamq";
        assert_eq!(
            code.decode(words).unwrap_err(),
            CodeError::input("invalid word `jamq` found")
        );
        let words = "ttunna next jazz";
        assert_eq!(
            code.decode(words).unwrap_err(),
            CodeError::input("invalid word `ttunna` found")
        );
        // Sometimes error correction fails due to similarity (jadz is the same distance from 'jade' and 'jazz')
        // Since 'jade' is earlier in the list it is the preferred decoding, this changes the decoding from the intended d99d6c to d99d6b
        let words = "tuna next jadz";
        assert_eq!(code.decode(words).unwrap(), "d99d6b");
    }

    #[test]
    fn test_encode_minwords() {
        let mut code = Bytewords::default();
        code.mode = ByteFormat::Hex;
        code.minwords = true;
        let bytes = "d99d6ca20150c7098580125e2ab0981253468b2dbc5202c11947dac904f40b";
        assert_eq!(
            "tantjzoeadgdstaslplabghydrpfmkbggufgludprfgmaosecffltnsoaawkbd",
            code.encode(bytes).unwrap(),
        );
    }

    #[test]
    fn test_decode_minwords() {
        let mut code = Bytewords::default();
        code.mode = ByteFormat::Hex;
        code.minwords = true;
        let words = "tantjzoeadgdstaslplabghydrpfmkbggufgludprfgmaosecffltnsoaawkbd";
        assert_eq!(
            code.decode(words).unwrap(),
            "d99d6ca20150c7098580125e2ab0981253468b2dbc5202c11947dac904f40b"
        );
    }

    #[test]
    fn test_decode_minwords_errs() {
        let mut code = Bytewords::default();
        code.minwords = true;
        let words = "tantj";
        assert_eq!(
            code.decode(words).unwrap_err(),
            CodeError::input("invalid word")
        );
        let words = "t!nt";
        assert_eq!(
            code.decode(words).unwrap_err(),
            CodeError::input("invalid word")
        );
        let words = "t私nt";
        assert_eq!(
            code.decode(words).unwrap_err(),
            CodeError::input("invalid word")
        );
    }
}
