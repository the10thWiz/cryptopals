
use crate::data::Bytes;
use crate::oracle::Oracle;
use crate::keys;
use crate::open_ssl::BLOCK_SIZE;

/**
 * Uses the provided Iterator to generate and test every possible key, and returns the value after xor (^)
 * that maximizes the score function provided
*/
pub fn decrypt_xor(data:Bytes, key:impl Iterator<Item = Bytes>, score:fn(&str)->isize) -> (String, Bytes, isize) {
    let mut max = isize::max_value();
    let mut k:Bytes = Bytes::zero(1);
    let mut best = String::default();
    for b in key {
        let tmp = (data.clone() ^ b.clone()).to_utf8();
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

pub fn decrypt_byte(oracle : &impl Oracle, known : &Bytes, ignore_blocks : usize) -> Bytes {
    let pre = Bytes::read_utf8("a")*(BLOCK_SIZE-1 - known.len()%BLOCK_SIZE);
    let known_size = known.len()/BLOCK_SIZE + ignore_blocks;
    let enc = oracle.encrypt(pre.clone()).truncate_start(known_size*BLOCK_SIZE).truncate(BLOCK_SIZE);
    for k in keys::KeyGen::new(1) {
        if enc == oracle.encrypt(pre.clone()+known.clone()+k.clone()).truncate_start(known_size*BLOCK_SIZE).truncate(BLOCK_SIZE) {
            return k;
        }
    }
    Bytes::zero(1)
}
pub fn decrypt_byte_2(oracle : &impl Oracle, known : &Bytes, prefix_size : usize) -> Bytes {
    // create prefix handler
    let pre = Bytes::read_utf8("a") * (BLOCK_SIZE - prefix_size%BLOCK_SIZE);
    let ignored = pre.len() + prefix_size;
    let cont = Bytes::read_utf8("a") * (BLOCK_SIZE-1 - known.len()%BLOCK_SIZE);
    let known_size = (known.len()/BLOCK_SIZE)*BLOCK_SIZE;

    let enc = oracle.encrypt(pre.clone() + cont.clone()).truncate_start(known_size + ignored).truncate(BLOCK_SIZE);
    for k in keys::KeyGen::new(1) {
        if enc == oracle.encrypt(pre.clone() + cont.clone()+known.clone()+k.clone()).truncate_start(known_size + ignored).truncate(BLOCK_SIZE) {
            return k;
        }
    }

    Bytes::zero(1)
}

