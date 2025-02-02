use crate::traits::Code;

pub const MAX_VN: u32 = 15;

pub fn von_neumann_iterative(n: u32, sep: &str) -> String {
    let mut acc: Vec<String> = vec![];
    for _ in 0..n {
        let current = format!("{{{}}}", acc.join(sep));
        acc.push(current);
    }
    format!("{{{}}}", acc.join(sep))
}

pub struct VonNeumann {
    pub comma: bool,
    pub null_set: bool,
}

impl Default for VonNeumann {
    fn default() -> Self {
        Self {
            comma: false,
            null_set: false,
        }
    }
}

impl Code for VonNeumann {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        let mut out = Vec::new();
        let sep = if self.comma { "," } else { "" };
        for value in text.split(",").map(|s| s.trim()) {
            if let Ok(n) = u32::from_str_radix(value, 10) {
                if n <= MAX_VN {
                    out.push(von_neumann_iterative(n, sep));
                } else {
                    out.push(String::from("INPUT TOO LARGE"));
                }
            } else {
                out.push(String::from("INVALID INPUT"));
            }
        }
        Ok(out.join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod vn_tests {
    use super::*;

    const PLAINTEXT: &str = "0, 1, 2, 3, 4, -2, 20";
    const CODETEXT: &str = "{}, {{}}, {{}{{}}}, {{}{{}}{{}{{}}}}, {{}{{}}{{}{{}}}{{}{{}}{{}{{}}}}}, INVALID INPUT, INPUT TOO LARGE";

    #[test]
    fn encode() {
        let code = VonNeumann::default();
        assert_eq!(CODETEXT, code.encode(PLAINTEXT).unwrap())
    }
}
