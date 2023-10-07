use utils::bits::Bit;

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
    pub fn u32_to_code(&self, n: u32) -> String {
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
        todo!()
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
            println!("{}", code.u32_to_code(n))
        }
    }
}
