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
        for i in 0..255 {
            arr[i as usize] = i;
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
        let mut arr = [0u8; 256];
        for n in 0..255 {
            arr[n as usize] = n;
        }
        // Perform 256 swaps
        let key_length = key.len();
        let mut j: u8 = 0;
        for n in 0..255 {
            j = j.wrapping_add(arr[n]).wrapping_add(key[n % key_length]);
            arr.swap(n, j as usize)
        }
        self.arr = arr;
        self.i = 0;
        self.j = 0;
    }

    pub fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.arr[self.i as usize]);
        self.arr.swap(self.i as usize, self.j as usize);
        let t = self.arr[self.i as usize].wrapping_add(self.arr[self.j as usize]);
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
