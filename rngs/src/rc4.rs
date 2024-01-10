use crate::traits::ClassicRng;

pub struct Rc4 {
    pub arr: [u8; 256],
    pub i: u8,
    pub j: u8,
    pub big_endian: bool,
}

impl Default for Rc4 {
    fn default() -> Self {
        let mut arr = [0u8; 256];
        for n in 0..256 {
            arr[n] = n as u8;
        }
        Self {
            arr,
            i: 0,
            j: 0,
            big_endian: true,
        }
    }
}

impl Rc4 {
    pub fn ksa(&mut self, key: &[u8]) {
        // Set array to identity permutation
        for n in 0..256 {
            self.arr[n] = n as u8;
        }
        // Perform 256 swaps
        let mut j: u8 = 0;
        for (i, k) in (0..256).zip(key.iter().cycle()) {
            j = j.wrapping_add(self.arr[i]).wrapping_add(*k);
            self.arr.swap(i, j as usize)
        }
        self.i = 0;
        self.j = 0;
    }

    pub fn si(&self) -> u8 {
        self.arr[self.i as usize]
    }

    pub fn sj(&self) -> u8 {
        self.arr[self.j as usize]
    }

    pub fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.si());
        self.arr.swap(self.i.into(), self.j.into());
        let t = self.si().wrapping_add(self.sj());
        self.arr[t as usize]
    }
}

impl ClassicRng for Rc4 {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        for i in 0..4 {
            bytes[i] = self.next_byte();
        }
        match self.big_endian {
            true => u32::from_be_bytes(bytes),
            false => u32::from_le_bytes(bytes),
        }
    }
}

#[cfg(test)]
mod rc4_tests {

    use super::*;

    #[test]
    fn keystream_test() {
        let mut cipher = Rc4::default();
        cipher.ksa(&0x0102030405060708_u64.to_be_bytes());

        println!("First 32 Bytes of Keystream for 0x0102030405060708");
        for byte in [
            0x97, 0xab, 0x8a, 0x1b, 0xf0, 0xaf, 0xb9, 0x61, 0x32, 0xf2, 0xf6, 0x72, 0x58, 0xda,
            0x15, 0xa8, 0x82, 0x63, 0xef, 0xdb, 0x45, 0xc4, 0xa1, 0x86, 0x84, 0xef, 0x87, 0xe6,
            0xb1, 0x9e, 0x5b, 0x09,
        ] {
            let b = cipher.next_byte();

            print!("{:02x} {:02x}", byte, b);
            if b != byte {
                println!(" ERROR")
            } else {
                println!("")
            }
        }

        cipher.ksa(&0x641910833222772a_u64.to_be_bytes());
        println!("\n\nFirst 32 Bytes of Keystream for 0x641910833222772a");
        for byte in [
            0xbb, 0xf6, 0x09, 0xde, 0x94, 0x13, 0x17, 0x2d, 0x07, 0x66, 0x0c, 0xb6, 0x80, 0x71,
            0x69, 0x26, 0x46, 0x10, 0x1a, 0x6d, 0xab, 0x43, 0x11, 0x5d, 0x6c, 0x52, 0x2b, 0x4f,
            0xe9, 0x36, 0x04, 0xa9,
        ] {
            let b = cipher.next_byte();

            print!("{:02x} {:02x}", byte, b);
            if b != byte {
                println!(" ERROR")
            } else {
                println!("")
            }
        }
    }
}
