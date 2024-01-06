pub trait ClassicHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_to_string(&self, bytes: &[u8]) -> String {
        let mut out = String::new();
        for byte in self.hash(bytes) {
            out.push_str(&format!("{:02x}", byte))
        }
        out
    }
}
