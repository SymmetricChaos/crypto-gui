// https://link.springer.com/article/10.1007/BF00203968

const ROTATE: [u32; 4] = [16,8,16,24];
const BLOCK_SIZE_WORDS: u32 = 16;

// The compression function is described by Merkle as a block cipher
pub fn e512(x: &[u8], passes: usize) {
    todo!("convert input to u32s, zero filled");
    let mut block: [u32; 16] = [0;16];
    for index in 1..=passes {
        for byte_in_word in 1..=4 {
            for i in 0..BLOCK_SIZE_WORDS {

            }
        }
    }

}