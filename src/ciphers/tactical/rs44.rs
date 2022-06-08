use rand::{prelude::{StdRng, SliceRandom}, SeedableRng, Rng};

use crate::grid::{Symbol, Grid};

pub struct RS44 {
    stencil: Grid<Symbol<char>>,
    xlabels: [&'static str; 25],
    ylabels: [&'static str; 24],
    message_key: [usize; 2],
    message_key_maxtrix: Grid<char>,
    time: String,
    seed: Option<u64>,
}

impl RS44 {
    pub const WIDTH: usize = 25;
    pub const HEIGHT: usize = 24;
    pub const LABELS: [&'static str; 25] = 
        ["aa", "ba", "ca", "da", "ea",
         "ab", "bb", "cb", "db", "eb",
         "ac", "bc", "cc", "dc", "ec",
         "ad", "bd", "cd", "dd", "de",
         "ae", "be", "ce", "de", "ee"];
    
    fn get_rng(&self) -> StdRng {
        match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        }
    }
    
    fn label_letter_to_matrix_column(&self, c: char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            _ => unreachable!("the only letter used for labels are 'abcde'")
        }
    }
    
    pub fn encrypt_message_key(&self) -> String {
        let mut message_key_string = String::with_capacity(4);
        let mut rng = self.get_rng();
        for c in self.xlabels[self.message_key[0]].chars().chain(self.ylabels[self.message_key[1]].chars()) {
            let row: usize = rng.gen_range(0..5);
            let col = self.label_letter_to_matrix_column(c);
            message_key_string.push(self.message_key_maxtrix[(row,col)]);
        }
        
        message_key_string
    }
    
    pub fn randomize_stencil(&mut self) {
        self.stencil.apply(|x| Symbol::Blocked);
        let mut rng = self.get_rng();
        let mut positions: Vec<usize> = (0..25).collect();
        
        for i in 0..24 {
            positions.shuffle(&mut rng);
            for n in &positions[0..10] {
                self.stencil[n+(i*25)] = Symbol::Empty;
            }
        }
    }
}