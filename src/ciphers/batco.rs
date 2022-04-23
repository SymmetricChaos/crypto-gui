// use std::cell::Cell;
// use itertools::Itertools;
// use rand::StdRng;


// /*
// BATCO is an example of a tactical cipher, one meant to be used quickly to send
// simple tactical messages. Its security is not in the algorithm itself but rather
// comes from three sources. First the BATCO message is meant to be a code looked
// up in a set of vocabulary cards, without these cards even a deciphered message
// is nearly useless. Second the messages are required to be short, no more than 22
// digits, which limits the amount of cipher text available to an attacker. Finally,
// tactical information is relevant only for minutes or hours making serious
// cryptanalysis a waste of resources for the attacker.

// Tactical ciphers are still taught in some armed forces but in practice have been
// replaced by secure voice channels. These are radios that uses modern digital
// encryption to transmit data.
// */

// const BATCO_DIGITS = ["0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", "."];

// #[derive(Clone,Debug)]
// pub struct Batco {
//     cipher_rows: [String; 26],
//     key_cols: [String; 6],
//     message_key: (char,char),
//     seed: Option<usize>
// }


// impl Default for Batco {
//     fn default() -> Self {
//         Batco { 
//             cipher_rows: // scrambled alphabets
//             key_cols: // scrambled alphabets
//             message_key: ('2','A')
//             seed: None,
//         }
//     }
// }


// impl Batco {

//     pub fn random_seeded() -> Batco {
//         let mut rng = Xoshiro256StarStar::seed_from_u64(self.seed);

//         let mut cipher_rows = Vec::with_capacity(26);
//         for _ in 0..26 {
//             cipher_rows.push( scramble_alphabet_seeded(LATIN26, &mut rng) )
//         }

//         let mut key_cols = Vec::with_capacity(6);
//         for _ in 0..6 {
//             key_cols.push( scramble_alphabet_seeded(LATIN26, &mut rng) )
//         }

//         Batco{ cipher_rows, key_cols, message_key: Cell::new(0) }
//     }

//     pub fn random() -> Batco {

//         let mut cipher_rows = Vec::with_capacity(26);
//         for _ in 0..26 {
//             cipher_rows.push( scramble_alphabet(LATIN26) )
//         }

//         let mut key_cols = Vec::with_capacity(6);
//         for _ in 0..6 {
//             key_cols.push( scramble_alphabet(LATIN26) )
//         }

//         Batco{ cipher_rows, key_cols, message_key:('2','A') }
//     }

//     pub fn show_code_page(&self) -> String {
//         let mut s = "2 3 4 5 6 7   0  0  1  2  3  4  5  6  7  8  9 CH  .".to_string();
//         for i in 0..26 {
//             s.push('\n');
//             for j in 0..6 {
//                 s.push( self.key_cols[j].chars().nth(i).unwrap() );
//                 s.push(' ')
//             }

//             s.push(' ');
//             let r = &self.cipher_rows[i];
//             let v = r.chars().collect_vec();
//             let ch = v.chunks(2).map(|x| format!("{}{} ",x[0],x[1])).collect_vec();
//             for pair in ch {
//                 s.push_str(&pair)
//             }
//         }
//         s
//     }

//     pub fn show_key_row(&self) -> String {
//         let mut s = " 0  0  1  2  3  4  5  6  7  8  9 CH  .\n".to_string();
//         let v = self.cipher_rows[self.message_key.get()].chars().collect_vec();
//         let ch = v.chunks(2).map(|x| format!("{}{} ",x[0],x[1])).collect_vec();
//         for pair in ch {
//             s.push_str(&pair)
//         }
//         s
//     }

//     // The key is usize but its defined by a digit from 2 to 7 (to select a column) and a letter (to select a row in that column)
//     fn key_to_row(&self) -> usize {
//         let x = c.0.to_digit(10).unwrap() as usize;
//         let alpha = &self.key_cols[x-2];
//         alpha.chars().position(|x| x == c.1).unwrap()
//     }
// }



// impl Cipher for Batco {

//     pub fn encrypt(&self, text: &str) -> Result<String,CipherError> {
//         if text.chars().count() > 22 {
//             return Err(CipherError::input("BATCO messages are limited to 22 characters per key for security reasons"))
//         }
//         let mut rng = thread_rng();
//         let alphabet = &self.cipher_rows[self.message_key.get()];
//         let mut symbols = text.chars();
//         let breaks = [0,4,6,8,10,12,14,16,18,20,22,24,26];

//         let mut out = String::with_capacity(text.len());
//         // loop while c is Some(char)
//         while let Some(c) = symbols.next() {
//             // H is ignored since it always follows C
//             if c == 'H' { continue }
//             // Convert the symbol to a number
//             let v = match c {
//                 '0' => 0,
//                 '1' => 1,
//                 '2' => 2,
//                 '3' => 3,
//                 '4' => 4,
//                 '5' => 5,
//                 '6' => 6,
//                 '7' => 7,
//                 '8' => 8,
//                 '9' => 9,
//                 'C' => 10,
//                 '.' => 11,
//                 _ => return Err(CipherError::input("the only valid symbols are digits, CH, and the period"))
//             };

//             // Select a random symbol from the allowed range for that number
//             let pos = rng.gen_range(breaks[v]..breaks[v+1]);
//             out.push( alphabet.chars().nth(pos).unwrap() );
//         }
//         Ok(out)
//     }

//     pub fn decrypt(&self, text: &str) -> Result<String,CipherError> {
//         let alphabet = &self.cipher_rows[self.message_key.get()];
//         let symbols = text.chars();

//         let mut out = String::with_capacity(text.len());
//         for c in symbols {
//             let pos = alphabet.chars().position(|x| x == c).unwrap()/2;
//             out.push_str(BATCO_DIGITS[pos])
//         }
//         Ok(out)
//     }
    
//     pub fn randomize(rng: &mut StdRng) {
    
//         for idx in 0..26 {
//             cipher_rows[idx] = scramble_alphabet(LATIN26,rng)
//         }

//         for idx in 0..7 {
//             key_cols[idx] = scramble_alphabet(LATIN26,rng)
//         }
//     }
    
//     pub fn reset(&mut self) {
//         *self = Self::Default();
//     }
// }