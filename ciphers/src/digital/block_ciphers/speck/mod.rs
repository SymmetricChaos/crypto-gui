pub mod speck128;
pub mod speck32;
pub mod speck64;

// These macros make it straightforward to implement speck for the various word sizes
#[macro_export]
macro_rules! enc {
    ($x:ident, $y:ident, $k:ident, $alpha:literal, $beta:literal) => {
        $x = $x.rotate_right($alpha);
        $x = $x.wrapping_add($y);
        $x ^= $k;
        $y = $y.rotate_left($beta);
        $y ^= $x;
    };
}

#[macro_export]
macro_rules! dec {
    ($x:ident, $y:ident, $k:ident, $alpha:literal, $beta:literal) => {
        $y ^= $x;
        $y = $y.rotate_right($beta);
        $x ^= $k;
        $x = $x.wrapping_sub($y);
        $x = $x.rotate_left($alpha);
    };
}
