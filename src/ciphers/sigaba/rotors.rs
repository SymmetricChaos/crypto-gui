use super::char_to_usize;
use crate::rotors::Rotor;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

pub type IndexRotor<'a> = Rotor<'a, 10>;
pub type CipherRotor<'a> = Rotor<'a, 26>;

impl<const N: usize> PartialEq for Rotor<'_, N> {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl<const N: usize> fmt::Display for Rotor<'_, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(N);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}

lazy_static! {
    pub static ref BIG_ROTOR_VEC: Vec<CipherRotor<'static>> = {
        let mut v = Vec::with_capacity(10);
        v.push(CipherRotor::new(
            "R-A",
            "YCHLQSUGBDIXNZKERPVJTAWFOM",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-B",
            "INPXBWETGUYSAOCHVLDMQKZJFR",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-C",
            "WNDRIOZPTAXHFJYQBMSVEKUCGL",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-D",
            "TZGHOBKRVUXLQDMPNFWCJYEIAS",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-E",
            "YWTAHRQJVLCEXUNGBIPZMSDFOK",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-F",
            "QSLRBTEKOGAICFWYVMHJNXZUDP",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-G",
            "CHJDQIGNBSAKVTUOXFWLEPRMZY",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-H",
            "CDFAJXTIMNBEQHSUGRYLWZKVPO",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-I",
            "XHFESZDNRBCGKQIJLTVMUOYAPW",
            &char_to_usize,
        ));
        v.push(CipherRotor::new(
            "R-J",
            "EZJQXMOGYTCSFRIUPVNADLHWBK",
            &char_to_usize,
        ));
        v
    };
    pub static ref BIG_ROTOR_MAP: HashMap<&'static str, CipherRotor<'static>> = {
        let mut m = HashMap::new();
        for rtr in BIG_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };
    pub static ref INDEX_ROTOR_VEC: Vec<IndexRotor<'static>> = {
        let mut v = Vec::with_capacity(5);
        v.push(IndexRotor::new("0", "7591482630", &char_to_usize));
        v.push(IndexRotor::new("1", "3810592764", &char_to_usize));
        v.push(IndexRotor::new("2", "4086153297", &char_to_usize));
        v.push(IndexRotor::new("3", "3980526174", &char_to_usize));
        v.push(IndexRotor::new("4", "6497135280", &char_to_usize));
        v
    };
}
