// TODO
pub struct Scrypt {
    salt: Vec<u8>,
    cost: u32,
    blocksize: u32,
    paralleism: u32,
    key_len: u32,
    h_len: u32,
    mf_len: u32,
}
