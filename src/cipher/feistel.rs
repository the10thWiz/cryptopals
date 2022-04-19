//
// feistel.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//
// Feistel Network block cipher

const BLOCK_SIZE: usize = 8;
type Key = u32;
type KeyPart = u8;

fn split_key(key: Key) -> [KeyPart; 4] {
    key.to_be_bytes()
}

fn feistel_fn(_block: [u8; BLOCK_SIZE / 2], key_part: KeyPart) -> [u8; BLOCK_SIZE / 2] {
    [key_part; BLOCK_SIZE / 2]
}

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

    let mut l = [block[0], block[1], block[2], block[3]];
    let mut r = [block[4], block[5], block[6], block[7]];
    for k in keys.iter().copied() {
        let tmp = xor(feistel_fn(r, k), l);
        l = r;
        r = tmp;
    }
    [l[0], l[1], l[2], l[3], r[0], r[1], r[2], r[3]]
}

pub fn decrypt_block(block: [u8; BLOCK_SIZE], key: Key) -> [u8; BLOCK_SIZE] {
    let keys = split_key(key);

    let mut r = [block[0], block[1], block[2], block[3]];
    let mut l = [block[4], block[5], block[6], block[7]];
    for k in keys.iter().rev().copied() {
        let tmp = xor(feistel_fn(r, k), l);
        l = r;
        r = tmp;
    }
    [r[0], r[1], r[2], r[3], l[0], l[1], l[2], l[3]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = 102498572;
        let data = *b"hello!:)";
        println!("{:X?}", data);
        let enc = encrypt_block(data, key);
        println!("{:X?}", enc);
        let dec = decrypt_block(enc, key);
        println!("{:X?}", dec);
        assert_eq!(data, dec);
    }
}
