#[macro_export]
/// Given a valid triple from xorshift32_triples.txt or xorshift64_triples.txt produces the steps of a maximum length transition
/// state ^= state << a;
/// state ^= state >> b;
/// state ^= state << c;
macro_rules! xorshift_lrl {
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
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
    ($state: expr, $a: literal, $b: literal, $c: literal) => {
        $state ^= $state >> $c;
        $state ^= $state >> $a;
        $state ^= $state << $b;
    };
}
