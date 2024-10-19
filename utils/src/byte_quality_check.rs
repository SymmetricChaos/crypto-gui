pub fn assert_eq_u8s<T: AsRef<[u8]>>(a: T, b: T) {
    assert!(
        a.as_ref() == b.as_ref(),
        "not equal:\n left:  {:02x?}\n right: {:02x?}",
        a.as_ref(),
        b.as_ref()
    );
}

pub fn assert_eq_u16s<T: AsRef<[u16]>>(a: T, b: T) {
    assert!(
        a.as_ref() == b.as_ref(),
        "not equal:\n left:  {:04x?}\n right: {:04x?}",
        a.as_ref(),
        b.as_ref()
    );
}

pub fn assert_eq_u32s<T: AsRef<[u32]>>(a: T, b: T) {
    assert!(
        a.as_ref() == b.as_ref(),
        "not equal:\n left:  {:08x?}\n right: {:08x?}",
        a.as_ref(),
        b.as_ref()
    );
}

pub fn assert_eq_u64s<T: AsRef<[u64]>>(a: T, b: T) {
    assert!(
        a.as_ref() == b.as_ref(),
        "not equal:\n left:  {:016x?}\n right: {:016x?}",
        a.as_ref(),
        b.as_ref()
    );
}
