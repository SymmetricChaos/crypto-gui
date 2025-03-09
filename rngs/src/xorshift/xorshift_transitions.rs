pub enum XorshiftMatrix {
    LRL,
    RLR,
    LLR,
    RRL,
}

/// Given a valid triple from xorshift64_triples.txt perform a maximum length transition on the state.
/// The matrix argument determine the matrix the function is based on.
/// Triples retain the maximum length property when reversed.
pub fn xorshift_transition_lrl(
    mut state: u64,
    triple: (u64, u64, u64),
    matrix: XorshiftMatrix,
) -> u64 {
    let (a, b, c) = triple;
    match matrix {
        XorshiftMatrix::LRL => {
            crate::xorshift_lrl!(state, a, b, c);
        }
        XorshiftMatrix::RLR => {
            crate::xorshift_rlr!(state, a, b, c);
        }
        XorshiftMatrix::LLR => {
            crate::xorshift_llr!(state, a, b, c);
        }
        XorshiftMatrix::RRL => {
            crate::xorshift_rrl!(state, a, b, c);
        }
    }
    state
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state << a;
/// state ^= state >> b;
/// state ^= state << c;
macro_rules! xorshift_lrl {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state << $a;
        $state ^= $state >> $b;
        $state ^= $state << $c;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state << $a;
        $state ^= $state >> $b;
        $state ^= $state << $c;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state << c;
/// state ^= state >> b;
/// state ^= state << a;
macro_rules! xorshift_lrl_inv {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state << $c;
        $state ^= $state >> $b;
        $state ^= $state << $a;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state << $c;
        $state ^= $state >> $b;
        $state ^= $state << $a;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state >> a;
/// state ^= state << b;
/// state ^= state >> c;
macro_rules! xorshift_rlr {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state >> $a;
        $state ^= $state << $b;
        $state ^= $state >> $c;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state >> $a;
        $state ^= $state << $b;
        $state ^= $state >> $c;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state >> c;
/// state ^= state << b;
/// state ^= state >> a;
macro_rules! xorshift_rlr_inv {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state >> $c;
        $state ^= $state << $b;
        $state ^= $state >> $a;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state >> $c;
        $state ^= $state << $b;
        $state ^= $state >> $a;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state << a;
/// state ^= state << c;
/// state ^= state >> b;
macro_rules! xorshift_llr {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state << $a;
        $state ^= $state << $c;
        $state ^= $state >> $b;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state << $a;
        $state ^= $state << $c;
        $state ^= $state >> $b;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state << c;
/// state ^= state << a;
/// state ^= state >> b;
macro_rules! xorshift_llr_inv {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state << $c;
        $state ^= $state << $a;
        $state ^= $state >> $b;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state << $c;
        $state ^= $state << $a;
        $state ^= $state >> $b;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state >> a;
/// state ^= state >> c;
/// state ^= state << b;
macro_rules! xorshift_rrl {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state >> $a;
        $state ^= $state >> $c;
        $state ^= $state << $b;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state >> $a;
        $state ^= $state >> $c;
        $state ^= $state << $b;
    };
}

#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state >> c;
/// state ^= state >> a;
/// state ^= state << b;
macro_rules! xorshift_rrl_inv {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state >> $c;
        $state ^= $state >> $a;
        $state ^= $state << $b;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state >> $c;
        $state ^= $state >> $a;
        $state ^= $state << $b;
    };
}
