use crate::data::Bytes;
use crate::keys;
use crate::open_ssl::BLOCK_SIZE;
use crate::oracle::{CBCPaddingOracle, Oracle};

/**
 * Uses the provided Iterator to generate and test every possible key, and returns the value after xor (^)
 * that minimizes the score function provided
*/
pub fn decrypt_xor(
    data: Bytes,
    key: impl Iterator<Item = Bytes>,
    score: fn(&str) -> isize,
) -> (Bytes, Bytes, isize) {
    let mut min = isize::max_value();
    let mut k: Bytes = Bytes::zero(1);
    let mut best = Bytes::zero(0);
    for b in key {
        let tmp = data.clone() ^ b.clone();
        let tmp_s = score(&tmp.to_utf8()[..]);
        if tmp_s < min {
            min = tmp_s;
            best = tmp;
            k = b;
        } else if tmp_s == min {
        }
    }
    (best, k, min)
}

/**
 * Counts the number of repeated data buffers in `data`
*/
pub fn count_repeats(data: Vec<Bytes>) -> usize {
    let mut repeats = 0;
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if data[i] == data[j] {
                repeats += 1;
                break;
            }
        }
    }
    repeats
}

/**
 * Decrpyts a single byte of AES ECB using the provided oracle, and the known data
 */
pub fn decrypt_byte(oracle: &impl Oracle, known: &Bytes, ignore_blocks: usize) -> Bytes {
    let pre = Bytes::read_utf8("a") * (BLOCK_SIZE - 1 - known.len() % BLOCK_SIZE);
    let known_size = known.len() / BLOCK_SIZE + ignore_blocks;
    let enc = oracle
        .encrypt(pre.clone())
        .truncate_start(known_size * BLOCK_SIZE)
        .truncate(BLOCK_SIZE);
    for k in keys::KeyGen::new(1) {
        if enc
            == oracle
                .encrypt(pre.clone() + known.clone() + k.clone())
                .truncate_start(known_size * BLOCK_SIZE)
                .truncate(BLOCK_SIZE)
        {
            return k;
        }
    }
    Bytes::zero(1)
}
/**
 * Alternate decrypt byte for 2.17
 *
 * TODO: Merge back into `decrypt_byte`
 */
pub fn decrypt_byte_2(oracle: &impl Oracle, known: &Bytes, prefix_size: usize) -> Bytes {
    // create prefix handler
    let pre = Bytes::read_utf8("a") * (BLOCK_SIZE - prefix_size % BLOCK_SIZE);
    let ignored = pre.len() + prefix_size;
    let cont = Bytes::read_utf8("a") * (BLOCK_SIZE - 1 - known.len() % BLOCK_SIZE);
    let known_size = (known.len() / BLOCK_SIZE) * BLOCK_SIZE;

    let enc = oracle
        .encrypt(pre.clone() + cont.clone())
        .truncate_start(known_size + ignored)
        .truncate(BLOCK_SIZE);
    for k in keys::KeyGen::new(1) {
        if enc
            == oracle
                .encrypt(pre.clone() + cont.clone() + known.clone() + k.clone())
                .truncate_start(known_size + ignored)
                .truncate(BLOCK_SIZE)
        {
            return k;
        }
    }

    Bytes::zero(1)
}

/**
 *
 */
pub fn attack_byte_padding(iv: Bytes, block: Bytes, oracle: &CBCPaddingOracle) -> Bytes {
    let mut known = Bytes::zero(16);
    for i in 1..17 {
        for k in keys::KeyGen::new(1) {
            let mut edited = iv.clone();
            if i < 16 {
                edited[15 - i] = 0xFFu8;
            }
            edited[16 - i] = k[0];
            for j in 1..i {
                edited[16 - j] = (known[16 - j] ^ i as u8) ^ iv[16 - j];
            }
            // println!("{:16X}", enc.1.clone() ^ edited.clone());
            if oracle.check_padding((edited, block.clone())) {
                // println!("{:02X}", (enc.1[enc.1.len() - 17] ^ k[0]) ^ i as u8);
                // oracle.print_raw((enc.0.clone(), edited));
                known[16 - i] = (iv[16 - i] ^ k[0]) ^ i as u8;
                break;
            }
        }
    }
    known
}
