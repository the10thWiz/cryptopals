//etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789
//etaoin srhldcumfpgwybvkxjqzETAOINSRHLDCUMFPGWYBVKXJQZ0123456789.?!
const ORDER: &str = "etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789";

/**
 * Makes a simple guess at whether the string provided is English, using etaoin order
 */
pub fn score_string(s: &str) -> isize {
    let mut score: isize = 0;
    for c in s.chars() {
        score += match ORDER.find(c) {
            None => 255,
            Some(i) => i as isize,
        }
    }
    score
}

/**
 * Returns the number of "invalid" letters in a string - the valid
 * 
 * Valid = Ascii, alpha-numeric, or selected puncuation
 */
pub fn count_invalid_letters(s: &str) -> isize {
    let mut score = 0;
    for c in s.chars() {
        score += match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | ',' | '?' | '!' | '-' | '\'' | '"' | '/' => -1,
            _ => 1
        }
    }
    score
}

#[allow(dead_code)]
const STD_FREQ : [f64;26] = [
    08.167, // a
    01.492, // b
    02.782, // c
    04.253, // d
    12.702, // e
    02.228, // f
    02.015, // g
    06.094, // h
    06.966, // i
    00.153, // j
    00.772, // k
    04.025, // l
    02.406, // m
    06.749, // n
    07.507, // o
    01.929, // p
    00.095, // q
    05.987, // r
    06.327, // s
    09.056, // t
    02.758, // u
    00.978, // v
    02.360, // w
    00.150, // x
    01.974, // y
    00.074, // z
];

#[allow(dead_code)]
pub fn histogram(s: &str) -> isize {
    let mut count = [0usize; 26];
    for c in s.chars() {
        match c {
            'a'..='z' => count[c as usize - 'a' as usize]+= 1,
            'A'..='Z' => count[c as usize - 'A' as usize]+= 1,
            _ => ()
        }
    }

    0
}

/**
 * Calculates the hamming_dist for `s1` and `s2`
 *
 * Hamming distance is the number of different bits
 */
pub fn hamming_dist(s1: &str, s2: &str) -> usize {
    let mut dist = 0usize;
    for cs in s1.bytes().zip(s2.bytes()) {
        for i in 0..8 {
            let tmp = 1u8 << i;
            if cs.0 & tmp != cs.1 & tmp {
                dist += 1;
            }
        }
    }
    dist
}
