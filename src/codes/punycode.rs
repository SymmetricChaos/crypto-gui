// https://datatracker.ietf.org/doc/html/rfc3492

use itertools::Itertools;

use crate::errors::Error;

use super::Code;

pub struct Punycode {
    pub tmin: u32,
    pub tmax: u32,
    pub damp: u32,
    pub skew: u32,
    pub init_bias: u32,
    pub init_n: u32,
    pub base: u32,
    pub delim: char,
}

impl Punycode {
    pub fn adaptation(&self, delta: u32, num_points: u32, first_time: bool) -> u32 {
        let mut delta = if first_time {
            delta / self.damp
        } else {
            delta / 2
        };
        delta += delta / num_points;
        let mut k = 0;
        while delta > ((self.base - self.tmin) * self.tmax) / 2 {
            delta /= self.base - self.tmin;
            k += self.base;
        }
        k + (((self.base - self.tmin + 1) * delta) / (delta + self.skew))
    }

    pub fn decode_digit(c: char) -> Result<u32, Error> {
        Ok(match c {
            'a'..='z' => u32::from(c) - 97,
            'A'..='Z' => u32::from(c) - 65,
            '0'..='9' => u32::from(c) - 22,
            _ => return Err(Error::state("invalid punycode digit")),
        })
    }

    pub fn encode_digit(n: u32) -> Result<char, Error> {
        Ok(match n {
            0..=25 => char::from_u32(n + 97).unwrap(),
            26..=35 => char::from_u32(n + 22).unwrap(),
            _ => return Err(Error::state("invalid punycode digit")),
        })
    }

    pub fn threshold(&self, k: u32, bias: u32) -> u32 {
        if k <= bias + self.tmin {
            self.tmin
        } else if k >= bias + self.tmax {
            self.tmax
        } else {
            k - bias
        }
    }
}

impl Default for Punycode {
    fn default() -> Self {
        Self {
            tmin: 1,
            tmax: 26,
            damp: 700,
            skew: 38,
            init_bias: 72,
            init_n: 128,
            base: 36,
            delim: '-',
        }
    }
}

impl Code for Punycode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut n = self.init_n;
        let mut delta = 0;
        let mut bias = self.init_bias;

        let mut output = text.chars().filter(|c| c.is_ascii()).collect_vec();

        let mut h = output.len() as u32;
        let b = h;

        if b > 0 {
            output.push(self.delim);
        }

        while h < text.chars().count() as u32 {
            // Find the first character that is greater than or equal to n
            let m = text
                .chars()
                .map(|c| u32::from(c))
                .filter(|c| c >= &n)
                .fold(u32::MAX, |acc, f| u32::from(f).min(acc));

            delta += m
                .checked_sub(n)
                .ok_or(Error::state("overflow subtracting m from n"))?
                .checked_mul(h + 1)
                .ok_or(Error::state("overflow multiplying (m-n) by (h+1)"))?;
            n = m;
            for c in text.chars() {
                if u32::from(c) < n {
                    delta += 1;
                }
                if u32::from(c) == n {
                    let mut q = delta;
                    for k in 1.. {
                        let t = self.threshold(k * self.base, bias);

                        if q < t {
                            break;
                        }

                        output.push(Self::encode_digit(t + (q - t) % (self.base - t))?);

                        q = (q - t) / (self.base - t)
                    }
                    output.push(Self::encode_digit(q)?);
                    bias = self.adaptation(delta, h + 1, h == b);
                    delta = 0;
                    h += 1
                }
            }

            delta += 1;
            n += 1;
        }

        Ok(output.iter().collect())
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut n = self.init_n;
        let mut i = 0;
        let mut bias = self.init_bias;

        if !text.is_ascii() {
            return Err(Error::input("Punycode can only decode ASCII characters"));
        }

        // This gives a byte index but because we only have ASCII characters this is also the character index
        let delim_pos = text.rfind(self.delim);

        let mut output = match delim_pos {
            Some(i) => text.chars().take(i).collect_vec(),
            None => Vec::new(),
        };

        // Rather than decoding whole punycode string we'll just decode the part after the delimeter
        let mut chars = match delim_pos {
            Some(i) => text.chars().skip(i + 1),
            None => text.chars().skip(0),
        };

        'outer: loop {
            let old_i = i;
            let mut w = 1;
            for k in 1.. {
                let digit = match chars.next() {
                    Some(c) => Self::decode_digit(c)?,
                    None => break 'outer,
                };

                i = digit
                    .checked_mul(w)
                    .ok_or(Error::state("overflow multipliying digit by w"))?
                    .checked_add(i)
                    .ok_or(Error::state("overflow incrementing i"))?;

                let t = self.threshold(k * self.base, bias);

                if digit < t {
                    break;
                }

                w = self
                    .base
                    .checked_sub(t)
                    .ok_or(Error::state("overflow subtracting t from self.base"))?
                    .checked_mul(w)
                    .ok_or(Error::state("overflow multiplying (self.base - t) by w"))?;
            }

            let len_plus_one = (output.len() + 1) as u32;

            bias = self.adaptation(i - old_i, len_plus_one, old_i == 0);

            n = n
                .checked_add(i / len_plus_one)
                .ok_or(Error::state("overflow while incrementing n"))?;

            i = i % len_plus_one;

            let c = char::from_u32(n).ok_or(Error::state("invalid unicode codepoint"))?;

            output.insert(i as usize, c);

            i += 1;
        }
        Ok(output.iter().collect())
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
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
    let code = Punycode::default();
    for (raw, punycode) in TEST_STRINGS {
        let decoded = code.decode(punycode).unwrap();
        assert!(raw == decoded, "expected {raw} but found {decoded}");
    }
}

#[test]
fn punycode_encode() {
    let code = Punycode::default();
    for (raw, punycode) in TEST_STRINGS {
        let encoded = code.encode(raw).unwrap();
        assert!(
            punycode == encoded,
            "expected {punycode} but found {encoded}"
        );
    }
}
