use std::ops::{Index, IndexMut};
 
pub fn str_to_grid_symbols(text: &str, empty_char: char, blocked_char: char) -> Vec<Symbol> {
    let mut v = Vec::with_capacity(text.chars().count());
    for c in text.chars() {
        if c == empty_char {
            v.push(Symbol::Empty)
        } else if c == blocked_char {
            v.push(Symbol::Blocked)
        } else {
            v.push(Symbol::Character(c))
        }
    }
    v
}
 
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Symbol {
    Character(char),
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
        let grid_size = num_rows * num_cols;
        // Drop excess symbols or pad with Empty as needed
        symbols.truncate(grid_size);
        symbols.resize(grid_size, Symbol::Empty);
 
        let mut grid = Vec::with_capacity(grid_size);
 
        for col in 0..num_cols {
            for row in 0..num_rows {
                grid[col+(grid_size)] = symbols.remove(0);
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
 
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
 
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }
 
    pub fn grid_size(&self) -> usize {
        self.num_rows * self.num_cols
    }
 
    pub fn coord_from_index(&self, idx: usize) -> Option<(usize,usize)> {
        if idx >= self.grid.len() {
            return None
        }
        Some((idx / self.num_cols, idx % self.num_cols))
    }
 
    pub fn index_from_coord(&self, coord: (usize,usize)) -> Option<usize> {
        if coord.0 < self.num_rows && coord.1 < self.num_cols {
            Some(coord.0 * self.row_len() + coord.1)
        } else {
            None
        }
    }
 
    pub fn get(&self, coord: (usize, usize)) -> Option<&Symbol> {
        let coord = self.index_from_coord(coord)?;
        self.grid.get(coord)
    }
 
    pub fn get_mut(&mut self, coord: (usize, usize)) -> Option<&mut Symbol> {
        let idx = self.index_from_coord(coord)?;
        self.grid.get_mut(idx)
    }
 
    pub fn row(&self, row_index: usize) -> impl Iterator<Item = &Symbol> {
        let start = self.index_from_coord((row_index, 0))
            .expect("Row index was out of bounds");
        let end = start + self.row_len();
        self.grid[start..end].iter()
    }
 
    pub fn col(&self, col_index: usize) -> impl Iterator<Item = &Symbol> {
        (0..self.col_len()).map(move |row_index| &self[(row_index, col_index)])
    }
 
    pub fn read_rows(&self) -> impl Iterator<Item = &Symbol> {
        self.grid.iter()
    }
 
    // Horrible hack that works fine
    pub fn read_cols(&self) -> impl Iterator<Item = &Symbol> {
        let mut symbols: Vec<&Symbol> = Vec::new();
        for n in 0..self.num_cols() {
            for symbol in self.col(n) {
                symbols.push(symbol)
            };
        }
        symbols.into_iter()
    }
 
    // Replace the cell with the Symbol::Empty and return what was overwritten, panics if out of bounds
    pub fn empty_cell(&mut self, coord: (usize, usize)) -> Option<Symbol> {
        let contents = *self.get(coord)?;
        self[coord] = Symbol::Empty;
        Some(contents)
    }
 
    // Replace the cell with the Symbol::Blocked and return what was overwritten
    pub fn block_cell(&mut self, coord: (usize, usize)) -> Option<Symbol> {
        let contents = *self.get(coord)?;
        self[coord] = Symbol::Blocked;
        Some(contents)
    }
 
    // Replace the cell with the Symbol and return what was overwritten
    pub fn replace(&mut self, coord: (usize, usize), new_sym: Symbol) -> Option<Symbol> {
        let contents = *self.get(coord)?;
        self[coord] = new_sym;
        Some(contents)
    }
 
    // Replace only if the position is Symbol::Empty
    pub fn replace_if_empty(&mut self, coord: (usize, usize), new_sym: Symbol) -> Option<Symbol> {
        let contents = *self.get(coord)?;
        if contents == Symbol::Empty {
            self[coord] = new_sym;
            return Some(contents)
        }
        None
    }
 
    // Replace if the position is anything other than Symbol::Blocked
    pub fn replace_if_not_blocked(&mut self, coord: (usize, usize), new_sym: Symbol) -> Option<Symbol> {
        let contents = *self.get(coord)?;
        if contents != Symbol::Blocked {
            self[coord] = new_sym;
            return Some(contents)
        }
        None
    }
 
 
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
    //     let mut out = String::with_capacity(self.num_rows * self.num_cols);
 
    //     for n in 0..self.num_rows {
    //         for s in self.grid.row(n) {
    //             match s {
    //                 Symbol::Symbol(c) => out.push(*c),
    //                 Symbol::Empty => out.push(' '),
    //                 Symbol::Blocked => out.push('🔒'),
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