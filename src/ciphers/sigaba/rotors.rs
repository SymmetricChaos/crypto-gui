use crate::rotors::Rotor;
use itertools::zip;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

pub type IndexRotor = Rotor<10>;
pub type CipherRotor = Rotor<26>;

impl<const N: usize> PartialEq for Rotor<N> {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl<const N: usize> fmt::Display for Rotor<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(N);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}

lazy_static! {
    pub static ref BIG_ROTOR_VEC: Vec<CipherRotor> = {
        let names = [
            "R-A", "R-B", "R-C", "R-D", "R-E", "R-F", "R-G", "R-H", "R-I", "R-H",
        ];
        let wirings = [
            "YCHLQSUGBDIXNZKERPVJTAWFOM",
            "INPXBWETGUYSAOCHVLDMQKZJFR",
            "WNDRIOZPTAXHFJYQBMSVEKUCGL",
            "TZGHOBKRVUXLQDMPNFWCJYEIAS",
            "YWTAHRQJVLCEXUNGBIPZMSDFOK",
            "QSLRBTEKOGAICFWYVMHJNXZUDP",
            "CHJDQIGNBSAKVTUOXFWLEPRMZY",
            "CDFAJXTIMNBEQHSUGRYLWZKVPO",
            "XHFESZDNRBCGKQIJLTVMUOYAPW",
            "EZJQXMOGYTCSFRIUPVNADLHWBK",
        ];
        let mut v = Vec::with_capacity(10);
        for (name, wiring) in zip(names, wirings) {
            v.push(CipherRotor::new(name, wiring, &|c: char| (c as u8 as usize) - 65).unwrap())
        }
        v
    };
    pub static ref BIG_ROTOR_MAP: HashMap<&'static str, CipherRotor> = {
        let mut m = HashMap::new();
        for rtr in BIG_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };
    pub static ref INDEX_ROTOR_VEC: Vec<IndexRotor> = {
        let names = ["0", "1", "2", "3", "4"];
        let wirings = [
            "7591482630",
            "3810592764",
            "4086153297",
            "3980526174",
            "6497135280",
        ];
        let mut v = Vec::with_capacity(5);
        for (name, wiring) in zip(names, wirings) {
            v.push(IndexRotor::new(name, wiring, &|c: char| (c as u8 as usize) - 48).unwrap())
        }
        v
    };
}
