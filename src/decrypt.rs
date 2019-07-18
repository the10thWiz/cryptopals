
use crate::data::Bytes;

/**
 * Uses the provided Iterator to generate and test every possible key, and returns the value after xor (^)
 * that maximizes the score function provided
*/
pub fn decrypt_xor(data:Bytes, key:impl Iterator<Item = Bytes>, score:fn(&str)->isize) -> (String, Bytes, isize) {
    let mut max = isize::max_value();
    let mut k:Bytes = Bytes::zero(1);
    let mut best = String::default();
    for b in key {
        let tmp = (data.clone() ^ b.clone()).to_ascii();
        let tmp_s = score(&tmp[..]);
        if tmp_s < max {
            max = tmp_s;
            best = tmp;
            k = b;
        }else if tmp_s == max {
        }
    }
    (best, k, max)
}

pub fn count_repeats(data:Vec<Bytes>) -> usize {
    let mut repeats = 0;
    for i in 0..data.len() {
        for j in (i+1)..data.len() {
            if data[i] == data[j] {
                repeats+= 1;
                break;
            }
        }
    }
    repeats
}
