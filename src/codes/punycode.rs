// https://datatracker.ietf.org/doc/html/rfc3492

use itertools::Itertools;

const TMIN: u32 = 1;
const TMAX: u32 = 26;
const DAMP: u32 = 700;
const SKEW: u32 = 38;
const INIT_BIAS: u32 = 72;
const INIT_N: u32 = 128;
const BASE: u32 = 36;
const DELIM: char = '-';

pub fn adaptation(delta: u32, num_points: u32, first_time: bool) -> u32 {
    let mut delta = if first_time { delta / DAMP } else { delta / 2 };
    delta += delta / num_points;
    let mut k = 0;
    while delta > ((BASE - TMIN) * TMAX) / 2 {
        delta /= BASE - TMIN;
        k += BASE;
    }
    k + (((BASE - TMIN + 1) * delta) / (delta + SKEW))
}

fn decode_digit(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - 97,
        'A'..='Z' => u32::from(c) - 65,
        '0'..='9' => u32::from(c) - 22,
        _ => panic!("invald punycode digit when decoding"),
    }
}

fn encode_digit(n: u32) -> char {
    match n {
        0..=25 => char::from_u32(n + 97).unwrap(),
        26..=35 => char::from_u32(n + 22).unwrap(),
        _ => panic!("invald punycode digit when encoding"),
    }
}

fn threshold(k: u32, bias: u32) -> u32 {
    if k <= bias + TMIN {
        TMIN
    } else if k >= bias + TMAX {
        TMAX
    } else {
        k - bias
    }
}

pub fn decode_punycode(input: &str) -> Result<String, &'static str> {
    let mut n = INIT_N;
    let mut i = 0;
    let mut bias = INIT_BIAS;

    if !input.is_ascii() {
        return Err("found non-ASCII characters");
    }

    // This gives a byte index but because we only have ASCII characters this is also the character index
    let delim_pos = input.rfind(DELIM);

    let mut output = match delim_pos {
        Some(i) => input.chars().take(i).collect_vec(),
        None => Vec::new(),
    };

    // Rather than decoding whole punycode string we'll just decode the part after the delimeter
    let mut chars = match delim_pos {
        Some(i) => input.chars().skip(i + 1),
        None => input.chars().skip(0),
    };

    'outer: loop {
        let old_i = i;
        let mut w = 1;
        for k in 1.. {
            let digit = match chars.next() {
                Some(c) => decode_digit(c),
                None => break 'outer,
            };

            i = digit
                .checked_mul(w)
                .expect("overflow multipliying digit by w")
                .checked_add(i)
                .expect("overflow while incrementing i");

            let t = threshold(k * BASE, bias);

            if digit < t {
                break;
            }

            w = BASE
                .checked_sub(t)
                .expect("overflow subtracting t from BASE")
                .checked_mul(w)
                .expect("overflow while increasing w");
        }

        let len_plus_one = (output.len() + 1) as u32;

        bias = adaptation(i - old_i, len_plus_one, old_i == 0);

        n = n
            .checked_add(i / len_plus_one)
            .expect("overflow while incrementing n");

        i = i % len_plus_one;

        let c = char::from_u32(n).expect("invalid unicode codepoint");

        output.insert(i as usize, c);

        i += 1;
    }
    Ok(output.iter().collect())
}

pub fn encode_punycode(input: &str) -> String {
    let mut n = INIT_N;
    let mut delta = 0;
    let mut bias = INIT_BIAS;

    let mut output = input.chars().filter(|c| c.is_ascii()).collect_vec();

    let mut h = output.len() as u32;
    let b = h;

    if b > 0 {
        output.push(DELIM);
    }

    while h < input.chars().count() as u32 {
        // Find the first character that is greater than or equal to n
        let m = input
            .chars()
            .map(|c| u32::from(c))
            .filter(|c| c >= &n)
            .fold(u32::MAX, |acc, f| u32::from(f).min(acc));

        delta += (m - n)
            .checked_mul(h + 1)
            .expect("overflow multiplying m-n with h+1");
        n = m;
        for c in input.chars() {
            if u32::from(c) < n {
                delta += 1;
            }
            if u32::from(c) == n {
                let mut q = delta;
                for k in 1.. {
                    let t = threshold(k * BASE, bias);

                    if q < t {
                        break;
                    }

                    output.push(encode_digit(t + (q - t) % (BASE - t)));

                    q = (q - t) / (BASE - t)
                }
                output.push(encode_digit(q));
                bias = adaptation(delta, h + 1, h == b);
                delta = 0;
                h += 1
            }
        }

        delta += 1;
        n += 1;
    }

    output.iter().collect()
}

#[test]
fn check_digit_value() {
    for (c, n) in ('a'..='z').chain('0'..='9').zip(0..=35) {
        assert!(
            decode_digit(c) == n,
            "the digit value of {c} should be {n}, but found {}",
            decode_digit(c)
        )
    }
    for (c, n) in ('A'..='Z').zip(0..=25) {
        assert!(
            decode_digit(c) == n,
            "the digit value of {c} should be {n}, but found {}",
            decode_digit(c)
        )
    }
}

#[test]
fn check_value_digit() {
    for (c, n) in ('a'..='z').chain('0'..='9').zip(0..=35) {
        assert!(
            encode_digit(n) == c,
            "the digit value of {c} should be {n}, but found {}",
            encode_digit(n)
        )
    }
}

#[cfg(test)]
static TEST_STRINGS: [(&'static str,&'static str); 3] = [
    //ليهمابتكلموشعربي؟
    ("\u{0644}\u{064A}\u{0647}\u{0645}\u{0627}\u{0628}\u{062A}\u{0643}\u{0644}\u{0645}\u{0648}\u{0634}\u{0639}\u{0631}\u{0628}\u{064A}\u{061F}","egbpdaj6bu4bxfgehfvwxn"),
    // なぜみんな日本語を話してくれないのか
    ("\u{306A}\u{305C}\u{307F}\u{3093}\u{306A}\u{65E5}\u{672C}\u{8A9E}\u{3092}\u{8A71}\u{3057}\u{3066}\u{304F}\u{308C}\u{306A}\u{3044}\u{306E}\u{304B}","n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa"),
    //TạisaohọkhôngthểchỉnóitiếngViệt
    ("\u{0054}\u{1EA1}\u{0069}\u{0073}\u{0061}\u{006F}\u{0068}\u{1ECD}\u{006B}\u{0068}\u{00F4}\u{006E}\u{0067}\u{0074}\u{0068}\u{1EC3}\u{0063}\u{0068}\u{1EC9}\u{006E}\u{00F3}\u{0069}\u{0074}\u{0069}\u{1EBF}\u{006E}\u{0067}\u{0056}\u{0069}\u{1EC7}\u{0074}","TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g")
];

#[test]
fn punycode_decode() {
    for (raw, punycode) in TEST_STRINGS {
        let decoded = decode_punycode(punycode).unwrap();
        assert!(raw == decoded, "expected {raw} but found {decoded}");
    }
}

#[test]
fn punycode_encode() {
    for (raw, punycode) in TEST_STRINGS {
        let encoded = encode_punycode(raw);
        assert!(
            punycode == encoded,
            "expected {punycode} but found {encoded}"
        );
    }
}
