// 64 bit
const PHILOX_M2_64_0: u64 = 0xD2B74407B1CE6E93;
const PHILOX_M2_64: u64 = 0xD2B74407B1CE6E93;

const PHILOX_M4_64_0: u64 = 0xD2E7470EE14C6C93;
const PHILOX_M4_64_1: u64 = 0xCA5A826395121157;
const PHILOX_M4_64: [u64; 2] = [0xD2E7470EE14C6C93, 0xCA5A826395121157];

// 32 bit
const PHILOX_M2_32_0: u32 = 0xD256D193;
const PHILOX_M2_32: u32 = 0xD256D193;

const PHILOX_M4_32_0: u32 = 0xD2511F53;
const PHILOX_M4_32_1: u32 = 0xCD9E8D57;
const PHILOX_M4_32: [u32; 2] = [0xD2511F53, 0xCD9E8D57];

//64 bit
const PHILOX_W_64_0: u64 = 0x9E3779B97F4A7C15;
const PHILOX_W_64_1: u64 = 0xBB67AE8584CAA73B;
const PHILOX_W_64: [u64; 2] = [0x9E3779B97F4A7C15, 0xBB67AE8584CAA73B];

// 32 bit
const PHILOX_W_32_0: u32 = 0x9E3779B9;
const PHILOX_W_32_1: u32 = 0xBB67AE85;
const PHILOX_W_32: [u32; 2] = [0x9E3779B9, 0xBB67AE85];

// # _round() in 2 versions for x2 or x4
// def philox2_round(counter, key, philox_m, len_w, mask_w):
//   # philox_mulhilo
//   prod = philox_m[VAL_1] * counter[VAL_1]
//   hi_2 = prod >> len_w
//   lo_2 = prod & mask_w
//   counter[VAL_1] = hi_2 ^ counter[VAL_2] ^ key[VAL_1]
//   counter[VAL_2] = lo_2

// def philox4_round(counter, key, philox_m, len_w, mask_w):
//   prod = philox_m[VAL_1] * counter[VAL_1]
//   hi_1 = prod >> len_w
//   lo_1 = prod & mask_w
//   prod = philox_m[VAL_2] * counter[VAL_3]
//   hi_2 = prod >> len_w
//   lo_2 = prod & mask_w
//   counter[VAL_1] = hi_2 ^ counter[VAL_2] ^ key[VAL_1]
//   counter[VAL_2] = lo_2
//   counter[VAL_3] = hi_1 ^ counter[VAL_4] ^ key[VAL_2]
//   counter[VAL_4] = lo_1
