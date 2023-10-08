use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};

pub struct Gray {
    pub maps: LetterAndWordCode<usize>,
    pub width: usize,
    pub mode: IOMode,
}

impl Gray {
    pub fn encode_u32(&self, n: u32) -> String {
        let gray = n ^ (n >> 1);
        format!("{:0<1$b}", gray, self.width)
    }
}

impl Default for Gray {
    fn default() -> Self {
        let maps = LetterAndWordCode::<usize>::default();
        Self {
            maps,
            width: 4,
            mode: IOMode::Integer,
        }
    }
}

impl Code for Gray {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if self.mode == IOMode::Integer {
            let m = 2_u32.pow(self.width as u32);
            let mut out = String::new();
            for s in text.split(" ") {
                let n =
                    u32::from_str_radix(s, 10).map_err(|_| CodeError::invalid_input_group(s))?;
                if n >= m {
                    return Err(CodeError::Input(format!(
                        "for a width of {} inputs must be less than {}",
                        self.width, m
                    )));
                };
                out.push_str(&self.encode_u32(n))
            }
            Ok(out)
        } else if self.mode == IOMode::Letter {
            todo!()
        } else {
            todo!()
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod rgray_tests {
    use super::*;

    #[test]
    fn gray_code_generator() {
        let code = Gray::default();
        for n in 0..16 {
            println!("{}", code.encode_u32(n))
        }
    }
}
