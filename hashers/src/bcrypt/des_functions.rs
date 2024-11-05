use std::ops::Shr;

use crate::errors::HasherError;

use super::des_arrays::{KEYSHIFT, SBOXES};

// All the bit manipulation is taken from here:
// https://docs.rs/des/latest/src/des/des.rs.html
pub fn delta_swap(a: u64, delta: u64, mask: u64) -> u64 {
    let b = (a ^ (a >> delta)) & mask;
    a ^ b ^ (b << delta)
}

/// Rotate a 28 bit number stored in a u64
pub fn rotate_28(mut val: u64, shift: u8) -> u64 {
    let top_bits = val >> (28 - shift);
    val <<= shift;

    (val | top_bits) & 0x0fff_ffff
}

///  Swap bits using the PC-1 table, note that the eight least significant bits are zero
pub fn pc1(mut key: u64) -> u64 {
    key = delta_swap(key, 2, 0x3333000033330000);
    key = delta_swap(key, 4, 0x0f0f0f0f00000000);
    key = delta_swap(key, 8, 0x009a000a00a200a8);
    key = delta_swap(key, 16, 0x00006c6c0000cccc);
    key = delta_swap(key, 1, 0x1045500500550550);
    key = delta_swap(key, 32, 0x00000000f0f0f5fa);
    key = delta_swap(key, 8, 0x00550055006a00aa);
    key = delta_swap(key, 2, 0x0000333330000300);
    key & 0xffffffffffffff00
}

/// Swap bits using the PC-2 table
pub fn pc2(key: u64) -> u64 {
    let key = key.rotate_left(61);
    let b1 = (key & 0x0021000002000000) >> 7;
    let b2 = (key & 0x0008020010080000) << 1;
    let b3 = key & 0x0002200000000000;
    let b4 = (key & 0x0000000000100020) << 19;
    let b5 = (key.rotate_left(54) & 0x0005312400000011).wrapping_mul(0x0000000094200201)
        & 0xea40100880000000;
    let b6 = (key.rotate_left(7) & 0x0022110000012001).wrapping_mul(0x0001000000610006)
        & 0x1185004400000000;
    let b7 = (key.rotate_left(6) & 0x0000520040200002).wrapping_mul(0x00000080000000c1)
        & 0x0028811000200000;
    let b8 = (key & 0x01000004c0011100).wrapping_mul(0x0000000000004284) & 0x0400082244400000;
    let b9 = (key.rotate_left(60) & 0x0000000000820280).wrapping_mul(0x0000000000089001)
        & 0x0000000110880000;
    let b10 = (key.rotate_left(49) & 0x0000000000024084).wrapping_mul(0x0000000002040005)
        & 0x000000000a030000;
    b1 | b2 | b3 | b4 | b5 | b6 | b7 | b8 | b9 | b10
}

/// Swap bits using the reverse final permutation table
pub fn final_permutation(mut message: u64) -> u64 {
    message = delta_swap(message, 24, 0x000000ff000000ff);
    message = delta_swap(message, 24, 0x00000000ff00ff00);
    message = delta_swap(message, 36, 0x000000000f0f0f0f);
    message = delta_swap(message, 18, 0x0000333300003333);
    delta_swap(message, 9, 0x0055005500550055)
}

/// Swap bits using the initial permutation table
pub fn initial_permutation(mut message: u64) -> u64 {
    message = delta_swap(message, 9, 0x0055005500550055);
    message = delta_swap(message, 18, 0x0000333300003333);
    message = delta_swap(message, 36, 0x000000000f0f0f0f);
    message = delta_swap(message, 24, 0x00000000ff00ff00);
    delta_swap(message, 24, 0x000000ff000000ff)
}

/// Swap bits using the expansion table
pub fn e(block: u64) -> u64 {
    const BLOCK_LEN: usize = 32;
    const RESULT_LEN: usize = 48;

    let b1 = (block << (BLOCK_LEN - 1)) & 0x8000000000000000;
    let b2 = (block >> 1) & 0x7c00000000000000;
    let b3 = (block >> 3) & 0x03f0000000000000;
    let b4 = (block >> 5) & 0x000fc00000000000;
    let b5 = (block >> 7) & 0x00003f0000000000;
    let b6 = (block >> 9) & 0x000000fc00000000;
    let b7 = (block >> 11) & 0x00000003f0000000;
    let b8 = (block >> 13) & 0x000000000fc00000;
    let b9 = (block >> 15) & 0x00000000003e0000;
    let b10 = (block >> (RESULT_LEN - 1)) & 0x0000000000010000;
    b1 | b2 | b3 | b4 | b5 | b6 | b7 | b8 | b9 | b10
}

/// Swap bits using the P table
pub fn p(block: u64) -> u64 {
    let block = block.rotate_left(44);
    let b1 = (block & 0x0000000000200000) << 32;
    let b2 = (block & 0x0000000000480000) << 13;
    let b3 = (block & 0x0000088000000000) << 12;
    let b4 = (block & 0x0000002020120000) << 25;
    let b5 = (block & 0x0000000442000000) << 14;
    let b6 = (block & 0x0000000001800000) << 37;
    let b7 = (block & 0x0000000004000000) << 24;
    let b8 = (block & 0x0000020280015000).wrapping_mul(0x0000020080800083) & 0x02000a6400000000;
    let b9 = (block.rotate_left(29) & 0x01001400000000aa).wrapping_mul(0x0000210210008081)
        & 0x0902c01200000000;
    let b10 = (block & 0x0000000910040000).wrapping_mul(0x0000000c04000020) & 0x8410010000000000;
    b1 | b2 | b3 | b4 | b5 | b6 | b7 | b8 | b9 | b10
}

pub fn f(block: u64, key: u64) -> u64 {
    let mut v = e(block);
    v ^= key;
    v = sboxes(v);
    p(v)
}

/// Take an input of 48 bits (in a u64), then take it as eight 6-bit chunks, feed each into a sbox that returns 4-bit value, and stitch those together into a 32-bit value (in a u64)
/// The data is stored and saved in the upper bits of the u64s, this is the reason for the initial shift values below
pub fn sboxes(input: u64) -> u64 {
    let mut out = 0;

    for (i, sbox) in SBOXES.iter().enumerate() {
        let six = (input >> (58 - (i * 6))) & 0x3F; // 0x3F = 0b111111 so this masking keeps only the lower six bits
        out |= u64::from(sbox[six as usize]) << (60 - (i * 4));
    }

    out
}

pub fn round(input: u64, key: u64) -> u64 {
    let l = input & (0xffff_ffff << 32);
    let r = input << 32;

    r | ((f(r, key) ^ l) >> 32)
}

pub fn des_ksa(key: u64) -> Result<[u64; 16], HasherError> {
    // test_des_key(key)?;
    let mut subkeys = [0; 16];
    let key = pc1(key) >> 8;
    let mut left: u64 = key.shr(28) & 0x0fff_ffff_u64;
    let mut right = key & 0x0fff_ffff;
    for i in 0..16 {
        left = rotate_28(left, KEYSHIFT[i]);
        right = rotate_28(right, KEYSHIFT[i]);
        // Overwrite the old state
        subkeys[i] = pc2(((left << 28) | right) << 8);
    }
    Ok(subkeys)
}

pub fn test_des_key(key: u64) -> Result<(), HasherError> {
    for byte in key.to_le_bytes() {
        if byte.count_ones() % 2 == 0 {
            return Err(HasherError::key(
                "all bytes of a DES key must have odd parity, the eighth bit is the parity bit",
            ));
        }
    }
    Ok(())
}

pub fn set_des_key_parity(key: u64) -> u64 {
    let mut bytes = key.to_le_bytes();
    for byte in bytes.iter_mut() {
        if byte.count_ones() % 2 != 1 {
            *byte ^= 0x01;
        }
    }
    u64::from_le_bytes(bytes)
}

pub fn expand_56_to_64(bytes_56: [u8; 7]) -> u64 {
    let mut bytes_64 = [0_u8; 8];

    bytes_64[0] = bytes_56[0];
    bytes_64[1] = (bytes_56[0] << 7) | (bytes_56[1] >> 1);
    bytes_64[2] = (bytes_56[1] << 6) | (bytes_56[2] >> 2);
    bytes_64[3] = (bytes_56[2] << 5) | (bytes_56[3] >> 3);
    bytes_64[4] = (bytes_56[3] << 4) | (bytes_56[4] >> 4);
    bytes_64[5] = (bytes_56[4] << 3) | (bytes_56[5] >> 5);
    bytes_64[6] = (bytes_56[5] << 2) | (bytes_56[6] >> 6);
    bytes_64[7] = bytes_56[6] << 1;

    let key = u64::from_be_bytes(bytes_64);

    set_des_key_parity(key)
}

#[cfg(test)]
mod des_tests {

    use super::*;

    #[test]
    fn test_expand_56() {
        let bytes_56 = [
            0b00000001, 0b00000010, 0b00000011, 0b00000100, 0b00000101, 0b00000110, 0b00000111,
        ];
        let key = 0b0000000110000000100000000110000101000000001010010001100100001110_u64;

        assert_eq!(key, expand_56_to_64(bytes_56));
    }
}
