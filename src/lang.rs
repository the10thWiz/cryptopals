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
const STD_FREQ: [f64; 26] = [
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
    for (i, (&ac, ex)) in count.iter().zip(STD_FREQ.iter()).enumerate() {
        let act = (ac as f64) / s.len() as f64;
        let mult = match ORDER
            .find(((i as u8 + 'a' as u8) as char).to_ascii_lowercase())
            .unwrap()
        {
            0..=0 => 1,
            _ => 0,
        };
        //println!(
        //"{}: {:05.2}% vs {:05.2}%, {}",
        //(i as u8 + 'a' as u8) as char,
        //act * 100.0,
        //ex,
        //mult
        //);
        diff += (act * 1000.0 - ex * 10.0).abs().round() * mult as f64;
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
