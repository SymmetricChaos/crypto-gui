use crate::{
    auxiliary::blowfish_arrays::{PARRAY, SBOXES},
    errors::HasherError,
    traits::StatefulHasher,
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

    // Salt is created in big endian order
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
    buffer: Vec<u8>,
    cost: usize,
    salt: [u8; 16],
}

impl Bcrypt {
    pub fn init(cost: usize, salt: &[u8]) -> Self {
        assert!(cost >= 4 && cost < 32, "cost must be in the range 4..32");
        assert!(
            salt.len() == 16,
            "exactly 16 bytes of salt must be provided"
        );

        Self {
            buffer: Vec::new(),
            cost,
            salt: salt.try_into().unwrap(),
        }
    }

    pub fn direct(cost: usize, salt: &[u8], password: &[u8]) -> Result<Vec<u8>, HasherError> {
        if password.is_empty() || password.len() > 72 {
            return Err(HasherError::general(
                "password cannot be empty or be greater than 72 bytes",
            ));
        }
        if salt.len() != 16 {
            return Err(HasherError::general(
                "exactly 16 bytes of salt must be provided",
            ));
        }
        if cost < 4 && cost > 31 {
            return Err(HasherError::general(
                "cost cannot be less than 4 or greater than 31",
            ));
        }
        Ok(Bcrypt::init(cost, salt).hash(password))
    }

    pub fn direct_crypt_format(
        cost: usize,
        salt: &[u8],
        password: &[u8],
    ) -> Result<String, HasherError> {
        let salt_str = ByteFormat::Base64.byte_slice_to_text(salt);
        let hash_str = ByteFormat::Base64.byte_slice_to_text(Self::direct(cost, salt, password)?);
        Ok(format!("$2a${cost}${salt_str}{hash_str}"))
    }
}

impl StatefulHasher for Bcrypt {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        // Silently limit to 72 bytes
        self.buffer.truncate(72);
    }

    fn finalize(self) -> Vec<u8> {
        let mut parray = PARRAY;
        let mut sboxes = SBOXES;
        eks_blowfish_setup(&self.buffer, self.salt, &mut parray, &mut sboxes, self.cost);

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

        // Official implementations discard the last byte and this is the defacto standard
        out[0..23].to_vec()
    }
}

crate::stateful_hash_tests!(
    test1, Bcrypt::init(6, &[
        0x14, 0x4b, 0x3d, 0x69, 0x1a, 0x7b, 0x4e, 0xcf, 0x39, 0xcf, 0x73, 0x5c, 0x7f, 0xa7, 0xa7, 0x9c,
    ]), b"\0",
    "557e94f34bf286e8719a26be94ac1e16d95ef9f819dee0";

    test2, Bcrypt::init(12, &[
        0x74, 0xbb, 0x41, 0xa1, 0x46, 0xff, 0x0a, 0xcb, 0x80, 0x81, 0xd6, 0x59, 0x8c, 0x5c, 0x2d, 0x11,
    ]), b"abc123xyz",
    "6de7cb05bb0dc36dbafee01ee0959482c684e42be26208";
);
