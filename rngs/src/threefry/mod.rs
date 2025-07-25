pub mod threefry;

// The number 240 encrypted with AES with an all zero key
const C240: u64 = 0x1BD11BDAA9FC1A22;

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

// Yes, this is incredibly awkard
// A real implementation should just pick a specific round count like 12 or 20
#[inline]
pub fn threefry_64_4_r(w: &mut [u64; 4], key: &[u64; 5], rounds: usize) {
    w[0] = w[0].wrapping_add(key[0]);
    w[1] = w[1].wrapping_add(key[1]);
    w[2] = w[2].wrapping_add(key[2]);
    w[3] = w[3].wrapping_add(key[3]);

    if rounds >= 1 {
        skein_mix!(w[0], w[1], 14);
        skein_mix!(w[2], w[3], 16);
    }

    if rounds >= 2 {
        skein_mix!(w[0], w[3], 52);
        skein_mix!(w[2], w[1], 57);
    }

    if rounds >= 3 {
        skein_mix!(w[0], w[1], 23);
        skein_mix!(w[2], w[3], 40);
    }

    if rounds >= 4 {
        skein_mix!(w[0], w[3], 5);
        skein_mix!(w[2], w[1], 37);

        w[0] = w[0].wrapping_add(key[1]);
        w[1] = w[1].wrapping_add(key[2]);
        w[2] = w[2].wrapping_add(key[3]);
        w[3] = w[3].wrapping_add(key[4]);
        w[3] = w[3].wrapping_add(1);
    }

    if rounds >= 5 {
        skein_mix!(w[0], w[1], 25);
        skein_mix!(w[2], w[3], 33);
    }

    if rounds >= 6 {
        skein_mix!(w[0], w[3], 46);
        skein_mix!(w[2], w[1], 12);
    }

    if rounds >= 7 {
        skein_mix!(w[0], w[1], 58);
        skein_mix!(w[2], w[3], 22);
    }

    if rounds >= 8 {
        skein_mix!(w[0], w[3], 32);
        skein_mix!(w[2], w[1], 32);

        w[0] = w[0].wrapping_add(key[2]);
        w[1] = w[1].wrapping_add(key[3]);
        w[2] = w[2].wrapping_add(key[4]);
        w[3] = w[3].wrapping_add(key[0]);
        w[3] = w[3].wrapping_add(2);
    }

    if rounds >= 9 {
        skein_mix!(w[0], w[1], 14);
        skein_mix!(w[2], w[3], 16);
    }

    if rounds >= 10 {
        skein_mix!(w[0], w[3], 52);
        skein_mix!(w[2], w[1], 57);
    }

    if rounds >= 11 {
        skein_mix!(w[0], w[1], 23);
        skein_mix!(w[2], w[3], 40);
    }

    if rounds >= 12 {
        skein_mix!(w[0], w[3], 5);
        skein_mix!(w[2], w[1], 37);

        w[0] = w[0].wrapping_add(key[3]);
        w[1] = w[1].wrapping_add(key[4]);
        w[2] = w[2].wrapping_add(key[0]);
        w[3] = w[3].wrapping_add(key[1]);
        w[3] = w[3].wrapping_add(3);
    }

    if rounds >= 13 {
        skein_mix!(w[0], w[1], 25);
        skein_mix!(w[2], w[3], 33);
    }

    if rounds >= 14 {
        skein_mix!(w[0], w[3], 46);
        skein_mix!(w[2], w[1], 12);
    }

    if rounds >= 15 {
        skein_mix!(w[0], w[1], 58);
        skein_mix!(w[2], w[3], 22);
    }

    if rounds >= 16 {
        skein_mix!(w[0], w[3], 32);
        skein_mix!(w[2], w[1], 32);

        w[0] = w[0].wrapping_add(key[4]);
        w[1] = w[1].wrapping_add(key[0]);
        w[2] = w[2].wrapping_add(key[1]);
        w[3] = w[3].wrapping_add(key[2]);
        w[3] = w[3].wrapping_add(4);
    }

    if rounds >= 17 {
        skein_mix!(w[0], w[1], 14);
        skein_mix!(w[2], w[3], 16);
    }

    if rounds >= 18 {
        skein_mix!(w[0], w[3], 52);
        skein_mix!(w[2], w[1], 57);
    }

    if rounds >= 19 {
        skein_mix!(w[0], w[1], 23);
        skein_mix!(w[2], w[3], 40);
    }

    if rounds >= 20 {
        skein_mix!(w[0], w[3], 5);
        skein_mix!(w[2], w[1], 37);

        w[0] = w[0].wrapping_add(key[0]);
        w[1] = w[1].wrapping_add(key[1]);
        w[2] = w[2].wrapping_add(key[2]);
        w[3] = w[3].wrapping_add(key[3]);
        w[3] = w[3].wrapping_add(5);
    }

    //this goes up to 72 rounds but I'm lazy and 20 rounds is the recommendation
}
