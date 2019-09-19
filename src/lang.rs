
//etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789
//etaoin srhldcumfpgwybvkxjqzETAOINSRHLDCUMFPGWYBVKXJQZ0123456789.?!
const ORDER : &str = "etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789";

/**
 * Makes a simple guess at whether the string provided is English, using etaoin order
 */
pub fn score_string(s:&str) -> isize {
    let mut score :isize = 0;
    for c in s.chars() {
        score+= match ORDER.find(c) {
            None => 255,
            Some(i) => i as isize
        }
    }
    score
}

/**
 * Calculates the hamming_dist for `s1` and `s2`
 * 
 * Hamming distance is the number of different bits
 */
pub fn hamming_dist(s1:&str, s2:&str) -> usize {
    let mut dist = 0usize;
    for cs in s1.bytes().zip(s2.bytes()) {
        for i in 0..8 {
            let tmp = 1u8 << i;
            if cs.0&tmp != cs.1&tmp {
                dist+= 1;
            }
        }
    }
    dist
}
