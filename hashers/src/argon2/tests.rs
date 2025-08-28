// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-argon2-01#section-4

// =======================================
// Argon2d version number 19
// =======================================
// Memory: 32 KiB
// Iterations: 3
// Parallelism: 4 lanes
// Tag length: 32 bytes
// Password[32]: 01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
// Salt[16]: 02 02 02 02 02 02 02 02 02 02 02 02 02 02 02 02
// Secret[8]: 03 03 03 03 03 03 03 03
// Associated data[12]: 04 04 04 04 04 04 04 04 04 04 04 04
// Pre-hashing digest: b8 81 97 91 a0 35 96 60
//                     bb 77 09 c8 5f a4 8f 04
//                     d5 d8 2c 05 c5 f2 15 cc
//                     db 88 54 91 71 7c f7 57
//                     08 2c 28 b9 51 be 38 14
//                     10 b5 fc 2e b7 27 40 33
//                     b9 fd c7 ae 67 2b ca ac
//                     5d 17 90 97 a4 af 31 09

//  After pass 0:
// Block 0000 [  0]: db2fea6b2c6f5c8a
// Block 0000 [  1]: 719413be00f82634
// Block 0000 [  2]: a1e3f6dd42aa25cc
// Block 0000 [  3]: 3ea8efd4d55ac0d1
// ...
// Block 0031 [124]: 28d17914aea9734c
// Block 0031 [125]: 6a4622176522e398
// Block 0031 [126]: 951aa08aeecb2c05
// Block 0031 [127]: 6a6c49d2cb75d5b6

//  After pass 1:
// Block 0000 [  0]: d3801200410f8c0d
// Block 0000 [  1]: 0bf9e8a6e442ba6d
// Block 0000 [  2]: e2ca92fe9c541fcc
// Block 0000 [  3]: 6269fe6db177a388
// ...
// Block 0031 [124]: 9eacfcfbdb3ce0fc
// Block 0031 [125]: 07dedaeb0aee71ac
// Block 0031 [126]: 074435fad91548f4
// Block 0031 [127]: 2dbfff23f31b5883

// After pass 2:
// Block 0000 [  0]: 5f047b575c5ff4d2
// Block 0000 [  1]: f06985dbf11c91a8
// Block 0000 [  2]: 89efb2759f9a8964
// Block 0000 [  3]: 7486a73f62f9b142
// ...
// Block 0031 [124]: 57cfb9d20479da49
// Block 0031 [125]: 4099654bc6607f69
// Block 0031 [126]: f142a1126075a5c8
// Block 0031 [127]: c341b3ca45c10da5
// Tag: 51 2b 39 1b 6f 11 62 97
//      53 71 d3 09 19 73 42 94
//      f8 68 e3 be 39 84 f3 c1
//      a1 3a 4d b9 fa be 4a cb

// =======================================
// Argon2i version number 19
// =======================================
// Memory: 32 KiB
// Iterations: 3
// Parallelism: 4 lanes
// Tag length: 32 bytes
// Password[32]: 01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
//               01 01 01 01 01 01 01 01
// Salt[16]: 02 02 02 02 02 02 02 02 02 02 02 02 02 02 02 02
// Secret[8]: 03 03 03 03 03 03 03 03
// Associated data[12]: 04 04 04 04 04 04 04 04 04 04 04 04
// Pre-hashing digest: c4 60 65 81 52 76 a0 b3
//                     e7 31 73 1c 90 2f 1f d8
//                     0c f7 76 90 7f bb 7b 6a
//                     5c a7 2e 7b 56 01 1f ee
//                     ca 44 6c 86 dd 75 b9 46
//                     9a 5e 68 79 de c4 b7 2d
//                     08 63 fb 93 9b 98 2e 5f
//                     39 7c c7 d1 64 fd da a9

//  After pass 0:
// Block 0000 [  0]: f8f9e84545db08f6
// Block 0000 [  1]: 9b073a5c87aa2d97
// Block 0000 [  2]: d1e868d75ca8d8e4
// Block 0000 [  3]: 349634174e1aebcc
// ...
// Block 0031 [124]: 975f596583745e30
// Block 0031 [125]: e349bdd7edeb3092
// Block 0031 [126]: b751a689b7a83659
// Block 0031 [127]: c570f2ab2a86cf00

//  After pass 1:
// Block 0000 [  0]: b2e4ddfcf76dc85a
// Block 0000 [  1]: 4ffd0626c89a2327
// Block 0000 [  2]: 4af1440fff212980
// Block 0000 [  3]: 1e77299c7408505b
// ...
// Block 0031 [124]: e4274fd675d1e1d6
// Block 0031 [125]: 903fffb7c4a14c98
// Block 0031 [126]: 7e5db55def471966
// Block 0031 [127]: 421b3c6e9555b79d

//  After pass 2:
// Block 0000 [  0]: af2a8bd8482c2f11
// Block 0000 [  1]: 785442294fa55e6d
// Block 0000 [  2]: 9256a768529a7f96
// Block 0000 [  3]: 25a1c1f5bb953766
// ...
// Block 0031 [124]: 68cf72fccc7112b9
// Block 0031 [125]: 91e8c6f8bb0ad70d
// Block 0031 [126]: 4f59c8bd65cbb765
// Block 0031 [127]: 71e436f035f30ed0
// Tag: c8 14 d9 d1 dc 7f 37 aa
//      13 f0 d7 7f 24 94 bd a1
//      c8 de 6b 01 6d d3 88 d2
//      99 52 a4 c4 67 2b 6c e8

#[cfg(test)]
mod big_test {
    use crate::argon2::argon2::Argon2;
    use crate::traits::StatefulHasher;

    crate::stateful_hash_tests!(
        test_argon2d, Argon2::init_argon2d(32, 4, 32, 3, &[0x02; 16], &[0x03; 8], &[0x04; 12]),
        &[0x01; 32],
        "512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb";
        test_argon2i, Argon2::init_argon2i(32, 4, 32, 3, &[0x02; 16], &[0x03; 8], &[0x04; 12]),
        &[0x01; 32],
        "c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8";
    );
}
