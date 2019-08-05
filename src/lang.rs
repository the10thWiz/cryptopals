
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

#[allow(dead_code)]
struct Histogram {
    v :[(char, usize, f64); 62]
}
#[allow(dead_code)]
impl Histogram {
    pub fn create(s :&str) -> Histogram {
        let mut v :[(char, usize, f64); 62] = [('a', 0, 0f64); 62];
        for i in 0..26 {
            v[i+00].0 = (i as u8 + 'a' as u8) as char;
        }
        for i in 0..26 {
            v[i+26].0 = (i as u8 + 'A' as u8) as char;
        }
        for i in 0..10 {
            v[i+52].0 = (i as u8 + '0' as u8) as char;
        }
        for _c in s.chars() {
            
        }

        Histogram {v: v}
    }
}
#[allow(dead_code)]
fn test_histogram() {

}