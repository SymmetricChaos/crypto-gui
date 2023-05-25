pub mod two_square;
use num::integer::Roots;
pub use two_square::TwoSquare;

pub mod four_square;
pub use four_square::FourSquare;

pub mod playfair;
pub use playfair::Playfair;

pub mod slidefair;
pub use slidefair::Slidefair;

pub mod seriated_playfair;

pub fn is_square(n: usize) -> bool {
    n.sqrt().pow(2) == n
}
