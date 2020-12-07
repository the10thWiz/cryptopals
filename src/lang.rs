//etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789
//etaoin srhldcumfpgwybvkxjqzETAOINSRHLDCUMFPGWYBVKXJQZ0123456789.?!
const ORDER: &str = "etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789";

/**
 * Makes a simple guess at whether the string provided is English, using etaoin order
 */
pub fn score_string(s: &str) -> f64 {
    let mut score: isize = 0;
    for c in s.chars() {
        score += match ORDER.find(c) {
            None => 255,
            Some(i) => i as isize,
        }
    }
    score as f64
}

/**
 * Returns the number of "invalid" letters in a string - the valid
 *
 * Valid = Ascii, alpha-numeric, or selected puncuation
 */
pub fn count_invalid_letters(s: &str) -> f64 {
    let mut score = 0;
    for c in s.chars() {
        score += match c {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | ' '
            | '.'
            | ','
            | '?'
            | '!'
            | '-'
            | '\''
            | '"'
            | '/' => -1,
            _ => 1,
        }
    }
    score as f64
}

#[allow(dead_code)]
/// (char, expected frequency, multiplier)
/// The multiplier can be used to weight
/// letters based on how close they should match
const STD_FREQ: [(char, f64, f64); 26] = [
    ('a', 08.167, 0.0),
    ('b', 01.492, 0.0),
    ('c', 02.782, 0.0),
    ('d', 04.253, 0.0),
    ('e', 12.702, 1.0),
    ('f', 02.228, 0.0),
    ('g', 02.015, 0.0),
    ('h', 06.094, 0.0),
    ('i', 06.966, 0.0),
    ('j', 00.153, 0.0),
    ('k', 00.772, 0.0),
    ('l', 04.025, 0.0),
    ('m', 02.406, 0.0),
    ('n', 06.749, 0.0),
    ('o', 07.507, 0.0),
    ('p', 01.929, 0.0),
    ('q', 00.095, 0.0),
    ('r', 05.987, 0.0),
    ('s', 06.327, 0.0),
    ('t', 09.056, 0.0),
    ('u', 02.758, 0.0),
    ('v', 00.978, 0.0),
    ('w', 02.360, 0.0),
    ('x', 00.150, 0.0),
    ('y', 01.974, 0.0),
    ('z', 00.074, 0.0),
];

#[allow(dead_code)]
pub fn histogram_score(s: &str) -> f64 {
    let mut count = [0usize; 26];
    for c in s.chars() {
        match c {
            'a'..='z' => count[c as usize - 'a' as usize] += 1,
            'A'..='Z' => count[c as usize - 'A' as usize] += 1,
            _ => (),
        }
    }
    let mut diff = 0f64;
    for (&ac, (_ch, ex, variance)) in count.iter().zip(STD_FREQ.iter()) {
        let act = (ac as f64) / s.len() as f64;
        diff += (act * 1000.0 - ex * 10.0).abs().round() * variance;
    }
    diff * 1f64
}

pub fn histogram(s: impl AsRef<str>) -> String {
    let mut count = [0usize; 26];
    for c in s.as_ref().chars() {
        match c {
            'a'..='z' => count[c as usize - 'a' as usize] += 1,
            'A'..='Z' => count[c as usize - 'A' as usize] += 1,
            _ => (),
        }
    }
    String::new()
}

pub fn mono(message: impl AsRef<str>, key: impl AsRef<str>) -> String {
    let key = key.as_ref();
    let mut ret = String::new();
    for ch in message.as_ref().chars() {
        if ch.is_ascii_alphabetic() {
            ret.push(
                key.chars()
                    .nth(ch.to_ascii_lowercase() as usize - 'a' as usize)
                    .unwrap(),
            );
        } else {
            ret.push(ch);
        }
    }
    ret
}

/**
 * Calculates the hamming_dist for `s1` and `s2`
 *
 * Hamming distance is the number of different bits
 */
pub fn hamming_dist(s1: impl AsRef<str>, s2: impl AsRef<str>) -> usize {
    let mut dist = 0usize;
    for cs in s1.as_ref().bytes().zip(s2.as_ref().bytes()) {
        for i in 0..8 {
            let tmp = 1u8 << i;
            if cs.0 & tmp != cs.1 & tmp {
                dist += 1;
            }
        }
    }
    dist
}
