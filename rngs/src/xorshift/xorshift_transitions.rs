#[macro_export]
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
/// Given a valid triple from xorshift32_triples.csv or xorshift64_triples.csv produces the steps of a maximum length transition
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
