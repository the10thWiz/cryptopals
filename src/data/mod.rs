mod ops;
mod display;
mod conversion;
use crate::cipher::BLOCK_SIZE;
use rand::prelude::random;
/*
    data.rs 1013 lines before I split it up into the 4 files that exist now. I expect
    to work on some better comments/docs, as well as adding new functionality as needed.

    TODO: Write better Documentation
*/

/**
 * A Standard buffer for binary data. This struct offers a variety of
 * convience methods for manipulating the data buffered inside.
 *
 * The following operator overloads are defined:
 *
 * Binary xor => xors raw data, byte for byte. right will be repeated
 * if it is shorter
 *
 * Add => Concatinates buffers
 *
 * Index => extracts raw data at index (u8), or slice(&[u8])
 *
 * Multiply => repeats buffer x number of times
 *
 * Equals/Compare => compares raw data in buffers
 *
 * Display and Debug => Defualts to printing as utf-8, UpperHex is also
 * implemented
 */
#[derive(Clone, Default)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn new() -> Bytes {
        Self {bytes: Vec::new()}
    }
    pub fn with_capacity(size: usize) -> Bytes {
        Self {bytes: Vec::with_capacity(size)}
    }
    pub fn empty() -> Bytes {
        Self {bytes: Vec::default()}
    }
    /**
     * Reads raw u8 values from `v`
     */
    pub fn from_vec(v: Vec<u8>) -> Bytes {
        Bytes { bytes: v }
    }
    /**
     * Reads raw u8 values from `v`
     */
    pub fn from_bytes(v: &[u8]) -> Bytes {
        let mut ret = Bytes {
            bytes: Vec::with_capacity(v.len()),
        };
        for val in v {
            ret.bytes.push(*val);
        }
        ret
    }
    /**
     * Creates a new buffer of length `size`, filled with
     * binary zeros
     */
    pub fn zero(size: usize) -> Bytes {
        let mut ret = Vec::new();
        for _ in 0..size {
            ret.push(0u8);
        }
        Bytes { bytes: ret }
    }
    /**
     * Generates a new buffer of length `size`, filled
     * with random data
     */
    pub fn rand(size: usize) -> Bytes {
        let mut ret = Vec::new();
        for _ in 0..size {
            ret.push(random());
        }
        Bytes { bytes: ret }
    }
    /**
     * Returns raw data buffer as a `&[u8]`
     */
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /**
     * Creates a `Vec` of size `block`, and creates a Bytes for each `block`
     *
     * e.g. [1F 22 33 44].collate(2) -> [[1F 33], [22 44]]
     */
    pub fn collate(&self, block: usize) -> Vec<Bytes> {
        let mut ret = Vec::new();
        for _ in 0..block {
            ret.push(Bytes { bytes: Vec::new() });
        }
        for b in self.bytes.iter().enumerate() {
            ret[b.0 % block].bytes.push(*b.1);
        }
        ret
    }
    /**
     * Undoes collate method
     * 
     * `Bytes::decollate(a.collate(x)) == a`, for any x
     */
    pub fn decollate(parts: Vec<Bytes>) -> Bytes {
        let mut ret = Vec::new();
        for i in 0..parts.first().unwrap().len() {
            for p in parts.iter() {
                match p.bytes.get(i) {
                    Some(v) => ret.push(*v),
                    None => (),
                }
            }
        }

        Bytes { bytes: ret }
    }
    /**
     * Splits raw data buffer into `Bytes` of size
     * block
     */
    pub fn split(&self, len: usize) -> Vec<Bytes> {
        let mut ret: Vec<Bytes> = Vec::new();
        for i in 0..self.bytes.len() / len {
            let mut cur = Bytes { bytes: Vec::new() };
            for j in i * len..(i + 1) * len {
                cur.bytes.push(self.bytes[j]);
            }
            ret.push(cur);
        }
        ret
    }
    /**
     * Pads data to a multiple of `padding`, using
     * PKCS#7
     */
    pub fn pad_pkcs7(&self, padding: usize) -> Bytes {
        let mut ret = self.clone();
        let num = padding - self.len() % padding;
        for _ in 0..num {
            ret.bytes.push(num as u8);
        }
        ret
    }
    /**
     * Trims PKCS#7 padding from bytes
     * 
     * Doesn't assume any padding has been applied, and
     * just returns itself if the last byte isn't a multiple
     * of the padding
     */
    pub fn trim_pkcs7(&self) -> Bytes {
        let pad_num = self.bytes[self.bytes.len() - 1] as usize;
        // println!("{}", pad_num);
        if pad_num <= BLOCK_SIZE {
            for b in self.bytes.len() - pad_num..self.bytes.len() {
                if self.bytes[b] != pad_num as u8 {
                    return self.clone();
                }
            }
            return self.clone().truncate(self.bytes.len() - pad_num);
        }
        self.clone()
    }
    /**
     * Truncates data to len bytes, discarding any data after
     * the cutoff
     */
    pub fn truncate(&self, len: usize) -> Bytes {
        let mut ret = self.clone();
        while ret.bytes.len() > len {
            ret.bytes.pop();
        }
        ret
    }
    /**
     * Discard first len bytes
     */
    pub fn truncate_start(&self, len: usize) -> Bytes {
        let mut ret = self.clone();
        for _ in 0..len {
            ret.bytes.remove(0);
        }
        ret
    }
    /**
     * replace bytes from `index` with `part`
     */
    pub fn replace(&self, part: &[u8], index: usize) -> Bytes {
        if part.len() + index > self.bytes.len() {
            panic!("Part is to long to fit");
        }
        let mut ret = self.bytes.clone();
        for i in 0..part.len() {
            ret[i + index] = part[i];
        }
        Bytes { bytes: ret }
    }
    /**
     * replace bytes from `block * open_ssl::BLOCK_SIZE` with `part`
     */
    pub fn replace_block(&self, part: &[u8], block: usize) -> Bytes {
        if part.len() + block * 16 > self.bytes.len() {
            panic!("Part is to long to fit");
        }
        let mut ret = self.bytes.clone();
        for i in 0..part.len() {
            ret[i + block * 16] = part[i];
        }
        Bytes { bytes: ret }
    }
    /**
     * Similar to `self[i]`, except the return type is `char`
     */
    pub fn get(&self, i: usize) -> char {
        return self.bytes[i] as char;
    }
    /**
     * Increments data from the right
     *
     * returns true if data rolls over from the max value
     */
    pub fn inc(&mut self) -> bool {
        for b in self.bytes.iter_mut().rev() {
            if *b == 255 {
                *b = 0;
            } else {
                *b += 1;
                return false;
            }
        }
        true
    }
    /**
     * Only returns true if data is entirely zeros
     */
    pub fn is_zero(&self) -> bool {
        for b in self.bytes.iter() {
            if *b != 0 {
                return false;
            }
        }
        true
    }
    /**
     * Length of buffer, in bytes
     */
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    /**
     * Creates a new buffer with only values that don't match
     * `val`
     */
    pub fn remove(&self, val: u8) -> Bytes {
        let mut ret = Vec::new();
        for b in self.bytes.iter() {
            if *b != val {
                ret.push(*b);
            }
        }
        Bytes { bytes: ret }
    }

    pub fn pivot(v: Vec<Bytes>) -> Vec<Bytes> {
        let mut len = usize::max_value();
        for t in v.iter() {
            if t.len() < len {
                len = t.len();
            }
        }
        let mut ret = Vec::with_capacity(len);
        for i in 0..len {
            let mut b = Self::zero(v.len());
            for j in 0..v.len() {
                b[j] = v[j][i];
            }
            ret.push(b);
        }
        ret
    }
}

