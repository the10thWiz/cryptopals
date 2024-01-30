//
// feistel.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//
// Feistel Network block cipher
//
// Implemented using the AES sbox vals

/// Block size is 16 bytes (128 bits)
const BLOCK_SIZE: usize = 16;
const ROUNDS: usize = 4;
type Key = u32;
type KeyPart = [u8; 8];

#[rustfmt::skip]
static AES_S_BOX_VALS: [u8;256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

fn s_box(i: u8) -> u8 {
    AES_S_BOX_VALS[i as usize]
}

/// Split the key into 4 subkeys for the feistel rounds
fn split_key(key: Key) -> [KeyPart; ROUNDS] {
    let [a, b, c, d] = key.to_be_bytes();
    [
        [
            s_box(a),
            s_box(b),
            s_box(c),
            s_box(d),
            s_box(a ^ b),
            s_box(a ^ c),
            s_box(a ^ d),
            s_box(b ^ c),
        ],
        [
            s_box(b ^ d),
            s_box(c ^ d),
            s_box(a) ^ s_box(b),
            s_box(a) ^ s_box(c),
            s_box(a) ^ s_box(d),
            s_box(b) ^ s_box(c),
            s_box(b) ^ s_box(d),
            s_box(c) ^ s_box(d),
        ],
        [
            s_box(b),
            s_box(a),
            s_box(d),
            s_box(c),
            s_box(a ^ c),
            s_box(a ^ b),
            s_box(b ^ c),
            s_box(a ^ d),
        ],
        [
            s_box(c ^ d),
            s_box(b ^ d),
            s_box(a) ^ s_box(c),
            s_box(a) ^ s_box(b),
            s_box(b) ^ s_box(c),
            s_box(a) ^ s_box(d),
            s_box(c) ^ s_box(d),
            s_box(b) ^ s_box(d),
        ],
    ]
}

/// Feistel function, xors the block with key after s_boxing them
fn feistel_fn(block: [u8; BLOCK_SIZE / 2], key_part: KeyPart) -> [u8; BLOCK_SIZE / 2] {
    [
        s_box(block[0]) ^ key_part[0],
        s_box(block[1]) ^ key_part[1],
        s_box(block[2]) ^ key_part[2],
        s_box(block[3]) ^ key_part[3],
        s_box(block[4]) ^ key_part[4],
        s_box(block[5]) ^ key_part[5],
        s_box(block[6]) ^ key_part[6],
        s_box(block[7]) ^ key_part[7],
    ]
}

/// byte-wise xor on any size array
fn xor<const N: usize>(mut lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
    for (a, b) in lhs.iter_mut().zip(rhs.iter()) {
        *a ^= b;
    }
    lhs
}

pub fn encrypt_block(block: [u8; BLOCK_SIZE], key: Key) -> [u8; BLOCK_SIZE] {
    // Given block [u8; N]
    // Given `key`, generate subkeys k [; N - 1]
    // Using F([u8; N], key) -> [u8; N], we don't need the inverse of F
    //
    // For each round: (at least 3)
    //  Split block into two: L, R
    //  block = [R, L ^ F(R, k_0)]
    //
    let keys = split_key(key);

    let mut l = [
        block[0], block[1], block[2], block[3], block[4], block[5], block[6], block[7],
    ];
    let mut r = [
        block[8], block[9], block[10], block[11], block[12], block[13], block[14], block[15],
    ];
    for k in keys.iter().copied() {
        let tmp = xor(feistel_fn(r, k), l);
        l = r;
        r = tmp;
    }
    [
        l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7], r[0], r[1], r[2], r[3], r[4], r[5], r[6],
        r[7],
    ]
}

pub fn decrypt_block(block: [u8; BLOCK_SIZE], key: Key) -> [u8; BLOCK_SIZE] {
    let keys = split_key(key);

    let mut r = [
        block[0], block[1], block[2], block[3], block[4], block[5], block[6], block[7],
    ];
    let mut l = [
        block[8], block[9], block[10], block[11], block[12], block[13], block[14], block[15],
    ];
    for k in keys.iter().rev().copied() {
        let tmp = xor(feistel_fn(r, k), l);
        l = r;
        r = tmp;
    }
    //[r[0], r[1], r[2], r[3], l[0], l[1], l[2], l[3]]
    [
        r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], l[0], l[1], l[2], l[3], l[4], l[5], l[6],
        l[7],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = 102498572;
        let data = *b"!hello world! :)";
        println!("{:X?}", data);
        let enc = encrypt_block(data, key);
        println!("{:X?}", enc);
        // [8B, EF, FE, 96, 79, FC, 68, CD, C1, 3D, C, 2A, 25, 45, 59, 4C]
        let dec = decrypt_block(enc, key);
        println!("{:X?}", dec);
        assert_eq!(data, dec);
    }

    #[test]
    fn all_zeros() {
        let key = 102498572;
        let data = [0; 16];
        println!("{:X?}", data);
        let enc = encrypt_block(data, key);
        println!("{:X?}", enc);
        // [88, E8, 22, 2, 1A, 18, C0, BB, 27, 90, B3, C9, AB, F8, A8, CC]
        let dec = decrypt_block(enc, key);
        println!("{:X?}", dec);
        assert_eq!(data, dec);

        let key = 0;
        let data = *b"!hello world! :)";
        println!("{:X?}", data);
        let enc = encrypt_block(data, key);
        println!("{:X?}", enc);
        // [B4, 12, F4, CC, 53, AC, 5A, 2A, 65, 8, 62, 6, 45, 5B, AA, 4]
        let dec = decrypt_block(enc, key);
        println!("{:X?}", dec);
        assert_eq!(data, dec);
    }
}
