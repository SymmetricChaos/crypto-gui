use crate::errors::CipherError;
use crate::grid::Grid;
use crate::text_functions::rank_str;
use super::Cipher;

pub struct Columnar {
    key: Vec<usize>,
    key_name: String,
}


impl Cipher for Columnar {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // let tlen = text.chars().count();
        // let n_rows = tlen.div_ceil(&self.key.len());
        // let g = Grid::new(text, n_rows, self.key.len());

        let mut out = String::with_capacity(text.len());
        // for k in self.key.iter() {
        //     let mut s: String = g.read_col_n(*k).iter().collect();
        //     s = s.replace('\0', "");
        //     out.push_str(&s);
        // }
        Ok(out)
    }

    // Decoding is very different
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // let tlen = text.chars().count();
        // let filled = match tlen % self.key.len() {
        //     0 => self.key.len(),
        //     a => a
        // };
        // let n_rows = tlen.div_ceil(&self.key.len());

        // let mut g = Grid::new_empty(n_rows, self.key.len());
        // let mut symbols = text.chars();

        // for k in self.key.iter() {
        //     let mut s = String::new();
        //     if *k < filled {
        //         for _ in 0..self.key.len() {
        //             s.push(symbols.next().unwrap())
        //         }
        //     } else {
        //         for _ in 0..self.key.len()-1 {
        //             s.push(symbols.next().unwrap())
        //         }
        //     }
            
        //     g.write_col_n(*k,&s);
        // }

        let mut out = String::with_capacity(text.len());
        // for i in 0..n_rows {
        //     let s: String = g.read_row_n(i).iter().collect();
        //     out.push_str(&s)
        // }
        // out = out.replace('\0', "");
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::ThreadRng) {
        todo!()
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn get_input_alphabet(&mut self) -> &String {
        todo!()
    }

    fn get_output_alphabet(&mut self) -> &String {
        todo!()
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }


}