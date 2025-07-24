pub mod threefry;

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;

macro_rules! subkey_add {
    ($state: expr, $key: expr) => {
        for (s, k) in $state.iter_mut().zip($key.iter()) {
            *s = s.wrapping_add(*k)
        }
    };
}

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

#[inline]
pub fn threefry_64_4_12(w: &mut [u64; 4], key: &[u64; 5]) {
    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    w[0] = w[0].wrapping_add(key[0]);
    w[1] = w[1].wrapping_add(key[1]);
    w[2] = w[2].wrapping_add(key[2]);
    w[3] = w[3].wrapping_add(key[3]);
    w[3] = w[3].wrapping_add(1);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);

    w[0] = w[0].wrapping_add(key[2]);
    w[1] = w[1].wrapping_add(key[3]);
    w[2] = w[2].wrapping_add(key[4]);
    w[3] = w[3].wrapping_add(key[0]);
    w[3] = w[3].wrapping_add(2);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    w[0] = w[0].wrapping_add(key[3]);
    w[1] = w[1].wrapping_add(key[4]);
    w[2] = w[2].wrapping_add(key[0]);
    w[3] = w[3].wrapping_add(key[1]);
    w[3] = w[3].wrapping_add(3);
}

#[inline]
pub fn threefry_64_4_20(w: &mut [u64; 4], key: &[u64; 5]) {
    w[0] = w[0].wrapping_add(key[0]);
    w[1] = w[1].wrapping_add(key[1]);
    w[2] = w[2].wrapping_add(key[2]);
    w[3] = w[3].wrapping_add(key[3]);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    w[0] = w[0].wrapping_add(key[1]);
    w[1] = w[1].wrapping_add(key[2]);
    w[2] = w[2].wrapping_add(key[3]);
    w[3] = w[3].wrapping_add(key[4]);
    w[3] = w[3].wrapping_add(1);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);

    w[0] = w[0].wrapping_add(key[2]);
    w[1] = w[1].wrapping_add(key[3]);
    w[2] = w[2].wrapping_add(key[4]);
    w[3] = w[3].wrapping_add(key[0]);
    w[3] = w[3].wrapping_add(2);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    w[0] = w[0].wrapping_add(key[3]);
    w[1] = w[1].wrapping_add(key[4]);
    w[2] = w[2].wrapping_add(key[0]);
    w[3] = w[3].wrapping_add(key[1]);
    w[3] = w[3].wrapping_add(3);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);

    w[0] = w[0].wrapping_add(key[4]);
    w[1] = w[1].wrapping_add(key[0]);
    w[2] = w[2].wrapping_add(key[1]);
    w[3] = w[3].wrapping_add(key[2]);
    w[3] = w[3].wrapping_add(4);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    w[0] = w[0].wrapping_add(key[0]);
    w[1] = w[1].wrapping_add(key[1]);
    w[2] = w[2].wrapping_add(key[2]);
    w[3] = w[3].wrapping_add(key[3]);
    w[3] = w[3].wrapping_add(5);
}

#[inline]
pub fn threefry32_4_round(w: &mut [u32; 4], subkey: &[[u32; 4]]) {
    subkey_add!(w, &subkey[0]);

    skein_mix!(w[0], w[1], 10);
    skein_mix!(w[2], w[3], 26);

    skein_mix!(w[0], w[3], 11);
    skein_mix!(w[2], w[1], 21);

    skein_mix!(w[0], w[1], 13);
    skein_mix!(w[2], w[3], 27);

    skein_mix!(w[0], w[3], 23);
    skein_mix!(w[2], w[1], 5);

    subkey_add!(w, &subkey[1]);

    skein_mix!(w[0], w[1], 6);
    skein_mix!(w[2], w[3], 20);

    skein_mix!(w[0], w[3], 17);
    skein_mix!(w[2], w[1], 11);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 10);

    skein_mix!(w[0], w[3], 18);
    skein_mix!(w[2], w[1], 20);
}
