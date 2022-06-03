use std::ops::{Index, IndexMut};
use std::fmt;

pub const EMPTY: char = '⬜';
pub const BLOCK: char = '⬛';

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Symbol<T> {
    Character(T),
    Empty,
    Blocked,
}

impl<T> Symbol<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            Symbol::Empty => true,
            _ => false,
        }
    }

    pub fn is_blocked(&self) -> bool {
        match self {
            Symbol::Blocked => true,
            _ => false,
        }
    }

    pub fn is_character(&self) -> bool {
        match self {
            Symbol::Character(_) => true,
            _ => false,
        }
    }
    
    pub fn contents(&self) -> Option<&T> {
        match self {
            Symbol::Character(x) => Some(x),
            _ => None,
        }
    }

}

pub fn str_to_char_grid(text: &str, empty_char: char, blocked_char: char) -> Vec<Symbol<char>> {
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

pub fn read_rows_characters(grid: &Grid<char>) -> impl Iterator<Item = char> + '_ {
    grid.get_rows()
        .filter(|x| x.is_character())
        .map(|x| *x.contents().unwrap())
}

pub fn read_cols_characters(grid: &Grid<char>) -> impl Iterator<Item = char> + '_ {
    grid.get_cols()
        .filter(|x| x.is_character())
        .map(|x| *x.contents().unwrap())
}


#[derive(Clone)]
pub struct Grid<T: Copy + Clone> {
    symbols: Vec<Symbol<T>>,
    num_rows: usize,
    num_cols: usize,
}

impl<T: Copy + Clone> Grid<T> {
    // Creation methods
    pub fn new_empty(num_rows: usize, num_cols: usize) -> Self {
        let grid = vec![Symbol::Empty; num_cols * num_rows];
        Self {
            symbols: grid,
            num_rows,
            num_cols,
        }
    }

    pub fn from_rows(
        symbols: Vec<Symbol<T>>,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {

        let mut symbols = symbols;
        let grid_size = num_rows * num_cols;
        // Drop excess symbols or pad with Empty as needed
        symbols.truncate(grid_size);
        symbols.resize(grid_size, Symbol::Empty);

        Self {
            symbols,
            num_rows,
            num_cols,
        }
    }

    pub fn from_cols(
        symbols: Vec<Symbol<T>>,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {

        let mut symbols = symbols;
        let grid_size = num_rows * num_cols;
        
        symbols.truncate(grid_size);
        symbols.resize(grid_size, Symbol::Empty);

        let mut grid = Grid::new_empty(num_rows, num_cols);

        for col in 0..num_cols {
            for row in 0..num_rows {
                grid[(row, col)] = symbols.remove(0);
            }
        }

        Self {
            symbols: grid.symbols,
            num_rows,
            num_cols,
        }
    }

    // Sizing information
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

    // Convert between coordinates on the 2D grid and indexs to the internal vector
    pub fn coord_from_index(&self, idx: usize) -> Option<(usize, usize)> {
        if idx >= self.symbols.len() {
            return None;
        }
        Some((idx / self.num_cols, idx % self.num_cols))
    }

    pub fn index_from_coord(&self, coord: (usize, usize)) -> Option<usize> {
        if coord.0 < self.num_rows && coord.1 < self.num_cols {
            Some(coord.0 * self.row_len() + coord.1)
        } else {
            None
        }
    }

    pub fn num_empty(&self) -> usize {
        self.get_rows().filter(|s| s.is_empty()).count()
    }

    pub fn num_blocked(&self) -> usize {
        self.get_rows().filter(|s| s.is_blocked()).count()
    }

    pub fn num_character(&self) -> usize {
        self.get_rows().filter(|s| s.is_character()).count()
    }

    pub fn num_noncharacter(&self) -> usize {
        self.get_rows().filter(|s| !s.is_character()).count()
    }

    // Shared getter methods
    pub fn get(&self, coord: (usize, usize)) -> Option<&Symbol<T>> {
        let coord = self.index_from_coord(coord)?;
        self.symbols.get(coord)
    }

    pub fn get_row(&self, row_index: usize) -> impl Iterator<Item = &Symbol<T>> {
        let start = self
            .index_from_coord((row_index, 0))
            .expect("Row index was out of bounds");
        let end = start + self.row_len();
        self.symbols[start..end].iter()
    }

    pub fn get_col(&self, col_index: usize) -> impl Iterator<Item = &Symbol<T>> {
        (0..self.col_len()).map(move |row_index| &self[(row_index, col_index)])
    }

    pub fn get_rows(&self) -> impl Iterator<Item = &Symbol<T>> {
        self.symbols.iter()
    }

    pub fn get_cols(&self) -> impl Iterator<Item = &Symbol<T>> {
        // Yes this is an absurd hack
        let mut symbols: Vec<&Symbol<T>> = Vec::new();
        for n in 0..self.num_cols() {
            for symbol in self.get_col(n) {
                symbols.push(symbol)
            }
        }
        symbols.into_iter()
    }

    // Mutable getter methods
    pub fn get_mut(&mut self, coord: (usize, usize)) -> Option<&mut Symbol<T>> {
        let idx = self.index_from_coord(coord)?;
        self.symbols.get_mut(idx)
    }

    pub fn get_row_mut(&mut self, row_index: usize) -> impl Iterator<Item = &mut Symbol<T>> {
        let start = self
            .index_from_coord((row_index, 0))
            .expect("Row index was out of bounds");
        let end = start + self.row_len();
        self.symbols[start..end].iter_mut()
    }

    pub fn get_col_mut(&mut self, col_index: usize) -> impl Iterator<Item = &mut Symbol<T>> {
        let cols = self.num_cols;
        self.symbols
            .iter_mut()
            .enumerate()
            .filter(move |(i, _)| (i % cols) == col_index)
            .map(|(_, e)| e)
    }

    pub fn get_rows_mut(&mut self) -> impl Iterator<Item = &mut Symbol<T>> {
        self.symbols.iter_mut()
    }

    // pub fn get_cols_mut(&mut self) -> impl Iterator<Item = &mut Symbol> {
    //  for some reason this is
    // }



    // Setter methods overwrite a specific coordinate with some Symbol variant
    // and return what was overwritten. If out of bounds all return None and
    // do nothing.
    pub fn empty_cell(&mut self, coord: (usize, usize)) -> Option<Symbol<T>> {
        let contents = *self.get(coord)?;
        self[coord] = Symbol::Empty;
        Some(contents)
    }

    pub fn block_cell(&mut self, coord: (usize, usize)) -> Option<Symbol<T>> {
        let contents = *self.get(coord)?;
        self[coord] = Symbol::Blocked;
        Some(contents)
    }

    pub fn replace(&mut self, coord: (usize, usize), new_sym: Symbol<T>) -> Option<Symbol<T>> {
        let contents = *self.get(coord)?;
        self[coord] = new_sym;
        Some(contents)
    }

    pub fn replace_if_empty(&mut self, coord: (usize, usize), new_sym: Symbol<T>) -> Option<Symbol<T>> {
        let contents = *self.get(coord)?;
        if contents.is_empty() {
            self[coord] = new_sym;
            return Some(contents);
        }
        None
    }

    pub fn replace_if_not_blocked(
        &mut self,
        coord: (usize, usize),
        new_sym: Symbol<T>,
    ) -> Option<Symbol<T>> {
        let contents = *self.get(coord)?;
        if !contents.is_blocked() {
            self[coord] = new_sym;
            return Some(contents);
        }
        None
    }

    // Apply some closure to each element of the grid, mutating it in place
    pub fn apply<F>(&mut self, mut func: F)
    where
        F: FnMut(Symbol<T>) -> Symbol<T>,
    {
        for sym in self.symbols.iter_mut() {
            *sym = func(*sym);
        }
    }

    /*
    Resize the grid adding Symbol::Empty if size increases and deleting from
    the end if size decreases.
    Relative position of elements in the 2D grid is not maintained
    */
    pub fn resize(&mut self, num_rows: usize, num_cols: usize) {
        self.num_rows = num_rows;
        self.num_cols = num_cols;
        self.symbols.resize(num_rows * num_cols, Symbol::Empty);
    }

    pub fn set_num_rows(&mut self, num_rows: usize) {
        self.num_rows = num_rows;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    pub fn set_num_cols(&mut self, num_cols: usize) {
        self.num_cols = num_cols;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    pub fn add_row(&mut self) {
        self.num_rows += 1;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    pub fn del_row(&mut self) {
        if self.num_rows == 1 {
            return ();
        }
        self.num_rows -= 1;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    pub fn add_col(&mut self) {
        self.num_cols += 1;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    pub fn del_col(&mut self) {
        if self.num_cols == 1 {
            return ();
        }
        self.num_cols -= 1;
        self.symbols
            .resize(self.num_rows * self.num_cols, Symbol::Empty);
    }

    // rotate the grid 90 degree
    pub fn rotate(&mut self) {
        let mut new_symbols = Vec::<Symbol<T>>::with_capacity(self.grid_size());

        for n in 0..self.num_cols {
            let cells: Vec<&Symbol<T>> = self.get_col(n).collect();
            for c in cells.iter().rev() {
              new_symbols.push(**c)
            }
        }

        let r = self.num_rows;
        self.num_rows = self.num_cols;
        self.num_cols = r;
        self.symbols = new_symbols;
    }
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.grid_size());
        for x in 0..self.num_rows {
            for sym in self.get_row(x) {
                match sym {
                    Symbol::Character(c) => out.push(*c),
                    Symbol::Empty => out.push(EMPTY),
                    Symbol::Blocked => out.push(BLOCK),
                }
            }
            out.push('\n')
        }
        write!(f, "{out}")
    }
}

// Two index methods. One for coords and one for index.
impl<T: Copy + Clone> Index<(usize, usize)> for Grid<T> {
    type Output = Symbol<T>;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        self.get(coord).unwrap()
    }
}

impl<T: Copy + Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        self.get_mut(coord).unwrap()
    }
}

impl<T: Copy + Clone> Index<usize> for Grid<T> {
    type Output = Symbol<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index]
    }
}

impl<T: Copy + Clone> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.symbols[index]
    }
}