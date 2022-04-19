use crate::data::Bytes;

/**
 * Encrypts a single AES block
 */
pub fn aes_block_encrypt(input: Bytes, k: Bytes) -> Bytes {
    let mut ret = input.clone();
    let keys = expand_key(k);
    // Num rounds = 10, key = 128 bit, input = 128 bit (will be split into blocks)

    add_round_key(&mut ret, &keys[0..4]);
    for i in 1..10 {
        sub_bytes(&mut ret); // See Sec. 5.1.1
        shift_rows(&mut ret); // See Sec. 5.1.2
        mix_columns(&mut ret); // See Sec. 5.1.3
        add_round_key(&mut ret, &keys[i * 4..i * 4 + 4]); //state, w[round*Nb, (round+1)*Nb-1])
    }
    sub_bytes(&mut ret); // See Sec. 5.1.1
    shift_rows(&mut ret); // See Sec. 5.1.2
    add_round_key(&mut ret, &keys[40..44]); //state, w[round*Nb, (round+1)*Nb-1])
    ret
}

/**
 * decrypts a single AES block
 */
pub fn aes_block_decrypt(cipher: Bytes, k: Bytes) -> Bytes {
    let mut ret = cipher.clone();
    let keys = expand_key(k);
    // Num rounds = 10, key = 128 bit, input = 128 bit (will be split into blocks)

    // println!("round[ 0].iinput   {:x}", ret);
    // println!("round[ 0].ik_sch   {:x}", Bytes::as_one(&keys[40..44]));
    add_round_key(&mut ret, &keys[40..44]); // See Sec. 5.1.4
    for i in (1..10).rev() {
        // println!("round[ {}].istart   {:x}", 10-i, ret);
        inv_shift_rows(&mut ret); // See Sec. 5.3.1
                                  // println!("round[ {}].is_row   {:x}", 10-i, ret);
        inv_sub_bytes(&mut ret); // See Sec. 5.3.2
                                 // println!("round[ {}].is_box   {:x}", 10-i, ret);
                                 // println!("round[ {}].ik_sch   {:x}", 10-i, Bytes::as_one(&keys[i*4..i*4+4]));
        add_round_key(&mut ret, &keys[i * 4..i * 4 + 4]);
        // println!("round[ {}].ik_add   {:x}", 10-i, ret);
        inv_mix_columns(&mut ret); // See Sec. 5.3.3
    }
    // println!("round[10].istart   {:x}", ret);
    inv_shift_rows(&mut ret);
    // println!("round[10].is_row   {:x}", ret);
    inv_sub_bytes(&mut ret);
    // println!("round[10].is_box   {:x}", ret);
    add_round_key(&mut ret, &keys[0..4]);
    // println!("round[10].ik_sch   {:x}", Bytes::as_one(&keys[0..4]));
    // println!("round[10].ioutput  {:x}", ret);
    ret
}

#[rustfmt::skip]
static S_BOX_VALS: [[u8;16];16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,],
];

#[rustfmt::skip]
static S_BOX_INV: [[u8;16];16] = [
    [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d],
];

// Sbox classifications:
//
// & -> non linear
// | -> non linear
// [a, b] -> [ a &  b]
// [a, b] -> [!a &  b]
// [a, b] -> [ a & !b]
// [a, b] -> [!a & !b]
// [a, b] -> [ a |  b]
// [a, b] -> [!a |  b]
// [a, b] -> [ a | !b]
// [a, b] -> [!a | !b]

/**
 * preforms the s_box transform
 */
fn s_box(byte: u8) -> u8 {
    S_BOX_VALS[((byte & 0xF0) >> 4) as usize][(byte & 0xF) as usize]
}
/**
 * preforms the inverse s_box transform
 */
fn inv_s_box(byte: u8) -> u8 {
    S_BOX_INV[((byte & 0xF0) >> 4) as usize][(byte & 0xF) as usize]
}

/**
 * preforms the s_box transform on each byte of a word
 */
fn sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        s_box(word[0]),
        s_box(word[1]),
        s_box(word[2]),
        s_box(word[3]),
    ]
}

/**
 * preforms the s_box transform on each byte of a word
 */
#[allow(dead_code)]
fn inv_sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        inv_s_box(word[0]),
        inv_s_box(word[1]),
        inv_s_box(word[2]),
        inv_s_box(word[3]),
    ]
}

/**
 * Preforms the s_box transform on each byte
 */
fn sub_bytes(block: &mut Bytes) {
    for byte in block.iter_mut() {
        *byte = s_box(*byte);
    }
}

/**
 * Preforms the s_box transform on each byte
 */
fn inv_sub_bytes(block: &mut Bytes) {
    for byte in block.iter_mut() {
        *byte = inv_s_box(*byte);
    }
}

/**
 * Shift the rows sequentially, 0 by 0, 1 by 1, etc
 */
fn shift_rows(block: &mut Bytes) {
    let tmp = block[1]; // first byte, second row
    block[1] = block[5];
    block[5] = block[9];
    block[9] = block[13];
    block[13] = tmp;

    let tmp = block[2]; // second byte, third row
    block[2] = block[10]; //  2  6 10 14
    block[10] = tmp; // 10 14  2  6
    let tmp = block[14];
    block[14] = block[6];
    block[6] = tmp;

    let tmp = block[3]; // third byte, fourth row
    block[3] = block[15]; // 3 7 11 15
    block[15] = block[11];
    block[11] = block[7];
    block[7] = tmp;
}

/**
 * Shift the rows sequentially, 0 by 0, 1 by 1, etc
 */
fn inv_shift_rows(block: &mut Bytes) {
    let tmp = block[3]; // first byte, second row
    block[3] = block[7]; // 1 5  9 13
    block[7] = block[11]; // 3 7 11 15
    block[11] = block[15];
    block[15] = tmp;

    let tmp = block[2]; // second byte, third row
    block[2] = block[10]; //  2  6 10 14
    block[10] = tmp; // 10 14  2  6
    let tmp = block[14];
    block[14] = block[6];
    block[6] = tmp;

    let tmp = block[1]; // third byte, fourth row
    block[1] = block[13]; // 3 7 11 15
    block[13] = block[9]; // 1 5  9 13
    block[9] = block[5];
    block[5] = tmp;
}

/**
 * mix the columns
 */
fn mix_columns(block: &mut Bytes) {
    mix_col(&mut block[0..4]);
    mix_col(&mut block[4..8]);
    mix_col(&mut block[8..12]);
    mix_col(&mut block[12..16]);
}

/**
 * mix the columns
 */
fn inv_mix_columns(block: &mut Bytes) {
    inv_mix_col(&mut block[0..4]);
    inv_mix_col(&mut block[4..8]);
    inv_mix_col(&mut block[8..12]);
    inv_mix_col(&mut block[12..16]);
}

/**
 * Mixes a column of the state
 */
fn mix_col(col: &mut [u8]) {
    let tmp = [col[0], col[1], col[2], col[3]];
    col[0] = mul(0x02, tmp[0]) ^ mul(0x03, tmp[1]) ^ tmp[2] ^ tmp[3];
    col[1] = tmp[0] ^ mul(0x02, tmp[1]) ^ mul(0x03, tmp[2]) ^ tmp[3];
    col[2] = tmp[0] ^ tmp[1] ^ mul(0x02, tmp[2]) ^ mul(0x03, tmp[3]);
    col[3] = mul(0x03, tmp[0]) ^ tmp[1] ^ tmp[2] ^ mul(0x02, tmp[3]);
}

/**
 * Mixes a column of the state
 */
fn inv_mix_col(col: &mut [u8]) {
    let tmp = [col[0], col[1], col[2], col[3]];
    col[0] = mul(0x0e, tmp[0]) ^ mul(0x0b, tmp[1]) ^ mul(0x0d, tmp[2]) ^ mul(0x09, tmp[3]);
    col[1] = mul(0x09, tmp[0]) ^ mul(0x0e, tmp[1]) ^ mul(0x0b, tmp[2]) ^ mul(0x0d, tmp[3]);
    col[2] = mul(0x0d, tmp[0]) ^ mul(0x09, tmp[1]) ^ mul(0x0e, tmp[2]) ^ mul(0x0b, tmp[3]);
    col[3] = mul(0x0b, tmp[0]) ^ mul(0x0d, tmp[1]) ^ mul(0x09, tmp[2]) ^ mul(0x0e, tmp[3]);
}

/**
 * Preforms a finite field multiplication; in the ch2, 256 field
 */
fn mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        b >>= 1;
        if a & 0x80 == 0x80 {
            a <<= 1;
            a ^= 0x1b;
        } else {
            a <<= 1;
        }
    }
    p
}

/**
 * Adds the round key to the state (xor, but making sure that they line up)
 */
fn add_round_key(block: &mut Bytes, key: &[Bytes]) {
    block[0] ^= key[0][0];
    block[4] ^= key[1][0];
    block[8] ^= key[2][0];
    block[12] ^= key[3][0];
    block[1] ^= key[0][1];
    block[5] ^= key[1][1];
    block[9] ^= key[2][1];
    block[13] ^= key[3][1];
    block[2] ^= key[0][2];
    block[6] ^= key[1][2];
    block[10] ^= key[2][2];
    block[14] ^= key[3][2];
    block[3] ^= key[0][3];
    block[7] ^= key[1][3];
    block[11] ^= key[2][3];
    block[15] ^= key[3][3];
}

/**
 * Rotate a word, i.e.: [a,b,c,d] => [b,c,d,a]
 */
fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

/**
 * Gets a [u8;4] out of a Bytes, starting at i
 */
fn get_word(vec: &Bytes, i: usize) -> [u8; 4] {
    [vec[i * 4], vec[i * 4 + 1], vec[i * 4 + 2], vec[i * 4 + 3]]
}

/**
 * XOR two 4 byte arrays, by each element
 */
fn xor(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

/**
 * RCON arr for key expansion (Pre computed, only kept the first bytes, the rest are zero anyway)
 */
const RCON_2: [u8; 11] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

/**
 * Expands key from 128 bit key into each round key
 *
 * returns a vector of them
 */
fn expand_key(key: Bytes) -> Vec<Bytes> {
    let mut ret = Bytes::zero(16 * 11);
    ret = ret.replace(&key, 0);
    let mut i = 4;
    while i < 44 {
        let mut temp = get_word(&ret, i - 1);
        if i % 4 == 0 {
            temp = sub_word(rot_word(temp));
            temp[0] ^= RCON_2[i / 4];
        } else if 4 > 6 && i % 4 == 4 {
            temp = sub_word(temp);
        }
        ret = ret.replace(&xor(get_word(&ret, i - 4), temp), i * 4);
        i = i + 1;
    }
    ret.split(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn test() {
        expand_key(Bytes::read_hex("2b7e151628aed2a6abf7158809cf4f3c"));
        println!(
            "{:X}",
            aes_block_encrypt(
                Bytes::read_hex("3243f6a8885a308d313198a2e0370734"),
                Bytes::read_hex("2b7e151628aed2a6abf7158809cf4f3c")
            )
        );
    }
    #[test]
    fn s_box_test() {
        assert_eq!(s_box(0x53), 0xed);
    }
    #[test]
    fn mul_test() {
        assert_eq!(mul(0x53, 0xCA), 0x01);
    }
    #[test]
    fn aes_block_test() {
        assert_eq!(
            aes_block_encrypt(
                Bytes::read_hex("00112233445566778899aabbccddeeff"),
                Bytes::read_hex("000102030405060708090a0b0c0d0e0f")
            ),
            Bytes::read_hex("69c4e0d86a7b0430d8cdb78070b4c55a")
        );
    }
    #[test]
    fn aes_block_inv_test() {
        assert_eq!(
            aes_block_decrypt(
                Bytes::read_hex("69c4e0d86a7b0430d8cdb78070b4c55a"),
                Bytes::read_hex("000102030405060708090a0b0c0d0e0f")
            ),
            Bytes::read_hex("00112233445566778899aabbccddeeff")
        );
    }
}
