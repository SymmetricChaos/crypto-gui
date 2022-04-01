use super::char_to_usize;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
 
// Specifically for SIGABA rotor
#[derive(Clone,Debug)]
pub struct Rotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr: Vec<usize>,
    pub position: usize,
    size: usize,
    pub reversed: bool,
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
impl Rotor {
    pub fn new(name: &'static str, wiring_str:  &'static str) -> Rotor {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = vec![0; size];
        let mut wiring_ltr = vec![0; size];
        for w in wiring_str.chars().map(|x| char_to_usize(x) ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, position: 0, size, reversed: false, wiring_str, name }
    }

    pub fn size(&self) -> usize {
        self.size
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    // Need to logically confirm that a reversed rotor works
    pub fn rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = match self.reversed {
            true => self.wiring_ltr[inner_position],
            false => self.wiring_rtl[inner_position],
        };
        (inner + self.size - self.position) % self.size
    }
 
    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = match self.reversed {
            true => self.wiring_rtl[inner_position],
            false => self.wiring_ltr[inner_position],
        };
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
    pub static ref BIG_ROTOR_VEC: Vec<Rotor> = {
        let mut v = Vec::new();
        v.push(Rotor::new("R-A", "YCHLQSUGBDIXNZKERPVJTAWFOM"));
        v.push(Rotor::new("R-B", "INPXBWETGUYSAOCHVLDMQKZJFR"));
        v.push(Rotor::new("R-C", "WNDRIOZPTAXHFJYQBMSVEKUCGL"));
        v.push(Rotor::new("R-D", "TZGHOBKRVUXLQDMPNFWCJYEIAS"));
        v.push(Rotor::new("R-E", "YWTAHRQJVLCEXUNGBIPZMSDFOK"));
        v.push(Rotor::new("R-F", "QSLRBTEKOGAICFWYVMHJNXZUDP"));
        v.push(Rotor::new("R-G", "CHJDQIGNBSAKVTUOXFWLEPRMZY"));
        v.push(Rotor::new("R-H", "CDFAJXTIMNBEQHSUGRYLWZKVPO"));
        v.push(Rotor::new("R-I", "XHFESZDNRBCGKQIJLTVMUOYAPW"));
        v.push(Rotor::new("R-J", "EZJQXMOGYTCSFRIUPVNADLHWBK"));
        v
    };

    pub static ref BIG_ROTOR_MAP: HashMap<&'static str, Rotor> = {
        let mut m = HashMap::new();
        for rtr in BIG_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };

    // Ideally these should use digits 0..9 but the converting function
    // makes this easier
    pub static ref INDEX_ROTOR_VEC: Vec<Rotor> = {
        let mut v = Vec::new();
        v.push(Rotor::new("Idx0", "HFJBEICGDA"));
        v.push(Rotor::new("Idx1", "DIBAFJCHGE"));
        v.push(Rotor::new("Idx2", "EAIGBFDCJH"));
        v.push(Rotor::new("Idx3", "DJIAFCGBHE"));
        v.push(Rotor::new("Idx4", "GEJHBDFCIA"));
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