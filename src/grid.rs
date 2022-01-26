use std::ops::{Index, IndexMut};

pub fn str_to_grid_symbols(text: &str, empty_char: char, blocked_char: char) -> Vec<Symbol> {
    let mut v = Vec::with_capacity(text.chars().count());
    for c in text.chars() {
        if c == empty_char {
            v.push(Symbol::Empty)
        } else if c == blocked_char {
            v.push(Symbol::Blocked)
        } else {
            v.push(Symbol::Symbol(c))
        }
    }
    v
}
  
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Symbol {
    Symbol(char),
    Empty,
    Blocked
  }
  
#[derive(Clone, Debug)]
pub struct Grid {
    grid: Vec<Symbol>,
    num_rows: usize,
    num_cols: usize,
  }
  
impl Grid {
    pub fn new_empty(num_rows: usize, num_cols: usize) -> Self {
        let grid = vec![Symbol::Empty; num_cols * num_rows];
        Self { grid, num_rows, num_cols }
    }
  
    pub fn new_row_major(text: &str, num_rows: usize, num_cols: usize, empty_char: char, blocked_char: char) -> Self {
        let mut symbols = str_to_grid_symbols(text, empty_char, blocked_char);
        // Drop excess symbols or pad with Empty as needed
        symbols.truncate(num_rows * num_cols);
        symbols.resize(num_rows * num_cols, Symbol::Empty);
  
        Self { grid: symbols, num_rows, num_cols }
    }
  
    pub fn new_col_major(text: &str, num_rows: usize, num_cols: usize, empty_char: char, blocked_char: char) -> Self {
        let mut symbols = str_to_grid_symbols(text, empty_char, blocked_char);
        // Drop excess symbols or pad with Empty as needed
        symbols.truncate(num_rows * num_cols);
        symbols.resize(num_rows * num_cols, Symbol::Empty);

        let mut grid = Vec::with_capacity(num_rows*num_cols);
  
        for col in 0..num_cols {
            for row in 0..num_rows {
                grid[col+(row*num_cols)] = symbols.remove(0);
            }
        }

        Self { grid, num_rows, num_cols }
    }
  
    pub fn row_len(&self) -> usize {
      self.num_cols
    }
  
    pub fn col_len(&self) -> usize {
        self.num_rows
    }
  
    pub fn num_row(&self) -> usize {
        self.num_rows
    }
  
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }
  
    fn get_index(&self, pos: (usize,usize)) -> Option<usize> {
        if pos.0 < self.num_rows && pos.1 < self.num_cols {
            Some(pos.0 * self.row_len() + pos.1)
        } else {
            None
        }
    }
  
    pub fn get(&self, pos: (usize, usize)) -> Option<&Symbol> {
        let idx = self.get_index(pos)?;
        self.grid.get(idx)
      }
  
    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut Symbol> {
        let idx = self.get_index(pos)?;
        self.grid.get_mut(idx)
      }
  
    pub fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &Symbol> {
        let start = self.get_index((row_index, 0))
            .expect("Row index was out of bounds");
        let end = start + self.row_len();
        self.grid[start..end].iter()
    }
  
    // pub fn col_iter(&self, col_index: usize) -> impl Iterator<Item = &Symbol> {
    //     let start = self.get_index((0, col_index))
    //         .expect("Col index was out of bounds");
    //     let end = start + self.row_len();
    //     (0..self.col_len()).map(move |row_index| &self[(row_index, col_index)])
    // }
  
    // pub fn rotate(&mut self) {
    //     let mut new_grid = Vec::<Symbol>::with_capacity(self.grid.len());
    
    //     for n in 0..self.num_cols {
    //         let mut cells = self.col_iter(n);
    //         cells.reverse();
    //         for c in cells {
    //           new_grid.push(*c)
    //         }
    //     }
    
    //     let r = self.num_rows;
    //     self.num_rows = self.num_cols;
    //     self.num_cols = r;
    //     self.grid = new_grid;
    // }
  
    // pub fn display(&self) -> String {
    //     let mut out = String::new();
  
    //     for row in 0..self.num_rows {
    //         for s in self.grid.row_iter(row) {
    //             match s {
    //                 Symbol::Symbol(c) => out.push(*c),
    //                 Symbol::Empty => out.push(' '),
    //                 Symbol::Blocked => out.push(' '),
    //           }
    //         }
    //         out.push('\n')
    //     }
    //     out
    // }
}
  
impl Index<(usize, usize)> for Grid {
    type Output = Symbol;
  
    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        self.get(indices).unwrap()
    }
}
  
impl IndexMut<(usize, usize)> for Grid {  
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        self.get_mut(indices).unwrap()
    }
}