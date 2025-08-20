pub struct Dek {
    state: u32,
}

// for byte in bytes {
//     h = (h << 5) ^ (h >> 27) * byte;
// }

//    for (i = 0; i < length; ++str, ++i)
//    {
//       hash = ((hash << 5) ^ (hash >> 27)) ^ (*str);
//    }
