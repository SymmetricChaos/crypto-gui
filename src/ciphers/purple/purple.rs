use crate::{ciphers::{substitution::Plugboard, Cipher}, errors::CipherError};

use super::switch::{Switch, SwitchSpeed};



#[derive(Clone)]
pub struct Switches {
    sixes: Switch<6_usize>,
    twenties: [Switch<20_usize>; 3],
}

impl Default for Switches {
    fn default() -> Self {
        Self { sixes: Switch::<6_usize>::sixes(), twenties: Switch::<20_usize>::twenties() }
    }
}
 
impl Switches {
    pub fn step(&mut self) {
        let spos = self.sixes.position;
        let mpos = self.get_switch(SwitchSpeed::Middle).position;
 
        // Sixes always steps
        self.sixes.step();
 
        // Exactly one of the Twenties steps at a time
        if spos == 23 && mpos == 24 {
            self.get_switch(SwitchSpeed::Slow).step();
        } else if spos == 24 {
            self.get_switch(SwitchSpeed::Middle).step();
        } else {
            self.get_switch(SwitchSpeed::Fast).step();
        }
 
    }
 
    pub fn encrypt(&mut self, c: char) -> char {
        todo!("")
    }
 
    pub fn decrypt(&mut self, c: char) -> char {
        todo!("")
    }
 
    fn get_switch(&mut self, speed: SwitchSpeed) -> &mut Switch<20> {
        for switch in self.twenties.iter_mut() {
            if switch.speed == speed {
                return switch
            }
        }
        unreachable!("every switch speed must be represented")
    }
}
 
pub struct Purple {
    switches: Switches, // this will be cloned during execution and then mutated
    input_plugboard: Plugboard,
    output_plugboard: Plugboard,
}

impl Default for Purple {
    fn default() -> Self {
        Self { switches: Default::default(), input_plugboard: Default::default(), output_plugboard: Default::default() }
    }
}
 
impl Purple {
    const SIXES: &'static str = "AEIOUY";
    const TWENTIES: &'static str = "BCDFGHJKLMNPQRSTVWXZ";
}
 
impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {

        let mut switches = self.switches.clone();

        let from_pb = self.input_plugboard.encrypt(text)?;

        let mut out = String::with_capacity(text.len());

        for c in from_pb.chars() {
            out.push(switches.encrypt(c));
            switches.step();
        }
 
        self.output_plugboard.encrypt(&out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}