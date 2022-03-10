use lazy_static::lazy_static;
use std::collections::HashMap;
 
// lazy_static! {
 
//     pub static ref SEVEN_BIT_ASCII_CODES: [String; 128] {
//         let mut arr = [String; 128];
//         for n in 0..128 {
//             arr[0] = format!("{:07b}",n)
//         }
//     }
 
//     pub static ref EIGHT_BIT_ASCII_CODES: [String; 128] {
//         let mut arr = [String; 128];
//         for n in 0..128 {
//             arr[0] = format!("{:08b}",n)
//         }
//     }
 
//     pub static ref ASCII_MAP8: HashMap<char, &'static str> = {
//         let mut m = HashMap::new();
//         for (letter, code) in ASCII128.chars().zip(EIGHT_BIT_ASCII_CODES.iter()) {
//             m.insert(letter, code);
//         }
//         m
//     };
 
//     pub static ref ASCII_MAP_INV8: HashMap<&'static str, char> = {
//         let mut m = HashMap::new();
//         for (letter, code) in ASCII128.chars().zip(EIGHT_BIT_ASCII_CODES.iter()) {
//             m.insert(code, letter);
//         }
//         m
//     };
 
//     pub static ref ASCII_MAP7: HashMap<char, &'static str> = {
//         let mut m = HashMap::new();
//         for (letter, code) in ASCII128.chars().zip(SEVEN_BIT_ASCII_CODES.iter()) {
//             m.insert(letter, code);
//         }
//         m
//     };
 
//     pub static ref ASCII_MAP_INV7: HashMap<&'static str, char> = {
//         let mut m = HashMap::new();
//         for (letter, code) in ASCII128.chars().zip(SEVEN_BIT_ASCII_CODES.iter()) {
//             m.insert(code, letter);
//         }
//         m
//     };
 
// }
 
 
pub struct ASCII {
    map: HashMap<char,String>,
    map_inv: HashMap<String,char>,
    width: usize,
    alphabet: &'static str,
}
 
impl ASCII {
 
    pub fn default8() -> ASCII {
        ASCII{ map: ASCII_MAP8.clone(), map_inv: ASCII_MAP_INV8.clone(), width: 8, alphabet: ASCII128 }
    }
 
    pub fn default7() -> ASCII {
        ASCII{ map: ASCII_MAP7.clone(), map_inv: ASCII_MAP_INV7.clone(), width: 7, alphabet: ASCII128 }
    }
 
}
 
 
impl Code for ASCII {
 
    fn encode(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::with_capacity(text.chars().count*self.width);
        for s in text.chars() {
            match self.map.get(&s) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(CodeError::Input(format!("The symbol `{}` is not in the ASCII alphabet",s)))
            }
        }
        Ok(out)
    }
 
    fn decode(&self, text: &str) -> Result<String,CodeError> {
        let mut out = String::with_capacity(text.chars().count/self.width);
        let w = self.width;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            match self.map.get(&group) {
                Some(code_group) => out.push_str(code_group),
                None => return Err(CodeError::Input(format!("The code group `{}` is not valid",s)))
            }
        }
        Ok(out)
    }
}