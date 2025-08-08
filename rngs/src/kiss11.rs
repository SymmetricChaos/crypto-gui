use crate::SimpleRng;

/*
static unsigned long Q[4194304],carry=0;
unsigned long int i,x,cng=123456789,xs=362436069;
unsigned long b32MWC(void)
{unsigned long t,x; static int j=4194303;
j=(j+1)&4194303;
x=Q[j]; t=(x<<28)+carry;
carry=(x>>4)-(t<x);
return (Q[j]=t-x);
}
#define CNG ( cng=69069*cng+13579 )
#define XS ( xs^=(xs<<13), xs^=(xs>>17), xs^=(xs<<5) )
#define KISS ( b32MWC()+CNG+XS )
*/

// Because of the 16MB state its unclear how to key this effectively
// Also probably not a great idea for the website?

pub struct Kiss11 {
    pub q: Box<[u32]>,
    pub carry: u32,
    pub j: u32,
    pub xs: u32,
    pub cng: u32,
}

impl Default for Kiss11 {
    fn default() -> Self {
        Self {
            q: vec![0; 4194304].into_boxed_slice(), // avoids causing a stack overflow
            carry: 0,
            j: 4194303,
            xs: 362436069,
            cng: 123456789,
        }
    }
}

impl Kiss11 {
    // Pair of 16 bit multiply with carry generators
    fn b32mwc(&mut self) -> u32 {
        self.j = (self.j + 1) & 0x3FFFFF;
        let x = self.q[self.j as usize];
        let t = (x << 28).wrapping_add(self.carry);
        self.carry = (x >> 4).wrapping_sub((t < x) as u32);
        self.q[self.j as usize] = t.wrapping_sub(x);
        self.q[self.j as usize]
    }

    // An xorshift generator
    fn shr3(&mut self) -> u32 {
        self.xs ^= self.xs << 13;
        self.xs ^= self.xs >> 17;
        self.xs ^= self.xs << 5;
        self.xs
    }

    // A linear congruential generator
    fn cng(&mut self) -> u32 {
        self.cng = 69069_u32.wrapping_mul(self.cng).wrapping_add(13579);
        self.cng
    }
}

impl SimpleRng for Kiss11 {
    fn next_u32(&mut self) -> u32 {
        self.b32mwc()
            .wrapping_add(self.cng())
            .wrapping_add(self.shr3())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs() {
        let mut rng = Kiss11::default();

        // // Test values generated from the C code with the word size defined as uint32_t
        assert_eq!(0x17001492, rng.next_u32());
        assert_eq!(0xa55c149a, rng.next_u32());
        assert_eq!(0xc588fbdb, rng.next_u32());
        assert_eq!(0x6b8118d5, rng.next_u32());
        assert_eq!(0x3f1839f5, rng.next_u32());
        assert_eq!(0x6bb10a95, rng.next_u32());
        assert_eq!(0x6f28b782, rng.next_u32());
        assert_eq!(0x33daffd8, rng.next_u32());
        assert_eq!(0x01c39cb6, rng.next_u32());
        assert_eq!(0x1cc08292, rng.next_u32());
        assert_eq!(0x2346db31, rng.next_u32());
        assert_eq!(0x6bca34a4, rng.next_u32());
        assert_eq!(0x3f4599ae, rng.next_u32());
        assert_eq!(0xc2129669, rng.next_u32());
        assert_eq!(0x39ba100b, rng.next_u32());
        assert_eq!(0x595cca50, rng.next_u32());
    }
}
