// Naming following Vigna

#[macro_export]
/// Given a valid triple from produces the steps of a maximum length transition
/// x ^= x << a;
/// x ^= x >> b;
/// x ^= x << c;
macro_rules! xorshift_a0 {
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
/// Given a valid triple produces the steps of a maximum length transition
/// x ^= x >> a;
/// x ^= x << b;
/// x ^= x >> c;
macro_rules! xorshift_a1 {
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
/// Given a valid triple produces the steps of a maximum length transition
/// x ^= x << c;
/// x ^= x >> b;
/// x ^= x << a;
macro_rules! xorshift_a2 {
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
/// Given a valid triple produces the steps of a maximum length transition
/// x ^= x >> c;
/// x ^= x << b;
/// x ^= x >> a;
macro_rules! xorshift_a3 {
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
/// Given a valid triple from produces the steps of a maximum length transition
/// x ^= x << a;
/// x ^= x << c;
/// x ^= x >> b;
macro_rules! xorshift_a4 {
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
/// Given a valid triple from produces the steps of a maximum length transition
/// x ^= x >> a;
/// x ^= x >> c;
/// x ^= x << b;
macro_rules! xorshift_a5 {
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
/// Given a valid triple from produces the steps of a maximum length transition
/// x ^= x >> b;
/// x ^= x << a;
/// x ^= x << c;
macro_rules! xorshift_a6 {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state >> $b;
        $state ^= $state << $a;
        $state ^= $state << $c;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state >> $b;
        $state ^= $state << $a;
        $state ^= $state << $c;
    };
}

#[macro_export]
/// Given a valid triple from produces the steps of a maximum length transition
/// x ^= x << b;
/// x ^= x >> c;
/// x ^= x >> a;
macro_rules! xorshift_a7 {
    ($state: ident, $a: ident, $b: ident, $c: ident) => {
        $state ^= $state << $b;
        $state ^= $state >> $a;
        $state ^= $state >> $c;
    };
    ($state: expr, $a: expr, $b: expr, $c: expr) => {
        $state ^= $state << $b;
        $state ^= $state >> $a;
        $state ^= $state >> $c;
    };
}
