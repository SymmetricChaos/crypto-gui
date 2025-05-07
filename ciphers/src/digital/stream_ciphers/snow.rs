use utils::byte_formatting::ByteFormat;

pub struct Snow1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for Snow1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

pub struct WordLfsr {
    words: [u32; 16],
}

pub struct Fsm {
    r1: u32,
    r2: u32,
}

impl Fsm {
    fn out(&self, s1: u32) -> u32 {
        s1.wrapping_add(self.r1) ^ self.r2
    }

    fn update(&mut self, s1: u32) {
        let temp = self.out(s1).wrapping_add(self.r2).rotate_left(7) ^ self.r1;
        self.r2 = sbox(self.r1);
        self.r1 = temp;
    }
}

fn sbox(n: u32) -> u32 {
    todo!()
}
