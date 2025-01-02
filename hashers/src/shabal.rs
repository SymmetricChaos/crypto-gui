use crate::traits::StatefulHasher;
use utils::{byte_formatting::make_u32s_le, padding::bit_padding};

// https://www.cs.rit.edu/~ark/20090927/Round2Candidates/Shabal.pdf

// 192
const IV192A: [u32; 12] = [
    0xFD749ED4, 0xB798E530, 0x33904B6F, 0x46BDA85E, 0x076934B4, 0x454B4058, 0x77F74527, 0xFB4CF465,
    0x62931DA9, 0xE778C8DB, 0x22B3998E, 0xAC15CFB9,
];
const IV192B: [u32; 16] = [
    0x58BCBAC4, 0xEC47A08E, 0xAEE933B2, 0xDFCBC824, 0xA7944804, 0xBF65BDB0, 0x5A9D4502, 0x59979AF7,
    0xC5CEA54E, 0x4B6B8150, 0x16E71909, 0x7D632319, 0x930573A0, 0xF34C63D1, 0xCAF914B4, 0xFDD6612C,
];
const IV192C: [u32; 16] = [
    0x61550878, 0x89EF2B75, 0xA1660C46, 0x7EF3855B, 0x7297B58C, 0x1BC67793, 0x7FB1C723, 0xB66FC640,
    0x1A48B71C, 0xF0976D17, 0x088CE80A, 0xA454EDF3, 0x1C096BF4, 0xAC76224B, 0x5215781C, 0xCD5D2669,
];

// 224
const IV224A: [u32; 12] = [
    0xA5201467, 0xA9B8D94A, 0xD4CED997, 0x68379D7B, 0xA7FC73BA, 0xF1A2546B, 0x606782BF, 0xE0BCFD0F,
    0x2F25374E, 0x069A149F, 0x5E2DFF25, 0xFAECF061,
];
const IV224B: [u32; 16] = [
    0xEC9905D8, 0xF21850CF, 0xC0A746C8, 0x21DAD498, 0x35156EEB, 0x088C97F2, 0x26303E40, 0x8A2D4FB5,
    0xFEEE44B6, 0x8A1E9573, 0x7B81111A, 0xCBC139F0, 0xA3513861, 0x1D2C362E, 0x918C580E, 0xB58E1B9C,
];
const IV224C: [u32; 16] = [
    0xE4B573A1, 0x4C1A0880, 0x1E907C51, 0x04807EFD, 0x3AD8CDE5, 0x16B21302, 0x02512C53, 0x2204CB18,
    0x99405F2D, 0xE5B648A1, 0x70AB1D43, 0xA10C25C2, 0x16F1AC05, 0x38BBEB56, 0x9B01DC60, 0xB1096D83,
];

// 256
const IV256A: [u32; 12] = [
    0x52F84552, 0xE54B7999, 0x2D8EE3EC, 0xB9645191, 0xE0078B86, 0xBB7C44C9, 0xD2B5C1CA, 0xB0D2EB8C,
    0x14CE5A45, 0x22AF50DC, 0xEFFDBC6B, 0xEB21B74A,
];
const IV256B: [u32; 16] = [
    0xB555C6EE, 0x3E710596, 0xA72A652F, 0x9301515F, 0xDA28C1FA, 0x696FD868, 0x9CB6BF72, 0x0AFE4002,
    0xA6E03615, 0x5138C1D4, 0xBE216306, 0xB38B8890, 0x3EA8B96B, 0x3299ACE4, 0x30924DD4, 0x55CB34A5,
];
const IV256C: [u32; 16] = [
    0xB405F031, 0xC4233EBA, 0xB3733979, 0xC0DD9D55, 0xC51C28AE, 0xA327B8E1, 0x56C56167, 0xED614433,
    0x88B59D60, 0x60E2CEBA, 0x758B4B8B, 0x83E82A7F, 0xBC968828, 0xE6E00BF7, 0xBA839E55, 0x9B491C60,
];

// 384
const IV384A: [u32; 12] = [
    0xC8FCA331, 0xE55C504E, 0x003EBF26, 0xBB6B8D83, 0x7B0448C1, 0x41B82789, 0x0A7C9601, 0x8D659CFF,
    0xB6E2673E, 0xCA54C77B, 0x1460FD7E, 0x3FCB8F2D,
];
const IV384B: [u32; 16] = [
    0x527291FC, 0x2A16455F, 0x78E627E5, 0x944F169F, 0x1CA6F016, 0xA854EA25, 0x8DB98ABE, 0xF2C62641,
    0x30117DCB, 0xCF5C4309, 0x93711A25, 0xF9F671B8, 0xB01D2116, 0x333F4B89, 0xB285D165, 0x86829B36,
];
const IV384C: [u32; 16] = [
    0xF764B11A, 0x76172146, 0xCEF6934D, 0xC6D28399, 0xFE095F61, 0x5E6018B4, 0x5048ECF5, 0x51353261,
    0x6E6E36DC, 0x63130DAD, 0xA9C69BD6, 0x1E90EA0C, 0x7C35073B, 0x28D95E6D, 0xAA340E0D, 0xCB3DEE70,
];

// 512
const IV512A: [u32; 12] = [
    0x20728DFD, 0x46C0BD53, 0xE782B699, 0x55304632, 0x71B4EF90, 0x0EA9E82C, 0xDBB930F1, 0xFAD06B8B,
    0xBE0CAE40, 0x8BD14410, 0x76D2ADAC, 0x28ACAB7F,
];
const IV512B: [u32; 16] = [
    0xC1099CB7, 0x07B385F3, 0xE7442C26, 0xCC8AD640, 0xEB6F56C7, 0x1EA81AA9, 0x73B9D314, 0x1DE85D08,
    0x48910A5A, 0x893B22DB, 0xC5A0DF44, 0xBBC4324E, 0x72D2F240, 0x75941D99, 0x6D8BDE82, 0xA1A7502B,
];
const IV512C: [u32; 16] = [
    0xD9BF68D1, 0x58BAD750, 0x56028CB2, 0x8134F359, 0xB5D469D8, 0x941A8CC2, 0x418B2A6E, 0x04052780,
    0x7F07D787, 0x5194358F, 0x3C60D665, 0xBE97D79A, 0x950C3434, 0xAED9A06D, 0x2537DC8D, 0x7CDB5969,
];

// These security parameter could be varied but Shabal only uses the values shown
const P: usize = 3;
const R: usize = 12;

// Offsets in the keyed permutation
const O1: usize = 13;
const O2: usize = 9;
const O3: usize = 6;

fn keyed_permutation(m: &[u32; 16], a: &mut [u32; 12], b: &mut [u32; 16], c: &[u32; 16]) {
    b.iter_mut().for_each(|b| *b = b.rotate_left(17));

    for j in 0..P {
        for i in 0..16 {
            let p = i + (16 * j);
            let t0 = a[p % R];
            let t1 = a[((i + 11) + (16 * j)) % R].rotate_left(15).wrapping_mul(5);
            let t2 = c[(24 - i) % 16];
            a[p % R] = (t0 ^ t1 ^ t2).wrapping_mul(3)
                ^ b[(i + O1) % 16]
                ^ (b[(i + O2) % 16] & !b[(i + O3) % 16])
                ^ m[i];
            b[i] = b[i].rotate_left(1) ^ !a[p % R];
        }
    }

    for j in 0..36 {
        a[j % R] = a[j % R].wrapping_add(c[(j + 3) % 16])
    }
}

macro_rules! shabal {
    ($name:ident, $a: ident, $b: ident, $c: ident, $hlen: literal) => {
        pub struct $name {
            a: [u32; 12],
            b: [u32; 16],
            c: [u32; 16],
            ctr: u64,
            buffer: Vec<u8>,
        }


        impl $name {
            pub fn init() -> Self {
                Self {
                    a: $a,
                    b: $b,
                    c: $c,
                    ctr: 0,
                    buffer: Vec::new()
                }
            }
        }

        impl StatefulHasher for $name {
            fn update(&mut self, bytes: &[u8]) {
                self.buffer.extend_from_slice(bytes);
                let chunks = self.buffer.chunks_exact(64);
                let rem = chunks.remainder().to_vec();

                for chunk in chunks {
                    self.ctr = self.ctr.wrapping_add(1);
                    let m = make_u32s_le::<16>(chunk);

                    // Insert the message into b by addition
                    for i in 0..16 {
                        self.b[i] = self.b[i].wrapping_add(m[i]);
                    }

                    // XOR the counter in A[0] and A[1]
                    self.a[0] ^= self.ctr as u32;
                    self.a[1] ^= (self.ctr >> 32) as u32;

                    // Apply the keyed permutation
                    keyed_permutation(&m, &mut self.a, &mut self.b, &self.c);

                    // Insert the message into c by subtraction
                    for i in 0..16 {
                        self.c[i] = self.c[i].wrapping_sub(m[i]);
                    }

                    // Swap B and C
                    std::mem::swap(&mut self.b, &mut self.c);
                }

                self.buffer = rem;
            }

            fn finalize(mut self) -> Vec<u8> {
                // Create the final block.
                bit_padding(&mut self.buffer, 64).unwrap();
                let final_block = make_u32s_le::<16>(&self.buffer);

                // One compression round on the final block.
                self.ctr = self.ctr.wrapping_add(1);
                for i in 0..16 {
                    self.b[i] = self.b[i].wrapping_add(final_block[i]);
                }
                self.a[0] ^= self.ctr as u32;
                self.a[1] ^= (self.ctr >> 32) as u32;
                keyed_permutation(&final_block, &mut self.a, &mut self.b, &self.c);
                for i in 0..16 {
                    self.c[i] = self.c[i].wrapping_sub(final_block[i]);
                }
                std::mem::swap(&mut self.b, &mut self.c);

                // Finalization rounds
                for _ in 0..3 {
                    let m = final_block;

                    for i in 0..16 {
                        self.b[i] = self.b[i].wrapping_add(m[i]);
                    }

                    self.a[0] ^= self.ctr as u32;
                    self.a[1] ^= (self.ctr >> 32) as u32;

                    keyed_permutation(&m, &mut self.a, &mut self.b, &self.c);

                    for i in 0..16 {
                        self.c[i] = self.c[i].wrapping_sub(m[i]);
                    }

                    std::mem::swap(&mut self.b, &mut self.c);
                }

                let mut out = Vec::with_capacity(32);
                for word in self.c[(16-$hlen)..].into_iter() {
                    out.extend(word.to_le_bytes())
                }
                out
            }

            crate::stateful_hash_helpers!();
        }
    };
}

shabal!(Shabal192, IV192A, IV192B, IV192C, 6);
shabal!(Shabal224, IV224A, IV224B, IV224C, 7);
shabal!(Shabal256, IV256A, IV256B, IV256C, 8);
shabal!(Shabal384, IV384A, IV384B, IV384C, 12);
shabal!(Shabal512, IV512A, IV512B, IV512C, 16);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShabalVariant {
    Shabal192,
    Shabal224,
    Shabal256,
    Shabal384,
    Shabal512,
}

#[cfg(test)]
mod shabal_test {
    use super::*;

    const TESTA: &[u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];
    const TESTB: &[u8] = &[
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
        0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x2d, 0x30, 0x31, 0x32,
        0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2d, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
        0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56,
        0x57, 0x58, 0x59, 0x5a, 0x2d, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
        0x2d, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e,
        0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a,
    ];

    crate::stateful_hash_tests!(
        test_a_192, Shabal192::init(), TESTA,
        "0f706ecb97cf4dce00bfbbd2fb64530c32870cb44839730d";

        test_b_192, Shabal192::init(), TESTB,
        "690fae79226d95760ae8fdb4f58c0537111756557d307b15";

        test_a_224, Shabal224::init(), TESTA,
        "99dda614f907d2e8817618f730696f3200aeca8b5f85f42543ba2031";

        test_b_224, Shabal224::init(), TESTB,
        "c7d62d8d2a3474b4f4a9d11a52db3d435bf158cf454c5d561d7125f5";

        test_a_256, Shabal256::init(), TESTA,
        "da8f08c02a67ba9a56bdd0798e48ae0714215e093b5b850649a37718993f54a2";

        test_b_256, Shabal256::init(), TESTB,
        "b49f34bf51864c30533cc46cc2542bdec2f96fd06f5c539aff6ead5883f7327a";

        test_a_384, Shabal384::init(), TESTA,
        "9dde1233910d85da3a5c780312b111c6fcca1b5dd25537035ee08e3b4e1e25154f726a6384e5a8f0afeaab4ac4c02f12";

        test_b_384, Shabal384::init(), TESTB,
        "30012c0e3edc460bd78627c2c30944d2a189669afa2d7a9713ef2f774c4474a43af1cbcec5fab4248c0873f038fbeba0";

        test_a_512, Shabal512::init(), TESTA,
        "158016c6c81f3f0a52d98d68ed2f9e8e7895ef23cba7e2bc6109d8a532e6c9e6a6a501979fb837f04ec4c620e73179dc82abb52b32cdadb35650e29c985e3022";

        test_b_512, Shabal512::init(), TESTB,
        "677e6f7f12d70af0b335662f59b56851f3653e66647d3386dfda0143254cc8a5db3e2194068c6f71597d7b60984d22b47a1f60d91ca8dfcb175d65b97359cecf";
    );
}
