use super::char_to_usize;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
 
// Specifically for SIGABA rotor
#[derive(Clone,Debug)]
pub struct Rotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr: Vec<usize>,
    position: usize,
    size: usize,
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
impl Rotor {
    pub fn new(name: &'static str, wiring_str:  &'static str) -> Rotor {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = Vec::new();
        let mut wiring_ltr = Vec::new();
        for w in wiring_str.chars().map(|x| char_to_usize(x) ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ name, wiring_rtl, wiring_ltr, position: 0, size, wiring_str }
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_rtl[inner_position];
        (inner + self.size - self.position) % self.size
    }
 
    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_ltr[inner_position];
        (inner + self.size - self.position) % self.size
    }
}

// Rotor equality is only based on the wiring not a specific position
impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}

impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(26);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}
lazy_static! {
    pub static ref CIPHER_ROTOR_VEC: Vec<Rotor> = {
        let mut v = Vec::new();
        v.push(Rotor::new("Cph0", "YCHLQSUGBDIXNZKERPVJTAWFOM"));
        v.push(Rotor::new("Cph1", "INPXBWETGUYSAOCHVLDMQKZJFR"));
        v.push(Rotor::new("Cph2", "WNDRIOZPTAXHFJYQBMSVEKUCGL"));
        v.push(Rotor::new("Cph3", "TZGHOBKRVUXLQDMPNFWCJYEIAS"));
        v.push(Rotor::new("Cph4", "YWTAHRQJVLCEXUNGBIPZMSDFOK"));
        v
    };

    pub static ref CIPHER_ROTOR_MAP: HashMap<&'static str, Rotor> = {
        let mut m = HashMap::new();
        for rtr in CIPHER_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };

    pub static ref CONTROL_ROTOR_VEC: Vec<Rotor> = {
        let mut v = Vec::new();
        v.push(Rotor::new("Ctrl0", "QSLRBTEKOGAICFWYVMHJNXZUDP"));
        v.push(Rotor::new("Ctrl1", "CHJDQIGNBSAKVTUOXFWLEPRMZY"));
        v.push(Rotor::new("Ctrl2", "CDFAJXTIMNBEQHSUGRYLWZKVPO"));
        v.push(Rotor::new("Ctrl3", "XHFESZDNRBCGKQIJLTVMUOYAPW"));
        v.push(Rotor::new("Ctrl4", "EZJQXMOGYTCSFRIUPVNADLHWBK"));
        v
    };

    pub static ref CONTROL_ROTOR_MAP: HashMap<&'static str, Rotor> = {
        let mut m = HashMap::new();
        for rtr in CONTROL_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };

    // TODO
    // These will not build correctlty
    // need to modify
    pub static ref INDEX_ROTOR_VEC: Vec<Rotor> = {
        let mut v = Vec::new();
        v.push(Rotor::new("Idx0", "7591482630"));
        v.push(Rotor::new("Idx1", "3810592764"));
        v.push(Rotor::new("Idx2", "4086153297"));
        v.push(Rotor::new("Idx3", "3980526174"));
        v.push(Rotor::new("Idx4", "6497135280"));
        v
    };

    pub static ref INDEX_ROTOR_MAP: HashMap<&'static str, Rotor> = {
        let mut m = HashMap::new();
        for rtr in INDEX_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };
}