use crate::traits::StatefulHasher;

// The Art Of Computer Programming Volume 3, Donald E. Knuth

pub struct Dek {
    buffer: Vec<u8>,
}

impl StatefulHasher for Dek {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        let mut out = self.buffer.len() as u32; // unclear what the original word size was meant to be
        for byte in self.buffer {
            out = (out << 5) ^ (out >> 27) * byte as u32;
        }
        out.to_be_bytes().to_vec()
    }
}
