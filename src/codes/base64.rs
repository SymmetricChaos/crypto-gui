const B64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
                            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
                            'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                            'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                            'w', 'x', 'y', 'z', '0', '1', '2', '3',
                            '4', '5', '6', '7', '8', '9', '+', '/'];
 
pub fn encode_b64(input: &[u8]) -> String {
    let mut out = String::new();
    // take three bytes at a time
    for chunk in input.chunks(3) {
 
        // turn the three bytes into four sextets
        // shr chunk[0] twice to keep only the top six bits
        let s1 = (chunk[0] >> 2) as usize; 
        // shl chunk[0] 4 times to put the bottom top 2 bits on top, mask the top two bits, then shr[1] 4 times to put the top four bits on the bottom, XOR together
        let s2 = (((chunk[0] << 4) & 0x3F) ^ (chunk[1] >> 4)) as usize;
        // shl chunk[1] 2 times to leave two bits open at the bottom, mask the top two bits, shr chunk[2] 6 times to put the bottom two bits on the bottom XOR together 
        let s3 = (((chunk[1] << 2) & 0x3F) ^ (chunk[2] >> 6)) as usize;
        // mask the top two bits of chunk[2]
        let s4 = (chunk[2] & 0x3F) as usize;
 
        out.push(B64[s1]);
        out.push(B64[s2]);
        out.push(B64[s3]);
        out.push(B64[s4]);
    }
    out
}

pub struct Base64 {
    
}