use lazy_static::lazy_static; // 1.4.0
use std::collections::HashMap;
 
const B64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
 
lazy_static! {
 
    static ref B64_MAP: HashMap<u8,u8> = {
        let mut m = HashMap::with_capacity(64);
        for (pos, val) in B64.chars().enumerate() {
            m.insert(pos as u8,val as u8);
        }
        m
    };
 
    static ref B64_MAP_INV: HashMap<u8,u8> = {
        let mut m = HashMap::with_capacity(64);
        for (pos, val) in B64.chars().enumerate() {
            m.insert(val as u8, pos as u8);
        }
        m
    };
}
 
pub fn encode_b64(input: &[u8]) -> String {
    let mut out = String::new();
    // take three bytes at a time
    for chunk in input.chunks(3) {
 
        // turn the three bytes into four sextets
        // shr chunk[0] twice to keep only the top six bits
        let s1 = chunk[0] >> 2; 
        // shl chunk[0] 4 times to put the bottom top 2 bits on top, mask the top two bits, then shr[1] 4 times to put the top four bits on the bottom, XOR together
        let s2 = ((chunk[0] << 4) & 0x3F) ^ (chunk[1] >> 4);
        // shl chunk[1] 2 times to leave two bits open at the bottom, mask the top two bits, shr chunk[2] 6 times to put the bottom two bits on the bottom XOR together 
        let s3 = ((chunk[1] << 2) & 0x3F) ^ (chunk[2] >> 6);
        // mask the top two bits of chunk[2]
        let s4 = chunk[2] & 0x3F;
 
        out.push(B64_MAP[&s1] as char);
        out.push(B64_MAP[&s2] as char);
        out.push(B64_MAP[&s3] as char);
        out.push(B64_MAP[&s4] as char);
    }
    out
}
 
pub fn decode_b64(input: &[u8]) -> String {
 
    let mut out = String::with_capacity( (input.len()/4)*3 );
    for chunk in input.chunks(4) {
 
        let s1 = B64_MAP_INV[&chunk[0]];
        let s2 = B64_MAP_INV[&chunk[1]];
        let s3 = B64_MAP_INV[&chunk[2]];
        let s4 = B64_MAP_INV[&chunk[3]];
 
        // shift s1 left twice to leave two bits at the bottom, shift s2 right twice to put the top two bits on the bottom, XOR together
        let o1 = (s1 << 2) ^ (s2 >> 4);
        // shift s2 left four to leave four at the bottom, shift s3 right two times to put the top four bits on the bottom, XOR together
        let o2 = (s2 << 4) ^ (s3 >> 2);
        // shift s3 left six to leave four at the bottom, shift s3 right two times to put the top four bits on the bottom, XOR together
        let o3 = (s3 << 6) ^ s4;
 
        out.push(o1 as char);
        out.push(o2 as char);
        out.push(o3 as char);
    }
    out
}