use crate::{
    auxiliary::blowfish_arrays::{PARRAY, SBOXES},
    errors::HasherError,
    traits::ClassicHasher,
};
use utils::byte_formatting::{make_u32s_be, ByteFormat};

pub fn encrypt_u64(n: &mut u64, parray: &[u32; 18], sboxes: &[[u32; 256]; 4]) {
    let mut lr = make_u32s_be::<2>(&n.to_be_bytes());
    for i in 0..16 {
        lr[0] ^= parray[i];
        lr[1] ^= f(lr[0], sboxes);
        lr.swap(0, 1);
    }
    lr.swap(0, 1);
    lr[1] ^= parray[16];
    lr[0] ^= parray[17];
    *n = (lr[0] as u64) << 32 | lr[1] as u64;
}

pub fn encrypt_u32s(lr: &mut [u32], parray: &[u32; 18], sboxes: &[[u32; 256]; 4]) {
    for i in 0..16 {
        lr[0] ^= parray[i];
        lr[1] ^= f(lr[0], sboxes);
        lr.swap(0, 1);
    }
    lr.swap(0, 1);
    lr[1] ^= parray[16];
    lr[0] ^= parray[17];
}

pub fn f(x: u32, sboxes: &[[u32; 256]; 4]) -> u32 {
    let a = sboxes[0][(x >> 24) as usize];
    let b = sboxes[1][((x >> 16) & 0xff) as usize];
    let c = sboxes[2][((x >> 8) & 0xff) as usize];
    let d = sboxes[3][(x & 0xff) as usize];
    (a.wrapping_add(b) ^ c).wrapping_add(d)
}

pub fn eks_blowfish_setup(
    password: &[u8],
    salt: [u8; 16],
    parray: &mut [u32; 18],
    sboxes: &mut [[u32; 256]; 4],
    cost: usize,
) {
    // Create new parray and sboxes from the password and salt
    expand_key(password, salt, parray, sboxes);

    // Repeatedly derive new parrays and sboxes. The expensive step.
    for _ in 0..(1 << cost) {
        expand_key(password, [0; 16], parray, sboxes);
        expand_key(&salt, [0; 16], parray, sboxes);
    }
}

pub fn expand_key(
    password: &[u8],
    salt: [u8; 16],
    parray: &mut [u32; 18],
    sboxes: &mut [[u32; 256]; 4],
) {
    // Endlessly repeat the key as needed
    let mut key_bytes = password.iter().cycle();

    // Xoring the password into the IV
    for word in parray.iter_mut() {
        let mut k = 0u32;
        for _ in 0..4 {
            k <<= 8;
            k |= (*key_bytes.next().unwrap()) as u32;
        }
        *word ^= k;
    }

    // Salt is created in be order
    let salt_halves = [
        [
            u32::from_be_bytes(salt[0..4].try_into().unwrap()),
            u32::from_be_bytes(salt[4..8].try_into().unwrap()),
        ],
        [
            u32::from_be_bytes(salt[8..12].try_into().unwrap()),
            u32::from_be_bytes(salt[12..16].try_into().unwrap()),
        ],
    ];

    let mut block: [u32; 2] = [0, 0];

    // Create the parray
    for i in 0..9 {
        block[0] ^= salt_halves[i % 2][0];
        block[1] ^= salt_halves[i % 2][1];
        encrypt_u32s(&mut block, parray, sboxes);
        parray[2 * i] = block[0];
        parray[2 * i + 1] = block[1];
    }

    // Create the four sboxes
    for i in 0..4 {
        for j in 0..128 {
            block[0] ^= salt_halves[j % 2][0];
            block[1] ^= salt_halves[j % 2][1];
            encrypt_u32s(&mut block, parray, sboxes);
            sboxes[i][j * 2] = block[0];
            sboxes[i][j * 2 + 1] = block[1];
        }
    }
}

pub struct Bcrypt {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub cost: usize,
    pub salt: [u8; 16],
    pub full_len: bool,
}

impl Default for Bcrypt {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            cost: 12,
            salt: [0; 16],
            full_len: false,
        }
    }
}

impl Bcrypt {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn salt(mut self, salt: [u8; 16]) -> Self {
        self.salt = salt;
        self
    }

    pub fn cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }

    pub fn full_len(mut self, full_len: bool) -> Self {
        self.full_len = full_len;
        self
    }

    pub fn direct(password: &[u8], salt: [u8; 16], cost: usize) -> Result<Vec<u8>, HasherError> {
        if password.is_empty() || password.len() > 72 {
            return Err(HasherError::general(
                "password cannot be empty or be greater than 72 bytes",
            ));
        }
        if cost >= 31 {
            return Err(HasherError::general(
                "cost cannot be less than 4 or greater than 31",
            ));
        }
        Ok(Bcrypt::default().salt(salt).cost(cost).hash(password))
    }
}

impl ClassicHasher for Bcrypt {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.cost >= 4 && self.cost < 32);
        assert!(bytes.len() > 0 && bytes.len() <= 72);

        let mut parray = PARRAY;
        let mut sboxes = SBOXES;
        eks_blowfish_setup(bytes, self.salt, &mut parray, &mut sboxes, self.cost);

        // The string "OrpheanBeholderScryDoubt" as u32s
        let mut ctext: [u32; 6] = [
            0x4f727068, 0x65616e42, 0x65686f6c, 0x64657253, 0x63727944, 0x6f756274,
        ];

        let mut out = [0; 24];

        for i in 0..3 {
            // Each 8 byte block is encrypted 64 times then copied into the output
            // As if encrypting the bytes in ECB mode
            let a = 2 * i;
            for _ in 0..64 {
                encrypt_u32s(&mut ctext[a..a + 2], &parray, &sboxes);
            }

            out[4 * a..][..4].copy_from_slice(&ctext[a].to_be_bytes());
            out[4 * (a + 1)..][..4].copy_from_slice(&ctext[a + 1].to_be_bytes());
        }

        if self.full_len {
            out.to_vec()
        } else {
            // Official implementations discard the last byte for some reason
            out[0..23].to_vec()
        }
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Bcrypt::default().salt([
        0x14, 0x4b, 0x3d, 0x69, 0x1a, 0x7b, 0x4e, 0xcf, 0x39, 0xcf, 0x73, 0x5c, 0x7f, 0xa7,
        0xa7, 0x9c,
    ]).cost(6), "\0",
    "557e94f34bf286e8719a26be94ac1e16d95ef9f819dee0";
);
