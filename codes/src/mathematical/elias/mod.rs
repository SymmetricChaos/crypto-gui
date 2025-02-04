pub mod delta;
pub mod elias;
pub mod gamma;
pub mod omega;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EliasVariant {
    Delta,
    Gamma,
    Omega,
}

#[macro_export]
macro_rules! next_bit_or_reset {
    ($bits: ident, $buffer: ident, $out: ident, $zero_ctr: ident, $outer: tt) => {
        if let Some(bit) = $bits.next() {
            if let Some(b) = bit {
                $buffer.push(b);
            } else {
                // If we get an invalid symbol interrupt and reset
                $out.push(None);
                $buffer.clear();
                $zero_ctr = 0;
                continue $outer;
            };
        } else {
            $out.push(None);
            continue $outer;
        }
    };
}
