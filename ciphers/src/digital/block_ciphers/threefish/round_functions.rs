#[inline]
fn subkey_add(state: &mut [u64], key: &[u64]) {
    for (s, k) in state.iter_mut().zip(key.iter()) {
        *s = s.wrapping_add(*k)
    }
}

#[inline]
fn subkey_sub(state: &mut [u64], key: &[u64]) {
    for (s, k) in state.iter_mut().zip(key.iter()) {
        *s = s.wrapping_sub(*k)
    }
}

macro_rules! threefish_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

macro_rules! threefish_unmix {
    ($a: expr, $b: expr, $r: literal) => {
        $b = ($a ^ $b).rotate_right($r);
        $a = $a.wrapping_sub($b);
    };
}

#[inline(always)]
pub fn octo_round_256(w: &mut [u64; 4], subkey: &[[u64; 4]]) {
    subkey_add(w, &subkey[0]);

    threefish_mix!(w[0], w[1], 14);
    threefish_mix!(w[2], w[3], 16);

    threefish_mix!(w[0], w[3], 52);
    threefish_mix!(w[2], w[1], 57);

    threefish_mix!(w[0], w[1], 23);
    threefish_mix!(w[2], w[3], 40);

    threefish_mix!(w[0], w[3], 5);
    threefish_mix!(w[2], w[1], 37);

    subkey_add(w, &subkey[1]);

    threefish_mix!(w[0], w[1], 25);
    threefish_mix!(w[2], w[3], 33);

    threefish_mix!(w[0], w[3], 46);
    threefish_mix!(w[2], w[1], 12);

    threefish_mix!(w[0], w[1], 58);
    threefish_mix!(w[2], w[3], 22);

    threefish_mix!(w[0], w[3], 32);
    threefish_mix!(w[2], w[1], 32);
}

#[inline(always)]
pub fn octo_round_256_inv(w: &mut [u64; 4], subkey: &[[u64; 4]]) {
    threefish_unmix!(w[2], w[1], 32);
    threefish_unmix!(w[0], w[3], 32);

    threefish_unmix!(w[2], w[3], 22);
    threefish_unmix!(w[0], w[1], 58);

    threefish_unmix!(w[2], w[1], 12);
    threefish_unmix!(w[0], w[3], 46);

    threefish_unmix!(w[2], w[3], 33);
    threefish_unmix!(w[0], w[1], 25);

    subkey_sub(w, &subkey[1]);

    threefish_unmix!(w[2], w[1], 37);
    threefish_unmix!(w[0], w[3], 5);

    threefish_unmix!(w[2], w[3], 40);
    threefish_unmix!(w[0], w[1], 23);

    threefish_unmix!(w[2], w[1], 57);
    threefish_unmix!(w[0], w[3], 52);

    threefish_unmix!(w[2], w[3], 16);
    threefish_unmix!(w[0], w[1], 14);

    subkey_sub(w, &subkey[0]);
}

#[inline(always)]
pub fn octo_round_512(w: &mut [u64; 8], subkey: &[[u64; 8]]) {
    subkey_add(w, &subkey[0]);

    threefish_mix!(w[0], w[1], 46);
    threefish_mix!(w[2], w[3], 36);
    threefish_mix!(w[4], w[5], 19);
    threefish_mix!(w[6], w[7], 37);

    threefish_mix!(w[2], w[1], 33);
    threefish_mix!(w[4], w[7], 27);
    threefish_mix!(w[6], w[5], 14);
    threefish_mix!(w[0], w[3], 42);

    threefish_mix!(w[4], w[1], 17);
    threefish_mix!(w[6], w[3], 49);
    threefish_mix!(w[0], w[5], 36);
    threefish_mix!(w[2], w[7], 39);

    threefish_mix!(w[6], w[1], 44);
    threefish_mix!(w[0], w[7], 9);
    threefish_mix!(w[2], w[5], 54);
    threefish_mix!(w[4], w[3], 56);

    subkey_add(w, &subkey[1]);

    threefish_mix!(w[0], w[1], 39);
    threefish_mix!(w[2], w[3], 30);
    threefish_mix!(w[4], w[5], 34);
    threefish_mix!(w[6], w[7], 24);

    threefish_mix!(w[2], w[1], 13);
    threefish_mix!(w[4], w[7], 50);
    threefish_mix!(w[6], w[5], 10);
    threefish_mix!(w[0], w[3], 17);

    threefish_mix!(w[4], w[1], 25);
    threefish_mix!(w[6], w[3], 29);
    threefish_mix!(w[0], w[5], 39);
    threefish_mix!(w[2], w[7], 43);

    threefish_mix!(w[6], w[1], 8);
    threefish_mix!(w[0], w[7], 35);
    threefish_mix!(w[2], w[5], 56);
    threefish_mix!(w[4], w[3], 22);
}

#[inline(always)]
pub fn octo_round_512_inv(w: &mut [u64; 8], subkey: &[[u64; 8]]) {
    threefish_unmix!(w[4], w[3], 22);
    threefish_unmix!(w[2], w[5], 56);
    threefish_unmix!(w[0], w[7], 35);
    threefish_unmix!(w[6], w[1], 8);

    threefish_unmix!(w[2], w[7], 43);
    threefish_unmix!(w[0], w[5], 39);
    threefish_unmix!(w[6], w[3], 29);
    threefish_unmix!(w[4], w[1], 25);

    threefish_unmix!(w[0], w[3], 17);
    threefish_unmix!(w[6], w[5], 10);
    threefish_unmix!(w[4], w[7], 50);
    threefish_unmix!(w[2], w[1], 13);

    threefish_unmix!(w[6], w[7], 24);
    threefish_unmix!(w[4], w[5], 34);
    threefish_unmix!(w[2], w[3], 30);
    threefish_unmix!(w[0], w[1], 39);

    subkey_sub(w, &subkey[1]);

    threefish_unmix!(w[4], w[3], 56);
    threefish_unmix!(w[2], w[5], 54);
    threefish_unmix!(w[0], w[7], 9);
    threefish_unmix!(w[6], w[1], 44);

    threefish_unmix!(w[2], w[7], 39);
    threefish_unmix!(w[0], w[5], 36);
    threefish_unmix!(w[6], w[3], 49);
    threefish_unmix!(w[4], w[1], 17);

    threefish_unmix!(w[0], w[3], 42);
    threefish_unmix!(w[6], w[5], 14);
    threefish_unmix!(w[4], w[7], 27);
    threefish_unmix!(w[2], w[1], 33);

    threefish_unmix!(w[6], w[7], 37);
    threefish_unmix!(w[4], w[5], 19);
    threefish_unmix!(w[2], w[3], 36);
    threefish_unmix!(w[0], w[1], 46);

    subkey_sub(w, &subkey[0]);
}

#[inline(always)]
pub fn octo_round_1024(w: &mut [u64; 16], subkey: &[[u64; 16]]) {
    subkey_add(w, &subkey[0]);

    threefish_mix!(w[0], w[1], 24);
    threefish_mix!(w[2], w[3], 13);
    threefish_mix!(w[4], w[5], 8);
    threefish_mix!(w[6], w[7], 47);
    threefish_mix!(w[8], w[9], 8);
    threefish_mix!(w[10], w[11], 17);
    threefish_mix!(w[12], w[13], 22);
    threefish_mix!(w[14], w[15], 37);

    threefish_mix!(w[0], w[9], 38);
    threefish_mix!(w[2], w[13], 19);
    threefish_mix!(w[6], w[11], 10);
    threefish_mix!(w[4], w[15], 55);
    threefish_mix!(w[10], w[7], 49);
    threefish_mix!(w[12], w[3], 18);
    threefish_mix!(w[14], w[5], 23);
    threefish_mix!(w[8], w[1], 52);

    threefish_mix!(w[0], w[7], 33);
    threefish_mix!(w[2], w[5], 4);
    threefish_mix!(w[4], w[3], 51);
    threefish_mix!(w[6], w[1], 13);
    threefish_mix!(w[12], w[15], 34);
    threefish_mix!(w[14], w[13], 41);
    threefish_mix!(w[8], w[11], 59);
    threefish_mix!(w[10], w[9], 17);

    threefish_mix!(w[0], w[15], 5);
    threefish_mix!(w[2], w[11], 20);
    threefish_mix!(w[6], w[13], 48);
    threefish_mix!(w[4], w[9], 41);
    threefish_mix!(w[14], w[1], 47);
    threefish_mix!(w[8], w[5], 28);
    threefish_mix!(w[10], w[3], 16);
    threefish_mix!(w[12], w[7], 25);

    subkey_add(w, &subkey[1]);

    threefish_mix!(w[0], w[1], 41);
    threefish_mix!(w[2], w[3], 9);
    threefish_mix!(w[4], w[5], 37);
    threefish_mix!(w[6], w[7], 31);
    threefish_mix!(w[8], w[9], 12);
    threefish_mix!(w[10], w[11], 47);
    threefish_mix!(w[12], w[13], 44);
    threefish_mix!(w[14], w[15], 30);

    threefish_mix!(w[0], w[9], 16);
    threefish_mix!(w[2], w[13], 34);
    threefish_mix!(w[6], w[11], 56);
    threefish_mix!(w[4], w[15], 51);
    threefish_mix!(w[10], w[7], 4);
    threefish_mix!(w[12], w[3], 53);
    threefish_mix!(w[14], w[5], 42);
    threefish_mix!(w[8], w[1], 41);

    threefish_mix!(w[0], w[7], 31);
    threefish_mix!(w[2], w[5], 44);
    threefish_mix!(w[4], w[3], 47);
    threefish_mix!(w[6], w[1], 46);
    threefish_mix!(w[12], w[15], 19);
    threefish_mix!(w[14], w[13], 42);
    threefish_mix!(w[8], w[11], 44);
    threefish_mix!(w[10], w[9], 25);

    threefish_mix!(w[0], w[15], 9);
    threefish_mix!(w[2], w[11], 48);
    threefish_mix!(w[6], w[13], 35);
    threefish_mix!(w[4], w[9], 52);
    threefish_mix!(w[14], w[1], 23);
    threefish_mix!(w[8], w[5], 31);
    threefish_mix!(w[10], w[3], 37);
    threefish_mix!(w[12], w[7], 20);
}

#[inline(always)]
pub fn octo_round_1024_inv(w: &mut [u64; 16], subkey: &[[u64; 16]]) {
    threefish_unmix!(w[12], w[7], 20);
    threefish_unmix!(w[10], w[3], 37);
    threefish_unmix!(w[8], w[5], 31);
    threefish_unmix!(w[14], w[1], 23);
    threefish_unmix!(w[4], w[9], 52);
    threefish_unmix!(w[6], w[13], 35);
    threefish_unmix!(w[2], w[11], 48);
    threefish_unmix!(w[0], w[15], 9);

    threefish_unmix!(w[10], w[9], 25);
    threefish_unmix!(w[8], w[11], 44);
    threefish_unmix!(w[14], w[13], 42);
    threefish_unmix!(w[12], w[15], 19);
    threefish_unmix!(w[6], w[1], 46);
    threefish_unmix!(w[4], w[3], 47);
    threefish_unmix!(w[2], w[5], 44);
    threefish_unmix!(w[0], w[7], 31);

    threefish_unmix!(w[8], w[1], 41);
    threefish_unmix!(w[14], w[5], 42);
    threefish_unmix!(w[12], w[3], 53);
    threefish_unmix!(w[10], w[7], 4);
    threefish_unmix!(w[4], w[15], 51);
    threefish_unmix!(w[6], w[11], 56);
    threefish_unmix!(w[2], w[13], 34);
    threefish_unmix!(w[0], w[9], 16);

    threefish_unmix!(w[14], w[15], 30);
    threefish_unmix!(w[12], w[13], 44);
    threefish_unmix!(w[10], w[11], 47);
    threefish_unmix!(w[8], w[9], 12);
    threefish_unmix!(w[6], w[7], 31);
    threefish_unmix!(w[4], w[5], 37);
    threefish_unmix!(w[2], w[3], 9);
    threefish_unmix!(w[0], w[1], 41);

    subkey_sub(w, &subkey[1]);

    threefish_unmix!(w[12], w[7], 25);
    threefish_unmix!(w[10], w[3], 16);
    threefish_unmix!(w[8], w[5], 28);
    threefish_unmix!(w[14], w[1], 47);
    threefish_unmix!(w[4], w[9], 41);
    threefish_unmix!(w[6], w[13], 48);
    threefish_unmix!(w[2], w[11], 20);
    threefish_unmix!(w[0], w[15], 5);

    threefish_unmix!(w[10], w[9], 17);
    threefish_unmix!(w[8], w[11], 59);
    threefish_unmix!(w[14], w[13], 41);
    threefish_unmix!(w[12], w[15], 34);
    threefish_unmix!(w[6], w[1], 13);
    threefish_unmix!(w[4], w[3], 51);
    threefish_unmix!(w[2], w[5], 4);
    threefish_unmix!(w[0], w[7], 33);

    threefish_unmix!(w[8], w[1], 52);
    threefish_unmix!(w[14], w[5], 23);
    threefish_unmix!(w[12], w[3], 18);
    threefish_unmix!(w[10], w[7], 49);
    threefish_unmix!(w[4], w[15], 55);
    threefish_unmix!(w[6], w[11], 10);
    threefish_unmix!(w[2], w[13], 19);
    threefish_unmix!(w[0], w[9], 38);

    threefish_unmix!(w[14], w[15], 37);
    threefish_unmix!(w[12], w[13], 22);
    threefish_unmix!(w[10], w[11], 17);
    threefish_unmix!(w[8], w[9], 8);
    threefish_unmix!(w[6], w[7], 47);
    threefish_unmix!(w[4], w[5], 8);
    threefish_unmix!(w[2], w[3], 13);
    threefish_unmix!(w[0], w[1], 24);

    subkey_sub(w, &subkey[0]);
}
