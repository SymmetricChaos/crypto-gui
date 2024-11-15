const MASK: u8 = 0b01111111;
const HI_BIT: u8 = 0b10000000;

pub struct Leb128 {
    signed: bool
}

impl Leb128 {
    fn i64_leb128(n: i64) -> Vec<u8> {
        todo!()
    }

    fn u64_leb128(n: u64) -> Vec<u8> {
        if n == 0 {
            return vec![HI_BIT]
        }
        let mut out =Vec::with_capacity(8);
        for i in 0..8 {
            out.push(((n >> (7*i)) as u8) & MASK);
        }

        todo!()
    }
}

impl Code for Leb128 {

}