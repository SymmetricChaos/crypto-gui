pub trait ClassicHasher<const N: usize> {
    fn hash(bytes: &[u8]) -> [u8; N];
    fn hash_string(bytes: &[u8]) -> String {
        let mut out = String::new();
        for byte in Self::hash(bytes) {
            out.push_str(&format!("{:02x}", byte))
        }
        out
    }
}
