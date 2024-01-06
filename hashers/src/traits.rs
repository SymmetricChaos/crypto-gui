pub trait ClassicHasher<const N: usize> {
    fn hash(bytes: &[u8]) -> [u8; N];
}
